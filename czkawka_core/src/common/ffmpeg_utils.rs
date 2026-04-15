use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::common::process_utils::disable_windows_console_window;
use crate::tools::video_optimizer::{HardwareEncoder, VideoCodec};

pub fn check_if_ffprobe_ffmpeg_exists() -> bool {
    let mut ffmpeg_command = Command::new("ffmpeg");
    disable_windows_console_window(&mut ffmpeg_command);
    let ffmpeg_ok = ffmpeg_command
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success());

    let mut ffprobe_command = Command::new("ffprobe");
    disable_windows_console_window(&mut ffprobe_command);
    let ffprobe_ok = ffprobe_command
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success());

    ffprobe_ok && ffmpeg_ok
}

/// Returns the subset of hardware encoders that actually work on this machine.
/// Each candidate is tested by attempting a 1-frame encode; encoders that fail
/// (missing driver, missing library like libcuda.so, unsupported GPU, …) are excluded.
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
    let mut cmd = Command::new("ffmpeg");
    disable_windows_console_window(&mut cmd);
    let Ok(mut child) = cmd
        .args([
            "-nostdin",
            "-f",
            "lavfi",
            "-i",
            "color=size=64x64:rate=1",
            "-frames:v",
            "1",
            "-c:v",
            encoder_name,
            "-f",
            "null",
            "-",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    else {
        return false;
    };
    wait_with_timeout(&mut child)
}

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
    for index in 128..=132u32 {
        let device = format!("/dev/dri/renderD{index}");
        if !std::path::Path::new(&device).exists() {
            continue;
        }
        let mut cmd = Command::new("ffmpeg");
        disable_windows_console_window(&mut cmd);
        let Ok(mut child) = cmd
            .args([
                "-nostdin",
                "-vaapi_device",
                &device,
                "-f",
                "lavfi",
                "-i",
                "color=size=128x128:rate=1",
                "-vf",
                "format=nv12,hwupload",
                "-frames:v",
                "1",
                "-c:v",
                encoder_name,
                "-f",
                "null",
                "-",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
        else {
            continue;
        };
        if wait_with_timeout(&mut child) {
            return true;
        }
    }
    false
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
