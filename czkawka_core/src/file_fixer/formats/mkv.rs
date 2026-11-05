//! Matroska (MKV/WebM) repair.
//!
//! The damage that actually happens to a Matroska file is a recording that stopped: the
//! camera, the screen recorder or the download was interrupted, and what is left is a Segment
//! whose declared length promises bytes that are not there, ending inside a cluster. Players
//! that trust the length refuse the whole file, which is why an interrupted recording looks
//! lost even though almost all of it is intact.
//!
//! The structural repair for that is exact and cheap: cut after the last cluster that is
//! entirely present and correct the Segment's length field in place. Every byte kept is the
//! byte that was recorded - no re-encoding, no remuxing, and the length field is the only
//! thing in the file that changes.
//!
//! Alongside it sit the two ffmpeg-driven strategies MP4 and AVI already use, because a
//! structural walk cannot tell whether the *frames* in a cluster decode. Candidates are judged
//! the same way as for those formats: by decoding them and counting what comes out.

use std::path::Path;

use super::ebml::{self, Element};
use super::ffmpeg::{self, DecodeReport, Probe};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{InputFile, RepairContext, Repairer, Strategy};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct MkvRepairer;

impl Repairer for MkvRepairer {
    fn format(&self) -> Format {
        Format::Mkv
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![
            Box::new(Verbatim),
            Box::new(TruncateToLastCluster),
            Box::new(TolerantRemux),
            Box::new(TrimToLastGoodFrame),
            Box::new(RebuildHeader),
        ]
    }
}

/// What the element tree says about the file.
struct Structure {
    segment: Element,
    clusters: Vec<Element>,
    has_tracks: bool,
    /// End of the last cluster that is entirely present.
    last_complete_cluster_end: Option<usize>,
}

impl Structure {
    fn of(data: &[u8]) -> Option<Self> {
        let top = ebml::walk(data, 0..data.len());
        if !top.first().is_some_and(|element| element.is(ebml::EBML_HEADER)) {
            return None;
        }
        let segment = top.into_iter().find(|element| element.is(ebml::SEGMENT))?;

        let children = ebml::walk(data, segment.payload.clone());
        let clusters: Vec<Element> = children.iter().filter(|child| child.is(ebml::CLUSTER)).cloned().collect();
        let last_complete_cluster_end = clusters.iter().take_while(|cluster| cluster.complete).map(Element::end).last();

        Some(Self {
            has_tracks: children.iter().any(|child| child.is(ebml::TRACKS)),
            segment,
            clusters,
            last_complete_cluster_end,
        })
    }

    /// True when the tree accounts for the whole file: the segment's length is known, it is
    /// not promising bytes that are missing, and every cluster inside it is complete.
    fn is_whole(&self, len: usize) -> bool {
        self.segment.complete && self.has_tracks && !self.clusters.is_empty() && self.clusters.iter().all(|cluster| cluster.complete) && self.segment.end() <= len
    }
}

/// A working copy of the input plus what the container says about it.
struct Source {
    input: InputFile,
    probe: Probe,
}

impl Source {
    fn of(ctx: &RepairContext<'_>) -> Result<Option<Self>> {
        if !ffmpeg::is_available() {
            return Err(FixerError::FfmpegMissing);
        }
        let input = ctx.input_file()?;
        let probe = ffmpeg::probe(input.path())?;
        if probe.video().is_none() && !probe.has_audio() {
            return Ok(None);
        }
        Ok(Some(Self { input, probe }))
    }

    fn path(&self) -> &Path {
        self.input.path()
    }

    fn has_video(&self) -> bool {
        self.probe.video().is_some()
    }

    fn expected_frames(&self) -> Option<u64> {
        self.probe.declared_frames()
    }
}

/// Content is counted in frames, or in milliseconds for a file with no video track.
fn content_of(report: &DecodeReport, has_video: bool) -> u64 {
    if has_video { report.frames } else { (report.duration * 1000.0).max(0.0) as u64 }
}

fn score_of(report: &DecodeReport, expected: Option<u64>, has_video: bool) -> Score {
    Score {
        content_recovered: content_of(report, has_video),
        content_total: expected,
        decoder_errors: report.errors,
        decodes_cleanly: report.is_clean(),
        streams_complete: true,
    }
}

fn temp_output() -> Result<tempfile::NamedTempFile> {
    tempfile::Builder::new()
        .prefix("file_fixer-")
        .suffix(".mkv")
        .tempfile()
        .map_err(|error| FixerError::write("<temp>", error))
}

/// Builds a candidate from a produced file, declining when the mode does not allow the
/// concealment a decoder complaint implies. The same rule as AVI and MP4: frames a decoder
/// complained about are not recovered content.
fn candidate_from(name: &str, ctx: &RepairContext<'_>, output: tempfile::NamedTempFile, report: &DecodeReport, expected: Option<u64>, has_video: bool) -> Option<Candidate> {
    let fidelity = if report.is_clean() { Fidelity::Lossless } else { Fidelity::Reconstructed };
    if fidelity == Fidelity::Reconstructed && !ctx.capabilities.guess {
        ctx.decline(Capabilities {
            guess: true,
            ..Capabilities::NONE
        });
        return None;
    }

    let frames = content_of(report, has_video);
    Some(Candidate {
        strategy: name.to_string(),
        data: Payload::TempFile(output.into_temp_path()),
        score: score_of(report, expected, has_video),
        fidelity,
        reconstructed: match fidelity {
            Fidelity::Lossless => Vec::new(),
            Fidelity::Reconstructed => std::iter::once(0..frames).collect(),
        },
    })
}

/// The tree accounts for the whole file and it plays from end to end.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "mkv_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(structure) = Structure::of(ctx.data) else {
            return Ok(None);
        };
        if !structure.is_whole(ctx.data.len()) {
            return Ok(None);
        }
        let Some(source) = Source::of(ctx)? else {
            return Ok(None);
        };

        let report = ffmpeg::decode_report(source.path())?;
        if !report.is_clean() {
            return Ok(None);
        }

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            // Not the container's declared frame count: a Matroska file states durations, not
            // frame totals, and ffprobe's figure for one is an estimate. What decoded to the
            // end is what the file holds.
            score: score_of(&report, Some(content_of(&report, source.has_video())), source.has_video()),
            data: source.input.into_payload(),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Cut after the last complete cluster and correct the Segment's length.
///
/// This is the interrupted-recording repair, and it is the only one here that keeps the file
/// byte for byte: the clusters that are written out are the original bytes at their original
/// offsets, and the single field that changes is the length the writer never got to fill in.
struct TruncateToLastCluster;

impl Strategy for TruncateToLastCluster {
    fn name(&self) -> &str {
        "mkv_truncate_to_cluster"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(structure) = Structure::of(ctx.data) else {
            return Ok(None);
        };
        if structure.is_whole(ctx.data.len()) {
            return Ok(None);
        }
        let Some(end) = structure.last_complete_cluster_end else {
            return Ok(None);
        };
        let Some(mut out) = ctx.data.get(..end).map(<[u8]>::to_vec) else {
            return Ok(None);
        };

        // The Segment's length field keeps its width, so nothing after it shifts.
        let size_at = structure.segment.start + structure.segment.header_len - structure.segment.size_len;
        let payload_len = (end - structure.segment.payload.start) as u64;
        if !ebml::write_size(&mut out, size_at, structure.segment.size_len, payload_len) {
            return Ok(None);
        }

        let output = temp_output()?;
        std::fs::write(output.path(), &out).map_err(|error| FixerError::write(output.path(), error))?;
        if !ffmpeg::is_available() {
            return Err(FixerError::FfmpegMissing);
        }
        let report = ffmpeg::decode_report(output.path())?;
        if report.frames == 0 && report.duration <= 0.0 {
            return Ok(None);
        }

        let probe = ffmpeg::probe(output.path())?;
        let has_video = probe.video().is_some();
        Ok(candidate_from(self.name(), ctx, output, &report, probe.declared_frames(), has_video))
    }
}

/// Rewrite the container, letting ffmpeg walk past damage instead of stopping at it.
struct TolerantRemux;

impl Strategy for TolerantRemux {
    fn name(&self) -> &str {
        "mkv_remux"
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
        let output = temp_output()?;
        ffmpeg::remux(source.path(), output.path(), true)?;

        let report = ffmpeg::decode_report(output.path())?;
        if report.frames == 0 {
            return Ok(None);
        }
        Ok(candidate_from(self.name(), ctx, output, &report, source.expected_frames(), source.has_video()))
    }
}

/// How many frame intervals to step back from the failure point, in order of preference.
const TRIM_BACK_OFF_FRAMES: [f64; 3] = [1.0, 5.0, 25.0];

/// Cut the recording just before the point where decoding stops being reliable.
struct TrimToLastGoodFrame;

impl Strategy for TrimToLastGoodFrame {
    fn name(&self) -> &str {
        "mkv_trim"
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

            let output = temp_output()?;
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

/// The first bytes are gone, and with them the declaration that says what the file is.
///
/// Nothing else in a Matroska file depends on that declaration: the Segment after it carries
/// the tracks, the timestamps and the clusters, and it states its own length. So the repair is
/// to write a standard header - the version numbers and the document type every writer emits -
/// in front of the Segment that survived. Nothing is guessed about the *content*; what is
/// asserted is only that this is a Matroska file, which its Segment already implies.
struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "mkv_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if Structure::of(ctx.data).is_some() {
            return Ok(None);
        }
        let Some(segment) = find_segment(ctx.data) else {
            return Ok(None);
        };

        // A WebM file is Matroska with a narrower codec set; saying so keeps players that only
        // accept WebM willing to open it.
        let webm = ctx.path.extension().is_some_and(|extension| extension.eq_ignore_ascii_case("webm"));
        let mut out = ebml_header(webm);
        out.extend_from_slice(ctx.data.get(segment.start..).unwrap_or_default());

        let output = temp_output()?;
        std::fs::write(output.path(), &out).map_err(|error| FixerError::write(output.path(), error))?;
        if !ffmpeg::is_available() {
            return Err(FixerError::FfmpegMissing);
        }
        let report = ffmpeg::decode_report(output.path())?;
        if report.frames == 0 && report.duration <= 0.0 {
            return Ok(None);
        }

        let probe = ffmpeg::probe(output.path())?;
        let has_video = probe.video().is_some();
        let frames = content_of(&report, has_video);
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::TempFile(output.into_temp_path()),
            score: score_of(&report, None, has_video),
            fidelity: Fidelity::Reconstructed,
            reconstructed: std::iter::once(0..frames).collect(),
        }))
    }
}

/// Finds the Segment when the tree cannot be walked from the start of the file.
///
/// The identifier is four bytes and is immediately followed by a length, so a candidate that
/// is really a Segment has a readable one - which is what separates it from the same four
/// bytes appearing inside a frame.
fn find_segment(data: &[u8]) -> Option<Element> {
    let limit = SEGMENT_SCAN_LIMIT.min(data.len());
    let window = data.get(..limit)?;
    let id = ebml::SEGMENT.to_be_bytes();

    let mut from = 0;
    while from + 8 <= window.len() {
        let found = window.get(from..)?.windows(4).position(|w| w == id)? + from;
        if let Some(element) = ebml::read_element(data, found)
            && element.is(ebml::SEGMENT)
            && !ebml::walk(data, element.payload.clone()).is_empty()
        {
            return Some(element);
        }
        from = found + 1;
    }
    None
}

/// Offsets past this point are not searched for a Segment.
const SEGMENT_SCAN_LIMIT: usize = 1 << 20;

/// The declaration every Matroska file opens with: the EBML version numbers and the document
/// type. These are fixed values, not a guess about the file.
fn ebml_header(webm: bool) -> Vec<u8> {
    let doc_type: &[u8] = if webm { b"webm" } else { b"matroska" };
    let mut payload = Vec::new();
    payload.extend_from_slice(&[0x42, 0x86, 0x81, 0x01]);
    payload.extend_from_slice(&[0x42, 0xF7, 0x81, 0x01]);
    payload.extend_from_slice(&[0x42, 0xF2, 0x81, 0x04]);
    payload.extend_from_slice(&[0x42, 0xF3, 0x81, 0x08]);
    payload.extend_from_slice(&[0x42, 0x82, 0x80 | doc_type.len() as u8]);
    payload.extend_from_slice(doc_type);
    payload.extend_from_slice(&[0x42, 0x87, 0x81, 0x04]);
    payload.extend_from_slice(&[0x42, 0x85, 0x81, 0x02]);

    let mut out = Vec::with_capacity(payload.len() + 5);
    out.extend_from_slice(&ebml::EBML_HEADER.to_be_bytes());
    out.push(0x80 | u8::try_from(payload.len()).unwrap_or(0x7F));
    out.extend_from_slice(&payload);
    out
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::PathBuf;

    use super::*;
    use crate::file_fixer::corrupt::{DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, repair_file};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::{Outcome, RepairReport};

    fn sample() -> PathBuf {
        PathBuf::from("tmp/samples/clip.mkv")
    }

    fn skip() -> bool {
        !ffmpeg::is_available() || !sample().exists()
    }

    fn damaged_file(keep: f64) -> tempfile::TempPath {
        let original = std::fs::read(sample()).expect("the sample is readable");
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(keep).unwrap(),
                },
                1,
            ),
        );

        let mut file = tempfile::Builder::new().suffix(".mkv").tempfile().expect("temp file");
        file.write_all(&damaged.data).expect("write");
        file.into_temp_path()
    }

    fn repair(path: &Path, mode: Mode) -> RepairReport {
        let options = RepairOptions {
            mode,
            output_dir: Some(std::env::temp_dir().join("file_fixer-mkv-tests")),
            force: true,
            ..RepairOptions::default()
        };
        repair_file(path, &options)
    }

    #[test]
    fn a_healthy_recording_is_recognized_and_left_alone() {
        if skip() {
            return;
        }
        let report = repair(&sample(), Mode::Balanced);

        assert_eq!(report.outcome, Outcome::Healthy, "{:?}", report.error);
        assert_eq!(report.winning_strategy.as_deref(), Some("mkv_verbatim"));
        assert!(report.output_path.is_none(), "a healthy file must not be rewritten");
    }

    #[test]
    fn the_element_tree_reads_back_the_way_it_was_written() {
        if skip() {
            return;
        }
        let data = std::fs::read(sample()).expect("readable");
        let structure = Structure::of(&data).expect("the sample parses");

        assert!(structure.has_tracks);
        assert!(!structure.clusters.is_empty());
        assert!(structure.is_whole(data.len()));
        assert_eq!(structure.last_complete_cluster_end, structure.clusters.last().map(Element::end));
    }

    /// The interrupted-recording case: the segment promises more than is there, and the file
    /// ends inside a cluster.
    #[test]
    fn a_cut_recording_keeps_the_clusters_that_arrived() {
        if skip() {
            return;
        }
        let damaged = damaged_file(0.6);
        let data = std::fs::read(&damaged).expect("readable");
        let structure = Structure::of(&data).expect("parses");
        assert!(!structure.is_whole(data.len()), "the damage must be visible in the tree");

        let report = repair(&damaged, Mode::Balanced);
        assert_ne!(report.outcome, Outcome::Healthy, "{:?}", report.error);
        assert!(report.output_path.is_some(), "{:?}", report.error);
        assert!(report.content_recovered.is_some_and(|frames| frames > 0));
    }

    /// The structural cut keeps the original bytes: whatever else wins, the file it produces
    /// has to be a prefix of the input with one length field corrected.
    #[test]
    fn the_structural_cut_changes_nothing_but_the_length_field() {
        if skip() {
            return;
        }
        let damaged = damaged_file(0.6);
        let original = std::fs::read(&damaged).expect("readable");
        // A fresh directory per run: a shared one would still hold candidates from an earlier
        // input, and the assertions below would be comparing two different files.
        let candidates = tempfile::tempdir().expect("temp dir");
        let options = RepairOptions {
            mode: Mode::Balanced,
            keep_candidates: Some(candidates.path().to_path_buf()),
            output_dir: Some(std::env::temp_dir().join("file_fixer-mkv-tests")),
            force: true,
            ..RepairOptions::default()
        };
        let _ = repair_file(&damaged, &options);

        let kept = candidates
            .path()
            .read_dir()
            .expect("the candidate directory exists")
            .filter_map(std::result::Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().contains("mkv_truncate_to_cluster"));
        let Some(kept) = kept else {
            return;
        };

        let produced = std::fs::read(kept.path()).expect("readable");
        assert!(produced.len() < original.len());
        let structure = Structure::of(&produced).expect("the result parses");
        let size_at = structure.segment.start + structure.segment.header_len - structure.segment.size_len;
        let before = original.get(..size_at).expect("prefix");
        assert_eq!(produced.get(..size_at), Some(before), "everything before the length field");
        assert_eq!(
            produced.get(size_at + structure.segment.size_len..),
            original.get(size_at + structure.segment.size_len..produced.len()),
            "and everything after it"
        );
    }

    #[test]
    fn recovery_grows_with_the_amount_of_surviving_data() {
        if skip() {
            return;
        }
        let mut previous = 0;
        for keep in [0.3, 0.6, 0.9] {
            let damaged = damaged_file(keep);
            let recovered = repair(&damaged, Mode::Balanced).content_recovered.unwrap_or(0);
            assert!(recovered >= previous, "keep {keep} gave {recovered} after {previous}");
            previous = recovered;
        }
    }

    #[test]
    fn safe_mode_will_not_shorten_a_recording() {
        if skip() {
            return;
        }
        let damaged = damaged_file(0.6);
        let report = repair(&damaged, Mode::Safe);

        assert_eq!(report.outcome, Outcome::Unrecoverable);
        assert!(report.output_path.is_none());
    }

    /// The declaration at the front is fixed boilerplate; the Segment behind it is the file.
    #[test]
    fn a_destroyed_declaration_is_written_back_in_front_of_the_segment() {
        if skip() {
            return;
        }
        let mut data = std::fs::read(sample()).expect("readable");
        if let Some(head) = data.get_mut(..32) {
            head.fill(0);
        }
        assert!(Structure::of(&data).is_none(), "the tree must not walk any more");
        assert!(find_segment(&data).is_some(), "the segment is still there");

        let mut file = tempfile::Builder::new().suffix(".mkv").tempfile().expect("temp file");
        file.write_all(&data).expect("write");
        let path = file.into_temp_path();

        assert_eq!(repair(&path, Mode::Balanced).outcome, Outcome::Unrecoverable);
        let report = repair(&path, Mode::Aggressive);
        assert_eq!(report.winning_strategy.as_deref(), Some("mkv_rebuild_header"));
        assert!(report.content_recovered.is_some_and(|frames| frames > 0));
    }

    #[test]
    fn the_written_declaration_is_a_well_formed_element() {
        for (webm, doc_type) in [(false, &b"matroska"[..]), (true, &b"webm"[..])] {
            let header = ebml_header(webm);
            let element = ebml::read_element(&header, 0).expect("the header parses");

            assert!(element.is(ebml::EBML_HEADER));
            assert!(element.complete);
            assert_eq!(element.end(), header.len(), "the length must describe it exactly");
            assert!(header.windows(doc_type.len()).any(|window| window == doc_type));
        }
    }

    #[test]
    fn a_file_that_is_not_matroska_is_not_claimed() {
        assert!(Structure::of(&[]).is_none());
        assert!(Structure::of(&vec![0x42u8; 4096]).is_none());
        assert!(Structure::of(b"\x1A\x45\xDF\xA3").is_none());
    }
}
