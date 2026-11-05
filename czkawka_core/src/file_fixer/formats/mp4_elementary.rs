//! Carving video out of an index-less file whose samples are not H.264.
//!
//! [`super::mp4_rebuild`] recovers the common case by following length-prefixed NAL units.
//! Three other things turn up in a carved collection of consumer video, and none is reachable
//! that way, because their samples are not NAL units at all:
//!
//! - **Motion JPEG**, which digital cameras wrote into QuickTime files for a decade. Every
//!   sample is a complete JPEG image, which makes it the easiest video there is to carve: the
//!   frames delimit themselves and carry their own dimensions and colour tables.
//! - **MPEG-4 Part 2**, which every phone recorded to 3GP before H.264 took over. Its samples
//!   are start-code delimited and equally easy to find - but the sequence header that states
//!   the picture dimensions lives in `esds`, inside the index that is gone, and nowhere else.
//!   Measured across seven carved phone recordings: not one carried a single sequence-header
//!   start code in its media data. So this one needs a reference, exactly as H.264 needs its
//!   parameter sets, and `--learn-from` supplies it from the rest of the batch.
//! - **H.263**, the oldest of the three and the only one that needs nothing at all: its picture
//!   headers carry the frame size, so the stream describes itself completely.
//!
//! All three produce reconstructed output. The timing is invented - nothing in the media data
//! says how fast to play it - and the scan cannot prove everything it accepted was a frame.
//!
//! Measured on a 189-file PhotoRec collection, these three between them were the *winning*
//! strategy for 25 files that the NAL scan recovered nothing from.

use std::io::Write;
use std::ops::Range;

use super::mp4::{candidate_from, reconstructed_span, temp_output};
use super::{ffmpeg, mp4_boxes, mp4_profile};
use crate::file_fixer::candidate::{Candidate, Fidelity};
use crate::file_fixer::engine::{RepairContext, Strategy};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::mode::Capabilities;

/// Frame rate assumed when nothing in the file says. Consumer cameras and phones of the era
/// that produced these formats recorded at fifteen frames a second far more often than not, and
/// a wrong rate changes how fast the result plays, not what it contains.
const ASSUMED_FRAME_RATE: f64 = 15.0;

/// Ceiling on how many frames to carve, so a hostile file cannot make the scan unbounded.
const MAX_FRAMES: usize = 500_000;

/// Recover a Motion JPEG recording whose index is gone.
///
/// Each sample is a whole JPEG, so the frames are found by their own markers and need nothing
/// from outside the file - no reference, no parameter sets, not even the dimensions.
pub struct CarveMotionJpeg;

impl Strategy for CarveMotionJpeg {
    fn name(&self) -> &str {
        "mp4_carve_mjpeg"
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
        let Some(mdat) = mp4_boxes::find_media_data(ctx.data) else {
            return Ok(None);
        };
        let frames = jpeg_frames(ctx.data, mdat.payload);
        if frames.len() < MIN_FRAMES {
            return Ok(None);
        }

        let stream = tempfile::Builder::new().suffix(".mjpg").tempfile().map_err(|error| FixerError::write("<temp>", error))?;
        let mut writer = std::io::BufWriter::new(&stream);
        for frame in &frames {
            if let Some(bytes) = ctx.data.get(frame.clone()) {
                writer.write_all(bytes).map_err(|error| FixerError::write("<temp>", error))?;
            }
        }
        writer.flush().map_err(|error| FixerError::write("<temp>", error))?;
        drop(writer);

        log::debug!("{}: carved {} JPEG frame(s)", ctx.path.display(), frames.len());
        build_candidate(self.name(), ctx, stream.path(), "mjpeg")
    }
}

/// Fewer frames than this is a stray JPEG - a thumbnail, a preview image - rather than a
/// recording worth calling recovered.
const MIN_FRAMES: usize = 8;

/// Every complete JPEG image in the media data, in order.
///
/// A JPEG ends at its end-of-image marker, and one cannot occur inside entropy-coded data
/// because every `0xFF` there is followed by a stuffed zero. That makes the boundaries exact
/// rather than heuristic, which is why this needs no validation pass afterwards.
fn jpeg_frames(data: &[u8], range: Range<usize>) -> Vec<Range<usize>> {
    const START: [u8; 3] = [0xFF, 0xD8, 0xFF];
    const END: [u8; 2] = [0xFF, 0xD9];

    let mut frames = Vec::new();
    let mut at = range.start;

    while at + 3 < range.end && frames.len() < MAX_FRAMES {
        if data.get(at..at + 3) != Some(&START) {
            at += 1;
            continue;
        }
        let Some(end) = find_marker(data, at + 2, range.end, END) else {
            break;
        };
        frames.push(at..end + 2);
        at = end + 2;
    }
    frames
}

fn find_marker(data: &[u8], from: usize, end: usize, marker: [u8; 2]) -> Option<usize> {
    let window = data.get(from..end)?;
    window.windows(2).position(|candidate| candidate == marker).map(|position| from + position)
}

/// Recover an MPEG-4 Part 2 recording whose index is gone.
///
/// The samples are start-code delimited and easy to find; the dimensions are not in them. A
/// sequence header from a healthy recording of the same device is prepended, which is the same
/// bargain H.264 makes with its parameter sets.
pub struct CarveMpeg4Part2;

impl Strategy for CarveMpeg4Part2 {
    fn name(&self) -> &str {
        "mp4_carve_mpeg4"
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
        let Some(mdat) = mp4_boxes::find_media_data(ctx.data) else {
            return Ok(None);
        };
        let Some(first) = first_picture(ctx.data, mdat.payload.clone()) else {
            return Ok(None);
        };
        let pictures = count_pictures(ctx.data, first..mdat.payload.end);
        if pictures < MIN_FRAMES {
            return Ok(None);
        }

        let mut headers = headers_for(ctx);
        if headers.is_empty() {
            log::debug!(
                "{}: MPEG-4 Part 2 video, but no sequence header to describe it - pass \
                 --reference or --learn-from a healthy recording from the same device",
                ctx.path.display()
            );
            return Ok(None);
        }
        headers.truncate(HEADERS_TO_TRY);

        let mut best: Option<(tempfile::NamedTempFile, ffmpeg::DecodeReport)> = None;
        for header in headers {
            if ctx.should_stop() {
                break;
            }
            let stream = tempfile::Builder::new().suffix(".m4v").tempfile().map_err(|error| FixerError::write("<temp>", error))?;
            let mut writer = std::io::BufWriter::new(&stream);
            writer
                .write_all(&header)
                .and_then(|()| writer.write_all(ctx.data.get(first..mdat.payload.end).unwrap_or_default()))
                .and_then(|()| writer.flush())
                .map_err(|error| FixerError::write("<temp>", error))?;
            drop(writer);

            let output = temp_output()?;
            if ffmpeg::mux_elementary_stream(stream.path(), output.path(), "m4v", ASSUMED_FRAME_RATE).is_err() {
                continue;
            }
            let report = ffmpeg::decode_report(output.path())?;
            if report.frames == 0 {
                continue;
            }
            if best.as_ref().is_none_or(|(_, current)| report.frames > current.frames) {
                best = Some((output, report));
            }
        }

        Ok(best.and_then(|(output, report)| reconstructed(self.name(), ctx, output, &report)))
    }
}

/// Recover an H.263 recording whose index is gone.
///
/// The oldest of the phone codecs, and the simplest to carve: every picture opens with a
/// twenty-two-bit start code, and the picture header that follows carries the frame size, so
/// nothing outside the file is needed. The one wrinkle is the container - MP4 has no tag for
/// H.263, so the result is written as 3GP, which does.
pub struct CarveH263;

impl Strategy for CarveH263 {
    fn name(&self) -> &str {
        "mp4_carve_h263"
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
        let Some(mdat) = mp4_boxes::find_media_data(ctx.data) else {
            return Ok(None);
        };
        let Some(first) = first_h263_picture(ctx.data, mdat.payload.clone()) else {
            return Ok(None);
        };
        if count_h263_pictures(ctx.data, first..mdat.payload.end) < MIN_FRAMES {
            return Ok(None);
        }

        let stream = tempfile::Builder::new().suffix(".263").tempfile().map_err(|error| FixerError::write("<temp>", error))?;
        let mut writer = std::io::BufWriter::new(&stream);
        writer
            .write_all(ctx.data.get(first..mdat.payload.end).unwrap_or_default())
            .and_then(|()| writer.flush())
            .map_err(|error| FixerError::write("<temp>", error))?;
        drop(writer);

        // 3GP rather than MP4: ffmpeg has no MP4 tag for H.263 and refuses to write the header.
        let output = temp_output_with_suffix(".3gp")?;
        if let Err(error) = ffmpeg::mux_elementary_stream(stream.path(), output.path(), "h263", ASSUMED_FRAME_RATE) {
            log::debug!("{}: the carved H.263 stream was refused: {error}", ctx.path.display());
            return Ok(None);
        }
        let report = ffmpeg::decode_report(output.path())?;
        if report.frames == 0 {
            return Ok(None);
        }
        Ok(reconstructed(self.name(), ctx, output, &report))
    }
}

fn temp_output_with_suffix(suffix: &str) -> Result<tempfile::NamedTempFile> {
    tempfile::Builder::new()
        .prefix("file_fixer-")
        .suffix(suffix)
        .tempfile()
        .map_err(|error| FixerError::write("<temp>", error))
}

/// A picture start code: twenty-two bits, `0000 0000 0000 0000 1000 00`, so the third byte's
/// top six bits are fixed and its bottom two belong to the temporal reference.
fn is_h263_picture_start(candidate: &[u8]) -> bool {
    matches!(candidate, [0, 0, third, ..] if third & 0xFC == 0x80)
}

fn first_h263_picture(data: &[u8], range: Range<usize>) -> Option<usize> {
    let window = data.get(range.clone())?;
    window.windows(3).position(is_h263_picture_start).map(|position| range.start + position)
}

fn count_h263_pictures(data: &[u8], range: Range<usize>) -> usize {
    data.get(range)
        .map_or(0, |window| window.windows(3).filter(|candidate| is_h263_picture_start(candidate)).count())
}

/// Sequence headers worth trying. Devices of this era used a handful of settings, and a wrong
/// one produces a stream ffmpeg refuses rather than a subtly wrong picture.
const HEADERS_TO_TRY: usize = 4;

/// The sequence headers available to describe this recording, nearest source first.
fn headers_for(ctx: &RepairContext<'_>) -> Vec<Vec<u8>> {
    let mut headers = Vec::new();
    let mut push = |header: Option<Vec<u8>>| {
        if let Some(header) = header
            && !headers.contains(&header)
        {
            headers.push(header);
        }
    };

    push(mp4_profile::mpeg4_headers(ctx.data));
    for reference in ctx.references {
        match std::fs::read(reference) {
            Ok(data) => push(mp4_profile::mpeg4_headers(&data)),
            Err(error) => log::warn!("cannot read reference {}: {error}", reference.display()),
        }
    }
    for learned in mp4_profile::learned_mpeg4() {
        push(Some(learned));
    }
    headers
}

/// Where the first coded picture begins.
fn first_picture(data: &[u8], range: Range<usize>) -> Option<usize> {
    let window = data.get(range.clone())?;
    window
        .windows(4)
        .position(|candidate| is_picture_start(candidate) || is_sequence_start(candidate))
        .map(|position| range.start + position)
}

fn count_pictures(data: &[u8], range: Range<usize>) -> usize {
    data.get(range)
        .map_or(0, |window| window.windows(4).filter(|candidate| is_picture_start(candidate)).count())
}

/// `000001B6` introduces a video object plane - one coded picture.
fn is_picture_start(candidate: &[u8]) -> bool {
    candidate == [0, 0, 1, 0xB6]
}

/// `000001B0` introduces the visual object sequence, which carries the dimensions.
fn is_sequence_start(candidate: &[u8]) -> bool {
    candidate == [0, 0, 1, 0xB0]
}

/// Muxes a carved stream and turns the result into a candidate.
fn build_candidate(name: &str, ctx: &RepairContext<'_>, stream: &std::path::Path, format: &str) -> Result<Option<Candidate>> {
    let output = temp_output()?;
    if let Err(error) = ffmpeg::mux_elementary_stream(stream, output.path(), format, ASSUMED_FRAME_RATE) {
        log::debug!("{}: the carved stream was refused: {error}", ctx.path.display());
        return Ok(None);
    }
    let report = ffmpeg::decode_report(output.path())?;
    if report.frames == 0 {
        return Ok(None);
    }
    Ok(reconstructed(name, ctx, output, &report))
}

/// Always reconstructed: the timing is assumed and the scan cannot prove every frame was real.
fn reconstructed(name: &str, ctx: &RepairContext<'_>, output: tempfile::NamedTempFile, report: &ffmpeg::DecodeReport) -> Option<Candidate> {
    let mut candidate = candidate_from(name, ctx, output, report, None, true)?;
    candidate.fidelity = Fidelity::Reconstructed;
    candidate.reconstructed = reconstructed_span(Fidelity::Reconstructed, report.frames);
    Some(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jpeg(size: usize) -> Vec<u8> {
        let mut out = vec![0xFF, 0xD8, 0xFF, 0xDB];
        out.extend(std::iter::repeat_n(0x41, size));
        out.extend_from_slice(&[0xFF, 0xD9]);
        out
    }

    #[test]
    fn every_complete_jpeg_in_the_media_data_is_found() {
        let mut data = vec![0u8; 64];
        let mut expected = Vec::new();
        for size in [100, 250, 80] {
            let start = data.len();
            data.extend_from_slice(&jpeg(size));
            expected.push(start..data.len());
        }

        assert_eq!(jpeg_frames(&data, 0..data.len()), expected);
    }

    /// A carve stops wherever the recovery tool stopped, which is regularly mid-frame. The
    /// half-written image is not a frame and must not be carried into the output.
    #[test]
    fn a_frame_the_file_ends_inside_is_left_out() {
        let mut data = jpeg(100);
        let whole = data.len();
        data.extend_from_slice(&[0xFF, 0xD8, 0xFF, 0xDB]);
        data.extend(std::iter::repeat_n(0x41, 50));

        assert_eq!(jpeg_frames(&data, 0..data.len()), vec![0..whole]);
    }

    #[test]
    fn pictures_are_counted_by_their_start_codes() {
        let mut data = vec![0x11, 0x22];
        for _ in 0..5 {
            data.extend_from_slice(&[0, 0, 1, 0xB6]);
            data.extend_from_slice(&[0x7Fu8; 40]);
        }
        assert_eq!(count_pictures(&data, 0..data.len()), 5);
        assert_eq!(first_picture(&data, 0..data.len()), Some(2));
    }

    #[test]
    fn a_sequence_header_counts_as_the_start_of_the_stream() {
        let mut data = vec![0u8; 8];
        data.extend_from_slice(&[0, 0, 1, 0xB0, 0x04]);
        data.extend_from_slice(&[0, 0, 1, 0xB6]);
        assert_eq!(first_picture(&data, 0..data.len()), Some(8));
    }

    /// The start code is twenty-two bits, so the third byte is only fixed in its top six -
    /// the rest is the temporal reference and differs from picture to picture.
    #[test]
    fn h263_pictures_are_found_by_their_twenty_two_bit_start_code() {
        let mut data = vec![0x99, 0x88];
        for temporal in 0..6u8 {
            data.extend_from_slice(&[0x00, 0x00, 0x80 | (temporal & 0x03)]);
            data.extend_from_slice(&[0x5Au8; 64]);
        }

        assert_eq!(count_h263_pictures(&data, 0..data.len()), 6);
        assert_eq!(first_h263_picture(&data, 0..data.len()), Some(2));
        assert!(is_h263_picture_start(&[0, 0, 0x83]));
        assert!(!is_h263_picture_start(&[0, 0, 0x84]), "the top six bits are fixed");
        assert!(!is_h263_picture_start(&[0, 1, 0x80]));
    }

    #[test]
    fn nothing_is_carved_out_of_data_that_holds_no_frames() {
        let data: Vec<u8> = (0..4096u32).map(|index| (index % 251) as u8).collect();
        assert!(jpeg_frames(&data, 0..data.len()).is_empty());
        assert_eq!(count_pictures(&data, 0..data.len()), 0);
    }
}
