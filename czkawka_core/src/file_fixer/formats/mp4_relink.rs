//! Making an index describe the media again - by freeing it, or by correcting what it says.
//!
//! Carving recovery tools write `ftyp` followed by a single `mdat` whose length claims the rest
//! of the file. That one wrong number hides the `moov` sitting behind the media data, and a
//! demuxer walking the box chain steps over the index into the compressed video and gives up:
//! "moov atom not found". Nothing is actually missing.
//!
//! The repair is to shorten the box that swallowed the index so the index becomes a top-level
//! box again. **No media data moves**, which is the whole point: sample offsets in `stco` are
//! absolute positions in the file, so leaving every byte where it is keeps all of them true.
//! Only two box headers are rewritten.
//!
//! That makes this much better than rebuilding from a stream scan whenever it applies. The
//! output carries the original sample tables, so audio and exact timing survive, and the media
//! is byte-identical to the input rather than reassembled from units a heuristic accepted.
//!
//! The catch, and the reason the result is always decoded before it is believed: a buried index
//! frequently belongs to a *different* recording than the media around it. Carving glues the
//! tail of one file to the body of another, so an index parsing cleanly proves nothing about
//! whether it describes these bytes. The decoder settles it.

use std::io::{Seek, SeekFrom, Write};

use super::mp4::{candidate_from, temp_output_like};
use super::mp4_boxes::BoxHeader;
use super::{ffmpeg, mp4_boxes, mp4_profile};
use crate::file_fixer::candidate::Candidate;
use crate::file_fixer::engine::{RepairContext, Strategy};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::mode::Capabilities;

/// Bytes rewritten in place: a box header, or a whole rewritten chunk-offset table.
type Patch = (usize, Vec<u8>);

/// Make a sample table describe the media again, without moving any media data.
///
/// Two ways for it not to: the index is buried inside an oversized `mdat` and a parser never
/// reaches it, or it is perfectly reachable and every offset in it is wrong by the same amount
/// because the carve started at a different byte. Both are fixed by rewriting numbers - a box
/// header in the first case, the chunk-offset tables in the second - and in neither does a
/// single byte of media move, which is what keeps everything else in the index true.
pub struct RelinkBuriedIndex;

impl Strategy for RelinkBuriedIndex {
    fn name(&self) -> &str {
        "mp4_relink"
    }

    /// Trailing bytes past the index get absorbed into an opaque `mdat`, and a candidate the
    /// decoder complains about is concealment, so this never claims more than truncation.
    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if !ffmpeg::is_available() {
            return Err(FixerError::FfmpegMissing);
        }
        let reachable = mp4_boxes::top_level(ctx.data).into_iter().find(|found| found.is(*b"moov"));
        // An index the top level already names *and* that addresses the bytes around it needs
        // nothing from here, and scanning a healthy file for buried ones would cost a full pass
        // over every byte for nothing.
        if let Some(index) = &reachable
            && mp4_profile::index_describes_media(ctx.data, index) != Some(false)
        {
            return Ok(None);
        }

        let mut best: Option<(tempfile::NamedTempFile, ffmpeg::DecodeReport, Option<u64>)> = None;
        for index in reachable.into_iter().chain(mp4_boxes::find_indexes(ctx.data)) {
            if ctx.should_stop() {
                break;
            }
            // Asked before anything is written: testing an index by building the candidate
            // costs a full copy of the file, and on a carved collection most indexes belong to
            // a recording that is not in this file at all. Measured on 189 carves, 79 of them
            // held an index whose samples were somewhere else or nowhere.
            let Some(mut patches) = patches_for(ctx.data, &index) else {
                continue;
            };

            // A sample table that does not address the bytes around it is usually an index from
            // a different recording, and then nothing can be done with it. But sometimes the
            // recording *is* here and the carve simply began at a different byte, in which case
            // every offset is wrong by the same amount and the whole thing - audio and timing
            // included - comes back by correcting them.
            if mp4_profile::index_describes_media(ctx.data, &index) == Some(false) {
                let Some(shift) = mp4_profile::chunk_offset_shift(ctx.data, &index) else {
                    log::debug!("{}: index at {} describes media that is not here", ctx.path.display(), index.start);
                    continue;
                };
                let Some(corrected) = offset_patches(ctx.data, &index, shift) else {
                    continue;
                };
                log::debug!("{}: index at {} is off by {shift} byte(s); correcting it", ctx.path.display(), index.start);
                patches.extend(corrected);
            }

            let output = temp_output_like(ctx.path)?;
            let output = write_patched(output, ctx.data, &patches)?;
            let report = ffmpeg::decode_report(output.path())?;
            if report.frames == 0 {
                continue;
            }

            let expected = mp4_profile::declared_video_samples(ctx.data, &index);
            if best.as_ref().is_none_or(|(_, current, _)| report.frames > current.frames) {
                best = Some((output, report, expected));
            }
        }

        Ok(best.and_then(|(output, report, expected)| candidate_from(self.name(), ctx, output, &report, expected, true)))
    }
}

/// The box headers to rewrite so that `index` becomes reachable, or `None` when it cannot be.
fn patches_for(data: &[u8], index: &BoxHeader) -> Option<Vec<Patch>> {
    let mut patches = Vec::with_capacity(2);

    // An index the top level already names is not buried and needs no freeing; what is wrong
    // with it is what it says, not where it sits.
    let Some(enclosing) = mp4_boxes::enclosing(data, index.start) else {
        return Some(patches);
    };

    // Shorten whatever box swallowed the index so that it ends exactly where the index begins.
    patches.push(header(enclosing.start, index.start.checked_sub(enclosing.start)?, *b"mdat")?);

    // Whatever follows the index becomes opaque media data rather than structure. The index
    // may well address it - a recording whose `moov` ended up in the middle still has samples
    // on both sides of it - and a trailing fragment of some other file must not be parsed.
    let tail = index.payload.end;
    if let Some(remaining) = data.len().checked_sub(tail)
        && remaining >= 8
    {
        patches.push(header(tail, remaining, *b"mdat")?);
    }
    Some(patches)
}

fn header(at: usize, size: usize, kind: [u8; 4]) -> Option<Patch> {
    if size < 8 {
        return None;
    }
    // A box needing a 64-bit length cannot be expressed by overwriting eight bytes in place.
    let size = u32::try_from(size).ok()?;
    let mut bytes = size.to_be_bytes().to_vec();
    bytes.extend_from_slice(&kind);
    Some((at, bytes))
}

/// Rewrites every chunk offset in the index by `shift`, so a sample table that describes this
/// recording at the wrong address describes it at the right one.
///
/// Nothing but those tables changes, and the media still does not move - the same property that
/// makes relinking safe. What differs is that the numbers were wrong rather than unreachable.
fn offset_patches(data: &[u8], index: &BoxHeader, shift: i64) -> Option<Vec<Patch>> {
    let tables = mp4_profile::chunk_offset_tables(data, index);
    if tables.is_empty() {
        return None;
    }
    tables.iter().map(|table| Some((table.entries_at, table.shifted(shift, data.len())?))).collect()
}

/// Writes the input with the patched headers applied.
///
/// Seeking over a copy rather than editing one in memory keeps a 400 MB recording from costing
/// 400 MB of heap per index tried, and there may be several.
fn write_patched(mut file: tempfile::NamedTempFile, data: &[u8], patches: &[Patch]) -> Result<tempfile::NamedTempFile> {
    let report = |error| FixerError::write("<temp>", error);

    file.write_all(data).map_err(report)?;
    for (at, bytes) in patches {
        file.seek(SeekFrom::Start(*at as u64)).map_err(report)?;
        file.write_all(bytes).map_err(report)?;
    }
    file.flush().map_err(report)?;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `ftyp`, then one `mdat` claiming everything left, with an index hidden behind the media
    /// data - the shape every file in a carved collection has.
    fn carved(media: &[u8], index: &[u8]) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&24u32.to_be_bytes());
        data.extend_from_slice(b"ftypmp42");
        data.extend_from_slice(&[0u8; 12]);

        let body: Vec<u8> = media.iter().chain(index).copied().collect();
        data.extend_from_slice(&((body.len() + 8) as u32).to_be_bytes());
        data.extend_from_slice(b"mdat");
        data.extend_from_slice(&body);
        data
    }

    fn index_box(payload_len: usize) -> Vec<u8> {
        let mut index = Vec::new();
        index.extend_from_slice(&((payload_len + 8) as u32).to_be_bytes());
        index.extend_from_slice(b"moov");
        index.extend_from_slice(&(payload_len as u32).to_be_bytes());
        index.extend_from_slice(b"mvhd");
        index.resize(payload_len + 8, 0);
        index
    }

    fn apply(data: &[u8], patches: &[Patch]) -> Vec<u8> {
        let mut out = data.to_vec();
        for (at, bytes) in patches {
            out[*at..*at + bytes.len()].copy_from_slice(bytes);
        }
        out
    }

    #[test]
    fn a_swallowed_index_is_found_and_freed() {
        let data = carved(&[0xAB; 512], &index_box(64));
        assert!(!mp4_boxes::structure(&data).has_moov, "the fixture must look broken");

        let index = mp4_boxes::find_indexes(&data).pop().expect("the index is findable");
        let patches = patches_for(&data, &index).expect("it can be freed");
        let repaired = apply(&data, &patches);

        let kinds: Vec<String> = mp4_boxes::top_level(&repaired).iter().map(mp4_boxes::BoxHeader::kind_str).collect();
        assert_eq!(kinds, vec!["ftyp", "mdat", "moov"]);
        assert!(mp4_boxes::structure(&repaired).has_moov);
    }

    /// The repair must be a header edit and nothing else, because every sample offset in the
    /// index is an absolute position that only stays true while the data does not move.
    #[test]
    fn no_media_data_moves() {
        let media = [0xAB; 512];
        let data = carved(&media, &index_box(64));
        let index = mp4_boxes::find_indexes(&data).pop().expect("the index is findable");
        let repaired = apply(&data, &patches_for(&data, &index).expect("it can be freed"));

        assert_eq!(repaired.len(), data.len());
        // Only the eight bytes of the `mdat` header differ; the media itself is untouched.
        let differing: Vec<usize> = (0..data.len()).filter(|&i| data[i] != repaired[i]).collect();
        assert!(differing.iter().all(|&i| (24..32).contains(&i)), "{differing:?}");
        assert_eq!(&repaired[32..32 + media.len()], &media[..]);
    }

    /// Data past the index is addressable by it, so it is kept as opaque media rather than
    /// dropped or left to be parsed as structure.
    #[test]
    fn bytes_after_the_index_become_media_data() {
        let mut data = carved(&[0xAB; 256], &index_box(32));
        data.extend_from_slice(&[0xCD; 128]);

        let index = mp4_boxes::find_indexes(&data).pop().expect("the index is findable");
        let patches = patches_for(&data, &index).expect("it can be freed");
        let repaired = apply(&data, &patches);

        let kinds: Vec<String> = mp4_boxes::top_level(&repaired).iter().map(mp4_boxes::BoxHeader::kind_str).collect();
        assert_eq!(kinds, vec!["ftyp", "mdat", "moov", "mdat"]);
    }

    /// Four bytes reading `moov` inside compressed video is not an index, and treating one as
    /// such would corrupt a file rather than repair it.
    #[test]
    fn the_four_character_code_alone_is_not_enough() {
        let mut media = vec![0xAB; 512];
        media[100..104].copy_from_slice(b"moov");
        let data = carved(&media, &index_box(64));

        let found = mp4_boxes::find_indexes(&data);
        assert_eq!(found.len(), 1, "the accidental match must be rejected");
        assert!(found[0].start > 500);
    }

    #[test]
    fn a_file_with_no_index_anywhere_offers_nothing() {
        let data = carved(&[0xAB; 512], &[]);
        assert!(mp4_boxes::find_indexes(&data).is_empty());
    }

    #[test]
    fn a_box_needing_a_64_bit_length_is_declined() {
        assert!(header(0, u32::MAX as usize + 1, *b"mdat").is_none());
        assert!(header(0, 4, *b"mdat").is_none());
        assert_eq!(header(16, 8, *b"mdat"), Some((16, vec![0, 0, 0, 8, b'm', b'd', b'a', b't'])));
    }
}
