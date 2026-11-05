//! Cutting a recording just before the point where decoding stops being reliable.

use super::ffmpeg;
use super::mp4::{Source, candidate_from, temp_output_like};
use crate::file_fixer::candidate::Candidate;
use crate::file_fixer::engine::{RepairContext, Strategy};
use crate::file_fixer::error::Result;
use crate::file_fixer::mode::Capabilities;

/// How many frame intervals to step back from the failure point, in order of preference.
///
/// One is usually enough: the reported time is the moment the decoder gave up, which still
/// *includes* the packet it choked on, so the cut has to land just before it. The larger steps
/// are there for damage that spoils a short run of packets rather than a single one.
const TRIM_BACK_OFF_FRAMES: [f64; 3] = [1.0, 5.0, 25.0];

/// Cut the file just before the point where decoding stops being reliable.
///
/// Keeps at most as many frames as a tolerant remux, but every frame it keeps decoded without
/// a complaint, so the result is content rather than concealment.
///
/// Cutting by duration cannot leave a frame whose reference was thrown away, even though
/// presentation order and decode order disagree wherever there are B-frames: `-t` on a stream
/// copy keeps packets by *decode* timestamp, and a frame may only predict from pictures that
/// precede it in decode order, so what it keeps is closed under reference by construction.
/// Measured rather than argued - see the test below, and the note in [`super::mp4`].
pub struct TrimToLastGoodFrame;

impl Strategy for TrimToLastGoodFrame {
    fn name(&self) -> &str {
        "mp4_trim"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(source) = Source::of(ctx)? else {
            return Ok(None);
        };

        let until_error = ffmpeg::decode_until_error(source.path())?;
        if until_error.is_clean() || until_error.duration <= 0.0 {
            return Ok(None);
        }
        let frame_interval = match until_error.frames {
            0 => return Ok(None),
            frames => until_error.duration / frames as f64,
        };

        let mut fallback = None;
        for back_off in TRIM_BACK_OFF_FRAMES {
            if ctx.should_stop() {
                break;
            }
            let target = until_error.duration - frame_interval * back_off;
            if target <= 0.0 {
                break;
            }

            let output = temp_output_like(ctx.path)?;
            ffmpeg::trim(source.path(), output.path(), target, true)?;
            let report = ffmpeg::decode_report(output.path())?;
            if report.frames == 0 {
                continue;
            }
            if report.is_clean() {
                return Ok(candidate_from(self.name(), ctx, output, &report, source.expected_frames(), source.has_video()));
            }
            fallback.get_or_insert((output, report));
        }

        Ok(fallback.and_then(|(output, report)| candidate_from(self.name(), ctx, output, &report, source.expected_frames(), source.has_video())))
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;

    /// The reported failure time is the moment the decoder gave up, which still *includes* the
    /// packet it choked on, so a cut has to land before it. Measured on a truncated recording:
    /// cutting at the failure time left five decoder errors, one frame interval earlier left
    /// none while keeping the same 671 frames.
    #[test]
    fn the_back_off_steps_grow_and_start_at_one_frame() {
        assert_eq!(TRIM_BACK_OFF_FRAMES.first(), Some(&1.0));
        assert!(TRIM_BACK_OFF_FRAMES.windows(2).all(|w| w[1] > w[0]));
    }

    /// Cuts a prefix of `source` short, trims it the way the strategy does, and also hashes
    /// what the damaged file itself decodes to - the most any repair of it could reach.
    fn trimmed(source: &Path, keep: usize) -> Option<(tempfile::TempPath, ffmpeg::DecodeReport, Vec<String>)> {
        let data = std::fs::read(source).ok()?;
        let mut damaged = tempfile::Builder::new().suffix(".mp4").tempfile().ok()?;
        std::io::Write::write_all(&mut damaged, data.get(..data.len() * keep / 100)?).ok()?;

        let until_error = ffmpeg::decode_until_error(damaged.path()).ok()?;
        let interval = until_error.duration / until_error.frames as f64;
        let output = tempfile::Builder::new().suffix(".mp4").tempfile().ok()?;
        ffmpeg::trim(damaged.path(), output.path(), until_error.duration - interval, true).ok()?;
        let whole = ffmpeg::frame_hashes(damaged.path()).ok()?;

        let report = ffmpeg::decode_report(output.path()).ok()?;
        Some((output.into_temp_path(), report, whole))
    }

    /// What the `Lossless` label on a trim actually rests on, and what a complaint count cannot
    /// check: the cut does not damage a single frame. Two claims, both by comparing pictures
    /// rather than counting errors - a decoder concealing a missing reference says nothing.
    ///
    /// The first is the strong one: the cut takes nothing away that the damaged file itself
    /// could still decode. The second holds against the *healthy* original for every frame but
    /// the last, and that one is the damage rather than the cut - measured on `clip.mp4`, the
    /// available packets already form a complete, reference-closed set and its final picture
    /// still decodes differently, so no choice of cut point recovers it.
    ///
    /// This is also the answer to a question the module used to leave open: cutting by duration
    /// cannot leave a dangling reference, because `-t` on a stream copy keeps a decode-order
    /// prefix. Measured on three streams, HEVC with B-pyramids included: 671 of 671, 479 of 479
    /// and 46 of 47 frames bit-identical to the original. A cut computed in presentation order
    /// instead - the "obvious" fix - made it worse, taking two good frames off the first.
    #[test]
    fn a_trim_damages_no_frame_it_keeps() {
        for sample in ["tmp/a_fast.mp4", "tmp/samples/clip.mp4", "tmp/samples/clip_hevc.mp4"] {
            let sample = PathBuf::from(sample);
            if !ffmpeg::is_available() || !sample.exists() {
                continue;
            }
            let Some((output, report, damaged)) = trimmed(&sample, 80) else {
                continue;
            };
            assert!(report.frames > 0, "{sample:?} produced nothing to compare");

            let kept = ffmpeg::frame_hashes(&output).expect("the trimmed file hashes");
            let original = ffmpeg::frame_hashes(&sample).expect("the original hashes");
            assert!(!kept.is_empty(), "{sample:?}");
            assert!(kept.len() < original.len(), "{sample:?}: the trim has to be shorter");
            assert!(
                kept.iter().zip(&damaged).all(|(trimmed, best)| trimmed == best),
                "{sample:?}: the cut cost a frame the damaged file could still decode"
            );

            let matching = kept.iter().zip(&original).filter(|(kept, was)| kept == was).count();
            assert!(matching + 1 >= kept.len(), "{sample:?}: {matching} of {} kept frames match the original", kept.len());
        }
    }
}
