// Codec Detection Module
// Detects video codec properties using ffprobe for GPU compatibility checking

use std::path::Path;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::common::gpu_detection::GpuGeneration;
use crate::common::process_utils::disable_windows_console_window;

/// Information about a video's codec and format
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VideoCodecInfo {
    pub codec_name: String,
    pub codec_type: CodecType,
    pub pixel_format: String,
    pub profile: String,
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub duration: Option<f64>,
}

/// Enumeration of known codec types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CodecType {
    H264,
    H265,
    VP9,
    AV1,
    MPEG2,
    VP8,
    #[default]
    Other,
}

impl CodecType {
    /// Parse codec type from codec name string
    pub fn from_codec_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "h264" | "avc" | "avc1" => CodecType::H264,
            "hevc" | "h265" | "hev1" | "hvc1" => CodecType::H265,
            "vp9" | "vp09" => CodecType::VP9,
            "av1" | "av01" => CodecType::AV1,
            "mpeg2video" | "mpeg2" => CodecType::MPEG2,
            "vp8" => CodecType::VP8,
            _ => CodecType::Other,
        }
    }

    /// Check if this codec type is supported by NVDEC on the given GPU generation
    pub fn is_nvdec_supported(&self, gpu_generation: GpuGeneration) -> bool {
        match self {
            CodecType::H264 => gpu_generation >= GpuGeneration::Kepler,
            CodecType::H265 => gpu_generation >= GpuGeneration::Maxwell,
            CodecType::VP9 => gpu_generation >= GpuGeneration::Pascal,
            CodecType::AV1 => gpu_generation >= GpuGeneration::Ampere,
            CodecType::MPEG2 => gpu_generation >= GpuGeneration::Kepler,
            CodecType::VP8 => gpu_generation >= GpuGeneration::Maxwell,
            CodecType::Other => false,
        }
    }

    /// Get the human-readable name for this codec type
    pub fn display_name(&self) -> &'static str {
        match self {
            CodecType::H264 => "H.264/AVC",
            CodecType::H265 => "H.265/HEVC",
            CodecType::VP9 => "VP9",
            CodecType::AV1 => "AV1",
            CodecType::MPEG2 => "MPEG-2",
            CodecType::VP8 => "VP8",
            CodecType::Other => "Other/Unknown",
        }
    }
}

/// Reason why a video might need transcoding
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranscodeReason {
    UnsupportedCodec(String),
    UnsupportedPixelFormat(String),
    UnsupportedProfile(String),
    IncompatibleGpuGeneration,
    ResolutionTooHigh,
}

impl TranscodeReason {
    pub fn display_message(&self) -> String {
        match self {
            TranscodeReason::UnsupportedCodec(codec) => format!("Unsupported codec: {codec}"),
            TranscodeReason::UnsupportedPixelFormat(fmt) => format!("Unsupported pixel format: {fmt}"),
            TranscodeReason::UnsupportedProfile(profile) => format!("Unsupported profile: {profile}"),
            TranscodeReason::IncompatibleGpuGeneration => "GPU generation too old".to_string(),
            TranscodeReason::ResolutionTooHigh => "Resolution exceeds GPU limits".to_string(),
        }
    }
}

/// Codec detector for analyzing video files
pub struct CodecDetector;

impl CodecDetector {
    /// Detect video codec information using ffprobe
    pub fn detect_codec(video_path: &Path) -> Result<VideoCodecInfo, String> {
        let mut command = Command::new("ffprobe");
        disable_windows_console_window(&mut command);

        let output = command
            .args([
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_streams",
                "-show_format",
                "-select_streams",
                "v:0",
            ])
            .arg(video_path)
            .output()
            .map_err(|e| format!("ffprobe execution failed: {e}"))?;

        if !output.status.success() {
            return Err(format!(
                "ffprobe returned error for \"{}\"",
                video_path.display()
            ));
        }

        let json: serde_json::Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("Failed to parse ffprobe JSON output: {e}"))?;

        Self::parse_ffprobe_output(&json)
    }

    /// Parse ffprobe JSON output into VideoCodecInfo
    fn parse_ffprobe_output(json: &serde_json::Value) -> Result<VideoCodecInfo, String> {
        let streams = json["streams"].as_array();
        let stream = streams
            .and_then(|s| s.first())
            .ok_or("No video stream found in ffprobe output")?;

        let codec_name = stream["codec_name"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let pixel_format = stream["pix_fmt"]
            .as_str()
            .unwrap_or("yuv420p")
            .to_string();

        let profile = stream["profile"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let width = stream["width"].as_u64().unwrap_or(0) as u32;
        let height = stream["height"].as_u64().unwrap_or(0) as u32;

        // Detect bit depth from pixel format
        let bit_depth = Self::detect_bit_depth(&pixel_format);

        // Try to get duration from format or stream
        let duration = json["format"]["duration"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .or_else(|| {
                stream["duration"]
                    .as_str()
                    .and_then(|s| s.parse::<f64>().ok())
            });

        let codec_type = CodecType::from_codec_name(&codec_name);

        Ok(VideoCodecInfo {
            codec_name,
            codec_type,
            pixel_format,
            profile,
            width,
            height,
            bit_depth,
            duration,
        })
    }

    /// Detect bit depth from pixel format string
    fn detect_bit_depth(pixel_format: &str) -> u8 {
        if pixel_format.contains("10le") || pixel_format.contains("10be") || pixel_format.contains("p010") {
            10
        } else if pixel_format.contains("12le") || pixel_format.contains("12be") || pixel_format.contains("p012") {
            12
        } else if pixel_format.contains("16le") || pixel_format.contains("16be") {
            16
        } else {
            8
        }
    }

    /// Check if a pixel format is supported by NVDEC
    pub fn is_pixel_format_supported(pixel_format: &str) -> bool {
        let format_lower = pixel_format.to_lowercase();
        
        // Supported 4:2:0 formats (most common)
        let supported_420 = [
            "yuv420p", "yuv420p10le", "yuv420p12le",
            "nv12", "p010le", "p016le", "p010", "p012",
        ];
        
        // 4:2:2 and 4:4:4 formats are generally NOT supported by NVDEC
        let unsupported_chroma = ["422", "444", "yuv422", "yuv444"];
        
        // Check if it's an unsupported chroma format
        for unsupported in unsupported_chroma {
            if format_lower.contains(unsupported) {
                return false;
            }
        }
        
        // Check if it's a known supported format
        for supported in supported_420 {
            if format_lower == supported || format_lower.starts_with(supported) {
                return true;
            }
        }
        
        // Default to true for unknown formats (let NVDEC decide)
        // This prevents false negatives for new/uncommon supported formats
        true
    }

    /// Get maximum resolution supported by NVDEC for a given GPU generation
    pub fn get_max_resolution(gpu_generation: GpuGeneration) -> (u32, u32) {
        match gpu_generation {
            GpuGeneration::Unknown => (1920, 1080),
            GpuGeneration::Kepler | GpuGeneration::Maxwell => (4096, 2160),
            GpuGeneration::Pascal | GpuGeneration::Volta => (4096, 4096),
            _ => (16384, 16384), // Turing and later
        }
    }

    /// Check if video resolution is within GPU limits
    pub fn is_resolution_supported(width: u32, height: u32, gpu_generation: GpuGeneration) -> bool {
        let (max_width, max_height) = Self::get_max_resolution(gpu_generation);
        let max_pixels = max_width as u64 * max_height as u64;
        let video_pixels = width as u64 * height as u64;
        
        video_pixels <= max_pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Phase 1 Unit Tests =====
    // Tests based on hypothetical issues that could cause problems

    #[test]
    fn test_codec_type_from_common_names() {
        // Test common codec name variations
        assert_eq!(CodecType::from_codec_name("h264"), CodecType::H264);
        assert_eq!(CodecType::from_codec_name("H264"), CodecType::H264);
        assert_eq!(CodecType::from_codec_name("avc"), CodecType::H264);
        assert_eq!(CodecType::from_codec_name("avc1"), CodecType::H264);
        
        assert_eq!(CodecType::from_codec_name("hevc"), CodecType::H265);
        assert_eq!(CodecType::from_codec_name("h265"), CodecType::H265);
        assert_eq!(CodecType::from_codec_name("hev1"), CodecType::H265);
        assert_eq!(CodecType::from_codec_name("hvc1"), CodecType::H265);
        
        assert_eq!(CodecType::from_codec_name("vp9"), CodecType::VP9);
        assert_eq!(CodecType::from_codec_name("vp09"), CodecType::VP9);
        
        assert_eq!(CodecType::from_codec_name("av1"), CodecType::AV1);
        assert_eq!(CodecType::from_codec_name("av01"), CodecType::AV1);
        
        assert_eq!(CodecType::from_codec_name("mpeg2video"), CodecType::MPEG2);
        assert_eq!(CodecType::from_codec_name("mpeg2"), CodecType::MPEG2);
        
        assert_eq!(CodecType::from_codec_name("vp8"), CodecType::VP8);
    }

    #[test]
    fn test_codec_type_unknown_codecs() {
        // Test that unknown codecs return Other
        assert_eq!(CodecType::from_codec_name("prores"), CodecType::Other);
        assert_eq!(CodecType::from_codec_name("dnxhd"), CodecType::Other);
        assert_eq!(CodecType::from_codec_name("ffv1"), CodecType::Other);
        assert_eq!(CodecType::from_codec_name("mjpeg"), CodecType::Other);
        assert_eq!(CodecType::from_codec_name("random_codec"), CodecType::Other);
        assert_eq!(CodecType::from_codec_name(""), CodecType::Other);
    }

    #[test]
    fn test_codec_type_nvdec_support() {
        // Test NVDEC support for each codec type
        
        // H.264 - supported on Kepler+
        assert!(CodecType::H264.is_nvdec_supported(GpuGeneration::Kepler));
        assert!(CodecType::H264.is_nvdec_supported(GpuGeneration::Ampere));
        assert!(!CodecType::H264.is_nvdec_supported(GpuGeneration::Unknown));
        
        // H.265 - supported on Maxwell+
        assert!(!CodecType::H265.is_nvdec_supported(GpuGeneration::Kepler));
        assert!(CodecType::H265.is_nvdec_supported(GpuGeneration::Maxwell));
        assert!(CodecType::H265.is_nvdec_supported(GpuGeneration::Ampere));
        
        // VP9 - supported on Pascal+
        assert!(!CodecType::VP9.is_nvdec_supported(GpuGeneration::Maxwell));
        assert!(CodecType::VP9.is_nvdec_supported(GpuGeneration::Pascal));
        assert!(CodecType::VP9.is_nvdec_supported(GpuGeneration::Ampere));
        
        // AV1 - supported on Ampere+
        assert!(!CodecType::AV1.is_nvdec_supported(GpuGeneration::Turing));
        assert!(CodecType::AV1.is_nvdec_supported(GpuGeneration::Ampere));
        assert!(CodecType::AV1.is_nvdec_supported(GpuGeneration::Ada));
        
        // Other - never supported
        assert!(!CodecType::Other.is_nvdec_supported(GpuGeneration::Ampere));
    }

    #[test]
    fn test_codec_type_display_names() {
        assert_eq!(CodecType::H264.display_name(), "H.264/AVC");
        assert_eq!(CodecType::H265.display_name(), "H.265/HEVC");
        assert_eq!(CodecType::VP9.display_name(), "VP9");
        assert_eq!(CodecType::AV1.display_name(), "AV1");
        assert_eq!(CodecType::MPEG2.display_name(), "MPEG-2");
        assert_eq!(CodecType::VP8.display_name(), "VP8");
        assert_eq!(CodecType::Other.display_name(), "Other/Unknown");
    }

    #[test]
    fn test_pixel_format_supported_420_formats() {
        // Common 4:2:0 formats should be supported
        assert!(CodecDetector::is_pixel_format_supported("yuv420p"));
        assert!(CodecDetector::is_pixel_format_supported("nv12"));
        assert!(CodecDetector::is_pixel_format_supported("yuv420p10le"));
        assert!(CodecDetector::is_pixel_format_supported("p010le"));
        assert!(CodecDetector::is_pixel_format_supported("p010"));
    }

    #[test]
    fn test_pixel_format_unsupported_422_formats() {
        // 4:2:2 formats should NOT be supported
        assert!(!CodecDetector::is_pixel_format_supported("yuv422p"));
        assert!(!CodecDetector::is_pixel_format_supported("yuv422p10le"));
        assert!(!CodecDetector::is_pixel_format_supported("422"));
    }

    #[test]
    fn test_pixel_format_unsupported_444_formats() {
        // 4:4:4 formats should NOT be supported
        assert!(!CodecDetector::is_pixel_format_supported("yuv444p"));
        assert!(!CodecDetector::is_pixel_format_supported("yuv444p10le"));
        assert!(!CodecDetector::is_pixel_format_supported("444"));
    }

    #[test]
    fn test_pixel_format_case_insensitivity() {
        // Test case insensitivity
        assert!(CodecDetector::is_pixel_format_supported("YUV420P"));
        assert!(CodecDetector::is_pixel_format_supported("NV12"));
        assert!(!CodecDetector::is_pixel_format_supported("YUV422P"));
    }

    #[test]
    fn test_bit_depth_detection() {
        assert_eq!(CodecDetector::detect_bit_depth("yuv420p"), 8);
        assert_eq!(CodecDetector::detect_bit_depth("yuv420p10le"), 10);
        assert_eq!(CodecDetector::detect_bit_depth("yuv420p10be"), 10);
        assert_eq!(CodecDetector::detect_bit_depth("p010le"), 10);
        assert_eq!(CodecDetector::detect_bit_depth("yuv420p12le"), 12);
        assert_eq!(CodecDetector::detect_bit_depth("p012"), 12);
        assert_eq!(CodecDetector::detect_bit_depth("yuv420p16le"), 16);
        assert_eq!(CodecDetector::detect_bit_depth("unknown_format"), 8);
    }

    #[test]
    fn test_max_resolution_by_gpu_generation() {
        // Test max resolution for different GPU generations
        let (w, h) = CodecDetector::get_max_resolution(GpuGeneration::Unknown);
        assert_eq!((w, h), (1920, 1080));
        
        let (w, h) = CodecDetector::get_max_resolution(GpuGeneration::Kepler);
        assert_eq!((w, h), (4096, 2160));
        
        let (w, h) = CodecDetector::get_max_resolution(GpuGeneration::Pascal);
        assert_eq!((w, h), (4096, 4096));
        
        let (w, h) = CodecDetector::get_max_resolution(GpuGeneration::Turing);
        assert_eq!((w, h), (16384, 16384));
        
        let (w, h) = CodecDetector::get_max_resolution(GpuGeneration::Ampere);
        assert_eq!((w, h), (16384, 16384));
    }

    #[test]
    fn test_resolution_support() {
        // Test resolution support checking
        
        // 1080p should be supported on all GPUs
        assert!(CodecDetector::is_resolution_supported(1920, 1080, GpuGeneration::Kepler));
        assert!(CodecDetector::is_resolution_supported(1920, 1080, GpuGeneration::Unknown));
        
        // 4K should be supported on Kepler+
        assert!(CodecDetector::is_resolution_supported(3840, 2160, GpuGeneration::Kepler));
        assert!(CodecDetector::is_resolution_supported(3840, 2160, GpuGeneration::Ampere));
        
        // 8K should only be supported on Turing+
        assert!(!CodecDetector::is_resolution_supported(7680, 4320, GpuGeneration::Kepler));
        assert!(!CodecDetector::is_resolution_supported(7680, 4320, GpuGeneration::Pascal));
        assert!(CodecDetector::is_resolution_supported(7680, 4320, GpuGeneration::Turing));
        assert!(CodecDetector::is_resolution_supported(7680, 4320, GpuGeneration::Ampere));
    }

    #[test]
    fn test_transcode_reason_display_messages() {
        assert_eq!(
            TranscodeReason::UnsupportedCodec("ProRes".to_string()).display_message(),
            "Unsupported codec: ProRes"
        );
        assert_eq!(
            TranscodeReason::UnsupportedPixelFormat("yuv422p".to_string()).display_message(),
            "Unsupported pixel format: yuv422p"
        );
        assert_eq!(
            TranscodeReason::UnsupportedProfile("High 4:2:2".to_string()).display_message(),
            "Unsupported profile: High 4:2:2"
        );
        assert_eq!(
            TranscodeReason::IncompatibleGpuGeneration.display_message(),
            "GPU generation too old"
        );
        assert_eq!(
            TranscodeReason::ResolutionTooHigh.display_message(),
            "Resolution exceeds GPU limits"
        );
    }

    #[test]
    fn test_video_codec_info_default() {
        let info = VideoCodecInfo::default();
        assert!(info.codec_name.is_empty());
        assert_eq!(info.codec_type, CodecType::Other);
        assert!(info.pixel_format.is_empty());
        assert!(info.profile.is_empty());
        assert_eq!(info.width, 0);
        assert_eq!(info.height, 0);
        assert_eq!(info.bit_depth, 0);
        assert!(info.duration.is_none());
    }

    #[test]
    fn test_codec_type_default() {
        assert_eq!(CodecType::default(), CodecType::Other);
    }

    // Hypothetical issue: Test edge case with very large resolution
    #[test]
    fn test_resolution_overflow_protection() {
        // Test that we don't overflow when calculating pixel counts
        let result = CodecDetector::is_resolution_supported(u32::MAX, u32::MAX, GpuGeneration::Ampere);
        // This should not panic and should return false (too large)
        assert!(!result);
    }

    // Hypothetical issue: Test empty pixel format
    #[test]
    fn test_empty_pixel_format() {
        // Empty pixel format should default to supported (conservative)
        assert!(CodecDetector::is_pixel_format_supported(""));
    }

    // Test serialization/deserialization
    #[test]
    fn test_codec_type_serialization() {
        let original = CodecType::H264;
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: CodecType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_transcode_reason_serialization() {
        let original = TranscodeReason::UnsupportedCodec("test".to_string());
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: TranscodeReason = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_video_codec_info_serialization() {
        let original = VideoCodecInfo {
            codec_name: "h264".to_string(),
            codec_type: CodecType::H264,
            pixel_format: "yuv420p".to_string(),
            profile: "High".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 8,
            duration: Some(120.5),
        };
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: VideoCodecInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original.codec_name, deserialized.codec_name);
        assert_eq!(original.codec_type, deserialized.codec_type);
        assert_eq!(original.width, deserialized.width);
        assert_eq!(original.height, deserialized.height);
        assert_eq!(original.duration, deserialized.duration);
    }

    // Hypothetical issue: Test parse_ffprobe_output with missing fields
    #[test]
    fn test_parse_ffprobe_output_missing_stream() {
        let json: serde_json::Value = serde_json::json!({
            "streams": [],
            "format": {}
        });
        
        let result = CodecDetector::parse_ffprobe_output(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No video stream found"));
    }

    #[test]
    fn test_parse_ffprobe_output_minimal_valid() {
        let json: serde_json::Value = serde_json::json!({
            "streams": [{}],
            "format": {}
        });
        
        let result = CodecDetector::parse_ffprobe_output(&json);
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.codec_name, "unknown");
        assert_eq!(info.codec_type, CodecType::Other);
    }

    #[test]
    fn test_parse_ffprobe_output_complete() {
        let json: serde_json::Value = serde_json::json!({
            "streams": [{
                "codec_name": "h264",
                "pix_fmt": "yuv420p10le",
                "profile": "High",
                "width": 1920,
                "height": 1080,
                "duration": "120.5"
            }],
            "format": {
                "duration": "120.5"
            }
        });
        
        let result = CodecDetector::parse_ffprobe_output(&json);
        assert!(result.is_ok());
        let info = result.unwrap();
        assert_eq!(info.codec_name, "h264");
        assert_eq!(info.codec_type, CodecType::H264);
        assert_eq!(info.pixel_format, "yuv420p10le");
        assert_eq!(info.bit_depth, 10);
        assert_eq!(info.width, 1920);
        assert_eq!(info.height, 1080);
        assert_eq!(info.duration, Some(120.5));
    }
}
