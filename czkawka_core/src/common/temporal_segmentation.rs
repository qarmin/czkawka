// Temporal Segmentation Module
// Provides chunked video hashing for detecting edited/trimmed videos

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// A timestamped video hash representing a chunk of video
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimestampedHash {
    /// Timestamp in seconds where this chunk starts
    pub timestamp: f64,
    /// Duration of this chunk in seconds
    pub chunk_duration: f64,
    /// Hash data (stored as bytes for serialization compatibility)
    pub hash_data: Vec<u8>,
}

impl TimestampedHash {
    /// Create a new timestamped hash
    pub fn new(timestamp: f64, chunk_duration: f64, hash_data: Vec<u8>) -> Self {
        Self {
            timestamp,
            chunk_duration,
            hash_data,
        }
    }

    /// Get the end timestamp of this chunk
    pub fn end_timestamp(&self) -> f64 {
        self.timestamp + self.chunk_duration
    }
}

/// Configuration for temporal segmentation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemporalSegmentationConfig {
    /// Duration of each chunk in seconds
    pub chunk_duration: u32,
    /// Overlap between chunks in seconds (0 for non-overlapping)
    pub chunk_overlap: u32,
    /// Maximum number of chunks to extract per video
    pub max_chunks: u32,
    /// Minimum number of matching chunks required to consider videos similar
    pub min_matching_chunks: u32,
    /// Whether to use temporal segmentation
    pub enabled: bool,
}

impl Default for TemporalSegmentationConfig {
    fn default() -> Self {
        Self {
            chunk_duration: 10,
            chunk_overlap: 0,
            max_chunks: 10,
            min_matching_chunks: 2,
            enabled: false,
        }
    }
}

impl TemporalSegmentationConfig {
    /// Create a new config for fast mode (single hash)
    pub fn fast() -> Self {
        Self {
            chunk_duration: 10,
            chunk_overlap: 0,
            max_chunks: 1,
            min_matching_chunks: 1,
            enabled: false,
        }
    }

    /// Create a new config for balanced mode (3-5 chunks)
    pub fn balanced() -> Self {
        Self {
            chunk_duration: 10,
            chunk_overlap: 2,
            max_chunks: 5,
            min_matching_chunks: 2,
            enabled: true,
        }
    }

    /// Create a new config for thorough mode (10+ chunks)
    pub fn thorough() -> Self {
        Self {
            chunk_duration: 10,
            chunk_overlap: 3,
            max_chunks: 15,
            min_matching_chunks: 3,
            enabled: true,
        }
    }

    /// Calculate chunk timestamps for a video of given duration
    pub fn calculate_chunk_timestamps(&self, video_duration: f64) -> Vec<(f64, f64)> {
        if !self.enabled || video_duration <= 0.0 {
            return vec![(0.0, video_duration.min(self.chunk_duration as f64))];
        }

        let chunk_dur = self.chunk_duration as f64;
        let overlap = self.chunk_overlap as f64;
        let step = chunk_dur - overlap;

        if step <= 0.0 {
            return vec![(0.0, chunk_dur.min(video_duration))];
        }

        let mut timestamps = Vec::new();
        let mut current_time = 0.0;

        while current_time < video_duration && timestamps.len() < self.max_chunks as usize {
            let end_time = (current_time + chunk_dur).min(video_duration);
            let actual_duration = end_time - current_time;

            // Only add chunk if it has meaningful duration
            if actual_duration >= 1.0 {
                timestamps.push((current_time, actual_duration));
            }

            current_time += step;
        }

        if timestamps.is_empty() {
            timestamps.push((0.0, video_duration.min(chunk_dur)));
        }

        timestamps
    }

    /// Get estimated time multiplier compared to single-hash mode
    pub fn estimated_time_multiplier(&self, video_duration: f64) -> f64 {
        if !self.enabled {
            return 1.0;
        }

        let chunks = self.calculate_chunk_timestamps(video_duration);
        chunks.len() as f64
    }
}

/// Result of matching two videos using temporal segmentation
#[derive(Clone, Debug)]
pub struct TemporalMatchResult {
    /// Number of chunks that matched
    pub matching_chunks: usize,
    /// Total chunks compared
    pub total_chunks_compared: usize,
    /// Matching timestamps (pairs of matched chunk timestamps)
    pub matched_timestamps: Vec<(f64, f64)>,
    /// Similarity score (0.0 - 1.0)
    pub similarity_score: f64,
}

impl TemporalMatchResult {
    /// Check if this is considered a match based on minimum matching chunks
    pub fn is_match(&self, min_matching_chunks: u32) -> bool {
        self.matching_chunks >= min_matching_chunks as usize
    }

    /// Get the match percentage
    pub fn match_percentage(&self) -> f64 {
        if self.total_chunks_compared == 0 {
            return 0.0;
        }
        (self.matching_chunks as f64 / self.total_chunks_compared as f64) * 100.0
    }
}

/// Compare two sets of timestamped hashes to find matches
pub fn compare_hash_sequences(
    hashes_a: &[TimestampedHash],
    hashes_b: &[TimestampedHash],
    tolerance: f64,
) -> TemporalMatchResult {
    if hashes_a.is_empty() || hashes_b.is_empty() {
        return TemporalMatchResult {
            matching_chunks: 0,
            total_chunks_compared: 0,
            matched_timestamps: Vec::new(),
            similarity_score: 0.0,
        };
    }

    let mut matching_chunks = 0;
    let mut matched_timestamps = Vec::new();

    // Compare each hash in A against all hashes in B
    for hash_a in hashes_a {
        for hash_b in hashes_b {
            if hashes_are_similar(&hash_a.hash_data, &hash_b.hash_data, tolerance) {
                matching_chunks += 1;
                matched_timestamps.push((hash_a.timestamp, hash_b.timestamp));
                break; // Only count one match per hash_a
            }
        }
    }

    let total_chunks_compared = hashes_a.len().max(hashes_b.len());
    let similarity_score = if total_chunks_compared > 0 {
        matching_chunks as f64 / total_chunks_compared as f64
    } else {
        0.0
    };

    TemporalMatchResult {
        matching_chunks,
        total_chunks_compared,
        matched_timestamps,
        similarity_score,
    }
}

/// Compare two hash byte arrays for similarity using Hamming distance
fn hashes_are_similar(hash_a: &[u8], hash_b: &[u8], tolerance: f64) -> bool {
    if hash_a.len() != hash_b.len() || hash_a.is_empty() {
        return false;
    }

    let total_bits = hash_a.len() * 8;
    let mut different_bits = 0;

    for (byte_a, byte_b) in hash_a.iter().zip(hash_b.iter()) {
        different_bits += (byte_a ^ byte_b).count_ones() as usize;
    }

    let similarity = 1.0 - (different_bits as f64 / total_bits as f64);
    similarity >= (1.0 - tolerance)
}

/// Estimate processing time for temporal segmentation
pub fn estimate_processing_time(
    video_count: usize,
    avg_video_duration: Duration,
    config: &TemporalSegmentationConfig,
    single_hash_time: Duration,
) -> Duration {
    let avg_chunks = config.calculate_chunk_timestamps(avg_video_duration.as_secs_f64()).len();
    let time_per_video = single_hash_time.as_secs_f64() * avg_chunks as f64;
    Duration::from_secs_f64(time_per_video * video_count as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Phase 1 Unit Tests =====
    // Tests based on hypothetical issues that could cause problems

    #[test]
    fn test_timestamped_hash_new() {
        let hash = TimestampedHash::new(10.0, 5.0, vec![1, 2, 3, 4]);
        assert!((hash.timestamp - 10.0).abs() < f64::EPSILON);
        assert!((hash.chunk_duration - 5.0).abs() < f64::EPSILON);
        assert_eq!(hash.hash_data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_timestamped_hash_end_timestamp() {
        let hash = TimestampedHash::new(10.0, 5.0, vec![]);
        assert!((hash.end_timestamp() - 15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_temporal_config_default() {
        let config = TemporalSegmentationConfig::default();
        assert_eq!(config.chunk_duration, 10);
        assert_eq!(config.chunk_overlap, 0);
        assert_eq!(config.max_chunks, 10);
        assert_eq!(config.min_matching_chunks, 2);
        assert!(!config.enabled);
    }

    #[test]
    fn test_temporal_config_fast() {
        let config = TemporalSegmentationConfig::fast();
        assert_eq!(config.max_chunks, 1);
        assert!(!config.enabled);
    }

    #[test]
    fn test_temporal_config_balanced() {
        let config = TemporalSegmentationConfig::balanced();
        assert_eq!(config.max_chunks, 5);
        assert!(config.chunk_overlap > 0);
        assert!(config.enabled);
    }

    #[test]
    fn test_temporal_config_thorough() {
        let config = TemporalSegmentationConfig::thorough();
        assert_eq!(config.max_chunks, 15);
        assert!(config.chunk_overlap > 0);
        assert!(config.enabled);
    }

    #[test]
    fn test_calculate_chunk_timestamps_disabled() {
        let config = TemporalSegmentationConfig::default();
        let timestamps = config.calculate_chunk_timestamps(120.0);
        assert_eq!(timestamps.len(), 1);
        assert!((timestamps[0].0 - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_chunk_timestamps_enabled() {
        let config = TemporalSegmentationConfig::balanced();
        let timestamps = config.calculate_chunk_timestamps(120.0);
        
        // Should have multiple chunks
        assert!(timestamps.len() > 1);
        assert!(timestamps.len() <= config.max_chunks as usize);
        
        // First chunk should start at 0
        assert!((timestamps[0].0 - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_chunk_timestamps_short_video() {
        let config = TemporalSegmentationConfig::balanced();
        let timestamps = config.calculate_chunk_timestamps(5.0);
        
        // Short video should have at least one chunk
        assert!(!timestamps.is_empty());
        // Duration should not exceed video length
        assert!(timestamps[0].1 <= 5.0);
    }

    #[test]
    fn test_calculate_chunk_timestamps_zero_duration() {
        let config = TemporalSegmentationConfig::balanced();
        let timestamps = config.calculate_chunk_timestamps(0.0);
        
        // Should return at least one entry
        assert!(!timestamps.is_empty());
    }

    #[test]
    fn test_calculate_chunk_timestamps_negative_duration() {
        let config = TemporalSegmentationConfig::balanced();
        let timestamps = config.calculate_chunk_timestamps(-10.0);
        
        // Should handle gracefully
        assert!(!timestamps.is_empty());
    }

    #[test]
    fn test_estimated_time_multiplier_disabled() {
        let config = TemporalSegmentationConfig::fast();
        let multiplier = config.estimated_time_multiplier(120.0);
        assert!((multiplier - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_estimated_time_multiplier_enabled() {
        let config = TemporalSegmentationConfig::balanced();
        let multiplier = config.estimated_time_multiplier(120.0);
        assert!(multiplier > 1.0);
    }

    #[test]
    fn test_temporal_match_result_is_match() {
        let result = TemporalMatchResult {
            matching_chunks: 3,
            total_chunks_compared: 5,
            matched_timestamps: vec![(0.0, 0.0), (10.0, 10.0), (20.0, 20.0)],
            similarity_score: 0.6,
        };

        assert!(result.is_match(2));
        assert!(result.is_match(3));
        assert!(!result.is_match(4));
    }

    #[test]
    fn test_temporal_match_result_percentage() {
        let result = TemporalMatchResult {
            matching_chunks: 3,
            total_chunks_compared: 6,
            matched_timestamps: vec![],
            similarity_score: 0.5,
        };

        assert!((result.match_percentage() - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_temporal_match_result_percentage_zero() {
        let result = TemporalMatchResult {
            matching_chunks: 0,
            total_chunks_compared: 0,
            matched_timestamps: vec![],
            similarity_score: 0.0,
        };

        assert!((result.match_percentage() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_compare_hash_sequences_empty() {
        let result = compare_hash_sequences(&[], &[], 0.3);
        assert_eq!(result.matching_chunks, 0);
        assert_eq!(result.total_chunks_compared, 0);
    }

    #[test]
    fn test_compare_hash_sequences_identical() {
        let hashes = vec![
            TimestampedHash::new(0.0, 10.0, vec![0xFF, 0x00, 0xFF, 0x00]),
            TimestampedHash::new(10.0, 10.0, vec![0x00, 0xFF, 0x00, 0xFF]),
        ];

        let result = compare_hash_sequences(&hashes, &hashes, 0.3);
        assert_eq!(result.matching_chunks, 2);
    }

    #[test]
    fn test_compare_hash_sequences_different() {
        let hashes_a = vec![
            TimestampedHash::new(0.0, 10.0, vec![0xFF, 0xFF, 0xFF, 0xFF]),
        ];
        let hashes_b = vec![
            TimestampedHash::new(0.0, 10.0, vec![0x00, 0x00, 0x00, 0x00]),
        ];

        let result = compare_hash_sequences(&hashes_a, &hashes_b, 0.3);
        assert_eq!(result.matching_chunks, 0);
    }

    #[test]
    fn test_compare_hash_sequences_partial_match() {
        let hashes_a = vec![
            TimestampedHash::new(0.0, 10.0, vec![0xFF, 0x00]),
            TimestampedHash::new(10.0, 10.0, vec![0x00, 0xFF]),
        ];
        let hashes_b = vec![
            TimestampedHash::new(0.0, 10.0, vec![0xFF, 0x00]), // Match
            TimestampedHash::new(10.0, 10.0, vec![0xFF, 0xFF]), // Different
        ];

        let result = compare_hash_sequences(&hashes_a, &hashes_b, 0.1);
        assert_eq!(result.matching_chunks, 1);
    }

    #[test]
    fn test_hashes_are_similar_identical() {
        let hash = vec![0xFF, 0x00, 0xFF, 0x00];
        assert!(hashes_are_similar(&hash, &hash, 0.0));
    }

    #[test]
    fn test_hashes_are_similar_different_length() {
        let hash_a = vec![0xFF, 0x00];
        let hash_b = vec![0xFF, 0x00, 0xFF];
        assert!(!hashes_are_similar(&hash_a, &hash_b, 0.5));
    }

    #[test]
    fn test_hashes_are_similar_empty() {
        let hash: Vec<u8> = vec![];
        assert!(!hashes_are_similar(&hash, &hash, 0.5));
    }

    #[test]
    fn test_hashes_are_similar_within_tolerance() {
        let hash_a = vec![0xFF, 0x00]; // 16 bits
        let hash_b = vec![0xFE, 0x00]; // 1 bit different
        // 1/16 = 6.25% different, 93.75% similar
        assert!(hashes_are_similar(&hash_a, &hash_b, 0.1)); // 90% threshold
    }

    #[test]
    fn test_hashes_are_similar_outside_tolerance() {
        let hash_a = vec![0xFF, 0xFF]; // 16 bits
        let hash_b = vec![0x00, 0x00]; // All different
        assert!(!hashes_are_similar(&hash_a, &hash_b, 0.1));
    }

    #[test]
    fn test_estimate_processing_time() {
        let config = TemporalSegmentationConfig::balanced();
        let time = estimate_processing_time(
            100,
            Duration::from_secs(120),
            &config,
            Duration::from_secs(2),
        );
        
        // Should be longer than single-hash processing
        assert!(time > Duration::from_secs(200));
    }

    // Serialization tests
    #[test]
    fn test_timestamped_hash_serialization() {
        let hash = TimestampedHash::new(10.0, 5.0, vec![1, 2, 3, 4]);
        let serialized = serde_json::to_string(&hash).unwrap();
        let deserialized: TimestampedHash = serde_json::from_str(&serialized).unwrap();
        assert!((hash.timestamp - deserialized.timestamp).abs() < f64::EPSILON);
        assert_eq!(hash.hash_data, deserialized.hash_data);
    }

    #[test]
    fn test_temporal_config_serialization() {
        let config = TemporalSegmentationConfig::balanced();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: TemporalSegmentationConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config.chunk_duration, deserialized.chunk_duration);
        assert_eq!(config.enabled, deserialized.enabled);
    }

    // Hypothetical edge case: Very long video
    #[test]
    fn test_calculate_chunk_timestamps_very_long_video() {
        let config = TemporalSegmentationConfig::balanced();
        let timestamps = config.calculate_chunk_timestamps(10000.0); // ~3 hours
        
        // Should not exceed max_chunks
        assert!(timestamps.len() <= config.max_chunks as usize);
    }

    // Hypothetical edge case: Chunk larger than video
    #[test]
    fn test_calculate_chunk_timestamps_chunk_larger_than_video() {
        let mut config = TemporalSegmentationConfig::balanced();
        config.chunk_duration = 60; // 60 second chunks
        
        let timestamps = config.calculate_chunk_timestamps(10.0); // 10 second video
        
        // Should still work
        assert!(!timestamps.is_empty());
        // Duration should be limited to video length
        assert!(timestamps[0].1 <= 10.0);
    }

    // Hypothetical edge case: Overlap larger than chunk
    #[test]
    fn test_calculate_chunk_timestamps_large_overlap() {
        let mut config = TemporalSegmentationConfig::balanced();
        config.chunk_duration = 10;
        config.chunk_overlap = 15; // Overlap > chunk duration
        
        let timestamps = config.calculate_chunk_timestamps(120.0);
        
        // Should handle gracefully (fallback to single chunk)
        assert!(!timestamps.is_empty());
    }
}
