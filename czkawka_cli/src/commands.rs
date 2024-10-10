use std::path::PathBuf;

use czkawka_core::broken_files::CheckedTypes;
use czkawka_core::common_dir_traversal::CheckingMethod;
use czkawka_core::common_tool::DeleteMethod;
use czkawka_core::duplicate::HashType;
use czkawka_core::same_music::MusicSimilarity;
use czkawka_core::similar_images::SimilarityPreset;
use czkawka_core::CZKAWKA_VERSION;
use image_hasher::{FilterType, HashAlg};

#[derive(clap::Parser)]
#[clap(
    name = "czkawka",
    help_template = HELP_TEMPLATE,
    version = CZKAWKA_VERSION
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    #[clap(
        name = "dup",
        about = "Finds duplicate files",
        after_help = "EXAMPLE:\n    czkawka dup -d /home/rafal - -e /home/rafal/Obrazy  -m 25 -x 7z rar IMAGE -s hash -f results.txt -D aeo"
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
    #[clap(name = "video", about = "Finds similar video files", after_help = "EXAMPLE:\n    czkawka videos -d /home/rafal -f results.txt")]
    SimilarVideos(SimilarVideosArgs),
    #[clap(
        name = "ext",
        about = "Finds files with invalid extensions",
        after_help = "EXAMPLE:\n    czkawka ext -d /home/czokolada/ -f results.txt"
    )]
    BadExtensions(BadExtensionsArgs),
    #[clap(name = "tester", about = "Small utility to test supported speed of ", after_help = "EXAMPLE:\n    czkawka tester")]
    Tester,
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
        long_help = "Methods to search files.\nNAME - Fast but but rarely usable,\nSIZE - Fast but not accurate, checking by the file's size,\nHASH - The slowest method, checking by the hash of the entire file"
    )]
    pub search_method: CheckingMethod,
    #[clap(flatten)]
    pub delete_method: DMethod,
    #[clap(
        short = 't',
        long,
        default_value = "BLAKE3",
        value_parser = parse_hash_type,
        help = "Hash type (BLAKE3, CRC32, XXH3)"
    )]
    pub hash_type: HashType,
    #[clap(flatten)]
    pub case_sensitive_name_comparison: CaseSensitiveNameComparison,
    #[clap(flatten)]
    pub allow_hard_links: AllowHardLinks,
    #[clap(flatten)]
    pub dry_run: DryRun,
}

#[derive(Debug, clap::Args)]
pub struct EmptyFoldersArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(short = 'D', long, help = "Delete found folders")]
    pub delete_folders: bool,
}

#[derive(Debug, clap::Args)]
pub struct BiggestFilesArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(short, long, default_value = "50", help = "Number of files to be shown")]
    pub number_of_files: usize,
    #[clap(short = 'D', long, help = "Delete found files")]
    pub delete_files: bool,
    #[clap(short = 'J', long, help = "Finds the smallest files instead the biggest")]
    pub smallest_mode: bool,
}

#[derive(Debug, clap::Args)]
pub struct EmptyFilesArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(short = 'D', long, help = "Delete found files")]
    pub delete_files: bool,
}

#[derive(Debug, clap::Args)]
pub struct TemporaryArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(short = 'D', long, help = "Delete found files")]
    pub delete_files: bool,
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
        short,
        long,
        default_value = "High",
        value_parser = parse_similar_images_similarity,
        help = "Similairty level (Minimal, VerySmall, Small, Medium, High, VeryHigh, Original)",
        long_help = "Methods to choose similarity level of images which will be considered as duplicated."
    )]
    pub similarity_preset: SimilarityPreset,
    #[clap(flatten)]
    pub delete_method: DMethod,
    #[clap(flatten)]
    pub allow_hard_links: AllowHardLinks,
    #[clap(flatten)]
    pub dry_run: DryRun,
    #[clap(flatten)]
    pub ignore_same_size: IgnoreSameSize,
    #[clap(
        short = 'g',
        long,
        default_value = "Gradient",
        value_parser = parse_similar_hash_algorithm,
        help = "Hash algorithm (allowed: Mean, Gradient, Blockhash, VertGradient, DoubleGradient, Median)"
    )]
    pub hash_alg: HashAlg,
    #[clap(
        short = 'z',
        long,
        default_value = "Nearest",
        value_parser = parse_similar_image_filter,
        help = "Hash algorithm (allowed: Lanczos3, Nearest, Triangle, Faussian, Catmullrom)"
    )]
    pub image_filter: FilterType,
    #[clap(
        short = 'c',
        long,
        default_value = "16",
        value_parser = parse_image_hash_size,
        help = "Hash size (allowed: 8, 16, 32, 64)"
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
    #[clap(flatten)]
    pub dry_run: DryRun,
    #[clap(short, long, help = "Approximate comparison of music tags.")]
    pub approximate_comparison: bool,
    #[clap(short, long, help = "Compare only m.")]
    pub compare_fingerprints_only_with_similar_titles: bool,
    #[clap(
        short = 'z',
        long,
        default_value = "track_title,track_artist",
        value_parser = parse_music_duplicate_type,
        help = "Search method (track_title,track_artist,year,bitrate,genre,length))",
        long_help = "Sets which rows must be equal to set this files as duplicates(may be mixed, but must be divided by commas)."
    )]
    pub music_similarity: MusicSimilarity,
    #[clap(
        short,
        long,
        default_value = "TAGS",
        value_parser = parse_checking_method_same_music,
        help = "Search method (CONTENT, TAGS)",
        long_help = "Methods to search files.\nCONTENT - finds similar audio files by content, TAGS - finds similar images by tags, needs to set"
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
        help = "Maximum size in bytes",
        long_help = "Minimum segment duration, smaller value will finds also shorter similar segments, which may increase false positives number"
    )]
    pub minimum_segment_duration: f32,
    #[clap(
        short = 'Y',
        long,
        value_parser = parse_maximum_difference,
        default_value = "2.0",
        help = "Maximum difference between segments",
        long_help = "Maximum difference between segments, 0.0 will find only identical segments, 10.0 will find also segments which are almost not similar at all"
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
    #[clap(short = 'D', long, help = "Delete found files")]
    pub delete_files: bool,
}

#[derive(Debug, clap::Args)]
pub struct BrokenFilesArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
    #[clap(short = 'D', long, help = "Delete found files")]
    pub delete_files: bool,
    #[clap(
        short,
        long,
        default_value = "PDF",
        value_parser = parse_broken_files,
        help = "Checking file types (PDF, AUDIO, IMAGE, ARCHIVE)",
        long_help = "Methods to search files - default PDF.\nPDF - finds broken PDF files,\nAUDIO - finds broken audio files,\nIMAGE - finds broken image files,\nARCHIVE - finds broken archive files"
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
    pub dry_run: DryRun,
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
}

#[derive(Debug, clap::Args)]
pub struct BadExtensionsArgs {
    #[clap(flatten)]
    pub common_cli_items: CommonCliItems,
}

#[derive(Debug, clap::Args)]
pub struct CommonCliItems {
    #[clap(short = 'T', long, default_value = "0", help = "Limits thread number, 0(default) will use all available threads")]
    pub thread_number: usize,
    #[clap(
        short,
        long,
        required = true,
        help = "Directorie(s) to search",
        long_help = "List of directorie(s) which will be searched(absolute path) - this directories are not set as reference folders"
    )]
    pub directories: Vec<PathBuf>,
    #[clap(
        short,
        long,
        help = "Excluded directorie(s)",
        long_help = "List of directorie(s) which will be excluded from search(absolute path)"
    )]
    pub excluded_directories: Vec<PathBuf>,
    #[clap(
        short = 'E',
        long,
        help = "Excluded item(s)",
        long_help = "List of excluded item(s) which contains * wildcard(may be slow, so use -e where possible)"
    )]
    pub excluded_items: Vec<String>,
    #[clap(
        short = 'x',
        long,
        help = "Allowed file extension(s)",
        long_help = "List of checked files with provided extension(s). There are also helpful macros which allow to easy use a typical extensions like:\nIMAGE(\"jpg,kra,gif,png,bmp,tiff,hdr,svg\"),\nTEXT(\"txt,doc,docx,odt,rtf\"),\nVIDEO(\"mp4,flv,mkv,webm,vob,ogv,gifv,avi,mov,wmv,mpg,m4v,m4p,mpeg,3gp\") or\nMUSIC(\"mp3,flac,ogg,tta,wma,webm\")\n "
    )]
    pub allowed_extensions: Vec<String>,
    #[clap(short = 'P', long, help = "Excluded file extension(s)", long_help = "List of extensions, that will be removed from search.\n ")]
    pub excluded_extensions: Vec<String>,
    #[clap(flatten)]
    pub file_to_save: FileToSave,
    #[clap(flatten)]
    pub json_compact_file_to_save: JsonCompactFileToSave,
    #[clap(flatten)]
    pub json_pretty_file_to_save: JsonPrettyFileToSave,
    #[clap(short = 'R', long, help = "Prevents from recursive check of folders")]
    pub not_recursive: bool,
    #[cfg(target_family = "unix")]
    #[clap(short = 'X', long, help = "Exclude files on other filesystems")]
    pub exclude_other_filesystems: bool,
}

#[derive(Debug, clap::Args)]
pub struct DMethod {
    #[clap(
        short = 'D',
        long,
        default_value = "NONE",
        value_parser = parse_delete_method,
        help = "Delete method (AEN, AEO, ON, OO, AEB, AES, OE, OS, HARD)",
        long_help = "Methods to delete the files.\nAEN - All files except the newest,\nAEO - All files except the oldest,\nON - Only 1 file, the newest,\nOO - Only 1 file, the oldest\nAEB - All files except the biggest,\nAES - All files except the smallest,\nOB - Only 1 file, the biggest,\nOS - Only 1 file, the smallest\nHARD - create hard link\nNONE - not delete files"
    )]
    pub delete_method: DeleteMethod,
}

#[derive(Debug, clap::Args)]
pub struct FileToSave {
    #[clap(short, long, value_name = "file-name", help = "Saves the results into the formatted txt file")]
    pub file_to_save: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct ReferenceDirectories {
    #[clap(
        short,
        long,
        help = "Reference directorie(s) to search",
        long_help = "List of directorie(s) which will be searched(absolute path) - this directories are set as reference folders, so will not be visible in the results"
    )]
    pub reference_directories: Vec<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct JsonCompactFileToSave {
    #[clap(short = 'C', long, value_name = "json-file-name", help = "Saves the results into the compact json file")]
    pub compact_file_to_save: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct JsonPrettyFileToSave {
    #[clap(short, long, value_name = "pretty-json-file-name", help = "Saves the results into the pretty json file")]
    pub pretty_file_to_save: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct AllowHardLinks {
    #[clap(short = 'L', long, help = "Do not ignore hard links")]
    pub allow_hard_links: bool,
}

#[derive(Debug, clap::Args)]
pub struct CaseSensitiveNameComparison {
    #[clap(short = 'l', long, help = "Use case sensitive name comparison")]
    pub case_sensitive_name_comparison: bool,
}

#[derive(Debug, clap::Args)]
pub struct DryRun {
    #[clap(long, help = "Do nothing and print the operation that would happen.")]
    pub dry_run: bool,
}

#[derive(Debug, clap::Args)]
pub struct IgnoreSameSize {
    #[clap(short = 'J', long, help = "Ignore files with the same size, leaving only one file of each size")]
    pub ignore_same_size: bool,
}

impl FileToSave {
    pub fn file_name(&self) -> Option<&str> {
        if let Some(file_name) = &self.file_to_save {
            return file_name.to_str();
        }

        None
    }
}
impl JsonCompactFileToSave {
    pub fn file_name(&self) -> Option<&str> {
        if let Some(file_name) = &self.compact_file_to_save {
            return file_name.to_str();
        }

        None
    }
}
impl JsonPrettyFileToSave {
    pub fn file_name(&self) -> Option<&str> {
        if let Some(file_name) = &self.pretty_file_to_save {
            return file_name.to_str();
        }

        None
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
        _ => Err("Couldn't parse the broken files type (allowed: PDF, AUDIO, IMAGE, ARCHIVE)"),
    }
}

fn parse_checking_method_same_music(src: &str) -> Result<CheckingMethod, &'static str> {
    match src.to_ascii_lowercase().as_str() {
        "tags" => Ok(CheckingMethod::AudioTags),
        "content" => Ok(CheckingMethod::AudioContent),
        _ => Err("Couldn't parse the searc method (allowed: TAGS, CONTENT)"),
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

fn parse_similar_images_similarity(src: &str) -> Result<SimilarityPreset, &'static str> {
    match src.to_lowercase().replace('_', "").as_str() {
        "minimal" => Ok(SimilarityPreset::Minimal),
        "verysmall" => Ok(SimilarityPreset::VerySmall),
        "small" => Ok(SimilarityPreset::Small),
        "medium" => Ok(SimilarityPreset::Medium),
        "high" => Ok(SimilarityPreset::High),
        "veryhigh" => Ok(SimilarityPreset::VeryHigh),
        _ => Err("Couldn't parse the image similarity preset (allowed: Minimal, VerySmall, Small, Medium, High, VeryHigh)"),
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
        "faussian" => FilterType::Gaussian,
        "catmullrom" => FilterType::CatmullRom,
        _ => return Err("Couldn't parse the image resize filter (allowed: Lanczos3, Nearest, Triangle, Faussian, Catmullrom)".to_string()),
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

const HELP_TEMPLATE: &str = r#"
{bin} {version}

USAGE:
    {usage} [SCFLAGS] [SCOPTIONS]

OPTIONS:
{options}

SUBCOMMANDS:
{subcommands}

    try "{usage} -h" to get more info about a specific tool

EXAMPLES:
    {bin} dup -d /home/rafal -e /home/rafal/Obrazy  -m 25 -x 7z rar IMAGE -s hash -f results.txt -D aeo
    {bin} empty-folders -d /home/rafal/rr /home/gateway -f results.txt
    {bin} big -d /home/rafal/ /home/piszczal -e /home/rafal/Roman -n 25 -x VIDEO -f results.txt
    {bin} empty-files -d /home/rafal /home/szczekacz -e /home/rafal/Pulpit -R -f results.txt
    {bin} temp -d /home/rafal/ -E */.git */tmp* *Pulpit -f results.txt -D
    {bin} image -d /home/rafal -e /home/rafal/Pulpit -f results.txt
    {bin} music -d /home/rafal -e /home/rafal/Pulpit -z "artist,year, ARTISTALBUM, ALBUM___tiTlE"  -f results.txt
    {bin} symlinks -d /home/kicikici/ /home/szczek -e /home/kicikici/jestempsem -x jpg -f results.txt
    {bin} broken -d /home/mikrut/ -e /home/mikrut/trakt -f results.txt
    {bin} extnp -d /home/mikrut/ -e /home/mikrut/trakt -f results.txt"#;
