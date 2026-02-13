use std::path::PathBuf;

#[cfg(not(feature = "no_colors"))]
use clap::builder::Styles;
#[cfg(not(feature = "no_colors"))]
use clap::builder::styling::AnsiColor;
use czkawka_core::CZKAWKA_VERSION;
use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::tool_data::DeleteMethod;
use czkawka_core::re_exported::{Cropdetect, FilterType, HashAlg};
use czkawka_core::tools::broken_files::CheckedTypes;
use czkawka_core::tools::same_music::MusicSimilarity;
use czkawka_core::tools::similar_videos::{ALLOWED_SKIP_FORWARD_AMOUNT, ALLOWED_VID_HASH_DURATION, DEFAULT_SKIP_FORWARD_AMOUNT, crop_detect_from_str_opt};
use czkawka_core::tools::video_optimizer::VideoCodec;

#[cfg(not(feature = "no_colors"))]
pub const CLAP_STYLING: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Cyan.on_default().bold())
    .placeholder(AnsiColor::Cyan.on_default().bold())
    .error(AnsiColor::Red.on_default().bold())
    .valid(AnsiColor::Green.on_default().bold())
    .invalid(AnsiColor::Yellow.on_default().bold());

#[derive(clap::Parser)]
#[clap(
    name = "czkawka",
    help_template = HELP_TEMPLATE,
    version = CZKAWKA_VERSION,
)]
#[cfg_attr(not(feature = "no_colors"), clap(styles = CLAP_STYLING))]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    #[clap(
        name = "dup",
        about = "Finds duplicate files",
        after_help = "EXAMPLE:\n    czkawka dup -d /home/rafal -e /home/rafal/Obrazy  -m 25 -x 7z rar IMAGE -s hash -f results.txt -D aeo"
    )]
    Duplicates(DuplicatesArgs),
    #[clap(
        name = "empty-folders",
        about = "Finds empty folders",
        after_help = "EXAMPLE:\n    czkawka empty-folders -d /home/rafal/rr /home/gateway -f results.txt"
    )]
    EmptyFolders(EmptyFoldersArgs),
    #[clap(
        name = "big",
        about = "Finds big files",
        after_help = "EXAMPLE:\n    czkawka big -d /home/rafal/ /home/piszczal -e /home/rafal/Roman -n 25 -J -x VIDEO -f results.txt"
    )]
    BiggestFiles(BiggestFilesArgs),
    #[clap(
        name = "empty-files",
        about = "Finds empty files",
        after_help = "EXAMPLE:\n    czkawka empty-files -d /home/rafal /home/szczekacz -e /home/rafal/Pulpit -R -f results.txt"
    )]
    EmptyFiles(EmptyFilesArgs),
    #[clap(
        name = "temp",
        about = "Finds temporary files",
        after_help = "EXAMPLE:\n    czkawka temp -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt -D"
    )]
    Temporary(TemporaryArgs),
    #[clap(
        name = "image",
        about = "Finds similar images",
        after_help = "EXAMPLE:\n    czkawka image -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt"
    )]
    SimilarImages(SimilarImagesArgs),
    #[clap(name = "music", about = "Finds same music by tags", after_help = "EXAMPLE:\n    czkawka music -d /home/rafal -f results.txt")]
    SameMusic(SameMusicArgs),
    #[clap(
        name = "symlinks",
        about = "Finds invalid symlinks",
        after_help = "EXAMPLE:\n    czkawka symlinks -d /home/kicikici/ /home/szczek -e /home/kicikici/jestempsem -x jpg -f results.txt"
    )]
    InvalidSymlinks(InvalidSymlinksArgs),
    #[clap(
        name = "broken",
        about = "Finds broken files",
        after_help = "EXAMPLE:\n    czkawka broken -d /home/kicikici/ /home/szczek -e /home/kicikici/jestempsem -x jpg -f results.txt"
    )]
    BrokenFiles(BrokenFilesArgs),
    #[clap(name = "video", about = "Finds similar video files", after_help = "EXAMPLE:\n    czkawka video -d /home/rafal -f results.txt")]
    SimilarVideos(SimilarVideosArgs),
    #[clap(
        name = "ext",
        about = "Finds files with invalid extensions",
        after_help = "EXAMPLE:\n    czkawka ext -d /home/czokolada/ -f results.txt"
    )]
    BadExtensions(BadExtensionsArgs),
    #[clap(
        name = "bad-names",
        about = "Finds files with bad names",
        after_help = "EXAMPLE:\n    czkawka bad-names -d /home/rafal -f results.txt"
    )]
    BadNames(BadNamesArgs),
    #[clap(
        name = "video-optimizer",
        about = "Optimizes video files (transcode or crop)",
        after_help = "EXAMPLE:\n    czkawka video-optimizer -d /home/rafal -f results.txt"
    )]
    VideoOptimizer(VideoOptimizerArgs),
    #[clap(
        name = "exif-remover",
        about = "Finds and removes EXIF tags from images",
        after_help = "EXAMPLE:\n    czkawka exif-remover -d /home/rafal -f results.txt"
    )]
    ExifRemover(ExifRemoverArgs),
}

#[derive(Debug, clap::Args)]
pub struct DuplicatesArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub reference_directories: ReferenceDirectories,
    #[clap(
        short = 'Z',
        long,
        value_parser = parse_minimal_file_size,
        default_value = "257144",
        help = "Minimum prehash cache file size in bytes",
        long_help = "Minimum size of prehash cached files in bytes"
    )]
    pub minimal_prehash_cache_file_size: u64,
    #[clap(
        short = 'u',
        long,
        help = "Use prehash cache",
        long_help = "Use prehash cache to speed up the scanning process by avoiding rehashing files that have already been hashed"
    )]
    pub use_prehash_cache: bool,
    #[clap(
        short,
        long,
        value_parser = parse_minimal_file_size,
        default_value = "8192",
        help = "Minimum size in bytes",
        long_help = "Minimum size of checked files in bytes, assigning bigger value may speed up searching"
    )]
    pub minimal_file_size: u64,
    #[clap(
        short = 'i',
        long,
        value_parser = parse_maximal_file_size,
        default_value = "18446744073709551615",
        help = "Maximum size in bytes",
        long_help = "Maximum size of checked files in bytes, assigning lower value may speed up searching"
    )]
    pub maximal_file_size: u64,
    #[clap(
        short = 'c',
        long,
        value_parser = parse_minimal_file_size,
        default_value = "257144",
        help = "Minimum cached file size in bytes",
        long_help = "Minimum size of cached files in bytes, assigning bigger value may speed up the scan but loading the cache will be slower, assigning smaller value may slow down the scan and some files may need to be hashed again but loading the cache will be faster"
    )]
    pub minimal_cached_file_size: u64,
    #[clap(
        short,
        long,
        default_value = "HASH",
        value_parser = parse_checking_method_duplicate,
        help = "Search method (NAME, SIZE, HASH)",
        long_help = "Methods to search files.\nNAME - Fast but rarely usable,\nSIZE - Fast but not accurate, checking by the file's size,\nHASH - The slowest method, checking by the hash of the entire file"
    )]
    pub search_method: CheckingMethod,
    #[clap(flatten)]
    pub delete_method: DMethod,
    #[clap(
        short = 't',
        long,
        default_value = "BLAKE3",
        value_parser = parse_hash_type,
        help = "Hash type (BLAKE3, CRC32, XXH3)",
        long_help = "Hash algorithm used to calculate file hashes. BLAKE3 is recommended for most cases (fast and secure), CRC32 is faster but less reliable, XXH3 is very fast but not cryptographically secure."
    )]
    pub hash_type: HashType,
    #[clap(flatten)]
    pub case_sensitive_name_comparison: CaseSensitiveNameComparison,
    #[clap(flatten)]
    pub allow_hard_links: AllowHardLinks,
}

#[derive(Debug, clap::Args)]
pub struct EmptyFoldersArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub delete_method: SDMethod,
}

#[derive(Debug, clap::Args)]
pub struct BiggestFilesArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(
        short,
        long,
        default_value = "50",
        help = "Number of files to be shown",
        long_help = "Number of biggest (or smallest with -J flag) files to display in results"
    )]
    pub number_of_files: usize,
    #[clap(flatten)]
    pub delete_method: SDMethod,
    #[clap(
        short = 'J',
        long,
        help = "Finds the smallest files instead the biggest",
        long_help = "Switch mode to find smallest files instead of biggest ones"
    )]
    pub smallest_mode: bool,
}

#[derive(Debug, clap::Args)]
pub struct EmptyFilesArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub delete_method: SDMethod,
}

#[derive(Debug, clap::Args)]
pub struct TemporaryArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub delete_method: SDMethod,
}

#[derive(Debug, clap::Args)]
pub struct SimilarImagesArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub reference_directories: ReferenceDirectories,
    #[clap(
        short,
        long,
        value_parser = parse_minimal_file_size,
        default_value = "16384",
        help = "Minimum size in bytes",
        long_help = "Minimum size of checked files in bytes, assigning bigger value may speed up searching"
    )]
    pub minimal_file_size: u64,
    #[clap(
        short = 'i',
        long,
        value_parser = parse_minimal_file_size,
        default_value = "18446744073709551615",
        help = "Maximum size in bytes",
        long_help = "Maximum size of checked files in bytes, assigning lower value may speed up searching"
    )]
    pub maximal_file_size: u64,
    #[clap(
        short = 's',
        long,
        default_value = "5",
        value_parser = clap::value_parser!(u32).range(0..=40),
        help = "Maximum difference between images (0-40)",
        long_help = "Maximum difference between images to be considered as similar (0-40). Lower values mean more strict matching. For hash_size 8, values up to 10 are recommended, for hash_size 16 up to 20 are recommended."
    )]
    pub max_difference: u32,
    #[clap(flatten)]
    pub delete_method: DMethod,
    #[clap(flatten)]
    pub allow_hard_links: AllowHardLinks,
    #[clap(flatten)]
    pub ignore_same_size: IgnoreSameSize,
    #[clap(
        short = 'g',
        long,
        default_value = "Gradient",
        value_parser = parse_similar_hash_algorithm,
        help = "Hash algorithm (Mean, Gradient, Blockhash, VertGradient, DoubleGradient, Median)",
        long_help = "Perceptual hash algorithm used to compare images. Gradient (default) works well for most cases, Mean is faster but less accurate, Blockhash is good for finding very similar images, VertGradient/DoubleGradient provide different matching characteristics, Median is robust against color changes."
    )]
    pub hash_alg: HashAlg,
    #[clap(
        short = 'z',
        long,
        default_value = "Nearest",
        value_parser = parse_similar_image_filter,
        help = "Image resize filter (Lanczos3, Nearest, Triangle, Gaussian, CatmullRom)",
        long_help = "Filter algorithm used when resizing images for comparison. Lanczos3 provides highest quality but is slower, Nearest is fastest but lowest quality, Triangle/Gaussian/CatmullRom offer different quality-speed tradeoffs."
    )]
    pub image_filter: FilterType,
    #[clap(
        short = 'c',
        long,
        default_value = "16",
        value_parser = parse_image_hash_size,
        help = "Hash size (8, 16, 32, 64)",
        long_help = "Size of the perceptual hash. Larger values provide more detailed comparison but require higher max_difference values. 8 is fastest and least detailed, 64 is slowest but most detailed. Recommended: 8 or 16 for typical use."
    )]
    pub hash_size: u8,
}

#[derive(Debug, clap::Args)]
pub struct SameMusicArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub reference_directories: ReferenceDirectories,
    #[clap(flatten)]
    pub delete_method: DMethod,
    #[clap(
        short,
        long,
        help = "Approximate comparison of music tags",
        long_help = "Use approximate comparison when comparing music tags (allows small differences in tag values)"
    )]
    pub approximate_comparison: bool,
    #[clap(
        short,
        long,
        help = "Compare fingerprints only with similar titles",
        long_help = "When using audio content comparison, only compare files that have similar titles to reduce false positives and speed up the process"
    )]
    pub compare_fingerprints_only_with_similar_titles: bool,
    #[clap(
        short = 'z',
        long,
        default_value = "track_title,track_artist",
        value_parser = parse_music_duplicate_type,
        help = "Search method (track_title,track_artist,year,bitrate,genre,length)",
        long_help = "Sets which rows must be equal to set these files as duplicates (may be mixed, but must be divided by commas)."
    )]
    pub music_similarity: MusicSimilarity,
    #[clap(
        short,
        long,
        default_value = "TAGS",
        value_parser = parse_checking_method_same_music,
        help = "Search method (CONTENT, TAGS)",
        long_help = "Methods to search files.\nCONTENT - finds similar audio files by content, TAGS - finds similar music by tags."
    )]
    pub search_method: CheckingMethod,
    #[clap(
        short,
        long,
        value_parser = parse_minimal_file_size,
        default_value = "8192",
        help = "Minimum size in bytes",
        long_help = "Minimum size of checked files in bytes, assigning bigger value may speed up searching"
    )]
    pub minimal_file_size: u64,
    #[clap(
        short = 'i',
        long,
        value_parser = parse_maximal_file_size,
        default_value = "18446744073709551615",
        help = "Maximum size in bytes",
        long_help = "Maximum size of checked files in bytes, assigning lower value may speed up searching"
    )]
    pub maximal_file_size: u64,
    #[clap(
        short = 'l',
        long,
        value_parser = parse_minimum_segment_duration,
        default_value = "10.0",
        help = "Minimum segment duration in seconds",
        long_help = "Minimum duration of audio segment to compare in seconds. Smaller values will find shorter similar segments but may increase false positives. Values should be between 0.0 and 3600.0"
    )]
    pub minimum_segment_duration: f32,
    #[clap(
        short = 'Y',
        long,
        value_parser = parse_maximum_difference,
        default_value = "2.0",
        help = "Maximum difference between audio segments",
        long_help = "Maximum allowed difference between audio segments (0.0-10.0). Value 0.0 will find only identical segments, while 10.0 will find segments that are barely similar. Lower values mean stricter matching."
    )]
    pub maximum_difference: f64,
}

fn parse_maximum_difference(src: &str) -> Result<f64, String> {
    match src.parse::<f64>() {
        Ok(maximum_difference) => {
            if maximum_difference <= 0.0 {
                Err("Maximum difference must be bigger than 0".to_string())
            } else if maximum_difference >= 10.0 {
                Err("Maximum difference must be smaller than 10.0".to_string())
            } else {
                Ok(maximum_difference)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
fn parse_minimum_segment_duration(src: &str) -> Result<f32, String> {
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

#[derive(Debug, clap::Args)]
pub struct InvalidSymlinksArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub delete_method: SDMethod,
}

#[derive(Debug, clap::Args)]
pub struct BrokenFilesArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub delete_method: SDMethod,
    #[clap(
        short,
        long,
        default_value = "PDF",
        value_parser = parse_broken_files,
        help = "Checking file types (PDF, AUDIO, IMAGE, ARCHIVE, VIDEO)",
        long_help = "Methods to search files - default PDF.\nPDF - finds broken PDF files,\nAUDIO - finds broken audio files,\nIMAGE - finds broken image files,\nARCHIVE - finds broken archive files,\nVIDEO - finds broken video files"
    )]
    pub checked_types: Vec<CheckedTypes>,
}

#[derive(Debug, clap::Args)]
pub struct SimilarVideosArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub reference_directories: ReferenceDirectories,
    #[clap(flatten)]
    pub delete_method: DMethod,
    #[clap(flatten)]
    pub allow_hard_links: AllowHardLinks,
    #[clap(flatten)]
    pub ignore_same_size: IgnoreSameSize,
    #[clap(
        short,
        long,
        value_parser = parse_minimal_file_size,
        default_value = "8192",
        help = "Minimum size in bytes",
        long_help = "Minimum size of checked files in bytes, assigning bigger value may speed up searching"
    )]
    pub minimal_file_size: u64,
    #[clap(
        short = 'i',
        long,
        value_parser = parse_maximal_file_size,
        default_value = "18446744073709551615",
        help = "Maximum size in bytes",
        long_help = "Maximum size of checked files in bytes, assigning lower value may speed up searching"
    )]
    pub maximal_file_size: u64,
    #[clap(
        short = 't',
        long,
        value_parser = parse_tolerance,
        default_value = "10",
        help = "Video maximum difference (allowed values <0,20>)",
        long_help = "Maximum difference between video frames, bigger value means that videos can looks more and more different (allowed values <0,20>)"
    )]
    pub tolerance: i32,
    #[clap(
        short = 'U',
        long,
        default_value_t = DEFAULT_SKIP_FORWARD_AMOUNT,
        value_parser = parse_skip_forward_amount,
        help = "Skip forward amount in seconds (allowed values: 0-300, default: 15)",
        long_help = "Amount of seconds to skip forward in video. Allowed values are from 0 to 300. 0 means that no skipping will be done. Default is 15."
    )]
    pub skip_forward_amount: u32,
    #[clap(
        short = 'B',
        long,
        default_value = "letterbox",
        value_parser = parse_crop_detect,
        help = "Crop detect method (none, letterbox, motion)",
        long_help = "Method to detect and crop black bars from video frames before comparison. 'none' disables cropping, 'letterbox' removes static black bars, 'motion' uses motion detection to find content area."
    )]
    pub crop_detect: Cropdetect,
    #[clap(
        short = 'A',
        long,
        default_value = "10",
        value_parser = parse_scan_duration,
        help = "Scan duration in seconds",
        long_help = "Duration of video scanning in seconds. Longer duration provides more accurate results but takes more time. Allowed values are predefined in the application."
    )]
    pub scan_duration: u32,
}

#[derive(Debug, clap::Args)]
pub struct BadExtensionsArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(
        short = 'F',
        long,
        help = "Fix bad extensions",
        long_help = "Automatically rename files to use proper extensions based on their detected file type"
    )]
    pub fix_extensions: bool,
}

#[derive(Debug, clap::Args)]
pub struct BadNamesArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(flatten)]
    pub delete_method: SDMethod,
    #[clap(
        short = 'u',
        long,
        help = "Check for uppercase extensions",
        long_help = "Detects files with uppercase extensions (e.g., .JPG instead of .jpg)"
    )]
    pub uppercase_extension: bool,
    #[clap(short = 'j', long, help = "Check for emoji in filenames", long_help = "Detects files with emoji characters in their names")]
    pub emoji_used: bool,
    #[clap(
        short = 'w',
        long,
        help = "Check for spaces at start or end",
        long_help = "Detects files with spaces at the beginning or end of their names"
    )]
    pub space_at_start_or_end: bool,
    #[clap(
        short = 'n',
        long,
        help = "Check for non-ASCII characters",
        long_help = "Detects files with non-ASCII graphical characters in their names"
    )]
    pub non_ascii_graphical: bool,
    #[clap(
        short = 'r',
        long,
        help = "Restricted charset (comma-separated)",
        long_help = "List of allowed special characters. Any other characters will be flagged as problematic. Example: '_- .' for underscore, dash, space, and dot"
    )]
    pub restricted_charset: Option<String>,
    #[clap(
        short = 'a',
        long,
        help = "Check for duplicated non-alphanumeric characters",
        long_help = "Detects files with duplicated non-alphanumeric characters (e.g., 'file__name' or 'file..txt')"
    )]
    pub remove_duplicated_non_alphanumeric: bool,
    #[clap(
        short = 'F',
        long,
        help = "Fix bad names automatically",
        long_help = "Automatically rename files to fix detected naming issues"
    )]
    pub fix_names: bool,
}

#[derive(Debug, clap::Args)]
pub struct VideoOptimizerArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(subcommand)]
    pub mode: VideoOptimizerMode,
}

#[derive(Debug, clap::Subcommand)]
pub enum VideoOptimizerMode {
    #[clap(name = "transcode", about = "Transcode videos to different codec")]
    Transcode(TranscodeArgs),
    #[clap(name = "crop", about = "Crop black bars from videos")]
    Crop(CropArgs),
}

#[derive(Debug, clap::Args)]
pub struct TranscodeArgs {
    #[clap(
        short = 'c',
        long,
        help = "Excluded video codecs (comma-separated)",
        long_help = "Comma-separated list of video codecs to exclude from transcoding (e.g., 'h265,av1,vp9')"
    )]
    pub excluded_codecs: Option<String>,
    #[clap(short = 't', long, help = "Generate thumbnails", long_help = "Generate video thumbnails for preview")]
    pub generate_thumbnails: bool,
    #[clap(
        short = 'V',
        long,
        default_value = "10",
        value_parser = clap::value_parser!(u8).range(1..=99),
        help = "Thumbnail position percentage (1-99)",
        long_help = "Percentage from start of video where thumbnail should be taken (1-99%)"
    )]
    pub thumbnail_percentage: u8,
    #[clap(short = 'g', long, help = "Generate thumbnail grid", long_help = "Generate a grid of thumbnails instead of single thumbnail")]
    pub thumbnail_grid: bool,
    #[clap(
        short = 'Z',
        long,
        default_value = "3",
        value_parser = clap::value_parser!(u8).range(2..=6),
        help = "Thumbnail grid tiles per side (2-6)",
        long_help = "Number of tiles per side for thumbnail grid (2-6). Only used if -g is enabled."
    )]
    pub thumbnail_grid_tiles_per_side: u8,
    #[clap(short = 'F', long, help = "Fix/optimize videos", long_help = "Actually perform the transcoding on found videos")]
    pub fix_videos: bool,
    #[clap(
        long,
        default_value = "h265",
        value_parser = parse_video_codec,
        help = "Target codec (h264, h265, av1, vp9)",
        long_help = "Target video codec for transcoding (h264, h265, av1, vp9). Only used with -F flag."
    )]
    pub target_codec: VideoCodec,
    #[clap(
        long,
        default_value = "23",
        value_parser = clap::value_parser!(u32).range(0..=51),
        help = "Encoding quality (0-51)",
        long_help = "Video encoding quality (0-51). Lower values mean better quality. 23 is default for h264/h265, 30 for av1/vp9."
    )]
    pub quality: u32,
    #[clap(long, help = "Fail if result not smaller", long_help = "Fail the optimization if resulting file is not smaller than original")]
    pub fail_if_not_smaller: bool,
    #[clap(long, help = "Overwrite original files", long_help = "Overwrite original video files with optimized versions")]
    pub overwrite_original: bool,
    #[clap(long, help = "Limit video size", long_help = "Limit maximum video dimensions")]
    pub limit_video_size: bool,
    #[clap(
        long,
        default_value = "1920",
        value_parser = clap::value_parser!(u32),
        help = "Maximum video width",
        long_help = "Maximum video width in pixels when limit_video_size is enabled"
    )]
    pub max_width: u32,
    #[clap(
        long,
        default_value = "1080",
        value_parser = clap::value_parser!(u32),
        help = "Maximum video height",
        long_help = "Maximum video height in pixels when limit_video_size is enabled"
    )]
    pub max_height: u32,
}

#[derive(Debug, clap::Args)]
pub struct CropArgs {
    #[clap(
        short = 'm',
        long,
        default_value = "blackbars",
        value_parser = parse_crop_mechanism,
        help = "Crop detection mechanism (blackbars, staticcontent)",
        long_help = "Mechanism for detecting areas to crop: 'blackbars' for removing black bars, 'staticcontent' for detecting static content areas"
    )]
    pub crop_mechanism: String,
    #[clap(
        short = 'k',
        long,
        default_value = "32",
        value_parser = clap::value_parser!(u8).range(0..=128),
        help = "Black pixel threshold (0-128)",
        long_help = "Threshold for considering a pixel as black when detecting black bars (0-128). Lower values are stricter."
    )]
    pub black_pixel_threshold: u8,
    #[clap(
        short = 'b',
        long,
        default_value = "90",
        value_parser = clap::value_parser!(u8).range(50..=100),
        help = "Black bar minimum percentage (50-100)",
        long_help = "Minimum percentage of black pixels in a line to consider it a black bar (50-100%)"
    )]
    pub black_bar_percentage: u8,
    #[clap(
        short = 's',
        long,
        default_value = "20",
        value_parser = parse_max_samples,
        help = "Maximum samples (5-1000)",
        long_help = "Maximum number of video frames to sample when detecting black bars (5-1000)"
    )]
    pub max_samples: usize,
    #[clap(
        short = 'z',
        long,
        default_value = "10",
        value_parser = parse_min_crop_size,
        help = "Minimum crop size (1-1000)",
        long_help = "Minimum size in pixels for crop area to be considered (1-1000)"
    )]
    pub min_crop_size: u32,
    #[clap(short = 't', long, help = "Generate thumbnails", long_help = "Generate video thumbnails for preview")]
    pub generate_thumbnails: bool,
    #[clap(
        short = 'V',
        long,
        default_value = "10",
        value_parser = clap::value_parser!(u8).range(1..=99),
        help = "Thumbnail position percentage (1-99)",
        long_help = "Percentage from start of video where thumbnail should be taken (1-99%)"
    )]
    pub thumbnail_percentage: u8,
    #[clap(short = 'g', long, help = "Generate thumbnail grid", long_help = "Generate a grid of thumbnails instead of single thumbnail")]
    pub thumbnail_grid: bool,
    #[clap(
        short = 'Z',
        long,
        default_value = "3",
        value_parser = clap::value_parser!(u8).range(2..=6),
        help = "Thumbnail grid tiles per side (2-6)",
        long_help = "Number of tiles per side for thumbnail grid (2-6). Only used if -g is enabled."
    )]
    pub thumbnail_grid_tiles_per_side: u8,
    #[clap(short = 'F', long, help = "Fix/crop videos", long_help = "Actually perform the cropping on found videos")]
    pub fix_videos: bool,
    #[clap(long, help = "Overwrite original files", long_help = "Overwrite original video files with cropped versions")]
    pub overwrite_original: bool,
    #[clap(
        long,
        value_parser = parse_video_codec,
        help = "Target codec (h264, h265, av1, vp9)",
        long_help = "Optional: Also transcode to different codec while cropping. Only used with -F flag."
    )]
    pub target_codec: Option<VideoCodec>,
    #[clap(
        long,
        value_parser = clap::value_parser!(u32).range(0..=51),
        help = "Encoding quality (0-51)",
        long_help = "Video encoding quality when transcoding (0-51). Only used when target_codec is specified."
    )]
    pub quality: Option<u32>,
}

#[derive(Debug, clap::Args)]
pub struct ExifRemoverArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(
        short = 'i',
        long,
        help = "Ignored EXIF tags (comma-separated)",
        long_help = "Comma-separated list of EXIF tag names to ignore (not remove). Example: 'Orientation,DateTime,Software'"
    )]
    pub ignored_tags: Option<String>,
    #[clap(short = 'F', long, help = "Remove EXIF tags", long_help = "Actually remove EXIF tags from files")]
    pub fix_exif: bool,
    #[clap(
        short = 'o',
        long,
        help = "Override original files",
        long_help = "Override original files instead of creating backup files with '_cleaned' suffix"
    )]
    pub override_file: bool,
}

#[derive(Debug, clap::Args)]
pub struct CommonCliItems {
    #[clap(
        short = 'T',
        long,
        default_value = "0",
        help = "Number of threads to use (0 = all available)",
        long_help = "Limits the number of threads used for scanning. Value 0 (default) will use all available CPU threads. Lower values can reduce CPU usage."
    )]
    pub thread_number: usize,
    #[clap(
        short,
        long,
        required = true,
        help = "Directory(ies) to search",
        long_help = "List of directory(ies) to search (absolute paths). These directories will be scanned but not set as reference folders."
    )]
    pub directories: Vec<PathBuf>,
    #[clap(
        short,
        long,
        help = "Excluded directory(ies)",
        long_help = "List of directory(ies) to exclude from search (absolute paths). Files in these directories will be completely ignored."
    )]
    pub excluded_directories: Vec<PathBuf>,
    #[clap(
        short = 'E',
        long,
        help = "Excluded item(s)",
        long_help = "List of excluded items using wildcards (e.g., */temp*, *.tmp). May be slower than -e, so use -e for directories when possible."
    )]
    pub excluded_items: Vec<String>,
    #[clap(
        short = 'x',
        long,
        help = "Allowed file extension(s)",
        long_help = "List of file extensions to check. Helpful macros are available: IMAGE (jpg,kra,gif,png,bmp,tiff,hdr,svg), TEXT (txt,doc,docx,odt,rtf), VIDEO (mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp,m2ts), MUSIC (mp3,flac,ogg,tta,wma,webm)"
    )]
    pub allowed_extensions: Vec<String>,
    #[clap(short = 'P', long, help = "Excluded file extension(s)", long_help = "List of file extensions to exclude from search.")]
    pub excluded_extensions: Vec<String>,
    #[clap(flatten)]
    pub file_to_save: FileToSave,
    #[clap(flatten)]
    pub json_compact_file_to_save: JsonCompactFileToSave,
    #[clap(flatten)]
    pub json_pretty_file_to_save: JsonPrettyFileToSave,
    #[clap(
        short = 'R',
        long,
        help = "Prevents recursive check of folders",
        long_help = "Disables recursive directory traversal. Only files in the top-level directories will be scanned."
    )]
    pub not_recursive: bool,
    #[cfg(target_family = "unix")]
    #[clap(
        short = 'X',
        long,
        help = "Exclude files on other filesystems",
        long_help = "Prevents scanning files on different filesystems (useful to avoid scanning mounted drives, network shares, etc.)"
    )]
    pub exclude_other_filesystems: bool,
    #[clap(flatten)]
    pub do_not_print: DoNotPrint,
    #[clap(
        short = 'W',
        long,
        help = "Ignore error code when files are found",
        long_help = "Suppresses error exit code when duplicate/similar files are found. Useful for scripts that should continue regardless of findings."
    )]
    pub ignore_error_code_on_found: bool,
    #[clap(
        short = 'H',
        long,
        help = "Disable cache",
        long_help = "Disables the cache system. This will make scanning slower but ensures fresh results without cached data."
    )]
    pub disable_cache: bool,
}

#[derive(Debug, clap::Args, Clone, Copy)]
pub struct DoNotPrint {
    #[clap(
        short = 'N',
        long,
        help = "Do not print results to console",
        long_help = "Suppresses printing of search results to the console. Useful when only saving results to files."
    )]
    pub do_not_print_results: bool,
    #[clap(
        short = 'M',
        long,
        help = "Do not print messages to console",
        long_help = "Suppresses all informational messages, warnings, and errors from being printed to console."
    )]
    pub do_not_print_messages: bool,
}

#[derive(Debug, clap::Args, Clone, Copy)]
pub struct DMethod {
    #[clap(
        short = 'D',
        long,
        default_value = "NONE",
        value_parser = parse_delete_method,
        help = "Delete method (AEN, AEO, ON, OO, AEB, AES, OB, OS, HARD)",
        long_help = "Method for selecting which files to delete from duplicate groups:\nAEN - All files Except Newest (keeps newest)\nAEO - All files Except Oldest (keeps oldest)\nON - Only 1 file, the Newest (deletes all but newest)\nOO - Only 1 file, the Oldest (deletes all but oldest)\nAEB - All files Except Biggest (keeps biggest)\nAES - All files Except Smallest (keeps smallest)\nOB - Only 1 file, the Biggest (deletes all but biggest)\nOS - Only 1 file, the Smallest (deletes all but smallest)\nHARD - create hard links to save space\nNONE - do not delete files (default)"
    )]
    pub delete_method: DeleteMethod,
    #[clap(
        short = 'Q',
        long,
        help = "Dry run - preview operations",
        long_help = "Performs a dry run showing what operations would be performed without actually executing them."
    )]
    pub dry_run: bool,
    #[clap(
        short = 'y',
        long,
        help = "Move items to trash",
        long_help = "Instead of permanently deleting files, move them to the system trash/recycle bin where they can be recovered."
    )]
    pub move_to_trash: bool,
}

// Simple delete method - delete files or not
#[derive(Debug, clap::Args, Clone, Copy)]
pub struct SDMethod {
    #[clap(short = 'D', long, help = "Delete found items", long_help = "Automatically delete all found items matching the criteria.")]
    pub delete_files: bool,
    #[clap(
        short = 'Q',
        long,
        help = "Dry run - preview operations",
        long_help = "Performs a dry run showing what operations would be performed without actually executing them."
    )]
    pub dry_run: bool,
    #[clap(
        short = 'y',
        long,
        help = "Move items to trash",
        long_help = "Instead of permanently deleting files, move them to the system trash/recycle bin where they can be recovered."
    )]
    pub move_to_trash: bool,
}

#[derive(Debug, clap::Args)]
pub struct FileToSave {
    #[clap(
        short,
        long,
        value_name = "file-name",
        help = "Save results to formatted text file",
        long_help = "Saves the search results into a human-readable formatted text file."
    )]
    pub file_to_save: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct ReferenceDirectories {
    #[clap(
        short,
        long,
        help = "Reference directory(ies)",
        long_help = "List of reference directory(ies) to search (absolute paths). Files in these directories will be scanned but won't appear in the results (useful for comparing against a known good set of files)."
    )]
    pub reference_directories: Vec<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct JsonCompactFileToSave {
    #[clap(
        short = 'C',
        long,
        value_name = "json-file-name",
        help = "Save results to compact JSON file",
        long_help = "Saves the search results into a compact (minified) JSON file without extra whitespace."
    )]
    pub compact_file_to_save: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct JsonPrettyFileToSave {
    #[clap(
        short,
        long,
        value_name = "pretty-json-file-name",
        help = "Save results to pretty JSON file",
        long_help = "Saves the search results into a pretty-printed (indented) JSON file for better readability."
    )]
    pub pretty_file_to_save: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct AllowHardLinks {
    #[clap(
        short = 'L',
        long,
        help = "Do not ignore hard links",
        long_help = "Treats hard links as separate files rather than ignoring them. By default, hard links are detected and only counted once."
    )]
    pub allow_hard_links: bool,
}

#[derive(Debug, clap::Args)]
pub struct CaseSensitiveNameComparison {
    #[clap(
        short = 'l',
        long,
        help = "Use case-sensitive name comparison",
        long_help = "Enables case-sensitive file name comparison. By default, comparisons are case-insensitive (e.g., 'File.txt' equals 'file.txt')."
    )]
    pub case_sensitive_name_comparison: bool,
}

#[derive(Debug, clap::Args)]
pub struct IgnoreSameSize {
    #[clap(
        short = 'J',
        long,
        help = "Ignore files with same size",
        long_help = "Groups files by size and keeps only one file from each size group, ignoring files with identical sizes (useful for quick deduplication based solely on file size)."
    )]
    pub ignore_same_size: bool,
}

impl FileToSave {
    pub(crate) fn file_name(&self) -> Option<&str> {
        if let Some(file_name) = &self.file_to_save {
            return file_name.to_str();
        }

        None
    }
}
impl JsonCompactFileToSave {
    pub(crate) fn file_name(&self) -> Option<&str> {
        if let Some(file_name) = &self.compact_file_to_save {
            return file_name.to_str();
        }

        None
    }
}
impl JsonPrettyFileToSave {
    pub(crate) fn file_name(&self) -> Option<&str> {
        if let Some(file_name) = &self.pretty_file_to_save {
            return file_name.to_str();
        }

        None
    }
}

fn parse_scan_duration(s: &str) -> Result<u32, String> {
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

fn parse_crop_detect(src: &str) -> Result<Cropdetect, String> {
    match crop_detect_from_str_opt(src) {
        Some(crop_detect) => Ok(crop_detect),
        None => Err(format!("Crop detect \"{src}\" is not valid")),
    }
}

fn parse_skip_forward_amount(src: &str) -> Result<u32, String> {
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
fn parse_hash_type(src: &str) -> Result<HashType, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "blake3" => Ok(HashType::Blake3),
        "crc32" => Ok(HashType::Crc32),
        "xxh3" => Ok(HashType::Xxh3),
        _ => Err("Couldn't parse the hash type (allowed: BLAKE3, CRC32, XXH3)"),
    }
}

fn parse_tolerance(src: &str) -> Result<i32, &'static str> {
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

fn parse_checking_method_duplicate(src: &str) -> Result<CheckingMethod, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "name" => Ok(CheckingMethod::Name),
        "size" => Ok(CheckingMethod::Size),
        "size_name" => Ok(CheckingMethod::SizeName),
        "hash" => Ok(CheckingMethod::Hash),
        _ => Err("Couldn't parse the search method (allowed: NAME, SIZE, HASH)"),
    }
}

fn parse_broken_files(src: &str) -> Result<CheckedTypes, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "pdf" => Ok(CheckedTypes::PDF),
        "audio" => Ok(CheckedTypes::AUDIO),
        "image" => Ok(CheckedTypes::IMAGE),
        "archive" => Ok(CheckedTypes::ARCHIVE),
        "video" => Ok(CheckedTypes::VIDEO),
        _ => Err("Couldn't parse the broken files type (allowed: PDF, AUDIO, IMAGE, ARCHIVE, VIDEO)"),
    }
}

fn parse_checking_method_same_music(src: &str) -> Result<CheckingMethod, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "tags" => Ok(CheckingMethod::AudioTags),
        "content" => Ok(CheckingMethod::AudioContent),
        _ => Err("Couldn't parse the search method (allowed: TAGS, CONTENT)"),
    }
}

fn parse_video_codec(src: &str) -> Result<VideoCodec, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "h264" => Ok(VideoCodec::H264),
        "h265" | "hevc" => Ok(VideoCodec::H265),
        "av1" => Ok(VideoCodec::Av1),
        "vp9" => Ok(VideoCodec::Vp9),
        _ => Err("Couldn't parse the video codec (allowed: h264, h265, av1, vp9)"),
    }
}

fn parse_max_samples(src: &str) -> Result<usize, String> {
    match src.parse::<usize>() {
        Ok(val) if (5..=1000).contains(&val) => Ok(val),
        Ok(_) => Err("Maximum samples must be between 5 and 1000".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_min_crop_size(src: &str) -> Result<u32, String> {
    match src.parse::<u32>() {
        Ok(val) if (1..=1000).contains(&val) => Ok(val),
        Ok(_) => Err("Minimum crop size must be between 1 and 1000".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_delete_method(src: &str) -> Result<DeleteMethod, &'static str> {
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

fn parse_minimal_file_size(src: &str) -> Result<u64, String> {
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

fn parse_maximal_file_size(src: &str) -> Result<u64, String> {
    match src.parse::<u64>() {
        Ok(maximal_file_size) => Ok(maximal_file_size),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_similar_image_filter(src: &str) -> Result<FilterType, String> {
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

fn parse_similar_hash_algorithm(src: &str) -> Result<HashAlg, String> {
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

fn parse_image_hash_size(src: &str) -> Result<u8, String> {
    let hash_size = match src.to_lowercase().as_str() {
        "8" => 8,
        "16" => 16,
        "32" => 32,
        "64" => 64,
        _ => return Err("Couldn't parse the image hash size (allowed: 8, 16, 32, 64)".to_string()),
    };
    Ok(hash_size)
}

fn parse_music_duplicate_type(src: &str) -> Result<MusicSimilarity, String> {
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

fn parse_crop_mechanism(src: &str) -> Result<String, String> {
    match src.to_lowercase().as_str() {
        "blackbars" | "staticcontent" => Ok(src.to_lowercase()),
        _ => Err("Invalid crop mechanism. Allowed values: blackbars, staticcontent".to_string()),
    }
}

const HELP_TEMPLATE: &str = r#"
{bin} {version}

USAGE:
    {usage} [FLAGS] [OPTIONS]

OPTIONS:
{options}

COMMANDS:
{subcommands}

    try "{usage} -h" to get more info about a specific tool

EXAMPLES:
    {bin} dup -d /home/rafal -e /home/rafal/Obrazy  -m 25 -x 7z rar IMAGE -s hash -f results.txt -D aeo
    {bin} empty-folders -d /home/rafal/rr /home/gateway -f results.txt
    {bin} big -d /home/rafal/ /home/piszczal -e /home/rafal/Roman -n 25 -x VIDEO -f results.txt
    {bin} empty-files -d /home/rafal /home/szczekacz -e /home/rafal/Pulpit -R -f results.txt
    {bin} temp -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt -D
    {bin} image -d /home/rafal -e /home/rafal/Pulpit -f results.txt
    {bin} music -d /home/rafal -e /home/rafal/Pulpit -z \"artist,year,ARTISTALBUM,ALBUM___tiTlE\"  -f results.txt
    {bin} symlinks -d /home/kicikici/ /home/szczek -e /home/kicikici/jestempsem -x jpg -f results.txt
    {bin} broken -d /home/mikrut/ -e /home/mikrut/trakt -f results.txt
    {bin} ext -d /home/mikrut/ -e /home/mikrut/trakt -f results.txt
    {bin} bad-names -d /home/rafal -u -j -w -n -f results.txt
    {bin} video-optimizer -d /home/rafal transcode -c h264 -f results.txt
    {bin} video-optimizer -d /home/rafal crop -m blackbars -f results.txt
    {bin} exif-remover -d /home/rafal -x IMAGE -f results.txt"#;
