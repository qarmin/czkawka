use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::tool_data::DeleteMethod;
use czkawka_core::re_exported::{FilterType, HashAlg};
use czkawka_core::tools::broken_files::CheckedTypes;
use czkawka_core::tools::same_music::MusicSimilarity;
use czkawka_core::tools::similar_images::GeometricInvariance;
use czkawka_core::tools::similar_videos::{
    ALLOWED_AUDIO_LENGTH_RATIO, ALLOWED_AUDIO_SIMILARITY_PERCENT, ALLOWED_DURATION_TOLERANCE_PCT, ALLOWED_MATCH_FRACTION, ALLOWED_SKIP_FORWARD_AMOUNT, ALLOWED_VID_HASH_DURATION,
    ALLOWED_WINDOW_COUNT,
};
use czkawka_core::tools::video_optimizer::{NoiseReductionMethod, VideoCodec};

/// Values above this threshold are practically meaningless for audio segment matching
const MAX_SAME_MUSIC_DIFFERENCE: f64 = 10.0;

pub(crate) fn parse_maximum_difference(src: &str) -> Result<f64, String> {
    match src.parse::<f64>() {
        Ok(maximum_difference) => {
            if maximum_difference <= 0.0 {
                Err("Maximum difference must be bigger than 0".to_string())
            } else if maximum_difference > MAX_SAME_MUSIC_DIFFERENCE {
                Err(format!("Maximum difference must be at most {MAX_SAME_MUSIC_DIFFERENCE}"))
            } else {
                Ok(maximum_difference)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_minimum_segment_duration(src: &str) -> Result<f32, String> {
    match src.parse::<f32>() {
        Ok(minimum_segment_duration) => {
            if minimum_segment_duration <= 0.0 {
                Err("Minimum segment duration must be bigger than 0".to_string())
            } else if minimum_segment_duration >= 3600.0 {
                Err("Minimum segment duration must be smaller than 3600(greater values not have much sense)".to_string())
            } else {
                Ok(minimum_segment_duration)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_audio_similarity_percent(src: &str) -> Result<f64, String> {
    match src.parse::<f64>() {
        Ok(v) => {
            if ALLOWED_AUDIO_SIMILARITY_PERCENT.contains(&v) {
                Ok(v)
            } else {
                Err(format!("Audio similarity percent must be in range {ALLOWED_AUDIO_SIMILARITY_PERCENT:?}"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_audio_maximum_difference(src: &str) -> Result<f64, String> {
    match src.parse::<f64>() {
        Ok(v) => {
            if v >= 0.0 {
                Ok(v)
            } else {
                Err("Audio maximum difference must be >= 0.0".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_audio_length_ratio(src: &str) -> Result<f64, String> {
    match src.parse::<f64>() {
        Ok(v) => {
            if ALLOWED_AUDIO_LENGTH_RATIO.contains(&v) {
                Ok(v)
            } else {
                Err(format!("Audio length ratio must be in range {ALLOWED_AUDIO_LENGTH_RATIO:?}"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_scan_duration(s: &str) -> Result<u32, String> {
    match s.parse::<u32>() {
        Ok(scan_duration) => {
            if ALLOWED_VID_HASH_DURATION.contains(&scan_duration) {
                Ok(scan_duration)
            } else {
                Err(format!("Scan duration must be one of: {ALLOWED_VID_HASH_DURATION:?}"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_window_count(src: &str) -> Result<u32, String> {
    match src.parse::<u32>() {
        Ok(wc) => {
            if ALLOWED_WINDOW_COUNT.contains(&wc) {
                Ok(wc)
            } else {
                Err(format!("Window count must be one of: {ALLOWED_WINDOW_COUNT:?}"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_duration_tolerance_pct(src: &str) -> Result<f64, String> {
    match src.parse::<f64>() {
        Ok(v) => {
            if ALLOWED_DURATION_TOLERANCE_PCT.contains(&v) {
                Ok(v)
            } else {
                Err(format!("Duration tolerance must be in range {ALLOWED_DURATION_TOLERANCE_PCT:?}"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_match_fraction(src: &str) -> Result<f64, String> {
    match src.parse::<f64>() {
        Ok(v) => {
            if ALLOWED_MATCH_FRACTION.contains(&v) {
                Ok(v)
            } else {
                Err(format!("Match fraction must be in range {ALLOWED_MATCH_FRACTION:?}"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_skip_forward_amount(src: &str) -> Result<u32, String> {
    match src.parse::<u32>() {
        Ok(skip_forward_amount) => {
            if !ALLOWED_SKIP_FORWARD_AMOUNT.contains(&skip_forward_amount) {
                Err(format!("Skip forward amount must be one of: {ALLOWED_SKIP_FORWARD_AMOUNT:?}"))
            } else {
                Ok(skip_forward_amount)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_hash_type(src: &str) -> Result<HashType, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "blake3" => Ok(HashType::Blake3),
        "crc32" => Ok(HashType::Crc32),
        "xxh3" => Ok(HashType::Xxh3),
        _ => Err("Couldn't parse the hash type (allowed: BLAKE3, CRC32, XXH3)"),
    }
}

pub(crate) fn parse_tolerance(src: &str) -> Result<i32, &'static str> {
    match src.parse::<i32>() {
        Ok(t) => {
            if (0..=20).contains(&t) {
                Ok(t)
            } else {
                Err("Tolerance should be in range <0,20>(Higher and lower similarity )")
            }
        }
        _ => Err("Failed to parse tolerance as i32 value."),
    }
}

pub(crate) fn parse_checking_method_duplicate(src: &str) -> Result<CheckingMethod, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "name" => Ok(CheckingMethod::Name),
        "size" => Ok(CheckingMethod::Size),
        "size_name" => Ok(CheckingMethod::SizeName),
        "hash" => Ok(CheckingMethod::Hash),
        _ => Err("Couldn't parse the search method (allowed: NAME, SIZE, HASH)"),
    }
}

pub(crate) fn parse_broken_files(src: &str) -> Result<CheckedTypes, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "pdf" => Ok(CheckedTypes::PDF),
        "audio" => Ok(CheckedTypes::AUDIO),
        "image" => Ok(CheckedTypes::IMAGE),
        "archive" => Ok(CheckedTypes::ARCHIVE),
        "video_ffprobe" => Ok(CheckedTypes::VIDEO_FFPROBE),
        "video_ffmpeg" => Ok(CheckedTypes::VIDEO_FFMPEG),
        "font" => Ok(CheckedTypes::FONT),
        "markup" => Ok(CheckedTypes::MARKUP),
        _ => Err("Couldn't parse the broken files type (allowed: PDF, AUDIO, IMAGE, ARCHIVE, FONT, MARKUP, VIDEO_FFPROBE, VIDEO_FFMPEG)"),
    }
}

pub(crate) fn parse_checking_method_same_music(src: &str) -> Result<CheckingMethod, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "tags" => Ok(CheckingMethod::AudioTags),
        "content" => Ok(CheckingMethod::AudioContent),
        _ => Err("Couldn't parse the search method (allowed: TAGS, CONTENT)"),
    }
}

pub(crate) fn parse_video_codec(src: &str) -> Result<VideoCodec, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "h264" => Ok(VideoCodec::H264),
        "h265" | "hevc" => Ok(VideoCodec::H265),
        "av1" => Ok(VideoCodec::Av1),
        "vp9" => Ok(VideoCodec::Vp9),
        _ => Err("Couldn't parse the video codec (allowed: h264, h265, av1, vp9)"),
    }
}

pub(crate) fn parse_max_samples(src: &str) -> Result<usize, String> {
    match src.parse::<usize>() {
        Ok(val) if (5..=1000).contains(&val) => Ok(val),
        Ok(_) => Err("Maximum samples must be between 5 and 1000".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_min_crop_size(src: &str) -> Result<u32, String> {
    match src.parse::<u32>() {
        Ok(val) if (1..=1000).contains(&val) => Ok(val),
        Ok(_) => Err("Minimum crop size must be between 1 and 1000".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_delete_method(src: &str) -> Result<DeleteMethod, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "none" => Ok(DeleteMethod::None),
        "aen" => Ok(DeleteMethod::AllExceptNewest),
        "aeo" => Ok(DeleteMethod::AllExceptOldest),
        "hard" => Ok(DeleteMethod::HardLink),
        "on" => Ok(DeleteMethod::OneNewest),
        "oo" => Ok(DeleteMethod::OneOldest),
        "aeb" => Ok(DeleteMethod::AllExceptBiggest),
        "aes" => Ok(DeleteMethod::AllExceptSmallest),
        "ob" => Ok(DeleteMethod::OneBiggest),
        "os" => Ok(DeleteMethod::OneSmallest),
        _ => Err("Couldn't parse the delete method (allowed: AEN, AEO, ON, OO, HARD, AEB, AES, OB, OS)"),
    }
}

pub(crate) fn parse_minimal_file_size(src: &str) -> Result<u64, String> {
    match src.parse::<u64>() {
        Ok(minimal_file_size) => {
            if minimal_file_size > 0 {
                Ok(minimal_file_size)
            } else {
                Err("Minimum file size must be at least 1 byte".to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_maximal_file_size(src: &str) -> Result<u64, String> {
    match src.parse::<u64>() {
        Ok(maximal_file_size) => {
            if maximal_file_size == 0 {
                Err("Maximum file size must be at least 1 byte".to_string())
            } else {
                Ok(maximal_file_size)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn parse_similar_image_filter(src: &str) -> Result<FilterType, String> {
    let filter_type = match src.to_lowercase().as_str() {
        "lanczos3" => FilterType::Lanczos3,
        "nearest" => FilterType::Nearest,
        "triangle" => FilterType::Triangle,
        "gaussian" => FilterType::Gaussian,
        "catmullrom" => FilterType::CatmullRom,
        _ => return Err("Couldn't parse the image resize filter (allowed: Lanczos3, Nearest, Triangle, Gaussian, Catmullrom)".to_string()),
    };
    Ok(filter_type)
}

pub(crate) fn parse_similar_hash_algorithm(src: &str) -> Result<HashAlg, String> {
    let algorithm = match src.to_lowercase().as_str() {
        "mean" => HashAlg::Mean,
        "gradient" => HashAlg::Gradient,
        "blockhash" => HashAlg::Blockhash,
        "vertgradient" => HashAlg::VertGradient,
        "doublegradient" => HashAlg::DoubleGradient,
        "median" => HashAlg::Median,
        _ => return Err("Couldn't parse the hash algorithm (allowed: Mean, Gradient, Blockhash, VertGradient, DoubleGradient, Median)".to_string()),
    };
    Ok(algorithm)
}

pub(crate) fn parse_image_hash_size(src: &str) -> Result<u8, String> {
    let hash_size = match src.to_lowercase().as_str() {
        "8" => 8,
        "16" => 16,
        "32" => 32,
        "64" => 64,
        _ => return Err("Couldn't parse the image hash size (allowed: 8, 16, 32, 64)".to_string()),
    };
    Ok(hash_size)
}

pub(crate) fn parse_geometric_invariance(src: &str) -> Result<GeometricInvariance, String> {
    let geometric_invariance = match src.to_lowercase().replace('_', "-").as_str() {
        "off" => GeometricInvariance::Off,
        "mirror-flip" => GeometricInvariance::MirrorFlip,
        "mirror-flip-rotate90" => GeometricInvariance::MirrorFlipRotate90,
        _ => return Err("Couldn't parse geometric invariance (allowed: off, mirror-flip, mirror-flip-rotate90)".to_string()),
    };
    Ok(geometric_invariance)
}

pub(crate) fn parse_music_duplicate_type(src: &str) -> Result<MusicSimilarity, String> {
    if src.trim().is_empty() {
        return Ok(MusicSimilarity::NONE);
    }

    let mut similarity: MusicSimilarity = MusicSimilarity::NONE;

    let parts: Vec<String> = src.split(',').map(|e| e.to_lowercase().replace('_', "")).collect();

    if parts.contains(&"tracktitle".into()) {
        similarity |= MusicSimilarity::TRACK_TITLE;
    }
    if parts.contains(&"trackartist".into()) {
        similarity |= MusicSimilarity::TRACK_ARTIST;
    }
    if parts.contains(&"year".into()) {
        similarity |= MusicSimilarity::YEAR;
    }
    if parts.contains(&"bitrate".into()) {
        similarity |= MusicSimilarity::BITRATE;
    }
    if parts.contains(&"genre".into()) {
        similarity |= MusicSimilarity::GENRE;
    }
    if parts.contains(&"length".into()) {
        similarity |= MusicSimilarity::LENGTH;
    }

    if similarity == MusicSimilarity::NONE {
        return Err("Couldn't parse the music search method (allowed: track_title,track_artist,year,bitrate,genre,length)".to_string());
    }

    Ok(similarity)
}

pub(crate) fn parse_crop_mechanism(src: &str) -> Result<String, String> {
    match src.to_lowercase().as_str() {
        "blackbars" | "staticcontent" => Ok(src.to_lowercase()),
        _ => Err("Invalid crop mechanism. Allowed values: blackbars, staticcontent".to_string()),
    }
}

pub(crate) fn parse_noise_reduction(src: &str) -> Result<NoiseReductionMethod, String> {
    src.parse::<NoiseReductionMethod>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_geometric_invariance() {
        assert_eq!(parse_geometric_invariance("off"), Ok(GeometricInvariance::Off));
        assert_eq!(parse_geometric_invariance("mirror-flip"), Ok(GeometricInvariance::MirrorFlip));
        assert_eq!(parse_geometric_invariance("mirror-flip-rotate90"), Ok(GeometricInvariance::MirrorFlipRotate90));
    }
}
