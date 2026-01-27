# Similar Videos Improvements - Implementation Plan and Changes

This document outlines the original implementation plan and all changes made to improve similar video detection in Czkawka.

## Background

Based on the discussion in `mon_jan_26_2026_identifying_similar_images_and_videos.md`, several improvements were proposed to enhance similar video detection capabilities:

1. **Temporal Segmentation (Chunked Hashing)** - Break videos into chunks and hash each independently to detect edited/trimmed videos
2. **GPU Acceleration** - Use NVIDIA NVDEC for faster video decoding via FFmpeg
3. **Codec Detection & Fallback** - Pre-scan videos for GPU compatibility and handle unsupported formats gracefully

---

## Original Implementation Plan

### Phase 1: Core Infrastructure
- Create GPU detection module for NVIDIA GPU capability analysis
- Create codec detection module using ffprobe
- Create decoder strategy module for intelligent GPU/CPU selection
- Update `SimilarVideosParameters` with new fields

### Phase 2: GPU Acceleration
- Implement GPU acceleration infrastructure
- Integrate with FFmpeg's CUDA/NVDEC support

### Phase 3: Pre-Scan & Transcoding Workflow
- Create pre-scan module for video compatibility analysis
- Create batch transcoder for unsupported videos (future)
- Implement user confirmation dialogs (future)

### Phase 4: Temporal Segmentation (Chunked Hashing)
- Create temporal segmentation module
- Implement chunked video hashing
- Add sequence matching algorithms

### Phase 5: UI Integration
- Add GPU acceleration toggle
- Add scan mode selection
- Add transcode confirmation dialogs

### Phase 6: Testing & Documentation
- Add comprehensive unit tests
- Add integration tests
- Update documentation

---

## Changes Made

### New Modules Created

#### 1. `gpu_detection.rs` (381 lines)
**Purpose:** Detect NVIDIA GPU capabilities for hardware-accelerated video decoding

**Key Components:**
- `GpuGeneration` enum - GPU generations (Kepler, Maxwell, Pascal, Volta, Turing, Ampere, Ada, Blackwell)
- `NvidiaGpuInfo` struct - GPU name, generation, compute capability, device ID
- `HWAccelType` enum - Hardware acceleration types (Auto, Cuda, Vaapi, Dxva2, Videotoolbox, None)
- `GpuDetector` - Detects NVIDIA GPU using nvidia-smi, checks FFmpeg NVDEC support

**Key Functions:**
- `GpuGeneration::from_compute_capability()` - Parse GPU generation from compute capability string
- `GpuGeneration::supports_codec()` - Check if codec is supported by GPU generation
- `GpuDetector::detect_nvidia_gpu()` - Detect NVIDIA GPU and return info
- `GpuDetector::check_ffmpeg_nvdec_support()` - Check if FFmpeg has NVDEC support

**Unit Tests:** 18 tests covering edge cases

---

#### 2. `codec_detection.rs` (572 lines)
**Purpose:** Detect video codec properties using ffprobe for GPU compatibility checking

**Key Components:**
- `VideoCodecInfo` struct - Codec name, type, pixel format, profile, resolution, bit depth, duration
- `CodecType` enum - Known codec types (H264, H265, VP9, AV1, MPEG2, VP8, Other)
- `TranscodeReason` enum - Reasons for transcoding (UnsupportedCodec, UnsupportedPixelFormat, etc.)
- `CodecDetector` - Detects video codec using ffprobe

**Key Functions:**
- `CodecType::from_codec_name()` - Parse codec type from name string
- `CodecType::is_nvdec_supported()` - Check if codec is supported by NVDEC on GPU generation
- `CodecDetector::detect_codec()` - Detect video codec info from file
- `CodecDetector::is_pixel_format_supported()` - Check if pixel format is supported (4:2:0 vs 4:2:2/4:4:4)
- `CodecDetector::get_max_resolution()` - Get max resolution supported by GPU generation

**Unit Tests:** 22 tests covering edge cases

---

#### 3. `decoder_strategy.rs` (635 lines)
**Purpose:** Intelligent decoder selection (GPU vs CPU) based on video properties and hardware capabilities

**Key Components:**
- `DecoderStrategy` enum - Selection strategies (GpuOnly, CpuOnly, GpuPreferred, Auto)
- `DecoderChoice` enum - Selected decoder (Gpu, Cpu, Error)
- `VideoCompatibility` enum - Compatibility status (GpuReady, NeedsTranscode, CpuOnly, Failed)
- `ScanMode` enum - Scan modes for temporal segmentation (Fast, Balanced, Thorough)
- `DecoderSelector` - Selects best decoder for each video

**Key Functions:**
- `DecoderSelector::select_decoder()` - Determine best decoder for a video file
- `DecoderSelector::select_decoder_for_codec()` - Determine best decoder based on codec info
- `DecoderSelector::analyze_video_compatibility()` - Analyze video compatibility with GPU acceleration

**Unit Tests:** 20 tests covering edge cases

---

#### 4. `pre_scan.rs` (708 lines)
**Purpose:** Pre-scan videos for GPU compatibility before processing

**Key Components:**
- `PreScanResults` struct - Categorized scan results (GPU-compatible, needs-transcoding, CPU-only, failed)
- `VideoTranscodeInfo` struct - Transcoding information (path, reason, codec, format, sizes, time estimates)
- `VideoPreScanner` - Scans videos for GPU compatibility
- `TranscodeDecision` enum - User decisions (TranscodeAll, UseCpuForIncompatible, Cancel, NoTranscodeNeeded)

**Key Functions:**
- `VideoPreScanner::scan_videos()` - Scan list of video paths for GPU compatibility
- `generate_transcode_prompt()` - Generate user-friendly prompt for transcoding confirmation
- `format_size()` - Format bytes as human-readable size
- `format_duration()` - Format duration with proper grammar (singular/plural)

**Constants:**
- `TRANSCODE_SIZE_RATIO` (0.6) - Estimated ratio of transcoded file size to original
- `TRANSCODE_TIME_MULTIPLIER` (1.2) - Estimated time multiplier for transcoding
- `DEFAULT_VIDEO_DURATION_SECS` (60.0) - Default assumed video duration when unknown

**Unit Tests:** 20 tests covering edge cases

---

#### 5. `temporal_segmentation.rs` (549 lines)
**Purpose:** Chunked video hashing for detecting edited/trimmed videos

**Key Components:**
- `TimestampedHash` struct - Hash with timestamp and chunk duration
- `TemporalSegmentationConfig` struct - Configuration (chunk_duration, chunk_overlap, max_chunks, min_matching_chunks)
- `TemporalMatchResult` struct - Match analysis (matching_chunks, similarity_score, matched_timestamps)

**Key Functions:**
- `TemporalSegmentationConfig::fast()` - Fast mode preset (1 chunk)
- `TemporalSegmentationConfig::balanced()` - Balanced mode preset (5 chunks)
- `TemporalSegmentationConfig::thorough()` - Thorough mode preset (15 chunks)
- `TemporalSegmentationConfig::calculate_chunk_timestamps()` - Calculate chunk timestamps for a video
- `compare_hash_sequences()` - Compare two sets of timestamped hashes to find matches
- `hashes_are_similar()` - Compare hashes using Hamming distance
- `estimate_processing_time()` - Estimate processing time for temporal segmentation

**Unit Tests:** 31 tests covering edge cases

---

### Modified Files

#### `czkawka_core/src/common/mod.rs`
Added new module declarations:
```rust
pub mod codec_detection;
pub mod decoder_strategy;
pub mod gpu_detection;
pub mod pre_scan;
pub mod temporal_segmentation;
```

#### `czkawka_core/src/tools/similar_videos/mod.rs`
Updated `SimilarVideosParameters` struct with new fields:
```rust
pub struct SimilarVideosParameters {
    // Existing fields...
    
    // NEW: GPU acceleration settings
    pub decoder_strategy: DecoderStrategy,
    pub hwaccel_type: HWAccelType,
    pub gpu_device_id: i32,
    
    // NEW: Temporal segmentation settings
    pub scan_mode: ScanMode,
}
```

Added new constructor and setter methods:
- `with_gpu_settings()` - Constructor with full GPU and scan mode configuration
- `set_decoder_strategy()` - Set the decoder strategy
- `set_hwaccel_type()` - Set the hardware acceleration type
- `set_gpu_device_id()` - Set the GPU device ID
- `set_scan_mode()` - Set the scan mode

---

## Summary Statistics

| Metric | Value |
|--------|-------|
| New Files Created | 5 |
| Total Lines Added | 2,921 |
| Unit Tests Added | 111 |
| Commits | 5 |

### Unit Test Breakdown:
| Module | Tests |
|--------|-------|
| gpu_detection.rs | 18 |
| codec_detection.rs | 22 |
| decoder_strategy.rs | 20 |
| pre_scan.rs | 20 |
| temporal_segmentation.rs | 31 |
| **Total** | **111** |

---

## Commit History

1. **0611d31** - Phase 1: Add GPU detection, codec detection, and decoder strategy modules with unit tests
2. **c4a405e** - Phase 3: Add pre-scan module for video GPU compatibility analysis with unit tests
3. **81f58f7** - Phase 4: Add temporal segmentation module for chunked video hashing with unit tests
4. **3599f8b** - Fix code review issues: grammar, constants, and safer defaults

---

## Future Enhancements (Not Yet Implemented)

### Phase 3 (Remaining):
- [ ] Create `batch_transcoder.rs` module for transcoding unsupported videos
- [ ] Implement user confirmation dialog integration in GUI

### Phase 4 (Remaining):
- [ ] Integration with `VideosEntry` to support multiple hashes
- [ ] Update cache format to support multiple hashes

### Phase 5 (UI Integration):
- [ ] Add GPU acceleration toggle in settings
- [ ] Add scan mode selection in UI
- [ ] Add transcode confirmation dialog
- [ ] Display compatibility information

### Phase 6 (Remaining):
- [ ] Add integration tests for GPU detection (requires GPU hardware)
- [ ] Update user documentation

---

## How to Use the New Features

### GPU Detection
```rust
use crate::common::gpu_detection::{GpuDetector, GpuGeneration, HWAccelType};

// Check if GPU acceleration is available
if GpuDetector::is_gpu_acceleration_available() {
    if let Some(gpu_info) = GpuDetector::detect_nvidia_gpu() {
        println!("GPU: {} ({})", gpu_info.name, gpu_info.generation);
    }
}
```

### Codec Detection
```rust
use crate::common::codec_detection::{CodecDetector, CodecType};
use crate::common::gpu_detection::GpuGeneration;

let codec_info = CodecDetector::detect_codec(&video_path)?;
if codec_info.codec_type.is_nvdec_supported(GpuGeneration::Ampere) {
    println!("Video is GPU-compatible");
}
```

### Pre-Scanning Videos
```rust
use crate::common::pre_scan::{VideoPreScanner, generate_transcode_prompt};

let scanner = VideoPreScanner::new(true); // use_gpu = true
let results = scanner.scan_videos(&video_paths);
println!("{}", results.summary_message());
```

### Temporal Segmentation
```rust
use crate::common::temporal_segmentation::{TemporalSegmentationConfig, compare_hash_sequences};

let config = TemporalSegmentationConfig::balanced();
let timestamps = config.calculate_chunk_timestamps(video_duration);

// Compare hash sequences
let match_result = compare_hash_sequences(&hashes_a, &hashes_b, 0.3);
if match_result.is_match(2) {
    println!("Videos are similar! {} chunks matched", match_result.matching_chunks);
}
```

---

*Document created: January 27, 2026*
