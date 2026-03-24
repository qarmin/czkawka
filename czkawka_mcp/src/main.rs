// rmcp #[tool] macro requires &self and owned params; these clippy warnings are false positives
#![allow(clippy::needless_pass_by_value, clippy::unused_self)]

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::unbounded;
use czkawka_core::common::image::register_image_decoding_hooks;
use czkawka_core::common::set_number_of_threads;
use czkawka_core::common::tool_data::CommonData as _;
use czkawka_core::common::traits::{AllTraits, PrintResults};
use czkawka_core::tools::bad_extensions::{BadExtensions, BadExtensionsParameters};
use czkawka_core::tools::bad_names::{BadNames, BadNamesParameters, NameIssues};
use czkawka_core::tools::big_file::{BigFile, BigFileParameters, SearchMode};
use czkawka_core::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};
use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};
use czkawka_core::tools::empty_files::EmptyFiles;
use czkawka_core::tools::empty_folder::EmptyFolder;
use czkawka_core::tools::exif_remover::{ExifRemover, ExifRemoverParameters};
use czkawka_core::tools::invalid_symlinks::InvalidSymlinks;
use czkawka_core::tools::same_music::{SameMusic, SameMusicParameters};
use czkawka_core::tools::similar_images::{SimilarImages, SimilarImagesParameters};
use czkawka_core::tools::similar_videos::{SimilarVideos, SimilarVideosParameters};
use czkawka_core::tools::temporary::Temporary;
use czkawka_core::tools::video_optimizer::{
    VideoCropParams, VideoCroppingMechanism, VideoOptimizer, VideoOptimizerParameters, VideoTranscodeParams,
};
use rmcp::model::{CallToolResult, Content, ServerCapabilities, ServerInfo};
use rmcp::{ServerHandler, ServiceExt, tool};
use schemars::JsonSchema;
use serde::Deserialize;

// ── Common parameter structs ──────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
struct CommonParams {
    #[schemars(description = "List of directories to search (required)")]
    directories: Vec<String>,
    #[schemars(description = "Directories to exclude from search")]
    excluded_directories: Option<Vec<String>>,
    #[schemars(description = "Wildcard patterns to exclude (e.g. '*/.git', '*.tmp')")]
    excluded_items: Option<Vec<String>>,
    #[schemars(description = "Only check files with these extensions (e.g. ['jpg', 'png'])")]
    allowed_extensions: Option<Vec<String>>,
    #[schemars(description = "Skip files with these extensions")]
    excluded_extensions: Option<Vec<String>>,
    #[schemars(description = "If true, do not recurse into subdirectories (default: false)")]
    not_recursive: Option<bool>,
    #[schemars(description = "Number of threads to use (0 = all available, default: 0)")]
    thread_number: Option<usize>,
    #[schemars(description = "Disable the cache system (default: false)")]
    disable_cache: Option<bool>,
}

fn apply_common<T: AllTraits>(tool: &mut T, p: &CommonParams, reference_dirs: Option<&[String]>) {
    set_number_of_threads(p.thread_number.unwrap_or(0));

    let mut included: Vec<PathBuf> = p.directories.iter().map(PathBuf::from).collect();
    if let Some(refs) = reference_dirs {
        let ref_paths: Vec<PathBuf> = refs.iter().map(PathBuf::from).collect();
        included.extend(ref_paths.clone());
        tool.set_reference_paths(ref_paths);
    }

    tool.set_included_paths(included);
    tool.set_excluded_paths(p.excluded_directories.as_ref().map_or_else(Vec::new, |v| v.iter().map(PathBuf::from).collect()));
    tool.set_excluded_items(p.excluded_items.clone().unwrap_or_default());
    tool.set_recursive_search(!p.not_recursive.unwrap_or(false));
    tool.set_allowed_extensions(p.allowed_extensions.clone().unwrap_or_default());
    tool.set_excluded_extensions(p.excluded_extensions.clone().unwrap_or_default());
    tool.set_use_cache(!p.disable_cache.unwrap_or(false));
}

/// Run a tool's search and serialize results to JSON via a temp file.
fn run_and_serialize<T: AllTraits + PrintResults>(tool: &mut T) -> String {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let (progress_sender, _progress_receiver) = unbounded();

    tool.search(&stop_flag, Some(&progress_sender));

    let tmp = format!("/tmp/czkawka_mcp_{}.json", std::process::id());
    let json = if tool.save_results_to_file_as_json(&tmp, true).is_ok() {
        std::fs::read_to_string(&tmp).unwrap_or_else(|_| "{}".to_string())
    } else {
        "{}".to_string()
    };
    let _ = std::fs::remove_file(&tmp);

    let messages = tool.get_text_messages();
    let warnings = &messages.warnings;
    let errors = &messages.errors;

    if warnings.is_empty() && errors.is_empty() {
        json
    } else {
        format!(
            "{{\n  \"results\": {json},\n  \"warnings\": {warnings},\n  \"errors\": {errors}\n}}",
            warnings = serde_json::to_string(warnings).unwrap_or_default(),
            errors = serde_json::to_string(errors).unwrap_or_default(),
        )
    }
}

fn ok_result(json: String) -> Result<CallToolResult, rmcp::Error> {
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

fn err_result(msg: String) -> Result<CallToolResult, rmcp::Error> {
    Ok(CallToolResult::error(vec![Content::text(msg)]))
}

// ── MCP Server ────────────────────────────────────────────────────────

#[derive(Clone)]
struct CzkawkaServer;

// ── Tool parameter structs ────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
struct FindDuplicatesParams {
    #[serde(flatten)]
    common: CommonParams,
    #[schemars(description = "Reference directories (duplicates are found relative to these)")]
    reference_directories: Option<Vec<String>>,
    #[schemars(description = "Search method: 'hash' (default), 'name', 'size', 'size_name'")]
    search_method: Option<String>,
    #[schemars(description = "Hash algorithm: 'blake3' (default), 'crc32', 'xxh3'")]
    hash_type: Option<String>,
    #[schemars(description = "Minimum file size in bytes (default: 8192)")]
    minimal_file_size: Option<u64>,
    #[schemars(description = "Maximum file size in bytes")]
    maximal_file_size: Option<u64>,
    #[schemars(description = "Allow hard links to be treated as duplicates (default: false)")]
    allow_hard_links: Option<bool>,
    #[schemars(description = "Case-sensitive name comparison (default: false)")]
    case_sensitive_name_comparison: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindEmptyFoldersParams {
    #[serde(flatten)]
    common: CommonParams,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindBiggestFilesParams {
    #[serde(flatten)]
    common: CommonParams,
    #[schemars(description = "Number of files to return (default: 50)")]
    number_of_files: Option<usize>,
    #[schemars(description = "If true, find smallest files instead of biggest (default: false)")]
    smallest_mode: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindEmptyFilesParams {
    #[serde(flatten)]
    common: CommonParams,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindTemporaryParams {
    #[serde(flatten)]
    common: CommonParams,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindSimilarImagesParams {
    #[serde(flatten)]
    common: CommonParams,
    #[schemars(description = "Reference directories")]
    reference_directories: Option<Vec<String>>,
    #[schemars(description = "Maximum difference between images (0-40, default: 3)")]
    max_difference: Option<u32>,
    #[schemars(description = "Hash algorithm: 'gradient' (default), 'mean', 'vertgradient', 'blockhash', 'doublegradient'")]
    hash_alg: Option<String>,
    #[schemars(description = "Image resize filter: 'lanczos3' (default), 'nearest', 'triangle', 'gaussian', 'catmullrom'")]
    image_filter: Option<String>,
    #[schemars(description = "Hash size: 8, 16 (default), 32, 64")]
    hash_size: Option<u8>,
    #[schemars(description = "Minimum file size in bytes")]
    minimal_file_size: Option<u64>,
    #[schemars(description = "Maximum file size in bytes")]
    maximal_file_size: Option<u64>,
    #[schemars(description = "Allow hard links (default: false)")]
    allow_hard_links: Option<bool>,
    #[schemars(description = "Ignore files with same size (default: false)")]
    ignore_same_size: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindSameMusicParams {
    #[serde(flatten)]
    common: CommonParams,
    #[schemars(description = "Reference directories")]
    reference_directories: Option<Vec<String>>,
    #[schemars(description = "Search method: 'tags' (default), 'content'")]
    search_method: Option<String>,
    #[schemars(description = "Music similarity level (0-10000, default: 0 = exact tags)")]
    music_similarity: Option<i32>,
    #[schemars(description = "Approximate tag comparison (default: false)")]
    approximate_comparison: Option<bool>,
    #[schemars(description = "Minimum file size in bytes")]
    minimal_file_size: Option<u64>,
    #[schemars(description = "Maximum file size in bytes")]
    maximal_file_size: Option<u64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindInvalidSymlinksParams {
    #[serde(flatten)]
    common: CommonParams,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindBrokenFilesParams {
    #[serde(flatten)]
    common: CommonParams,
    #[schemars(description = "Types to check: list of 'pdf', 'audio', 'image', 'archive', 'video' (default: all)")]
    checked_types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindSimilarVideosParams {
    #[serde(flatten)]
    common: CommonParams,
    #[schemars(description = "Reference directories")]
    reference_directories: Option<Vec<String>>,
    #[schemars(description = "Tolerance for similarity (1-20, default: 10)")]
    tolerance: Option<i32>,
    #[schemars(description = "Minimum file size in bytes")]
    minimal_file_size: Option<u64>,
    #[schemars(description = "Maximum file size in bytes")]
    maximal_file_size: Option<u64>,
    #[schemars(description = "Allow hard links (default: false)")]
    allow_hard_links: Option<bool>,
    #[schemars(description = "Ignore files with same size (default: false)")]
    ignore_same_size: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindBadExtensionsParams {
    #[serde(flatten)]
    common: CommonParams,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindBadNamesParams {
    #[serde(flatten)]
    common: CommonParams,
    #[schemars(description = "Flag uppercase extensions as bad (default: true)")]
    uppercase_extension: Option<bool>,
    #[schemars(description = "Flag emoji in filenames (default: true)")]
    emoji_used: Option<bool>,
    #[schemars(description = "Flag leading/trailing spaces (default: true)")]
    space_at_start_or_end: Option<bool>,
    #[schemars(description = "Flag non-ASCII characters (default: true)")]
    non_ascii_graphical: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindVideoOptimizerParams {
    #[serde(flatten)]
    common: CommonParams,
    #[schemars(description = "Mode: 'transcode' (default), 'crop'")]
    mode: Option<String>,
    #[schemars(description = "Codecs to exclude (comma-separated, default: 'hevc,h265,av1,vp9')")]
    excluded_codecs: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FindExifTagsParams {
    #[serde(flatten)]
    common: CommonParams,
    #[schemars(description = "EXIF tags to ignore (comma-separated)")]
    ignored_tags: Option<String>,
}

// ── Tool implementations ──────────────────────────────────────────────

#[tool(tool_box)]
impl CzkawkaServer {
    #[tool(description = "Find duplicate files by hash, name, or size in specified directories. Returns groups of duplicate files. Read-only analysis, no files are modified or deleted.")]
    fn find_duplicates(&self, #[tool(aggr)] params: FindDuplicatesParams) -> Result<CallToolResult, rmcp::Error> {
        use czkawka_core::common::model::CheckingMethod;
        use czkawka_core::common::model::HashType;

        let check_method = match params.search_method.as_deref() {
            Some("name") => CheckingMethod::Name,
            Some("size") => CheckingMethod::Size,
            Some("size_name") => CheckingMethod::SizeName,
            _ => CheckingMethod::Hash,
        };
        let hash_type = match params.hash_type.as_deref() {
            Some("crc32") => HashType::Crc32,
            Some("xxh3") => HashType::Xxh3,
            _ => HashType::Blake3,
        };

        let dup_params = DuplicateFinderParameters::new(
            check_method,
            hash_type,
            true,  // use_prehash_cache
            0,     // minimal_cached_file_size
            0,     // minimal_prehash_cache_file_size
            params.case_sensitive_name_comparison.unwrap_or(false),
        );
        let mut tool = DuplicateFinder::new(dup_params);

        apply_common(&mut tool, &params.common, params.reference_directories.as_deref());
        if let Some(min) = params.minimal_file_size {
            tool.set_minimal_file_size(min);
        }
        if let Some(max) = params.maximal_file_size {
            tool.set_maximal_file_size(max);
        }
        tool.set_hide_hard_links(!params.allow_hard_links.unwrap_or(false));
        tool.set_dry_run(true);

        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find empty folders in specified directories. Returns list of empty directory paths. Read-only analysis.")]
    fn find_empty_folders(&self, #[tool(aggr)] params: FindEmptyFoldersParams) -> Result<CallToolResult, rmcp::Error> {
        let mut tool = EmptyFolder::new();
        apply_common(&mut tool, &params.common, None);
        tool.set_dry_run(true);
        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find the biggest (or smallest) files in specified directories. Returns a ranked list of files by size. Read-only analysis.")]
    fn find_biggest_files(&self, #[tool(aggr)] params: FindBiggestFilesParams) -> Result<CallToolResult, rmcp::Error> {
        let mode = if params.smallest_mode.unwrap_or(false) {
            SearchMode::SmallestFiles
        } else {
            SearchMode::BiggestFiles
        };
        let bf_params = BigFileParameters::new(params.number_of_files.unwrap_or(50), mode);
        let mut tool = BigFile::new(bf_params);
        apply_common(&mut tool, &params.common, None);
        tool.set_dry_run(true);
        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find empty files (0 bytes) in specified directories. Read-only analysis.")]
    fn find_empty_files(&self, #[tool(aggr)] params: FindEmptyFilesParams) -> Result<CallToolResult, rmcp::Error> {
        let mut tool = EmptyFiles::new();
        apply_common(&mut tool, &params.common, None);
        tool.set_dry_run(true);
        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find temporary files (e.g. .tmp, ~, .swp) in specified directories. Read-only analysis.")]
    fn find_temporary_files(&self, #[tool(aggr)] params: FindTemporaryParams) -> Result<CallToolResult, rmcp::Error> {
        let mut tool = Temporary::new();
        apply_common(&mut tool, &params.common, None);
        tool.set_dry_run(true);
        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find visually similar images using perceptual hashing. Returns groups of similar images with similarity scores. Read-only analysis.")]
    fn find_similar_images(&self, #[tool(aggr)] params: FindSimilarImagesParams) -> Result<CallToolResult, rmcp::Error> {
        use czkawka_core::re_exported::{FilterType, HashAlg};

        let hash_alg = match params.hash_alg.as_deref() {
            Some("mean") => HashAlg::Mean,
            Some("vertgradient") => HashAlg::VertGradient,
            Some("blockhash") => HashAlg::Blockhash,
            Some("doublegradient") => HashAlg::DoubleGradient,
            _ => HashAlg::Gradient,
        };
        let image_filter = match params.image_filter.as_deref() {
            Some("nearest") => FilterType::Nearest,
            Some("triangle") => FilterType::Triangle,
            Some("gaussian") => FilterType::Gaussian,
            Some("catmullrom") => FilterType::CatmullRom,
            _ => FilterType::Lanczos3,
        };
        let hash_size = params.hash_size.unwrap_or(16);
        let max_difference = params.max_difference.unwrap_or(3);

        let si_params = SimilarImagesParameters::new(
            max_difference,
            hash_size,
            hash_alg,
            image_filter,
            params.ignore_same_size.unwrap_or(false),
        );
        let mut tool = SimilarImages::new(si_params);

        apply_common(&mut tool, &params.common, params.reference_directories.as_deref());
        if let Some(min) = params.minimal_file_size {
            tool.set_minimal_file_size(min);
        }
        if let Some(max) = params.maximal_file_size {
            tool.set_maximal_file_size(max);
        }
        tool.set_hide_hard_links(!params.allow_hard_links.unwrap_or(false));
        tool.set_dry_run(true);

        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find similar or duplicate music files by tags or audio content fingerprint. Returns groups of similar tracks. Read-only analysis.")]
    fn find_same_music(&self, #[tool(aggr)] params: FindSameMusicParams) -> Result<CallToolResult, rmcp::Error> {
        use czkawka_core::common::model::CheckingMethod;
        use czkawka_core::tools::same_music::MusicSimilarity;

        let search_method = match params.search_method.as_deref() {
            Some("content") => CheckingMethod::AudioContent,
            _ => CheckingMethod::AudioTags,
        };

        let music_similarity = if search_method == CheckingMethod::AudioTags {
            let sim_val = params.music_similarity.unwrap_or(0);
            if sim_val == 0 {
                MusicSimilarity::TRACK_TITLE
                    | MusicSimilarity::TRACK_ARTIST
                    | MusicSimilarity::YEAR
                    | MusicSimilarity::BITRATE
                    | MusicSimilarity::GENRE
                    | MusicSimilarity::LENGTH
            } else {
                MusicSimilarity::TRACK_TITLE
            }
        } else {
            // Content mode still requires non-empty MusicSimilarity (assertion in SameMusicParameters::new)
            MusicSimilarity::TRACK_TITLE
        };

        let sm_params = SameMusicParameters::new(
            music_similarity,
            params.approximate_comparison.unwrap_or(false),
            search_method,
            10.0, // minimum_segment_duration
            2.0,  // maximum_difference
            false, // compare_fingerprints_only_with_similar_titles
        );
        let mut tool = SameMusic::new(sm_params);

        apply_common(&mut tool, &params.common, params.reference_directories.as_deref());
        if let Some(min) = params.minimal_file_size {
            tool.set_minimal_file_size(min);
        }
        if let Some(max) = params.maximal_file_size {
            tool.set_maximal_file_size(max);
        }
        tool.set_dry_run(true);

        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find broken/invalid symbolic links in specified directories. Read-only analysis.")]
    fn find_invalid_symlinks(&self, #[tool(aggr)] params: FindInvalidSymlinksParams) -> Result<CallToolResult, rmcp::Error> {
        let mut tool = InvalidSymlinks::new();
        apply_common(&mut tool, &params.common, None);
        tool.set_dry_run(true);
        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find corrupted/broken files (PDF, audio, image, archive, video). Returns list of files that cannot be properly opened. Read-only analysis.")]
    fn find_broken_files(&self, #[tool(aggr)] params: FindBrokenFilesParams) -> Result<CallToolResult, rmcp::Error> {
        let mut checked = CheckedTypes::NONE;
        if let Some(types) = &params.checked_types {
            for t in types {
                match t.to_lowercase().as_str() {
                    "pdf" => checked |= CheckedTypes::PDF,
                    "audio" => checked |= CheckedTypes::AUDIO,
                    "image" => checked |= CheckedTypes::IMAGE,
                    "archive" => checked |= CheckedTypes::ARCHIVE,
                    "video" => checked |= CheckedTypes::VIDEO,
                    other => return err_result(format!("Unknown checked type: '{other}'. Valid: pdf, audio, image, archive, video")),
                }
            }
        } else {
            checked = CheckedTypes::PDF | CheckedTypes::AUDIO | CheckedTypes::IMAGE | CheckedTypes::ARCHIVE | CheckedTypes::VIDEO;
        }

        let bf_params = BrokenFilesParameters::new(checked);
        let mut tool = BrokenFiles::new(bf_params);
        apply_common(&mut tool, &params.common, None);
        tool.set_dry_run(true);

        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find visually similar videos using perceptual hashing. Returns groups of similar video files. Read-only analysis. Requires ffmpeg.")]
    fn find_similar_videos(&self, #[tool(aggr)] params: FindSimilarVideosParams) -> Result<CallToolResult, rmcp::Error> {
        use czkawka_core::re_exported::Cropdetect;

        let sv_params = SimilarVideosParameters::new(
            params.tolerance.unwrap_or(10),
            params.ignore_same_size.unwrap_or(false),
            15,                  // skip_forward_amount (default)
            10,                  // scan_duration (default, must be 2..=60)
            Cropdetect::None,    // crop_detect
            false,               // generate_thumbnails (no sense in MCP)
            10,
            false,
            2,
        );
        let mut tool = SimilarVideos::new(sv_params);

        apply_common(&mut tool, &params.common, params.reference_directories.as_deref());
        if let Some(min) = params.minimal_file_size {
            tool.set_minimal_file_size(min);
        }
        if let Some(max) = params.maximal_file_size {
            tool.set_maximal_file_size(max);
        }
        tool.set_hide_hard_links(!params.allow_hard_links.unwrap_or(false));
        tool.set_dry_run(true);

        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find files with incorrect/mismatched extensions (e.g. a PNG file named .jpg). Read-only analysis.")]
    fn find_bad_extensions(&self, #[tool(aggr)] params: FindBadExtensionsParams) -> Result<CallToolResult, rmcp::Error> {
        let be_params = BadExtensionsParameters::new();
        let mut tool = BadExtensions::new(be_params);
        apply_common(&mut tool, &params.common, None);
        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Find files with problematic names (uppercase extensions, emoji, leading/trailing spaces, non-ASCII characters). Read-only analysis.")]
    fn find_bad_names(&self, #[tool(aggr)] params: FindBadNamesParams) -> Result<CallToolResult, rmcp::Error> {
        let name_issues = NameIssues {
            uppercase_extension: params.uppercase_extension.unwrap_or(true),
            emoji_used: params.emoji_used.unwrap_or(true),
            space_at_start_or_end: params.space_at_start_or_end.unwrap_or(true),
            non_ascii_graphical: params.non_ascii_graphical.unwrap_or(true),
            restricted_charset_allowed: None,
            remove_duplicated_non_alphanumeric: false,
        };
        let bn_params = BadNamesParameters::new(name_issues);
        let mut tool = BadNames::new(bn_params);
        apply_common(&mut tool, &params.common, None);
        tool.set_dry_run(true);

        ok_result(run_and_serialize(&mut tool))
    }

    #[tool(description = "Analyze videos for optimization opportunities (find videos that could be transcoded to better codecs or have black bars cropped). Read-only analysis, no files are modified.")]
    fn analyze_videos(&self, #[tool(aggr)] params: FindVideoOptimizerParams) -> Result<CallToolResult, rmcp::Error> {
        let mode = params.mode.as_deref().unwrap_or("transcode");

        match mode {
            "transcode" => {
                let excluded_codecs: Vec<String> = params
                    .excluded_codecs
                    .as_deref()
                    .unwrap_or("hevc,h265,av1,vp9")
                    .split(',')
                    .map(|c| c.trim().to_string())
                    .collect();

                let vo_params = VideoOptimizerParameters::VideoTranscode(VideoTranscodeParams::new(
                    excluded_codecs,
                    false, // generate_thumbnails
                    50,
                    false,
                    2,
                ));
                let mut tool = VideoOptimizer::new(vo_params);
                apply_common(&mut tool, &params.common, None);
                ok_result(run_and_serialize(&mut tool))
            }
            "crop" => {
                let vo_params = VideoOptimizerParameters::VideoCrop(VideoCropParams::with_custom_params(
                    VideoCroppingMechanism::BlackBars,
                    16,   // black_pixel_threshold
                    3,    // black_bar_percentage
                    10,   // max_samples
                    10,   // min_crop_size
                    false, // generate_thumbnails
                    50,
                    false,
                    2,
                ));
                let mut tool = VideoOptimizer::new(vo_params);
                apply_common(&mut tool, &params.common, None);
                ok_result(run_and_serialize(&mut tool))
            }
            other => err_result(format!("Unknown mode: '{other}'. Valid: transcode, crop")),
        }
    }

    #[tool(description = "Find image files that contain EXIF metadata tags (GPS location, camera info, etc). Read-only analysis, no tags are removed.")]
    fn find_exif_tags(&self, #[tool(aggr)] params: FindExifTagsParams) -> Result<CallToolResult, rmcp::Error> {
        let ignored_tags_vec: Vec<String> = params
            .ignored_tags
            .map(|s| s.split(',').map(|tag| tag.trim().to_string()).collect())
            .unwrap_or_default();

        let er_params = ExifRemoverParameters::new(ignored_tags_vec);
        let mut tool = ExifRemover::new(er_params);
        apply_common(&mut tool, &params.common, None);

        ok_result(run_and_serialize(&mut tool))
    }
}

// ── ServerHandler ─────────────────────────────────────────────────────

#[tool(tool_box)]
impl ServerHandler for CzkawkaServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Czkawka MCP server: 14 file analysis tools for finding duplicates, similar images/videos/music, \
                 broken files, empty files/folders, bad names/extensions, EXIF tags, and video optimization candidates. \
                 All tools are read-only by default - no files are modified or deleted."
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

// ── Entry point ───────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    register_image_decoding_hooks();

    let server = CzkawkaServer;
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;

    Ok(())
}
