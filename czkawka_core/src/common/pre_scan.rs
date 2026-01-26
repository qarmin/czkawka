// Pre-Scan Module
// Analyzes videos for GPU compatibility before processing

use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::common::codec_detection::{CodecDetector, TranscodeReason, VideoCodecInfo};
use crate::common::decoder_strategy::VideoCompatibility;
use crate::common::gpu_detection::{GpuDetector, NvidiaGpuInfo};

/// Results of pre-scanning videos for GPU compatibility
#[derive(Debug, Clone, Default)]
pub struct PreScanResults {
    pub total_videos: usize,
    pub gpu_compatible: Vec<PathBuf>,
    pub needs_transcoding: Vec<VideoTranscodeInfo>,
    pub cpu_only: Vec<PathBuf>,
    pub failed: Vec<(PathBuf, String)>,
}

impl PreScanResults {
    /// Get estimated total transcode time
    pub fn estimated_transcode_time(&self) -> Duration {
        self.needs_transcoding
            .iter()
            .map(|v| v.estimated_time)
            .sum()
    }

    /// Get estimated total transcode disk space
    pub fn estimated_transcode_space(&self) -> u64 {
        self.needs_transcoding
            .iter()
            .map(|v| v.estimated_transcode_size)
            .sum()
    }

    /// Check if any videos need transcoding
    pub fn has_transcoding_needed(&self) -> bool {
        !self.needs_transcoding.is_empty()
    }

    /// Get count of GPU-compatible videos
    pub fn gpu_compatible_count(&self) -> usize {
        self.gpu_compatible.len()
    }

    /// Get count of videos needing transcoding
    pub fn transcoding_needed_count(&self) -> usize {
        self.needs_transcoding.len()
    }

    /// Get count of CPU-only videos
    pub fn cpu_only_count(&self) -> usize {
        self.cpu_only.len()
    }

    /// Get count of failed videos
    pub fn failed_count(&self) -> usize {
        self.failed.len()
    }

    /// Generate a summary message for display
    pub fn summary_message(&self) -> String {
        format!(
            "Scanned {} videos: {} GPU-compatible, {} need transcoding, {} CPU-only, {} failed",
            self.total_videos,
            self.gpu_compatible.len(),
            self.needs_transcoding.len(),
            self.cpu_only.len(),
            self.failed.len()
        )
    }
}

/// Information about a video that needs transcoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoTranscodeInfo {
    pub path: PathBuf,
    pub reason: TranscodeReason,
    pub current_codec: String,
    pub current_format: String,
    pub file_size: u64,
    pub estimated_transcode_size: u64,
    pub estimated_time: Duration,
}

/// Estimated ratio of transcoded file size to original (H.264 typically ~50-70% of original)
const TRANSCODE_SIZE_RATIO: f64 = 0.6;

/// Estimated time multiplier for transcoding (roughly 1.2x realtime on modern CPU)
const TRANSCODE_TIME_MULTIPLIER: f64 = 1.2;

/// Default assumed video duration in seconds when unknown
const DEFAULT_VIDEO_DURATION_SECS: f64 = 60.0;

impl VideoTranscodeInfo {
    /// Create new transcode info from path and codec info
    pub fn new(
        path: PathBuf,
        reason: TranscodeReason,
        codec_info: &VideoCodecInfo,
        file_size: u64,
    ) -> Self {
        // Estimate transcode size using predefined ratio
        let estimated_transcode_size = (file_size as f64 * TRANSCODE_SIZE_RATIO) as u64;

        // Estimate transcode time using predefined multiplier
        let duration = codec_info.duration.unwrap_or(DEFAULT_VIDEO_DURATION_SECS);
        let estimated_time = Duration::from_secs_f64(duration * TRANSCODE_TIME_MULTIPLIER);

        Self {
            path,
            reason,
            current_codec: codec_info.codec_name.clone(),
            current_format: codec_info.pixel_format.clone(),
            file_size,
            estimated_transcode_size,
            estimated_time,
        }
    }

    /// Get a human-readable description of why transcoding is needed
    pub fn reason_display(&self) -> String {
        self.reason.display_message()
    }
}

/// Video pre-scanner for analyzing GPU compatibility
pub struct VideoPreScanner {
    gpu_info: Option<NvidiaGpuInfo>,
    use_gpu: bool,
}

impl VideoPreScanner {
    /// Create a new pre-scanner
    pub fn new(use_gpu: bool) -> Self {
        let gpu_info = if use_gpu {
            GpuDetector::detect_nvidia_gpu()
        } else {
            None
        };

        Self { gpu_info, use_gpu }
    }

    /// Create a pre-scanner with specific GPU info (for testing)
    pub fn with_gpu_info(use_gpu: bool, gpu_info: Option<NvidiaGpuInfo>) -> Self {
        Self { gpu_info, use_gpu }
    }

    /// Scan a list of video paths for GPU compatibility
    pub fn scan_videos(&self, video_paths: &[PathBuf]) -> PreScanResults {
        let mut results = PreScanResults {
            total_videos: video_paths.len(),
            ..Default::default()
        };

        for path in video_paths {
            match self.analyze_video(path) {
                VideoCompatibility::GpuReady => {
                    results.gpu_compatible.push(path.clone());
                }
                VideoCompatibility::NeedsTranscode(reason) => {
                    // Get codec info for detailed transcode info
                    if let Ok(codec_info) = CodecDetector::detect_codec(path) {
                        let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                        let info = VideoTranscodeInfo::new(
                            path.clone(),
                            reason,
                            &codec_info,
                            file_size,
                        );
                        results.needs_transcoding.push(info);
                    } else {
                        results.cpu_only.push(path.clone());
                    }
                }
                VideoCompatibility::CpuOnly => {
                    results.cpu_only.push(path.clone());
                }
                VideoCompatibility::Failed(error) => {
                    results.failed.push((path.clone(), error));
                }
            }
        }

        results
    }

    /// Analyze a single video for GPU compatibility
    fn analyze_video(&self, path: &PathBuf) -> VideoCompatibility {
        // If not using GPU, everything goes to CPU
        if !self.use_gpu {
            return VideoCompatibility::CpuOnly;
        }

        let gpu_info = match &self.gpu_info {
            Some(info) => info,
            None => return VideoCompatibility::CpuOnly,
        };

        // Detect codec
        let codec_info = match CodecDetector::detect_codec(path) {
            Ok(info) => info,
            Err(e) => return VideoCompatibility::Failed(e),
        };

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

    /// Get GPU info if available
    pub fn get_gpu_info(&self) -> Option<&NvidiaGpuInfo> {
        self.gpu_info.as_ref()
    }

    /// Check if GPU is available
    pub fn has_gpu(&self) -> bool {
        self.gpu_info.is_some()
    }
}

/// User decision on how to handle transcoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscodeDecision {
    /// Transcode all incompatible videos
    TranscodeAll,
    /// Use CPU fallback for incompatible videos
    UseCpuForIncompatible,
    /// Cancel the scan
    Cancel,
    /// No transcoding needed (all compatible)
    NoTranscodeNeeded,
}

/// Generate a prompt message for user confirmation
pub fn generate_transcode_prompt(results: &PreScanResults) -> String {
    if results.needs_transcoding.is_empty() {
        return "All videos are GPU-compatible!".to_string();
    }

    let total_incompatible = results.needs_transcoding.len();
    let total_space = format_size(results.estimated_transcode_space());
    let total_time = format_duration(results.estimated_transcode_time());

    let breakdown = generate_breakdown(results);

    format!(
        "Found {} videos incompatible with GPU acceleration:\n\n\
         {}\n\n\
         Transcoding will:\n\
         • Create temporary H.264 copies in tmp folder\n\
         • Use approximately {} of disk space\n\
         • Take approximately {} to complete\n\
         • Original files will NOT be modified\n\
         • Temporary files will be deleted after scan\n\n\
         Would you like to transcode these videos for GPU acceleration?\n\
         (Choosing 'No' will process them with CPU only - slower but still works)",
        total_incompatible, breakdown, total_space, total_time,
    )
}

/// Generate a breakdown of incompatible videos by codec and reason
fn generate_breakdown(results: &PreScanResults) -> String {
    use std::collections::HashMap;

    let mut codec_counts: HashMap<String, usize> = HashMap::new();
    let mut reason_counts: HashMap<String, usize> = HashMap::new();

    for video in &results.needs_transcoding {
        *codec_counts.entry(video.current_codec.clone()).or_insert(0) += 1;
        let reason_str = video.reason.display_message();
        *reason_counts.entry(reason_str).or_insert(0) += 1;
    }

    let mut breakdown = String::new();
    breakdown.push_str("Breakdown by codec:\n");
    for (codec, count) in codec_counts.iter() {
        breakdown.push_str(&format!("  • {}: {} files\n", codec, count));
    }

    breakdown.push_str("\nReasons:\n");
    for (reason, count) in reason_counts.iter() {
        breakdown.push_str(&format!("  • {}: {} files\n", reason, count));
    }

    breakdown
}

/// Format bytes as human-readable size
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Format duration as human-readable string with proper grammar
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        if secs == 1 {
            "1 second".to_string()
        } else {
            format!("{} seconds", secs)
        }
    } else if secs < 3600 {
        let minutes = secs / 60;
        if minutes == 1 {
            "1 minute".to_string()
        } else {
            format!("{} minutes", minutes)
        }
    } else {
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let hours_str = if hours == 1 { "1 hour" } else { &format!("{} hours", hours) };
        let minutes_str = if minutes == 1 { "1 minute" } else { &format!("{} minutes", minutes) };
        format!("{} {}", hours_str, minutes_str)
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
    fn test_pre_scan_results_default() {
        let results = PreScanResults::default();
        assert_eq!(results.total_videos, 0);
        assert!(results.gpu_compatible.is_empty());
        assert!(results.needs_transcoding.is_empty());
        assert!(results.cpu_only.is_empty());
        assert!(results.failed.is_empty());
    }

    #[test]
    fn test_pre_scan_results_counts() {
        let mut results = PreScanResults::default();
        results.gpu_compatible.push(PathBuf::from("/test/a.mp4"));
        results.gpu_compatible.push(PathBuf::from("/test/b.mp4"));
        results.cpu_only.push(PathBuf::from("/test/c.mp4"));
        results.failed.push((PathBuf::from("/test/d.mp4"), "error".to_string()));

        assert_eq!(results.gpu_compatible_count(), 2);
        assert_eq!(results.cpu_only_count(), 1);
        assert_eq!(results.failed_count(), 1);
        assert_eq!(results.transcoding_needed_count(), 0);
    }

    #[test]
    fn test_pre_scan_results_has_transcoding_needed() {
        let mut results = PreScanResults::default();
        assert!(!results.has_transcoding_needed());

        results.needs_transcoding.push(VideoTranscodeInfo {
            path: PathBuf::from("/test/a.mp4"),
            reason: TranscodeReason::UnsupportedCodec("prores".to_string()),
            current_codec: "prores".to_string(),
            current_format: "yuv422p10le".to_string(),
            file_size: 1000000,
            estimated_transcode_size: 600000,
            estimated_time: Duration::from_secs(60),
        });

        assert!(results.has_transcoding_needed());
    }

    #[test]
    fn test_pre_scan_results_estimated_totals() {
        let mut results = PreScanResults::default();

        results.needs_transcoding.push(VideoTranscodeInfo {
            path: PathBuf::from("/test/a.mp4"),
            reason: TranscodeReason::UnsupportedCodec("prores".to_string()),
            current_codec: "prores".to_string(),
            current_format: "yuv422p10le".to_string(),
            file_size: 1000000,
            estimated_transcode_size: 600000,
            estimated_time: Duration::from_secs(60),
        });

        results.needs_transcoding.push(VideoTranscodeInfo {
            path: PathBuf::from("/test/b.mp4"),
            reason: TranscodeReason::UnsupportedCodec("prores".to_string()),
            current_codec: "prores".to_string(),
            current_format: "yuv422p10le".to_string(),
            file_size: 2000000,
            estimated_transcode_size: 1200000,
            estimated_time: Duration::from_secs(120),
        });

        assert_eq!(results.estimated_transcode_space(), 1800000);
        assert_eq!(results.estimated_transcode_time(), Duration::from_secs(180));
    }

    #[test]
    fn test_pre_scan_results_summary_message() {
        let mut results = PreScanResults {
            total_videos: 10,
            ..Default::default()
        };
        results.gpu_compatible.push(PathBuf::from("/test/a.mp4"));
        results.gpu_compatible.push(PathBuf::from("/test/b.mp4"));
        results.cpu_only.push(PathBuf::from("/test/c.mp4"));

        let summary = results.summary_message();
        assert!(summary.contains("10 videos"));
        assert!(summary.contains("2 GPU-compatible"));
        assert!(summary.contains("1 CPU-only"));
    }

    #[test]
    fn test_video_transcode_info_new() {
        let codec_info = VideoCodecInfo {
            codec_name: "prores".to_string(),
            codec_type: CodecType::Other,
            pixel_format: "yuv422p10le".to_string(),
            profile: "HQ".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 10,
            duration: Some(120.0),
        };

        let info = VideoTranscodeInfo::new(
            PathBuf::from("/test/a.mov"),
            TranscodeReason::UnsupportedCodec("prores".to_string()),
            &codec_info,
            1000000,
        );

        assert_eq!(info.path, PathBuf::from("/test/a.mov"));
        assert_eq!(info.current_codec, "prores");
        assert_eq!(info.current_format, "yuv422p10le");
        assert_eq!(info.file_size, 1000000);
        // Estimated size should be ~60% of original
        assert_eq!(info.estimated_transcode_size, 600000);
        // Estimated time should be ~1.2x duration
        assert_eq!(info.estimated_time.as_secs(), 144);
    }

    #[test]
    fn test_video_transcode_info_reason_display() {
        let info = VideoTranscodeInfo {
            path: PathBuf::from("/test/a.mp4"),
            reason: TranscodeReason::UnsupportedCodec("prores".to_string()),
            current_codec: "prores".to_string(),
            current_format: "yuv422p10le".to_string(),
            file_size: 1000000,
            estimated_transcode_size: 600000,
            estimated_time: Duration::from_secs(60),
        };

        assert_eq!(info.reason_display(), "Unsupported codec: prores");
    }

    #[test]
    fn test_video_pre_scanner_no_gpu() {
        let scanner = VideoPreScanner::with_gpu_info(true, None);
        assert!(!scanner.has_gpu());
        assert!(scanner.get_gpu_info().is_none());
    }

    #[test]
    fn test_video_pre_scanner_with_gpu() {
        let gpu_info = NvidiaGpuInfo {
            name: "RTX 3080".to_string(),
            generation: GpuGeneration::Ampere,
            compute_capability: "8.6".to_string(),
            device_id: 0,
        };

        let scanner = VideoPreScanner::with_gpu_info(true, Some(gpu_info.clone()));
        assert!(scanner.has_gpu());
        assert_eq!(scanner.get_gpu_info().unwrap().name, "RTX 3080");
    }

    #[test]
    fn test_video_pre_scanner_disabled() {
        let gpu_info = NvidiaGpuInfo {
            name: "RTX 3080".to_string(),
            generation: GpuGeneration::Ampere,
            compute_capability: "8.6".to_string(),
            device_id: 0,
        };

        // Even with GPU info, if use_gpu is false, it should not use GPU
        let scanner = VideoPreScanner::with_gpu_info(false, Some(gpu_info));
        // When use_gpu is false, gpu_info is set but analyze_video returns CpuOnly
        assert!(scanner.get_gpu_info().is_some());
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 bytes");
        assert_eq!(format_size(512), "512 bytes");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
        assert_eq!(format_size(1572864), "1.5 MB");
        assert_eq!(format_size(1073741824), "1.0 GB");
        assert_eq!(format_size(1610612736), "1.5 GB");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(0)), "0 seconds");
        assert_eq!(format_duration(Duration::from_secs(1)), "1 second");
        assert_eq!(format_duration(Duration::from_secs(30)), "30 seconds");
        assert_eq!(format_duration(Duration::from_secs(59)), "59 seconds");
        assert_eq!(format_duration(Duration::from_secs(60)), "1 minute");
        assert_eq!(format_duration(Duration::from_secs(120)), "2 minutes");
        assert_eq!(format_duration(Duration::from_secs(3600)), "1 hour 0 minutes");
        assert_eq!(format_duration(Duration::from_secs(3660)), "1 hour 1 minute");
        assert_eq!(format_duration(Duration::from_secs(7200)), "2 hours 0 minutes");
        assert_eq!(format_duration(Duration::from_secs(7261)), "2 hours 1 minute");
    }

    #[test]
    fn test_transcode_decision_equality() {
        assert_eq!(TranscodeDecision::TranscodeAll, TranscodeDecision::TranscodeAll);
        assert_eq!(TranscodeDecision::UseCpuForIncompatible, TranscodeDecision::UseCpuForIncompatible);
        assert_eq!(TranscodeDecision::Cancel, TranscodeDecision::Cancel);
        assert_eq!(TranscodeDecision::NoTranscodeNeeded, TranscodeDecision::NoTranscodeNeeded);
        assert_ne!(TranscodeDecision::TranscodeAll, TranscodeDecision::Cancel);
    }

    #[test]
    fn test_generate_transcode_prompt_no_transcoding() {
        let results = PreScanResults::default();
        let prompt = generate_transcode_prompt(&results);
        assert_eq!(prompt, "All videos are GPU-compatible!");
    }

    #[test]
    fn test_generate_transcode_prompt_with_transcoding() {
        let mut results = PreScanResults::default();
        results.needs_transcoding.push(VideoTranscodeInfo {
            path: PathBuf::from("/test/a.mp4"),
            reason: TranscodeReason::UnsupportedCodec("prores".to_string()),
            current_codec: "prores".to_string(),
            current_format: "yuv422p10le".to_string(),
            file_size: 1073741824, // 1 GB
            estimated_transcode_size: 644245094, // ~0.6 GB
            estimated_time: Duration::from_secs(3660), // 61 minutes
        });

        let prompt = generate_transcode_prompt(&results);
        assert!(prompt.contains("1 videos incompatible"));
        assert!(prompt.contains("prores"));
        assert!(prompt.contains("Unsupported codec"));
        assert!(prompt.contains("disk space"));
        assert!(prompt.contains("Temporary files will be deleted"));
    }

    #[test]
    fn test_generate_breakdown() {
        let mut results = PreScanResults::default();
        results.needs_transcoding.push(VideoTranscodeInfo {
            path: PathBuf::from("/test/a.mp4"),
            reason: TranscodeReason::UnsupportedCodec("prores".to_string()),
            current_codec: "prores".to_string(),
            current_format: "yuv422p10le".to_string(),
            file_size: 1000000,
            estimated_transcode_size: 600000,
            estimated_time: Duration::from_secs(60),
        });
        results.needs_transcoding.push(VideoTranscodeInfo {
            path: PathBuf::from("/test/b.mp4"),
            reason: TranscodeReason::UnsupportedPixelFormat("yuv422p".to_string()),
            current_codec: "h264".to_string(),
            current_format: "yuv422p".to_string(),
            file_size: 2000000,
            estimated_transcode_size: 1200000,
            estimated_time: Duration::from_secs(120),
        });

        let breakdown = generate_breakdown(&results);
        assert!(breakdown.contains("prores"));
        assert!(breakdown.contains("h264"));
        assert!(breakdown.contains("Unsupported codec"));
        assert!(breakdown.contains("Unsupported pixel format"));
    }

    // Test serialization
    #[test]
    fn test_video_transcode_info_serialization() {
        let info = VideoTranscodeInfo {
            path: PathBuf::from("/test/a.mp4"),
            reason: TranscodeReason::UnsupportedCodec("prores".to_string()),
            current_codec: "prores".to_string(),
            current_format: "yuv422p10le".to_string(),
            file_size: 1000000,
            estimated_transcode_size: 600000,
            estimated_time: Duration::from_secs(60),
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: VideoTranscodeInfo = serde_json::from_str(&serialized).unwrap();

        assert_eq!(info.path, deserialized.path);
        assert_eq!(info.current_codec, deserialized.current_codec);
        assert_eq!(info.file_size, deserialized.file_size);
    }

    // Hypothetical issue: Test with zero duration
    #[test]
    fn test_video_transcode_info_zero_duration() {
        let codec_info = VideoCodecInfo {
            codec_name: "prores".to_string(),
            codec_type: CodecType::Other,
            pixel_format: "yuv422p10le".to_string(),
            profile: "HQ".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 10,
            duration: Some(0.0),
        };

        let info = VideoTranscodeInfo::new(
            PathBuf::from("/test/a.mov"),
            TranscodeReason::UnsupportedCodec("prores".to_string()),
            &codec_info,
            1000000,
        );

        // Should not panic with zero duration
        assert_eq!(info.estimated_time.as_secs(), 0);
    }

    // Hypothetical issue: Test with None duration
    #[test]
    fn test_video_transcode_info_none_duration() {
        let codec_info = VideoCodecInfo {
            codec_name: "prores".to_string(),
            codec_type: CodecType::Other,
            pixel_format: "yuv422p10le".to_string(),
            profile: "HQ".to_string(),
            width: 1920,
            height: 1080,
            bit_depth: 10,
            duration: None,
        };

        let info = VideoTranscodeInfo::new(
            PathBuf::from("/test/a.mov"),
            TranscodeReason::UnsupportedCodec("prores".to_string()),
            &codec_info,
            1000000,
        );

        // Should default to 60s duration
        assert_eq!(info.estimated_time.as_secs(), 72); // 60 * 1.2
    }

    // Hypothetical issue: Test empty video list
    #[test]
    fn test_pre_scanner_empty_list() {
        let scanner = VideoPreScanner::with_gpu_info(true, None);
        let results = scanner.scan_videos(&[]);

        assert_eq!(results.total_videos, 0);
        assert!(results.gpu_compatible.is_empty());
        assert!(results.needs_transcoding.is_empty());
        assert!(results.cpu_only.is_empty());
        assert!(results.failed.is_empty());
    }
}
