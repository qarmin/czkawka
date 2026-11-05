//! Cutting a fragmented recording at a fragment boundary.
//!
//! A fragmented MP4 has no sample tables. `moov` carries only the track descriptions, and the
//! timing and offsets arrive in a `moof` in front of each chunk of media - which is what makes
//! the format streamable, and what makes a half-written one unusually easy to repair: every
//! fragment describes itself, so a file cut short is a run of perfectly good fragments followed
//! by a stump.
//!
//! Cutting at the last complete `moof`+`mdat` pair therefore changes nothing but the length.
//! There is no re-muxing, no decoding involved in deciding where to cut, and no judgement call -
//! the boundary is structural. It is the ISO-BMFF counterpart of Matroska's "cut after the last
//! complete cluster", and it earns the same byte-faithful label.
//!
//! This is what a dashcam, a screen recorder or anything that was being streamed when the power
//! went out leaves behind.

use super::mp4::{Source, temp_output_like};
use super::{ffmpeg, mp4_boxes};
use crate::file_fixer::candidate::{Candidate, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Strategy};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::mode::Capabilities;

/// One `moof` and the media it describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fragment {
    /// Offset of the `moof` box.
    pub start: usize,
    /// First offset past the media data belonging to it.
    pub end: usize,
}

/// Every fragment the file wholly contains, in order.
///
/// A fragment counts only when both of its boxes are complete: a `moof` whose media was cut
/// short describes samples that are not there, and a demuxer that reads it will walk off the
/// end of the file looking for them.
pub fn complete(data: &[u8]) -> Vec<Fragment> {
    let boxes = mp4_boxes::top_level(data);
    let mut fragments = Vec::new();

    for (index, header) in boxes.iter().enumerate() {
        if !header.is(*b"moof") || header.status != mp4_boxes::BoxStatus::Complete {
            continue;
        }
        // The media follows its own `moof`, possibly behind padding a writer left between them.
        let media = boxes
            .iter()
            .skip(index + 1)
            .take_while(|following| !following.is(*b"moof"))
            .find(|following| following.is(*b"mdat"));

        if let Some(media) = media.filter(|media| media.status == mp4_boxes::BoxStatus::Complete) {
            fragments.push(Fragment {
                start: header.start,
                end: media.payload.end,
            });
        }
    }
    fragments
}

/// Where a cut has to fall for every fragment kept to be whole, or `None` when none is.
pub fn last_complete_end(data: &[u8]) -> Option<usize> {
    complete(data).last().map(|fragment| fragment.end)
}

/// Cut a fragmented recording at the last fragment it wholly contains.
///
/// Nothing is rewritten and nothing is judged by a decoder: the boundary comes from the box
/// structure, so the result is a prefix of the input byte for byte.
pub struct CutAtLastFragment;

impl Strategy for CutAtLastFragment {
    fn name(&self) -> &str {
        "mp4_fragment_cut"
    }

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
        if !mp4_boxes::structure(ctx.data).has_moof {
            return Ok(None);
        }
        let Some(end) = last_complete_end(ctx.data) else {
            return Ok(None);
        };
        // Everything is already whole; whatever is wrong with this file is not its tail.
        if end >= ctx.data.len() {
            return Ok(None);
        }
        let Some(kept) = ctx.data.get(..end) else {
            return Ok(None);
        };

        let output = temp_output_like(ctx.path)?;
        std::fs::write(output.path(), kept).map_err(|error| FixerError::write(output.path(), error))?;

        let report = ffmpeg::decode_report(output.path())?;
        if report.frames == 0 {
            return Ok(None);
        }
        log::debug!(
            "{}: cut {} byte(s) of incomplete fragment, {} frame(s) left",
            ctx.path.display(),
            ctx.data.len() - end,
            report.frames
        );

        let expected = Source::of(ctx)?.and_then(|source| source.expected_frames());
        let fidelity = super::mp4::fidelity_of(&report);
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::TempFile(output.into_temp_path()),
            score: Score {
                content_recovered: report.frames,
                content_total: expected,
                decoder_errors: report.errors,
                decodes_cleanly: report.is_clean(),
                streams_complete: false,
            },
            // The output is a prefix of the input, so its bytes are the original's whatever the
            // decoder makes of them - but a complaint means the pictures are concealment, and
            // that is what the label is about.
            fidelity,
            reconstructed: super::mp4::reconstructed_span(fidelity, report.frames),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boxed(kind: [u8; 4], payload: usize) -> Vec<u8> {
        let mut out = ((payload + 8) as u32).to_be_bytes().to_vec();
        out.extend_from_slice(&kind);
        out.extend(std::iter::repeat_n(0x5Au8, payload));
        out
    }

    fn fragmented(pairs: usize) -> Vec<u8> {
        let mut data = boxed(*b"ftyp", 16);
        data.extend_from_slice(&boxed(*b"moov", 64));
        for _ in 0..pairs {
            data.extend_from_slice(&boxed(*b"moof", 40));
            data.extend_from_slice(&boxed(*b"mdat", 200));
        }
        data
    }

    #[test]
    fn every_whole_fragment_is_found() {
        let data = fragmented(4);
        let found = complete(&data);

        assert_eq!(found.len(), 4);
        assert_eq!(last_complete_end(&data), Some(data.len()));
        assert!(found.windows(2).all(|pair| pair[0].end <= pair[1].start));
    }

    /// The case this exists for: the recorder stopped mid-fragment. What it wrote of that
    /// fragment describes samples that are not there, so the cut falls in front of it.
    #[test]
    fn a_half_written_fragment_is_left_out() {
        let whole = fragmented(4);
        let complete_end = complete(&whole).get(2).map(|fragment| fragment.end).expect("three");

        for lost in [1usize, 50, 150] {
            let cut = whole.len() - lost;
            let damaged = whole.get(..cut).expect("a prefix");
            assert_eq!(last_complete_end(damaged), Some(complete_end), "losing {lost} byte(s) must drop the whole last fragment");
        }
    }

    /// A `moof` with no media behind it at all describes nothing, and keeping it would leave a
    /// demuxer looking for samples past the end of the file.
    #[test]
    fn a_fragment_header_without_its_media_does_not_count() {
        let mut data = fragmented(2);
        let two = last_complete_end(&data).expect("two fragments");
        data.extend_from_slice(&boxed(*b"moof", 40));

        assert_eq!(last_complete_end(&data), Some(two));
    }

    #[test]
    fn a_file_with_no_fragments_offers_no_cut() {
        let mut plain = boxed(*b"ftyp", 16);
        plain.extend_from_slice(&boxed(*b"mdat", 512));
        plain.extend_from_slice(&boxed(*b"moov", 128));

        assert!(complete(&plain).is_empty());
        assert_eq!(last_complete_end(&plain), None);
        assert!(!mp4_boxes::structure(&plain).has_moof);
    }

    /// Padding between the header and its media is legal and must not hide the pair.
    #[test]
    fn padding_between_a_fragment_and_its_media_is_stepped_over() {
        let mut data = boxed(*b"ftyp", 16);
        data.extend_from_slice(&boxed(*b"moof", 40));
        data.extend_from_slice(&boxed(*b"free", 24));
        data.extend_from_slice(&boxed(*b"mdat", 100));

        assert_eq!(complete(&data).len(), 1);
        assert_eq!(last_complete_end(&data), Some(data.len()));
    }

    #[test]
    fn the_real_sample_is_a_run_of_fragments() {
        let Ok(data) = std::fs::read("tmp/samples/clip_frag.mp4") else {
            return;
        };
        let found = complete(&data);

        assert!(found.len() > 2, "{} fragment(s)", found.len());
        assert!(mp4_boxes::structure(&data).has_moof);
        // `mfra` trails the last fragment, so the cut is not the end of the file.
        assert!(last_complete_end(&data).is_some_and(|end| end <= data.len()));
    }
}
