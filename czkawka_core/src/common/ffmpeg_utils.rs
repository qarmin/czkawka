use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::common::process_utils::disable_windows_console_window;
use crate::flc;
use crate::helpers::messages::Messages;
use crate::tools::video_optimizer::{HardwareEncoder, VideoCodec};

/// Whether the external programs needed by the video tools can be started from `PATH`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfmpegAvailability {
    Available,
    MissingFfmpeg,
    MissingFfprobe,
    MissingFfmpegAndFfprobe,
}

impl FfmpegAvailability {
    fn from_probes(ffmpeg_starts: bool, ffprobe_starts: bool) -> Self {
        match (ffmpeg_starts, ffprobe_starts) {
            (true, true) => Self::Available,
            (false, true) => Self::MissingFfmpeg,
            (true, false) => Self::MissingFfprobe,
            (false, false) => Self::MissingFfmpegAndFfprobe,
        }
    }

    /// Names of the programs that cannot be started, empty when everything is available.
    pub fn missing_programs(self) -> &'static str {
        match self {
            Self::Available => "",
            Self::MissingFfmpeg => "ffmpeg",
            Self::MissingFfprobe => "ffprobe",
            Self::MissingFfmpegAndFfprobe => "ffmpeg, ffprobe",
        }
    }
}

fn program_starts(name: &str) -> bool {
    let mut command = Command::new(name);
    disable_windows_console_window(&mut command);
    command.arg("-version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok_and(|s| s.success())
}

/// Checks which of `ffmpeg` and `ffprobe` can be started from `PATH`.
pub fn check_ffmpeg_availability() -> FfmpegAvailability {
    FfmpegAvailability::from_probes(program_starts("ffmpeg"), program_starts("ffprobe"))
}

/// `true` when both `ffmpeg` and `ffprobe` can be started from `PATH`.
pub fn check_if_ffprobe_ffmpeg_exists() -> bool {
    check_ffmpeg_availability() == FfmpegAvailability::Available
}

/// Returns `false` and sets a critical message naming the missing programs when `ffmpeg` or `ffprobe`
/// cannot be started.
pub(crate) fn ensure_ffmpeg_programs_available(messages: &mut Messages) -> bool {
    let availability = check_ffmpeg_availability();
    if availability == FfmpegAvailability::Available {
        return true;
    }
    messages.critical = Some(format!(
        "{}\n{}",
        flc!("core_ffmpeg_not_found"),
        flc!("core_ffmpeg_missing_programs", programs = availability.missing_programs())
    ));
    #[cfg(target_os = "windows")]
    messages.errors.push(flc!("core_ffmpeg_not_found_windows"));
    false
}

/// Returns the subset of hardware encoders that actually work on this machine.
/// Each candidate is tested by attempting a 1-frame encode; encoders that fail
/// (missing driver, missing library like libcuda.so, unsupported GPU, ...) are excluded.
pub fn get_working_hardware_encoders() -> Vec<HardwareEncoder> {
    HardwareEncoder::all_non_none().iter().copied().filter(|&enc| test_hardware_encoder(enc)).collect()
}

fn test_hardware_encoder(encoder: HardwareEncoder) -> bool {
    let Some(encoder_name) = encoder.encoder_name_for_codec(VideoCodec::H264) else {
        return false;
    };
    match encoder {
        HardwareEncoder::Vaapi => test_vaapi_encoder(encoder_name),
        _ => test_encoder_simple(encoder_name),
    }
}

fn test_encoder_simple(encoder_name: &str) -> bool {
    PROBE_FRAME_SIZES.iter().any(|&size| run_probe(encoder_name, size, None))
}

fn run_probe(encoder_name: &str, size: u32, vaapi_device: Option<&str>) -> bool {
    let mut cmd = Command::new("ffmpeg");
    disable_windows_console_window(&mut cmd);
    cmd.arg("-nostdin");
    if let Some(device) = vaapi_device {
        cmd.args(["-vaapi_device", device]);
    }
    cmd.args(["-f", "lavfi", "-i", &format!("color=size={size}x{size}:rate=1")]);
    if vaapi_device.is_some() {
        cmd.args(["-vf", "format=nv12,hwupload"]);
    }
    cmd.args(["-frames:v", "1", "-c:v", encoder_name, "-f", "null", "-"]);
    let Ok(mut child) = cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn() else {
        return false;
    };
    wait_with_timeout(&mut child)
}

// Encoders have different minimum frame sizes (NVENC rejects 128x128, AMF rejects 64x64), so try big to small
const PROBE_FRAME_SIZES: [u32; 4] = [512, 256, 128, 64];

const ENCODER_PROBE_TIMEOUT: Duration = Duration::from_secs(5);

fn wait_with_timeout(child: &mut std::process::Child) -> bool {
    let deadline = Instant::now() + ENCODER_PROBE_TIMEOUT;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return status.success(),
            Ok(None) => {
                if Instant::now() >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    return false;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => return false,
        }
    }
}

/// Returns the path of the first available DRI render node, or None if none exist.
pub fn find_vaapi_device() -> Option<String> {
    (128..=132u32)
        .map(|index| format!("/dev/dri/renderD{index}"))
        .find(|device| std::path::Path::new(device).exists())
}

/// VAAPI encoding requires an explicit DRI render node and a hwupload filter step.
/// Scan /dev/dri/renderD* in order and return true on the first successful test encode.
fn test_vaapi_encoder(encoder_name: &str) -> bool {
    // renderD128 is the standard first render node; try up to renderD132 for multi-GPU systems
    (128..=132u32)
        .map(|index| format!("/dev/dri/renderD{index}"))
        .filter(|device| std::path::Path::new(device).exists())
        .any(|device| PROBE_FRAME_SIZES.iter().any(|&size| run_probe(encoder_name, size, Some(&device))))
}

/// Returns a list of hardware acceleration methods supported by the installed ffmpeg,
/// e.g. `["cuda", "vaapi", "qsv", "videotoolbox"]`.
/// Returns an empty Vec if ffmpeg is not found or reports no hwaccels.
pub fn get_available_hw_accelerations() -> Vec<String> {
    let mut command = Command::new("ffmpeg");
    disable_windows_console_window(&mut command);
    let Ok(output) = command.arg("-hwaccels").stderr(Stdio::null()).output() else {
        return Vec::new();
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .skip(1) // first line is the "Hardware acceleration methods:" header
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn availability_from_probes() {
        assert_eq!(FfmpegAvailability::from_probes(true, true), FfmpegAvailability::Available);
        assert_eq!(FfmpegAvailability::from_probes(false, true), FfmpegAvailability::MissingFfmpeg);
        assert_eq!(FfmpegAvailability::from_probes(true, false), FfmpegAvailability::MissingFfprobe);
        assert_eq!(FfmpegAvailability::from_probes(false, false), FfmpegAvailability::MissingFfmpegAndFfprobe);
    }
}
