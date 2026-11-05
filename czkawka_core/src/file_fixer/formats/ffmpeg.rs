//! Driving the `ffmpeg` and `ffprobe` binaries.
//!
//! Video is the one place this crate cannot answer its own questions. Deciding whether a
//! candidate is better than another means actually decoding it, and there is no pure-Rust
//! H.264 decoder worth trusting for that. The binaries are invoked as processes rather than
//! linked, so the tool has no build-time dependency on ffmpeg's headers.

use std::path::Path;
use std::process::Command;
use std::sync::OnceLock;

use crate::file_fixer::cancel::output_cancellable;
use crate::file_fixer::error::{FixerError, Result};

static AVAILABLE: OnceLock<bool> = OnceLock::new();

/// Whether ffmpeg and ffprobe can be run at all. Probed once per process.
pub fn is_available() -> bool {
    *AVAILABLE.get_or_init(|| {
        ["ffmpeg", "ffprobe"]
            .iter()
            .all(|binary| Command::new(binary).arg("-version").output().is_ok_and(|o| o.status.success()))
    })
}

fn require_available() -> Result<()> {
    if is_available() { Ok(()) } else { Err(FixerError::FfmpegMissing) }
}

fn run(binary: &str, args: &[&str], context: &str) -> Result<String> {
    require_available()?;
    let output = output_cancellable(Command::new(binary).args(args)).map_err(|error| FixerError::Ffmpeg {
        context: context.to_string(),
        stderr: error.to_string(),
    })?;

    if !output.status.success() {
        return Err(FixerError::Ffmpeg {
            context: context.to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamKind {
    Video,
    Audio,
    Other,
}

#[derive(Debug, Clone)]
pub struct Stream {
    pub kind: StreamKind,
    pub codec: String,
    /// Frame count as declared by the container, absent when it does not say.
    pub frames: Option<u64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    /// Average frame rate, which for a bare elementary stream is ffmpeg's own assumption as
    /// often as it is a figure the encoder recorded.
    pub frame_rate: Option<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct Probe {
    pub streams: Vec<Stream>,
    pub duration: Option<f64>,
}

impl Probe {
    pub fn video(&self) -> Option<&Stream> {
        self.streams.iter().find(|s| s.kind == StreamKind::Video)
    }

    pub fn has_audio(&self) -> bool {
        self.streams.iter().any(|s| s.kind == StreamKind::Audio)
    }

    /// Frames the container claims for the video track.
    pub fn declared_frames(&self) -> Option<u64> {
        self.video().and_then(|stream| stream.frames)
    }
}

/// Reads the container's own description of the file, without decoding anything.
pub fn probe(path: &Path) -> Result<Probe> {
    let path = path.to_string_lossy().into_owned();
    let json = run("ffprobe", &["-v", "error", "-show_streams", "-show_format", "-of", "json", &path], "probing the container")?;

    let parsed: serde_json::Value = serde_json::from_str(&json).map_err(|error| FixerError::Ffmpeg {
        context: "parsing the probe output".to_string(),
        stderr: error.to_string(),
    })?;

    Ok(Probe {
        streams: parsed
            .get("streams")
            .and_then(serde_json::Value::as_array)
            .map_or_else(Vec::new, |items| items.iter().map(parse_stream).collect()),
        duration: parsed
            .get("format")
            .and_then(|format| format.get("duration"))
            .and_then(serde_json::Value::as_str)
            .and_then(|duration| duration.parse().ok()),
    })
}

fn parse_stream(value: &serde_json::Value) -> Stream {
    let field = |name: &str| value.get(name).and_then(serde_json::Value::as_str);
    let number = |name: &str| value.get(name).and_then(serde_json::Value::as_u64);
    Stream {
        kind: match field("codec_type") {
            Some("video") => StreamKind::Video,
            Some("audio") => StreamKind::Audio,
            _ => StreamKind::Other,
        },
        codec: field("codec_name").unwrap_or("unknown").to_string(),
        frames: field("nb_frames").and_then(|count| count.parse().ok()),
        width: number("width").and_then(|width| u32::try_from(width).ok()),
        height: number("height").and_then(|height| u32::try_from(height).ok()),
        frame_rate: field("avg_frame_rate").and_then(parse_ratio),
    }
}

/// ffprobe states a frame rate as `numerator/denominator`, and `0/0` when it has none.
fn parse_ratio(ratio: &str) -> Option<f64> {
    let (numerator, denominator) = ratio.split_once('/')?;
    let numerator: f64 = numerator.parse().ok()?;
    let denominator: f64 = denominator.parse().ok()?;
    (denominator != 0.0).then_some(numerator / denominator)
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecodeReport {
    /// Frames the decoder actually produced.
    pub frames: u64,
    /// Presentation time reached, in seconds.
    pub duration: f64,
    /// Lines the decoder wrote to stderr, one per complaint.
    pub errors: u32,
}

impl DecodeReport {
    pub fn is_clean(&self) -> bool {
        self.errors == 0
    }
}

/// Decodes the whole file and reports how far it got.
///
/// This is the scoring function for video candidates: a candidate that yields more decoded
/// frames with fewer complaints is the better repair.
pub fn decode_report(path: &Path) -> Result<DecodeReport> {
    require_available()?;
    let path = path.to_string_lossy().into_owned();

    // `-progress pipe:1` is the machine-readable channel; stderr stays reserved for errors.
    let output = output_cancellable(Command::new("ffmpeg").args(["-v", "error", "-progress", "pipe:1", "-i", &path, "-f", "null", "-"])).map_err(|error| FixerError::Ffmpeg {
        context: "decoding the candidate".to_string(),
        stderr: error.to_string(),
    })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    Ok(DecodeReport {
        frames: last_progress_value(&stdout, "frame=").unwrap_or(0),
        duration: last_progress_value(&stdout, "out_time_us=").unwrap_or(0) as f64 / 1_000_000.0,
        errors: count_complaints(&stderr),
    })
}

/// Decodes until the first error and reports how far it got.
///
/// `-xerror` turns the first decoder complaint into an exit, so the reported presentation time
/// is the point past which the stream stops being reliable - which is exactly where a trim
/// should cut.
pub fn decode_until_error(path: &Path) -> Result<DecodeReport> {
    require_available()?;
    let path = path.to_string_lossy().into_owned();

    let output =
        output_cancellable(Command::new("ffmpeg").args(["-v", "error", "-xerror", "-progress", "pipe:1", "-i", &path, "-f", "null", "-"])).map_err(|error| FixerError::Ffmpeg {
            context: "decoding up to the first error".to_string(),
            stderr: error.to_string(),
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    Ok(DecodeReport {
        frames: last_progress_value(&stdout, "frame=").unwrap_or(0),
        duration: last_progress_value(&stdout, "out_time_us=").unwrap_or(0) as f64 / 1_000_000.0,
        errors: count_complaints(&stderr),
    })
}

/// Complaints that are about *writing* the throwaway output rather than about the input.
///
/// A decode to `-f null` still runs a muxer, and that muxer objects to things the decoder was
/// perfectly happy with. The one that matters in practice is a duplicate timestamp: a phone
/// recording with two frames sharing a presentation time produces "Application provided
/// invalid, non monotonically increasing dts to muxer", once per frame, and the picture is
/// completely fine. Counting those as decoder errors made the tool refuse to call such a file
/// healthy, mark every candidate built from it reconstructed, and then report `unrecoverable`
/// in balanced mode - for a recording with nothing wrong with it. Measured on a carved
/// collection of 355 files with complaints, 30 had no other kind.
fn is_about_the_output(line: &str) -> bool {
    const MUXER_SOURCES: [&str; 4] = ["[null @", "[out#", "[aost#", "[vost#"];

    line.contains("non monotonically increasing")
        // A log artifact rather than a new complaint.
        || line.trim_start().starts_with("Last message repeated")
        || MUXER_SOURCES.iter().any(|source| line.trim_start().starts_with(source))
}

/// Decoder complaints, which is what a candidate's quality is judged by.
fn count_complaints(stderr: &str) -> u32 {
    let complaints = stderr.lines().filter(|line| !line.trim().is_empty() && !is_about_the_output(line)).count();
    u32::try_from(complaints).unwrap_or(u32::MAX)
}

fn last_progress_value(progress: &str, key: &str) -> Option<u64> {
    progress
        .lines()
        .filter_map(|line| line.strip_prefix(key))
        .filter_map(|value| value.trim().parse().ok())
        .next_back()
}

/// A checksum per decoded video frame, in presentation order.
///
/// This is how a repair is checked against the original rather than merely against the
/// decoder's opinion of itself: a candidate whose frames hash the same as the corresponding
/// frames of the healthy file did not just decode without complaint, it decoded to the same
/// pictures. Complaint counts cannot see the difference - a decoder concealing a missing
/// reference produces a wrong picture silently.
pub fn frame_hashes(path: &Path) -> Result<Vec<String>> {
    let path = path.to_string_lossy().into_owned();
    let listing = run("ffmpeg", &["-v", "error", "-i", &path, "-an", "-f", "framemd5", "-"], "hashing the frames")?;

    Ok(listing
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| line.split(',').next_back())
        .map(|hash| hash.trim().to_string())
        .filter(|hash| !hash.is_empty())
        .collect())
}

/// `-movflags` is a private option of the mov/mp4 muxer; passing it while writing any other
/// container (AVI included) is at best ignored with a warning ffmpeg's own docs don't fully
/// pin down, so it is only added when the output actually is one of that family.
fn supports_movflags(output: &Path) -> bool {
    output
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| matches!(ext.to_ascii_lowercase().as_str(), "mp4" | "mov" | "m4v" | "m4a"))
}

/// Rewrites the container without touching the encoded streams.
///
/// `tolerant` turns on the flags that let ffmpeg walk past damage instead of stopping at it,
/// and regenerates timestamps that a broken index left missing.
pub fn remux(input: &Path, output: &Path, tolerant: bool) -> Result<Copied> {
    copy_streams(input, output, tolerant, None, "remuxing the container")
}

/// What a copy managed to carry over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Copied {
    /// True when the audio had to be left behind for the copy to succeed at all.
    pub video_only: bool,
}

/// Copies the streams into a new container, dropping audio only if that is the only way.
///
/// The retry is not a nicety. A muxer refuses a stream it cannot describe - AMR audio out of a
/// damaged 3GP produces "fatal error, input is not a single packet, implement a AVParser for
/// it" - and the whole copy fails with it, so a recording whose *picture* is perfectly fine
/// comes back unrecoverable. Measured on a carved collection: 67 files were in exactly that
/// state, each needing nothing but its last frame cut off.
fn copy_streams(input: &Path, output: &Path, tolerant: bool, seconds: Option<f64>, context: &str) -> Result<Copied> {
    let movflags = supports_movflags(output);
    let input = input.to_string_lossy().into_owned();
    let output = output.to_string_lossy().into_owned();
    let duration = seconds.map(|seconds| format!("{seconds:.3}"));

    let build = |video_only: bool| {
        let mut args: Vec<&str> = vec!["-v", "error", "-y"];
        if tolerant {
            args.extend_from_slice(&["-err_detect", "ignore_err", "-fflags", "+genpts+igndts"]);
        }
        args.extend_from_slice(&["-i", &input]);
        if let Some(duration) = &duration {
            args.extend_from_slice(&["-t", duration]);
        }
        if video_only {
            args.extend_from_slice(&["-map", "0:v"]);
        }
        args.extend_from_slice(&["-c", "copy"]);
        if movflags {
            args.extend_from_slice(&["-movflags", "+faststart"]);
        }
        args.push(&output);
        args
    };

    match run("ffmpeg", &build(false), context) {
        Ok(_) => Ok(Copied { video_only: false }),
        Err(error) if crate::file_fixer::cancel::is_cancelled() => Err(error),
        Err(error) => {
            log::debug!("{context} failed, retrying without audio: {error}");
            run("ffmpeg", &build(true), context).map(|_| Copied { video_only: true })
        }
    }
}

/// Wraps a raw byte stream into an MP4 without re-encoding it.
///
/// `format` names ffmpeg's demuxer for the stream: `h264` for a NAL byte stream, `m4v` for
/// MPEG-4 Part 2, `mjpeg` for concatenated JPEG images. The stream carries no timing in any of
/// those forms, so the frame rate has to be supplied. Letting ffmpeg build the container is
/// deliberate: writing correct sample tables by hand would add a large surface of fiddly code
/// for no benefit.
pub fn mux_elementary_stream(input: &Path, output: &Path, format: &str, frame_rate: f64) -> Result<()> {
    let input = input.to_string_lossy().into_owned();
    let output = output.to_string_lossy().into_owned();
    let rate = format!("{frame_rate:.6}");

    run(
        "ffmpeg",
        &[
            "-v",
            "error",
            "-y",
            "-f",
            format,
            "-r",
            &rate,
            "-i",
            &input,
            "-c",
            "copy",
            "-movflags",
            "+faststart",
            &output,
        ],
        "muxing the rebuilt elementary stream",
    )
    .map(|_| ())
}

/// Copies the streams up to `seconds`, cutting at packet boundaries without re-encoding.
pub fn trim(input: &Path, output: &Path, seconds: f64, tolerant: bool) -> Result<Copied> {
    copy_streams(input, output, tolerant, Some(seconds), "trimming the streams")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn sample() -> PathBuf {
        PathBuf::from("tmp/a.mp4")
    }

    fn skip() -> bool {
        !is_available() || !sample().exists()
    }

    /// A decode to `-f null` runs a muxer too, and its complaints say nothing about the input.
    #[test]
    fn only_the_decoders_complaints_are_counted() {
        let stderr = concat!(
            "[h264 @ 0x1] top block unavailable for requested intra mode\n",
            "[null @ 0x2] Application provided invalid, non monotonically increasing dts to muxer in stream 0: 3 >= 3\n",
            "[out#0/null @ 0x3] Error writing trailer\n",
            "    Last message repeated 4 times\n",
            "[mov,mp4,m4a,3gp,3g2,mj2 @ 0x4] moov atom not found\n",
            "\n",
        );
        assert_eq!(count_complaints(stderr), 2, "the decoder and the demuxer, not the muxer");
        assert_eq!(count_complaints(""), 0);
        assert!(is_about_the_output("[vost#0:0/copy @ 0x5] Application provided invalid, non monotonically increasing dts"));
        assert!(!is_about_the_output("[mpeg4 @ 0x6] ac-tex damaged at 12 7"));
    }

    #[test]
    fn a_frame_rate_ratio_is_read_only_when_there_is_one() {
        assert_eq!(parse_ratio("12000/1000"), Some(12.0));
        assert_eq!(parse_ratio("0/0"), None, "ffprobe says this when it has no rate");
        assert_eq!(parse_ratio("25"), None);
        assert_eq!(parse_ratio(""), None);
    }

    #[test]
    fn progress_values_take_the_last_reported_figure() {
        let progress = "frame=10\nout_time_us=100\nprogress=continue\nframe=830\nout_time_us=27666667\nprogress=end\n";
        assert_eq!(last_progress_value(progress, "frame="), Some(830));
        assert_eq!(last_progress_value(progress, "out_time_us="), Some(27_666_667));
        assert_eq!(last_progress_value(progress, "missing="), None);
        assert_eq!(last_progress_value("", "frame="), None);
    }

    #[test]
    fn probing_the_sample_reports_both_streams() {
        if skip() {
            return;
        }
        let probe = probe(&sample()).expect("the sample probes");

        assert_eq!(probe.video().map(|s| s.codec.as_str()), Some("h264"));
        assert!(probe.has_audio());
        assert_eq!(probe.declared_frames(), Some(830));
        assert!(probe.duration.is_some_and(|d| d > 27.0 && d < 28.0));
    }

    #[test]
    fn a_healthy_file_decodes_every_frame_without_complaint() {
        if skip() {
            return;
        }
        let report = decode_report(&sample()).expect("the sample decodes");

        assert_eq!(report.frames, 830);
        assert!(report.is_clean(), "unexpected decoder errors: {}", report.errors);
        assert!(report.duration > 27.0);
    }

    /// ffprobe does not reject a non-media file, it describes it as a text stream. Callers
    /// must therefore test for a video stream rather than for a successful probe.
    #[test]
    fn probing_something_that_is_not_media_reports_no_video_stream() {
        if !is_available() {
            return;
        }
        let probe = probe(Path::new("Cargo.toml")).expect("ffprobe tolerates arbitrary files");
        assert!(probe.video().is_none());
        assert!(!probe.has_audio());
    }

    #[test]
    fn a_missing_file_fails_with_context() {
        if !is_available() {
            return;
        }
        let error = probe(Path::new("/nonexistent/clip.mp4")).unwrap_err();
        assert!(error.to_string().contains("ffmpeg failed"), "{error}");
    }

    #[test]
    fn a_healthy_file_reaches_the_end_before_any_error() {
        if skip() {
            return;
        }
        let report = decode_until_error(&sample()).expect("the sample decodes");
        assert_eq!(report.frames, 830);
        assert!(report.is_clean());
    }

    #[test]
    fn trimming_shortens_the_result() {
        if skip() {
            return;
        }
        let output = tempfile::Builder::new().suffix(".mp4").tempfile().expect("temp file");
        trim(&sample(), output.path(), 5.0, false).expect("trimming works");

        let probe = probe(output.path()).expect("the trimmed file probes");
        assert!(probe.duration.is_some_and(|d| d > 4.0 && d < 6.0), "{:?}", probe.duration);
    }

    #[test]
    fn remuxing_preserves_the_frame_count() {
        if skip() {
            return;
        }
        let output = tempfile::Builder::new().suffix(".mp4").tempfile().expect("temp file");
        remux(&sample(), output.path(), true).expect("remuxing works");

        assert_eq!(decode_report(output.path()).expect("decodes").frames, 830);
    }

    #[test]
    fn movflags_is_only_offered_to_the_mov_mp4_family() {
        assert!(supports_movflags(Path::new("out.mp4")));
        assert!(supports_movflags(Path::new("out.MOV")));
        assert!(!supports_movflags(Path::new("out.avi")));
        assert!(!supports_movflags(Path::new("out")));
    }

    /// A remux must preserve the container family rather than silently turning an AVI input
    /// into an MP4-flavoured output - the two are different enough that players won't open a
    /// mismatch.
    #[test]
    fn remuxing_to_avi_keeps_the_avi_container() {
        let avi_sample = PathBuf::from("tmp/a.avi");
        if !is_available() || !avi_sample.exists() {
            return;
        }
        let output = tempfile::Builder::new().suffix(".avi").tempfile().expect("temp file");
        remux(&avi_sample, output.path(), true).expect("remuxing works");

        let probe = probe(output.path()).expect("the remuxed file probes");
        assert!(probe.video().is_some());
    }
}
