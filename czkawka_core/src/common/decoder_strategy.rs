// Decoder Strategy Module
// Provides intelligent decoder selection (GPU vs CPU) based on video properties and hardware capabilities

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::common::codec_detection::{CodecDetector, TranscodeReason, VideoCodecInfo};
use crate::common::gpu_detection::{GpuDetector, HWAccelType, NvidiaGpuInfo};

/// Strategy for decoder selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DecoderStrategy {
    /// Force GPU decoding, fail if unsupported
    GpuOnly,
    /// Force CPU decoding (no hardware acceleration)
    CpuOnly,
    /// Try GPU first, fallback to CPU if needed
    #[default]
    GpuPreferred,
    /// Automatically select based on codec/format detection
    Auto,
}

impl DecoderStrategy {
    /// Get human-readable name for the strategy
    pub fn display_name(&self) -> &'static str {
        match self {
            DecoderStrategy::GpuOnly => "GPU Only",
            DecoderStrategy::CpuOnly => "CPU Only",
            DecoderStrategy::GpuPreferred => "GPU Preferred",
            DecoderStrategy::Auto => "Automatic",
        }
    }
}

/// Result of decoder selection
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoderChoice {
    /// Use GPU decoder
    Gpu,
    /// Use CPU decoder
    Cpu,
    /// Decoder selection failed with an error
    Error(String),
}

/// Video compatibility status with GPU acceleration
#[derive(Debug, Clone)]
pub enum VideoCompatibility {
    /// Video is ready for GPU processing
    GpuReady,
    /// Video needs transcoding for GPU compatibility
    NeedsTranscode(TranscodeReason),
    /// Video can only be processed with CPU
    CpuOnly,
    /// Failed to analyze video
    Failed(String),
}

/// Decoder selector that chooses the best decoder for each video
pub struct DecoderSelector {
    gpu_info: Option<NvidiaGpuInfo>,
    strategy: DecoderStrategy,
    hwaccel_type: HWAccelType,
}

impl DecoderSelector {
    /// Create a new decoder selector with the given strategy
    pub fn new(strategy: DecoderStrategy) -> Self {
        let gpu_info = if strategy != DecoderStrategy::CpuOnly {
            GpuDetector::detect_nvidia_gpu()
        } else {
            None
        };

        Self {
            gpu_info,
            strategy,
            hwaccel_type: HWAccelType::Auto,
        }
    }

    /// Create a decoder selector with specific GPU info (for testing)
    pub fn with_gpu_info(strategy: DecoderStrategy, gpu_info: Option<NvidiaGpuInfo>) -> Self {
        Self {
            gpu_info,
            strategy,
            hwaccel_type: HWAccelType::Auto,
        }
    }

    /// Set the hardware acceleration type
    pub fn set_hwaccel_type(&mut self, hwaccel_type: HWAccelType) {
        self.hwaccel_type = hwaccel_type;
    }

    /// Get the GPU info
    pub fn get_gpu_info(&self) -> Option<&NvidiaGpuInfo> {
        self.gpu_info.as_ref()
    }

    /// Determine the best decoder for a video file
    pub fn select_decoder(&self, video_path: &Path) -> DecoderChoice {
        match self.strategy {
            DecoderStrategy::CpuOnly => return DecoderChoice::Cpu,
            DecoderStrategy::GpuOnly => {
                if self.gpu_info.is_none() {
                    return DecoderChoice::Error("No GPU detected, but GPU-only mode requested".into());
                }
                return DecoderChoice::Gpu;
            }
            _ => {}
        }

        // For Auto and GpuPreferred, detect codec and check compatibility
        let codec_info = match CodecDetector::detect_codec(video_path) {
            Ok(info) => info,
            Err(e) => {
                // If we can't detect codec, fallback based on strategy
                return match self.strategy {
                    DecoderStrategy::GpuPreferred => DecoderChoice::Cpu,
                    DecoderStrategy::Auto => DecoderChoice::Cpu,
                    _ => DecoderChoice::Error(e),
                };
            }
        };

        self.select_decoder_for_codec(&codec_info)
    }

    /// Determine the best decoder based on codec info
    pub fn select_decoder_for_codec(&self, codec_info: &VideoCodecInfo) -> DecoderChoice {
        // Check GPU availability
        let gpu_info = match &self.gpu_info {
            Some(info) => info,
            None => return DecoderChoice::Cpu,
        };

        // Check codec support
        if !codec_info.codec_type.is_nvdec_supported(gpu_info.generation) {
            return DecoderChoice::Cpu;
        }

        // Check pixel format support
        if !CodecDetector::is_pixel_format_supported(&codec_info.pixel_format) {
            return DecoderChoice::Cpu;
        }

        // Check resolution support
        if !CodecDetector::is_resolution_supported(
            codec_info.width,
            codec_info.height,
            gpu_info.generation,
        ) {
            return DecoderChoice::Cpu;
        }

        DecoderChoice::Gpu
    }

    /// Analyze video compatibility with GPU acceleration
    pub fn analyze_video_compatibility(&self, video_path: &Path) -> VideoCompatibility {
        // If not using GPU, everything goes to CPU
        if self.strategy == DecoderStrategy::CpuOnly {
            return VideoCompatibility::CpuOnly;
        }

        let gpu_info = match &self.gpu_info {
            Some(info) => info,
            None => return VideoCompatibility::CpuOnly,
        };

        // Detect codec
        let codec_info = match CodecDetector::detect_codec(video_path) {
            Ok(info) => info,
            Err(e) => return VideoCompatibility::Failed(e),
        };

        self.analyze_codec_compatibility(&codec_info, gpu_info)
    }

    /// Analyze codec compatibility with GPU
    fn analyze_codec_compatibility(
        &self,
        codec_info: &VideoCodecInfo,
        gpu_info: &NvidiaGpuInfo,
    ) -> VideoCompatibility {
        // Check codec support
        if !codec_info.codec_type.is_nvdec_supported(gpu_info.generation) {
            return VideoCompatibility::NeedsTranscode(TranscodeReason::UnsupportedCodec(
                codec_info.codec_name.clone(),
            ));
        }

        // Check pixel format
        if !CodecDetector::is_pixel_format_supported(&codec_info.pixel_format) {
            return VideoCompatibility::NeedsTranscode(TranscodeReason::UnsupportedPixelFormat(
                codec_info.pixel_format.clone(),
            ));
        }

        // Check resolution
        if !CodecDetector::is_resolution_supported(
            codec_info.width,
            codec_info.height,
            gpu_info.generation,
        ) {
            return VideoCompatibility::NeedsTranscode(TranscodeReason::ResolutionTooHigh);
        }

        VideoCompatibility::GpuReady
    }
}

/// Scan mode for video similarity detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ScanMode {
    /// Fast mode: Single hash per video (current behavior)
    #[default]
    Fast,
    /// Balanced mode: 3-5 strategic chunks (3-5x slower)
    Balanced,
    /// Thorough mode: 10+ chunks for heavily edited videos (10x+ slower)
    Thorough,
}

impl ScanMode {
    /// Get human-readable name for the scan mode
    pub fn display_name(&self) -> &'static str {
        match self {
            ScanMode::Fast => "Fast (Single Hash)",
            ScanMode::Balanced => "Balanced (3-5 Chunks)",
            ScanMode::Thorough => "Thorough (10+ Chunks)",
        }
    }

    /// Get the number of chunks to use for this scan mode
    pub fn chunk_count(&self) -> usize {
        match self {
            ScanMode::Fast => 1,
            ScanMode::Balanced => 5,
            ScanMode::Thorough => 10,
        }
    }

    /// Get estimated time multiplier compared to Fast mode
    pub fn time_multiplier(&self) -> f64 {
        match self {
            ScanMode::Fast => 1.0,
            ScanMode::Balanced => 4.0,
            ScanMode::Thorough => 10.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::codec_detection::CodecType;
    use crate::common::gpu_detection::GpuGeneration;

    // ===== Phase 1 Unit Tests =====
    // Tests based on hypothetical issues that could cause problems

    #[test]
    fn test_decoder_strategy_display_names() {
        assert_eq!(DecoderStrategy::GpuOnly.display_name(), "GPU Only");
        assert_eq!(DecoderStrategy::CpuOnly.display_name(), "CPU Only");
        assert_eq!(DecoderStrategy::GpuPreferred.display_name(), "GPU Preferred");
        assert_eq!(DecoderStrategy::Auto.display_name(), "Automatic");
    }

    #[test]
    fn test_decoder_strategy_default() {
        assert_eq!(DecoderStrategy::default(), DecoderStrategy::GpuPreferred);
    }

    #[test]
    fn test_scan_mode_display_names() {
        assert_eq!(ScanMode::Fast.display_name(), "Fast (Single Hash)");
        assert_eq!(ScanMode::Balanced.display_name(), "Balanced (3-5 Chunks)");
        assert_eq!(ScanMode::Thorough.display_name(), "Thorough (10+ Chunks)");
    }

    #[test]
    fn test_scan_mode_chunk_counts() {
        assert_eq!(ScanMode::Fast.chunk_count(), 1);
        assert_eq!(ScanMode::Balanced.chunk_count(), 5);
        assert_eq!(ScanMode::Thorough.chunk_count(), 10);
    }

    #[test]
    fn test_scan_mode_time_multipliers() {
        assert!((ScanMode::Fast.time_multiplier() - 1.0).abs() < f64::EPSILON);
        assert!((ScanMode::Balanced.time_multiplier() - 4.0).abs() < f64::EPSILON);
        assert!((ScanMode::Thorough.time_multiplier() - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_scan_mode_default() {
        assert_eq!(ScanMode::default(), ScanMode::Fast);
    }

    #[test]
    fn test_decoder_selector_cpu_only_strategy() {
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::CpuOnly, None);
        
        // CPU only should always return CPU, even with fake GPU info
        let selector_with_gpu = DecoderSelector::with_gpu_info(
            DecoderStrategy::CpuOnly,
            Some(NvidiaGpuInfo {
                name: "RTX 3080".to_string(),
                generation: GpuGeneration::Ampere,
                compute_capability: "8.6".to_string(),
                device_id: 0,
            }),
        );
        
        assert!(selector.get_gpu_info().is_none());
        assert!(selector_with_gpu.get_gpu_info().is_some());
    }

    #[test]
    fn test_decoder_selector_gpu_only_no_gpu() {
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::GpuOnly, None);
        
        // Create a fake codec info for testing
        let codec_info = VideoCodecInfo {
            codec_name: "h264".to_string(),
            codec_type: CodecType::H264,
            pixel_format: "yuv420p".to_string(),
            profile: "High".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 8,
            duration: Some(120.0),
        };
        
        let result = selector.select_decoder_for_codec(&codec_info);
        assert_eq!(result, DecoderChoice::Cpu); // No GPU available
    }

    #[test]
    fn test_decoder_selector_with_compatible_video() {
        let gpu_info = NvidiaGpuInfo {
            name: "RTX 3080".to_string(),
            generation: GpuGeneration::Ampere,
            compute_capability: "8.6".to_string(),
            device_id: 0,
        };
        
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::Auto, Some(gpu_info));
        
        // H.264 video with 4:2:0 format should be GPU compatible
        let codec_info = VideoCodecInfo {
            codec_name: "h264".to_string(),
            codec_type: CodecType::H264,
            pixel_format: "yuv420p".to_string(),
            profile: "High".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 8,
            duration: Some(120.0),
        };
        
        let result = selector.select_decoder_for_codec(&codec_info);
        assert_eq!(result, DecoderChoice::Gpu);
    }

    #[test]
    fn test_decoder_selector_with_unsupported_codec() {
        let gpu_info = NvidiaGpuInfo {
            name: "GTX 750".to_string(),
            generation: GpuGeneration::Maxwell,
            compute_capability: "5.0".to_string(),
            device_id: 0,
        };
        
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::Auto, Some(gpu_info));
        
        // AV1 is not supported on Maxwell
        let codec_info = VideoCodecInfo {
            codec_name: "av1".to_string(),
            codec_type: CodecType::AV1,
            pixel_format: "yuv420p".to_string(),
            profile: "Main".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 8,
            duration: Some(120.0),
        };
        
        let result = selector.select_decoder_for_codec(&codec_info);
        assert_eq!(result, DecoderChoice::Cpu);
    }

    #[test]
    fn test_decoder_selector_with_unsupported_pixel_format() {
        let gpu_info = NvidiaGpuInfo {
            name: "RTX 3080".to_string(),
            generation: GpuGeneration::Ampere,
            compute_capability: "8.6".to_string(),
            device_id: 0,
        };
        
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::Auto, Some(gpu_info));
        
        // H.264 with 4:2:2 format is not supported by NVDEC
        let codec_info = VideoCodecInfo {
            codec_name: "h264".to_string(),
            codec_type: CodecType::H264,
            pixel_format: "yuv422p".to_string(),
            profile: "High 4:2:2".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 8,
            duration: Some(120.0),
        };
        
        let result = selector.select_decoder_for_codec(&codec_info);
        assert_eq!(result, DecoderChoice::Cpu);
    }

    #[test]
    fn test_decoder_selector_with_high_resolution() {
        let gpu_info = NvidiaGpuInfo {
            name: "GTX 750".to_string(),
            generation: GpuGeneration::Kepler,
            compute_capability: "3.5".to_string(),
            device_id: 0,
        };
        
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::Auto, Some(gpu_info));
        
        // 8K video on Kepler GPU (max 4K)
        let codec_info = VideoCodecInfo {
            codec_name: "h264".to_string(),
            codec_type: CodecType::H264,
            pixel_format: "yuv420p".to_string(),
            profile: "High".to_string(),
            width: 7680,
            height: 4320,
            bit_depth: 8,
            duration: Some(120.0),
        };
        
        let result = selector.select_decoder_for_codec(&codec_info);
        assert_eq!(result, DecoderChoice::Cpu);
    }

    #[test]
    fn test_video_compatibility_gpu_ready() {
        let gpu_info = NvidiaGpuInfo {
            name: "RTX 3080".to_string(),
            generation: GpuGeneration::Ampere,
            compute_capability: "8.6".to_string(),
            device_id: 0,
        };
        
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::Auto, Some(gpu_info.clone()));
        
        let codec_info = VideoCodecInfo {
            codec_name: "h264".to_string(),
            codec_type: CodecType::H264,
            pixel_format: "yuv420p".to_string(),
            profile: "High".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 8,
            duration: Some(120.0),
        };
        
        let result = selector.analyze_codec_compatibility(&codec_info, &gpu_info);
        assert!(matches!(result, VideoCompatibility::GpuReady));
    }

    #[test]
    fn test_video_compatibility_needs_transcode_codec() {
        let gpu_info = NvidiaGpuInfo {
            name: "GTX 750".to_string(),
            generation: GpuGeneration::Kepler,
            compute_capability: "3.5".to_string(),
            device_id: 0,
        };
        
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::Auto, Some(gpu_info.clone()));
        
        // HEVC is not supported on Kepler
        let codec_info = VideoCodecInfo {
            codec_name: "hevc".to_string(),
            codec_type: CodecType::H265,
            pixel_format: "yuv420p".to_string(),
            profile: "Main".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 8,
            duration: Some(120.0),
        };
        
        let result = selector.analyze_codec_compatibility(&codec_info, &gpu_info);
        assert!(matches!(
            result,
            VideoCompatibility::NeedsTranscode(TranscodeReason::UnsupportedCodec(_))
        ));
    }

    #[test]
    fn test_video_compatibility_needs_transcode_pixel_format() {
        let gpu_info = NvidiaGpuInfo {
            name: "RTX 3080".to_string(),
            generation: GpuGeneration::Ampere,
            compute_capability: "8.6".to_string(),
            device_id: 0,
        };
        
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::Auto, Some(gpu_info.clone()));
        
        let codec_info = VideoCodecInfo {
            codec_name: "h264".to_string(),
            codec_type: CodecType::H264,
            pixel_format: "yuv444p".to_string(),
            profile: "High 4:4:4".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 8,
            duration: Some(120.0),
        };
        
        let result = selector.analyze_codec_compatibility(&codec_info, &gpu_info);
        assert!(matches!(
            result,
            VideoCompatibility::NeedsTranscode(TranscodeReason::UnsupportedPixelFormat(_))
        ));
    }

    #[test]
    fn test_video_compatibility_needs_transcode_resolution() {
        let gpu_info = NvidiaGpuInfo {
            name: "GTX 750".to_string(),
            generation: GpuGeneration::Kepler,
            compute_capability: "3.5".to_string(),
            device_id: 0,
        };
        
        let selector = DecoderSelector::with_gpu_info(DecoderStrategy::Auto, Some(gpu_info.clone()));
        
        // 8K video on Kepler
        let codec_info = VideoCodecInfo {
            codec_name: "h264".to_string(),
            codec_type: CodecType::H264,
            pixel_format: "yuv420p".to_string(),
            profile: "High".to_string(),
            width: 7680,
            height: 4320,
            bit_depth: 8,
            duration: Some(120.0),
        };
        
        let result = selector.analyze_codec_compatibility(&codec_info, &gpu_info);
        assert!(matches!(
            result,
            VideoCompatibility::NeedsTranscode(TranscodeReason::ResolutionTooHigh)
        ));
    }

    // Test serialization
    #[test]
    fn test_decoder_strategy_serialization() {
        let original = DecoderStrategy::GpuPreferred;
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: DecoderStrategy = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_scan_mode_serialization() {
        let original = ScanMode::Balanced;
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: ScanMode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    // Hypothetical issue: Test all GPU generations with all codecs
    #[test]
    fn test_comprehensive_codec_gpu_compatibility() {
        let generations = [
            GpuGeneration::Kepler,
            GpuGeneration::Maxwell,
            GpuGeneration::Pascal,
            GpuGeneration::Volta,
            GpuGeneration::Turing,
            GpuGeneration::Ampere,
            GpuGeneration::Ada,
            GpuGeneration::Blackwell,
        ];
        
        let codecs = [
            (CodecType::H264, GpuGeneration::Kepler),
            (CodecType::MPEG2, GpuGeneration::Kepler),
            (CodecType::H265, GpuGeneration::Maxwell),
            (CodecType::VP8, GpuGeneration::Maxwell),
            (CodecType::VP9, GpuGeneration::Pascal),
            (CodecType::AV1, GpuGeneration::Ampere),
        ];
        
        for (codec, min_gen) in codecs {
            for gpu_gen in &generations {
                let expected = *gpu_gen >= min_gen;
                let actual = codec.is_nvdec_supported(*gpu_gen);
                assert_eq!(
                    actual, expected,
                    "Codec {:?} support on {:?}: expected {}, got {}",
                    codec, gpu_gen, expected, actual
                );
            }
        }
    }

    // Hypothetical issue: Test decoder choice equality
    #[test]
    fn test_decoder_choice_equality() {
        assert_eq!(DecoderChoice::Gpu, DecoderChoice::Gpu);
        assert_eq!(DecoderChoice::Cpu, DecoderChoice::Cpu);
        assert_eq!(
            DecoderChoice::Error("test".to_string()),
            DecoderChoice::Error("test".to_string())
        );
        assert_ne!(DecoderChoice::Gpu, DecoderChoice::Cpu);
        assert_ne!(
            DecoderChoice::Error("test1".to_string()),
            DecoderChoice::Error("test2".to_string())
        );
    }
}
