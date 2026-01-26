// GPU Detection Module
// Detects NVIDIA GPU capabilities for hardware-accelerated video decoding

use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::common::process_utils::disable_windows_console_window;

/// Represents different NVIDIA GPU generations with their NVDEC capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum GpuGeneration {
    #[default]
    Unknown,
    Kepler,    // GTX 600/700 series (Compute 3.x)
    Maxwell,   // GTX 900 series (Compute 5.x)
    Pascal,    // GTX 10 series (Compute 6.x)
    Volta,     // Titan V (Compute 7.0)
    Turing,    // RTX 20 series (Compute 7.5)
    Ampere,    // RTX 30 series (Compute 8.x)
    Ada,       // RTX 40 series (Compute 8.9)
    Blackwell, // RTX 50 series (Compute 9.x+)
}

impl GpuGeneration {
    /// Parse GPU generation from CUDA compute capability string (e.g., "8.6")
    pub fn from_compute_capability(compute_cap: &str) -> Self {
        let major = compute_cap.split('.').next().unwrap_or("0");
        let minor = compute_cap.split('.').nth(1).unwrap_or("0");

        match major {
            "3" => GpuGeneration::Kepler,
            "5" => GpuGeneration::Maxwell,
            "6" => GpuGeneration::Pascal,
            "7" => {
                if minor == "0" {
                    GpuGeneration::Volta
                } else {
                    GpuGeneration::Turing
                }
            }
            "8" => {
                if minor == "9" {
                    GpuGeneration::Ada
                } else {
                    GpuGeneration::Ampere
                }
            }
            "9" | "10" => GpuGeneration::Blackwell,
            _ => GpuGeneration::Unknown,
        }
    }

    /// Check if this GPU generation supports a specific codec
    pub fn supports_codec(&self, codec: &str) -> bool {
        match codec.to_lowercase().as_str() {
            "h264" | "avc" => *self >= GpuGeneration::Kepler,
            "hevc" | "h265" => *self >= GpuGeneration::Maxwell,
            "vp9" => *self >= GpuGeneration::Pascal,
            "av1" => *self >= GpuGeneration::Ampere,
            "mpeg2" | "mpeg2video" => *self >= GpuGeneration::Kepler,
            "vp8" => *self >= GpuGeneration::Maxwell,
            _ => false,
        }
    }
}

/// Information about a detected NVIDIA GPU
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NvidiaGpuInfo {
    pub name: String,
    pub generation: GpuGeneration,
    pub compute_capability: String,
    pub device_id: i32,
}

/// Hardware acceleration type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HWAccelType {
    #[default]
    Auto,        // Auto-detect best option
    Cuda,        // NVIDIA CUDA/NVDEC
    Vaapi,       // Intel/AMD on Linux
    Dxva2,       // Windows DirectX Video Acceleration
    Videotoolbox, // macOS Video Toolbox
    None,        // CPU only (no hardware acceleration)
}

impl HWAccelType {
    pub fn as_ffmpeg_arg(&self) -> Option<&'static str> {
        match self {
            HWAccelType::Auto => Some("auto"),
            HWAccelType::Cuda => Some("cuda"),
            HWAccelType::Vaapi => Some("vaapi"),
            HWAccelType::Dxva2 => Some("dxva2"),
            HWAccelType::Videotoolbox => Some("videotoolbox"),
            HWAccelType::None => None,
        }
    }
}

/// GPU detector for finding and analyzing available GPUs
pub struct GpuDetector;

impl GpuDetector {
    /// Detect NVIDIA GPU using nvidia-smi
    pub fn detect_nvidia_gpu() -> Option<NvidiaGpuInfo> {
        let mut command = Command::new("nvidia-smi");
        disable_windows_console_window(&mut command);

        let output = command
            .args(["--query-gpu=name,compute_cap", "--format=csv,noheader"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let info = String::from_utf8_lossy(&output.stdout);
        let line = info.lines().next()?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 2 {
            return None;
        }

        let name = parts[0].trim().to_string();
        let compute_capability = parts[1].trim().to_string();
        let generation = GpuGeneration::from_compute_capability(&compute_capability);

        Some(NvidiaGpuInfo {
            name,
            generation,
            compute_capability,
            device_id: 0,
        })
    }

    /// Check if FFmpeg has NVDEC/CUDA support
    pub fn check_ffmpeg_nvdec_support() -> bool {
        let mut command = Command::new("ffmpeg");
        disable_windows_console_window(&mut command);

        let output = command.args(["-hwaccels"]).output();

        if let Ok(output) = output {
            let hwaccels = String::from_utf8_lossy(&output.stdout);
            return hwaccels.contains("cuda") || hwaccels.contains("cuvid");
        }

        false
    }

    /// Get the count of available NVIDIA GPUs
    pub fn get_nvidia_gpu_count() -> usize {
        let mut command = Command::new("nvidia-smi");
        disable_windows_console_window(&mut command);

        let output = command
            .args(["--query-gpu=name", "--format=csv,noheader"])
            .output();

        if let Ok(output) = output {
            let gpus = String::from_utf8_lossy(&output.stdout);
            return gpus.lines().count();
        }

        0
    }

    /// Check if GPU acceleration is available (GPU detected + FFmpeg support)
    pub fn is_gpu_acceleration_available() -> bool {
        Self::detect_nvidia_gpu().is_some() && Self::check_ffmpeg_nvdec_support()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Phase 1 Unit Tests =====
    // These tests are based on hypothetical issues that could cause problems

    #[test]
    fn test_gpu_generation_ordering() {
        // Test that GPU generations are properly ordered for comparison
        assert!(GpuGeneration::Kepler < GpuGeneration::Maxwell);
        assert!(GpuGeneration::Maxwell < GpuGeneration::Pascal);
        assert!(GpuGeneration::Pascal < GpuGeneration::Volta);
        assert!(GpuGeneration::Volta < GpuGeneration::Turing);
        assert!(GpuGeneration::Turing < GpuGeneration::Ampere);
        assert!(GpuGeneration::Ampere < GpuGeneration::Ada);
        assert!(GpuGeneration::Ada < GpuGeneration::Blackwell);
    }

    #[test]
    fn test_gpu_generation_from_compute_capability() {
        // Test parsing various compute capability strings
        assert_eq!(GpuGeneration::from_compute_capability("3.0"), GpuGeneration::Kepler);
        assert_eq!(GpuGeneration::from_compute_capability("3.5"), GpuGeneration::Kepler);
        assert_eq!(GpuGeneration::from_compute_capability("5.0"), GpuGeneration::Maxwell);
        assert_eq!(GpuGeneration::from_compute_capability("5.2"), GpuGeneration::Maxwell);
        assert_eq!(GpuGeneration::from_compute_capability("6.0"), GpuGeneration::Pascal);
        assert_eq!(GpuGeneration::from_compute_capability("6.1"), GpuGeneration::Pascal);
        assert_eq!(GpuGeneration::from_compute_capability("7.0"), GpuGeneration::Volta);
        assert_eq!(GpuGeneration::from_compute_capability("7.5"), GpuGeneration::Turing);
        assert_eq!(GpuGeneration::from_compute_capability("8.0"), GpuGeneration::Ampere);
        assert_eq!(GpuGeneration::from_compute_capability("8.6"), GpuGeneration::Ampere);
        assert_eq!(GpuGeneration::from_compute_capability("8.9"), GpuGeneration::Ada);
        assert_eq!(GpuGeneration::from_compute_capability("9.0"), GpuGeneration::Blackwell);
    }

    #[test]
    fn test_gpu_generation_from_invalid_compute_capability() {
        // Test handling of invalid/unexpected compute capability strings
        assert_eq!(GpuGeneration::from_compute_capability(""), GpuGeneration::Unknown);
        assert_eq!(GpuGeneration::from_compute_capability("invalid"), GpuGeneration::Unknown);
        assert_eq!(GpuGeneration::from_compute_capability("0.0"), GpuGeneration::Unknown);
        assert_eq!(GpuGeneration::from_compute_capability("99.99"), GpuGeneration::Unknown);
    }

    #[test]
    fn test_codec_support_h264() {
        // H.264 should be supported on all GPU generations except Unknown
        assert!(!GpuGeneration::Unknown.supports_codec("h264"));
        assert!(GpuGeneration::Kepler.supports_codec("h264"));
        assert!(GpuGeneration::Maxwell.supports_codec("h264"));
        assert!(GpuGeneration::Ampere.supports_codec("h264"));

        // Test case insensitivity
        assert!(GpuGeneration::Kepler.supports_codec("H264"));
        assert!(GpuGeneration::Kepler.supports_codec("avc"));
        assert!(GpuGeneration::Kepler.supports_codec("AVC"));
    }

    #[test]
    fn test_codec_support_hevc() {
        // HEVC requires Maxwell or newer
        assert!(!GpuGeneration::Unknown.supports_codec("hevc"));
        assert!(!GpuGeneration::Kepler.supports_codec("hevc"));
        assert!(GpuGeneration::Maxwell.supports_codec("hevc"));
        assert!(GpuGeneration::Pascal.supports_codec("hevc"));
        assert!(GpuGeneration::Ampere.supports_codec("hevc"));

        // Test alternative names
        assert!(GpuGeneration::Maxwell.supports_codec("h265"));
        assert!(GpuGeneration::Maxwell.supports_codec("H265"));
    }

    #[test]
    fn test_codec_support_vp9() {
        // VP9 requires Pascal or newer
        assert!(!GpuGeneration::Unknown.supports_codec("vp9"));
        assert!(!GpuGeneration::Kepler.supports_codec("vp9"));
        assert!(!GpuGeneration::Maxwell.supports_codec("vp9"));
        assert!(GpuGeneration::Pascal.supports_codec("vp9"));
        assert!(GpuGeneration::Turing.supports_codec("vp9"));
        assert!(GpuGeneration::Ampere.supports_codec("vp9"));
    }

    #[test]
    fn test_codec_support_av1() {
        // AV1 requires Ampere or newer
        assert!(!GpuGeneration::Unknown.supports_codec("av1"));
        assert!(!GpuGeneration::Kepler.supports_codec("av1"));
        assert!(!GpuGeneration::Maxwell.supports_codec("av1"));
        assert!(!GpuGeneration::Pascal.supports_codec("av1"));
        assert!(!GpuGeneration::Turing.supports_codec("av1"));
        assert!(GpuGeneration::Ampere.supports_codec("av1"));
        assert!(GpuGeneration::Ada.supports_codec("av1"));
        assert!(GpuGeneration::Blackwell.supports_codec("av1"));
    }

    #[test]
    fn test_codec_support_unsupported_codecs() {
        // Test that unsupported codecs return false
        assert!(!GpuGeneration::Ampere.supports_codec("prores"));
        assert!(!GpuGeneration::Ampere.supports_codec("dnxhd"));
        assert!(!GpuGeneration::Ampere.supports_codec("ffv1"));
        assert!(!GpuGeneration::Ampere.supports_codec("unknown_codec"));
    }

    #[test]
    fn test_hwaccel_type_ffmpeg_args() {
        // Test that HWAccelType produces correct FFmpeg arguments
        assert_eq!(HWAccelType::Auto.as_ffmpeg_arg(), Some("auto"));
        assert_eq!(HWAccelType::Cuda.as_ffmpeg_arg(), Some("cuda"));
        assert_eq!(HWAccelType::Vaapi.as_ffmpeg_arg(), Some("vaapi"));
        assert_eq!(HWAccelType::Dxva2.as_ffmpeg_arg(), Some("dxva2"));
        assert_eq!(HWAccelType::Videotoolbox.as_ffmpeg_arg(), Some("videotoolbox"));
        assert_eq!(HWAccelType::None.as_ffmpeg_arg(), None);
    }

    #[test]
    fn test_nvidia_gpu_info_default() {
        // Test default values for NvidiaGpuInfo
        let info = NvidiaGpuInfo::default();
        assert!(info.name.is_empty());
        assert_eq!(info.generation, GpuGeneration::Unknown);
        assert!(info.compute_capability.is_empty());
        assert_eq!(info.device_id, 0);
    }

    #[test]
    fn test_gpu_generation_default() {
        // Test that default GPU generation is Unknown
        assert_eq!(GpuGeneration::default(), GpuGeneration::Unknown);
    }

    #[test]
    fn test_hwaccel_type_default() {
        // Test that default HWAccelType is Auto
        assert_eq!(HWAccelType::default(), HWAccelType::Auto);
    }

    // Edge case: Test boundary conditions for compute capability parsing
    #[test]
    fn test_compute_capability_edge_cases() {
        // Single digit without minor version
        assert_eq!(GpuGeneration::from_compute_capability("8"), GpuGeneration::Ampere);

        // Whitespace handling (should be trimmed before calling)
        assert_eq!(GpuGeneration::from_compute_capability("8.6"), GpuGeneration::Ampere);

        // Extra precision (real-world scenario)
        // Note: This tests that we only look at major.minor
        assert_eq!(GpuGeneration::from_compute_capability("8.6.0"), GpuGeneration::Ampere);
    }

    // Hypothetical issue: MPEG2 codec name variations
    #[test]
    fn test_mpeg2_codec_name_variations() {
        assert!(GpuGeneration::Kepler.supports_codec("mpeg2"));
        assert!(GpuGeneration::Kepler.supports_codec("mpeg2video"));
        assert!(GpuGeneration::Kepler.supports_codec("MPEG2"));
        assert!(GpuGeneration::Kepler.supports_codec("MPEG2VIDEO"));
    }

    // Hypothetical issue: VP8 codec support (Maxwell 2nd gen+)
    #[test]
    fn test_vp8_codec_support() {
        assert!(!GpuGeneration::Kepler.supports_codec("vp8"));
        assert!(GpuGeneration::Maxwell.supports_codec("vp8"));
        assert!(GpuGeneration::Pascal.supports_codec("vp8"));
        assert!(GpuGeneration::Ampere.supports_codec("vp8"));
    }

    // Test serialization/deserialization round-trip
    #[test]
    fn test_gpu_generation_serialization() {
        let original = GpuGeneration::Ampere;
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: GpuGeneration = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_hwaccel_type_serialization() {
        let original = HWAccelType::Cuda;
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: HWAccelType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_nvidia_gpu_info_serialization() {
        let original = NvidiaGpuInfo {
            name: "NVIDIA GeForce RTX 3080".to_string(),
            generation: GpuGeneration::Ampere,
            compute_capability: "8.6".to_string(),
            device_id: 0,
        };
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: NvidiaGpuInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original.name, deserialized.name);
        assert_eq!(original.generation, deserialized.generation);
        assert_eq!(original.compute_capability, deserialized.compute_capability);
        assert_eq!(original.device_id, deserialized.device_id);
    }
}
