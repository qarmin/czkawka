//! Rebuilding the container around an AVI whose header is gone.
//!
//! An AVI keeps everything that describes the recording - picture size, codec, frame rate, the
//! index - in the first couple of kilobytes, and the media data in one long chain of chunks
//! after it. Lose those kilobytes and ffmpeg cannot open the file at all, while every frame is
//! still sitting there untouched. Found in the wild on a DVR recording: the first 2048 bytes
//! zeroed, 46,415 HEVC frames intact behind them.
//!
//! The chunk chain is walked by [`super::avi_scan`], which recovers the frame boundaries the
//! camera recorded, and a fresh header is written around them: payloads are copied byte for
//! byte, so what comes out is the original content in a new wrapper rather than a re-encode.
//! Only the description has to be reconstructed, and most of it can be read back off the data -
//! the codec from the units a chunk opens with, the picture size by having ffprobe look at the
//! elementary stream. The frame rate cannot: nothing in the surviving bytes records it, which
//! is what `--reference` is for, and what the note on the report says when there isn't one.

use std::io::{BufWriter, Write};
use std::path::Path;

use super::avi_scan::{Chunk, Codec, ScanReport};
use super::ffmpeg::Stream;
use super::{avi, avi_scan, ffmpeg, riff};
use crate::file_fixer::candidate::{Candidate, Score};
use crate::file_fixer::engine::{RepairContext, Strategy};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::format::Format;
use crate::file_fixer::io::read_u32_le;
use crate::file_fixer::mode::Capabilities;

/// Chunks that have to be found before the file is believed to hold a recording at all.
const MIN_CHUNKS: usize = 8;

/// Chunks written out for the probe that reads the picture size back off the stream.
///
/// The parameter sets sit in the first chunk of a recording, so this only has to be large
/// enough to survive a scan that started partway in.
const PROBE_CHUNKS: usize = 64;

/// Frame rate assumed when nothing says what it was.
///
/// Wrong only in how fast the result plays: every frame is still there, in order. A reference
/// from the same camera replaces it with the real figure.
const DEFAULT_FRAME_RATE: f64 = 25.0;

/// Whether a frame rate is worth believing from a probe of a raw elementary stream.
fn plausible_rate(rate: f64) -> bool {
    (1.0..=240.0).contains(&rate)
}

/// Bytes of the `LIST`/`hdrl` chunk that describes the file, which is a fixed size here because
/// exactly one stream is ever written: the main header, one stream header, one stream format.
const HDRL_LEN: u64 = 8 + 4 + (8 + 56) + (8 + 4 + (8 + 56) + (8 + 40));

/// Bytes of a `LIST` or chunk header.
const CHUNK_HEADER: u64 = 8;

/// One `idx1` entry: chunk code, flags, offset, length.
const INDEX_ENTRY: u64 = 16;

const AVIF_HASINDEX: u32 = 0x10;
const AVIF_TRUSTCKTYPE: u32 = 0x800;
const AVIIF_KEYFRAME: u32 = 0x10;

/// Rebuild the container for an AVI whose header did not survive.
///
/// Needs `guess` because the header is written from scratch rather than recovered, and
/// `truncate` because a chain that breaks partway leaves the rest of the file behind.
pub struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "avi_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if !ffmpeg::is_available() {
            return Err(FixerError::FfmpegMissing);
        }
        if header_is_intact(ctx.data) {
            return Ok(None);
        }

        let scan = avi_scan::scan(ctx.data);
        if scan.video_chunks() < MIN_CHUNKS {
            return Ok(None);
        }
        let Some(format) = describe(ctx, &scan)? else {
            return Ok(None);
        };
        log::debug!(
            "{}: {} chunk(s) from {} to {}{}, {} resynchronization(s), {}x{} {} at {} fps",
            ctx.path.display(),
            scan.chunks.len(),
            scan.start,
            scan.stopped_at,
            if scan.ends_cleanly { "" } else { " (cut short)" },
            scan.resynchronizations,
            format.width,
            format.height,
            String::from_utf8_lossy(&format.fourcc),
            format.frame_rate
        );

        // Only the video is carried over. A stream header has to state a sample rate, a channel
        // count and a codec, and nothing in a bare run of audio chunks says what those were -
        // the same reason `mp4_rebuild` recovers picture without sound.
        let video: Vec<Chunk> = scan.chunks.iter().filter(|chunk| chunk.is_video()).cloned().collect();
        let output = avi::temp_output()?;
        let written = write_avi(ctx.data, &video, &format, output.path())?;

        let decoded = ffmpeg::decode_report(output.path())?;
        if decoded.frames == 0 {
            return Ok(None);
        }
        if video.len() < scan.chunks.len() {
            ctx.note(
                "the rebuilt file holds the picture only: an audio stream cannot be described \
                      without the header that said what it was"
                    .to_string(),
            );
        }
        if format.rate_is_assumed {
            ctx.note(format!(
                "nothing in the surviving data records the frame rate; the rebuilt file assumes \
                 {DEFAULT_FRAME_RATE} fps, so it plays at the wrong speed if the camera used another - \
                 pass --reference with a healthy recording from the same camera to take the real one"
            ));
        }

        let complete = scan.covers_everything() && written == video.len();
        let fidelity = avi::fidelity_of(&decoded);
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: crate::file_fixer::candidate::Payload::TempFile(output.into_temp_path()),
            score: Score {
                content_recovered: decoded.frames,
                // No surviving field says how long the recording was, so "everything" can only
                // mean the chain accounted for every byte of the file and every chunk of it was
                // written out. Anything less leaves the total genuinely unknown.
                content_total: complete.then_some(decoded.frames),
                decoder_errors: decoded.errors,
                decodes_cleanly: decoded.is_clean(),
                streams_complete: complete,
            },
            fidelity,
            reconstructed: avi::reconstructed_span(fidelity, decoded.frames),
        }))
    }
}

/// Whether the file still has the header this strategy would otherwise rebuild.
fn header_is_intact(data: &[u8]) -> bool {
    if crate::file_fixer::format::detect_by_magic(data) != Some(Format::Avi) {
        return false;
    }
    riff::parse(data).is_some_and(|riff| riff.chunks.iter().any(|chunk| chunk.is(*b"LIST") && form_of(data, chunk) == Some(*b"hdrl")))
}

fn form_of(data: &[u8], chunk: &riff::Chunk) -> Option<[u8; 4]> {
    data.get(chunk.payload.start..chunk.payload.start + 4)?.try_into().ok()
}

/// What the rebuilt header has to state about the stream.
#[derive(Debug, Clone, PartialEq)]
struct StreamFormat {
    fourcc: [u8; 4],
    width: u32,
    height: u32,
    frame_rate: f64,
    /// True when nothing pinned the frame rate down and the default was used.
    rate_is_assumed: bool,
}

/// Works out what to say about the stream, from the data first and a reference second.
///
/// The picture size comes from the stream itself: the parameter sets the camera embedded are
/// right there in the first chunk, and ffprobe reads them without being told anything. A
/// reference is only consulted for the frame rate, which no surviving byte records, and for the
/// four-character code, where the camera's own spelling is worth more than a guess at it.
fn describe(ctx: &RepairContext<'_>, scan: &ScanReport) -> Result<Option<StreamFormat>> {
    let Some(codec) = scan.codec else {
        return Ok(None);
    };
    let probed = probe_stream(ctx.data, scan, codec)?;
    let reference = references(ctx).into_iter().find(|found| codec.names(found.fourcc));

    let width = probed.as_ref().and_then(|stream| stream.width);
    let height = probed.as_ref().and_then(|stream| stream.height);
    let (Some(width), Some(height)) = (
        width.or(reference.as_ref().map(|found| found.width)),
        height.or(reference.as_ref().map(|found| found.height)),
    ) else {
        log::debug!("{}: nothing describes the picture size", ctx.path.display());
        return Ok(None);
    };

    Ok(Some(match reference {
        Some(reference) => StreamFormat {
            fourcc: reference.fourcc,
            width,
            height,
            frame_rate: reference.frame_rate,
            rate_is_assumed: false,
        },
        None => StreamFormat {
            fourcc: codec.fourcc(),
            width,
            height,
            // A raw elementary stream only carries timing when the encoder wrote it into the
            // parameter sets, and ffprobe answers with its own default when it did not. There
            // is no way to tell those apart from here, so either way this counts as assumed.
            frame_rate: probed
                .and_then(|stream| stream.frame_rate)
                .filter(|rate| plausible_rate(*rate))
                .unwrap_or(DEFAULT_FRAME_RATE),
            rate_is_assumed: true,
        },
    }))
}

/// Hands ffprobe the opening frames as a bare elementary stream and asks what they are.
fn probe_stream(data: &[u8], scan: &ScanReport, codec: Codec) -> Result<Option<Stream>> {
    let file = tempfile::Builder::new()
        .prefix("file_fixer-")
        .suffix(&format!(".{}", codec.extension()))
        .tempfile()
        .map_err(|error| FixerError::write("<temp>", error))?;

    let mut writer = BufWriter::new(&file);
    for chunk in scan.chunks.iter().filter(|chunk| chunk.is_video()).take(PROBE_CHUNKS) {
        if let Some(payload) = data.get(chunk.payload.clone()) {
            writer.write_all(payload).map_err(|error| FixerError::write("<temp>", error))?;
        }
    }
    writer.flush().map_err(|error| FixerError::write("<temp>", error))?;
    drop(writer);

    // A refusal here is this file not being readable as the codec it looked like, not a failure
    // of the repair: the caller falls back to whatever a reference says.
    Ok(ffmpeg::probe(file.path())
        .inspect_err(|error| log::debug!("the recovered stream did not probe: {error}"))
        .ok()
        .and_then(|probe| probe.video().cloned()))
}

/// Stream descriptions read out of the healthy files the user offered as references.
fn references(ctx: &RepairContext<'_>) -> Vec<StreamFormat> {
    ctx.references
        .iter()
        .filter_map(|path| match std::fs::read(path) {
            Ok(data) => from_reference(&data),
            Err(error) => {
                log::warn!("cannot read reference {}: {error}", path.display());
                None
            }
        })
        .collect()
}

/// Reads the stream description out of a healthy AVI.
fn from_reference(data: &[u8]) -> Option<StreamFormat> {
    let riff = riff::parse(data)?;
    if riff.form != *b"AVI " && riff.form != *b"AVIX" {
        return None;
    }
    let hdrl = riff.chunks.iter().find(|chunk| chunk.is(*b"LIST") && form_of(data, chunk) == Some(*b"hdrl"))?;

    let strl = riff::chunks_in(data, hdrl.payload.start + 4..hdrl.payload.end)
        .into_iter()
        .find(|chunk| chunk.is(*b"LIST") && form_of(data, chunk) == Some(*b"strl"))?;
    let stream = riff::chunks_in(data, strl.payload.start + 4..strl.payload.end);
    let header = stream.iter().find(|chunk| chunk.is(*b"strh"))?;
    let format = stream.iter().find(|chunk| chunk.is(*b"strf"))?;

    let at = header.payload.start;
    if data.get(at..at + 4)? != b"vids" {
        return None;
    }
    let scale = read_u32_le(data, at + 20)?;
    let rate = read_u32_le(data, at + 24)?;

    Some(StreamFormat {
        fourcc: data.get(at + 4..at + 8)?.try_into().ok()?,
        width: read_u32_le(data, format.payload.start + 4)?,
        height: read_u32_le(data, format.payload.start + 8)?,
        frame_rate: match scale {
            0 => return None,
            scale => f64::from(rate) / f64::from(scale),
        },
        rate_is_assumed: false,
    })
}

/// The chunk code as the rebuilt file spells it.
///
/// The number in a chunk code says which stream it belongs to, and the rebuilt file declares
/// exactly one. A camera that put its picture on stream one would otherwise hand every chunk to
/// a stream the header never described, and ffmpeg would find no video at all.
fn code_of(chunk: &Chunk) -> [u8; 4] {
    let mut code = chunk.id;
    if let Some(stream) = code.get_mut(..2) {
        stream.copy_from_slice(b"00");
    }
    code
}

/// Bytes a chunk occupies: its header, its payload and RIFF's pad byte for an odd length.
fn chunk_span(payload: usize) -> u64 {
    CHUNK_HEADER + payload as u64 + (payload % 2) as u64
}

/// How much of the recording fits in a rebuilt file, and what its fixed fields must say.
///
/// Every size in an AVI is a `u32`, so a recording longer than four gigabytes cannot be
/// described by one at all - that needs the OpenDML extension, which is a different container
/// shape. Rather than refusing such a file outright, as much of it as the format can hold is
/// written and the result is reported as incomplete.
struct Layout {
    chunks: usize,
    movi_payload: u64,
    largest_chunk: u32,
}

fn plan(chunks: &[Chunk]) -> Layout {
    let mut layout = Layout {
        chunks: 0,
        movi_payload: 4,
        largest_chunk: 0,
    };
    for chunk in chunks {
        let index = INDEX_ENTRY * (layout.chunks as u64 + 1);
        let movi = layout.movi_payload + chunk_span(chunk.payload.len());
        let riff = 4 + HDRL_LEN + CHUNK_HEADER + movi + CHUNK_HEADER + index;
        if riff > u64::from(u32::MAX) {
            break;
        }
        layout.chunks += 1;
        layout.movi_payload = movi;
        layout.largest_chunk = layout.largest_chunk.max(chunk.payload.len().try_into().unwrap_or(u32::MAX));
    }
    layout
}

/// Writes the rebuilt file and returns how many chunks it holds.
fn write_avi(data: &[u8], chunks: &[Chunk], format: &StreamFormat, path: &Path) -> Result<usize> {
    let layout = plan(chunks);
    let kept = chunks.get(..layout.chunks).unwrap_or(chunks);
    let index_payload = INDEX_ENTRY * kept.len() as u64;
    let riff_payload = 4 + HDRL_LEN + CHUNK_HEADER + layout.movi_payload + CHUNK_HEADER + index_payload;

    let file = std::fs::File::create(path).map_err(|error| FixerError::write(path, error))?;
    let mut out = BufWriter::new(file);
    let write = |out: &mut BufWriter<std::fs::File>, bytes: &[u8]| out.write_all(bytes).map_err(|error| FixerError::write(path, error));

    write(&mut out, b"RIFF")?;
    write(&mut out, &(riff_payload as u32).to_le_bytes())?;
    write(&mut out, b"AVI ")?;
    write(&mut out, &headers(format, &layout, kept.len()))?;
    write(&mut out, b"LIST")?;
    write(&mut out, &(layout.movi_payload as u32).to_le_bytes())?;
    write(&mut out, b"movi")?;

    for chunk in kept {
        let payload = data.get(chunk.payload.clone()).unwrap_or_default();
        write(&mut out, &code_of(chunk))?;
        write(&mut out, &(payload.len() as u32).to_le_bytes())?;
        write(&mut out, payload)?;
        if payload.len() % 2 == 1 {
            write(&mut out, &[0])?;
        }
    }

    write(&mut out, b"idx1")?;
    write(&mut out, &(index_payload as u32).to_le_bytes())?;
    // Offsets are relative to the `movi` code itself, so the first chunk sits at four. Recomputed
    // here rather than collected during the copy above: a recording can hold millions of chunks,
    // and holding the whole index in memory to write it out afterwards is a pointless cost.
    let mut offset = 4u64;
    for chunk in kept {
        let flags = if chunk.keyframe { AVIIF_KEYFRAME } else { 0 };
        write(&mut out, &code_of(chunk))?;
        write(&mut out, &flags.to_le_bytes())?;
        write(&mut out, &(offset as u32).to_le_bytes())?;
        write(&mut out, &(chunk.payload.len() as u32).to_le_bytes())?;
        offset += chunk_span(chunk.payload.len());
    }

    out.flush().map_err(|error| FixerError::write(path, error))?;
    Ok(kept.len())
}

/// The `LIST`/`hdrl` chunk: what the file is, and what its one stream is.
fn headers(format: &StreamFormat, layout: &Layout, frames: usize) -> Vec<u8> {
    let frames = u32::try_from(frames).unwrap_or(u32::MAX);
    let mut strl = b"strl".to_vec();
    chunk(&mut strl, *b"strh", &stream_header(format, layout, frames));
    chunk(&mut strl, *b"strf", &stream_format(format));

    let mut hdrl = b"hdrl".to_vec();
    chunk(&mut hdrl, *b"avih", &main_header(format, layout, frames));
    chunk(&mut hdrl, *b"LIST", &strl);

    let mut out = Vec::with_capacity(HDRL_LEN as usize);
    chunk(&mut out, *b"LIST", &hdrl);
    out
}

fn chunk(out: &mut Vec<u8>, id: [u8; 4], payload: &[u8]) {
    out.extend_from_slice(&id);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
}

fn fields(out: &mut Vec<u8>, values: &[u32]) {
    for value in values {
        out.extend_from_slice(&value.to_le_bytes());
    }
}

fn main_header(format: &StreamFormat, layout: &Layout, frames: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(56);
    fields(
        &mut out,
        &[
            (1_000_000.0 / format.frame_rate).round() as u32,
            0, // dwMaxBytesPerSec, advisory and ignored by every reader worth the name
            0, // dwPaddingGranularity
            AVIF_HASINDEX | AVIF_TRUSTCKTYPE,
            frames,
            0, // dwInitialFrames
            1, // dwStreams
            layout.largest_chunk,
            format.width,
            format.height,
            0,
            0,
            0,
            0,
        ],
    );
    out
}

fn stream_header(format: &StreamFormat, layout: &Layout, frames: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(56);
    out.extend_from_slice(b"vids");
    out.extend_from_slice(&format.fourcc);
    fields(
        &mut out,
        &[
            0, // dwFlags
            0, // wPriority and wLanguage
            0, // dwInitialFrames
            1000,
            (format.frame_rate * 1000.0).round() as u32,
            0, // dwStart
            frames,
            layout.largest_chunk,
            u32::MAX, // dwQuality: leave it to the decoder
            0,        // dwSampleSize: frames vary in size
        ],
    );
    for edge in [0, 0, format.width, format.height] {
        out.extend_from_slice(&i16::try_from(edge).unwrap_or(i16::MAX).to_le_bytes());
    }
    out
}

/// A `BITMAPINFOHEADER`, which is how AVI describes a video stream whatever the codec.
fn stream_format(format: &StreamFormat) -> Vec<u8> {
    let mut out = Vec::with_capacity(40);
    fields(&mut out, &[40, format.width, format.height]);
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&24u16.to_le_bytes());
    out.extend_from_slice(&format.fourcc);
    fields(
        &mut out,
        &[
            format.width.saturating_mul(format.height).saturating_mul(3),
            0, // biXPelsPerMeter
            0, // biYPelsPerMeter
            0, // biClrUsed
            0, // biClrImportant
        ],
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format() -> StreamFormat {
        StreamFormat {
            fourcc: *b"H265",
            width: 1920,
            height: 1080,
            frame_rate: 12.0,
            rate_is_assumed: false,
        }
    }

    fn chunks(count: usize, payload: usize) -> Vec<Chunk> {
        (0..count)
            .map(|index| Chunk {
                id: *b"00dc",
                start: index * (8 + payload),
                payload: index * (8 + payload) + 8..index * (8 + payload) + 8 + payload,
                keyframe: index == 0,
            })
            .collect()
    }

    #[test]
    fn the_header_is_exactly_the_size_its_fields_promise() {
        let layout = Layout {
            chunks: 10,
            movi_payload: 1024,
            largest_chunk: 500,
        };
        let written = headers(&format(), &layout, 10);

        assert_eq!(written.len() as u64, HDRL_LEN, "the layout arithmetic depends on this");
        assert_eq!(written.get(..4), Some(&b"LIST"[..]));
        assert_eq!(main_header(&format(), &layout, 10).len(), 56);
        assert_eq!(stream_header(&format(), &layout, 10).len(), 56);
        assert_eq!(stream_format(&format()).len(), 40);
    }

    #[test]
    fn the_rebuilt_file_reads_back_as_the_stream_it_describes() {
        let data = vec![0x5Au8; 8192];
        let output = tempfile::Builder::new().suffix(".avi").tempfile().expect("temp file");
        let written = write_avi(&data, &chunks(4, 100), &format(), output.path()).expect("writes");
        assert_eq!(written, 4);

        let rebuilt = std::fs::read(output.path()).expect("readable");
        let parsed = riff::parse(&rebuilt).expect("a RIFF file");
        assert_eq!(parsed.form, *b"AVI ");
        assert!(!parsed.promises_more_than(rebuilt.len()), "the size field must be exact");

        let read_back = from_reference(&rebuilt).expect("its own header parses");
        assert_eq!(read_back.fourcc, format().fourcc);
        assert_eq!((read_back.width, read_back.height), (1920, 1080));
        assert!((read_back.frame_rate - 12.0).abs() < 1e-6, "{}", read_back.frame_rate);
        assert!(header_is_intact(&rebuilt));
    }

    /// The index is what makes a rebuilt recording seekable, and an offset that is off by the
    /// size of a chunk header sends the player into the middle of a frame.
    #[test]
    fn the_index_points_at_the_chunks_it_describes() {
        let data = vec![0x5Au8; 8192];
        let output = tempfile::Builder::new().suffix(".avi").tempfile().expect("temp file");
        write_avi(&data, &chunks(3, 101), &format(), output.path()).expect("writes");
        let rebuilt = std::fs::read(output.path()).expect("readable");

        let index = riff::parse(&rebuilt).expect("parses").chunk(*b"idx1").expect("an index was written").payload.clone();
        let movi = rebuilt.windows(4).position(|w| w == b"movi").expect("a movi list");
        assert_eq!(index.len(), 3 * 16);

        for entry in 0..3 {
            let at = index.start + entry * 16;
            let offset = read_u32_le(&rebuilt, at + 8).expect("an offset") as usize;
            let length = read_u32_le(&rebuilt, at + 12).expect("a length") as usize;
            assert_eq!(rebuilt.get(movi + offset..movi + offset + 4), Some(&b"00dc"[..]));
            assert_eq!(read_u32_le(&rebuilt, movi + offset + 4), Some(length as u32));
            assert_eq!(length, 101);
        }
        // Only the first chunk was marked a keyframe, and that has to survive into the index.
        assert_eq!(read_u32_le(&rebuilt, index.start + 4), Some(AVIIF_KEYFRAME));
        assert_eq!(read_u32_le(&rebuilt, index.start + 20), Some(0));
    }

    /// An odd-length payload is padded to an even offset, and the index must describe the
    /// payload rather than the padding.
    #[test]
    fn odd_payloads_are_padded_without_disturbing_the_index() {
        assert_eq!(chunk_span(100), 108);
        assert_eq!(chunk_span(101), 110);

        let data = vec![0x5Au8; 8192];
        let output = tempfile::Builder::new().suffix(".avi").tempfile().expect("temp file");
        write_avi(&data, &chunks(2, 101), &format(), output.path()).expect("writes");
        let rebuilt = std::fs::read(output.path()).expect("readable");

        let index = riff::parse(&rebuilt).expect("parses").chunk(*b"idx1").expect("index").start;
        assert_eq!(read_u32_le(&rebuilt, index + 16), Some(4), "the first chunk sits at four");
        assert_eq!(read_u32_le(&rebuilt, index + 32), Some(114), "padded past the odd length");
        assert_eq!(rebuilt.len() % 2, 0, "every chunk lands on an even offset");
    }

    /// AVI states every size in a `u32`. A recording too long to describe is written as far as
    /// the format reaches rather than refused.
    #[test]
    fn a_recording_too_long_for_the_format_is_cut_where_it_stops_fitting() {
        let huge = vec![
            Chunk {
                id: *b"00dc",
                start: 0,
                payload: 0..(1 << 30),
                keyframe: true,
            };
            8
        ];
        let layout = plan(&huge);

        assert!(layout.chunks < 8, "four gigabytes cannot all be described");
        assert!(layout.chunks >= 3, "as much as fits must still be written");
        assert!(4 + HDRL_LEN + 16 + layout.movi_payload + 16 * layout.chunks as u64 <= u64::from(u32::MAX));
    }

    #[test]
    fn a_file_that_still_has_its_header_is_left_to_the_other_strategies() {
        let data = vec![0x5Au8; 8192];
        let output = tempfile::Builder::new().suffix(".avi").tempfile().expect("temp file");
        write_avi(&data, &chunks(4, 100), &format(), output.path()).expect("writes");
        let rebuilt = std::fs::read(output.path()).expect("readable");

        assert!(header_is_intact(&rebuilt));
        let mut zeroed = rebuilt;
        if let Some(head) = zeroed.get_mut(..64) {
            head.fill(0);
        }
        assert!(!header_is_intact(&zeroed));
        assert!(!header_is_intact(&[0u8; 64]));
    }

    #[test]
    fn a_reference_that_is_not_an_avi_supplies_nothing() {
        assert!(from_reference(b"not a riff file at all").is_none());
        let mut riff = b"RIFF".to_vec();
        riff.extend_from_slice(&8u32.to_le_bytes());
        riff.extend_from_slice(b"WAVEfmt ");
        assert!(from_reference(&riff).is_none());
    }

    #[test]
    fn the_real_sample_describes_itself() {
        let Ok(data) = std::fs::read("tmp/a.avi") else {
            return;
        };
        let format = from_reference(&data).expect("the sample has a stream header");
        assert!(format.width > 0 && format.height > 0);
        assert!(plausible_rate(format.frame_rate), "{}", format.frame_rate);
    }
}
