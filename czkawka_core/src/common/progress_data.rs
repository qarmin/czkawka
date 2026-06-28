use humansize::{BINARY, format_size};

use crate::common::model::CheckingMethod;
use crate::flc;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CacheLoadPhase {
    Loading,
    FilteringOutdated,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DuplicateStage {
    HidingHardLinks,
    LoadingPreHashCache(CacheLoadPhase),
    PreHashing,
    SavingPreHashCache,
    LoadingHashCache(CacheLoadPhase),
    FullHashing,
    SavingHashCache,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SameMusicMode {
    AudioTags,
    AudioContent,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SameMusicStage {
    LoadingTagsCache(CacheLoadPhase),
    ReadingTags,
    SavingTagsCache,
    ComparingTags,
    LoadingFingerprintCache(CacheLoadPhase),
    CalculatingFingerprints,
    SavingFingerprintCache,
    ComparingFingerprints,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SimilarImagesStage {
    HidingHardLinks,
    CalculatingHashes,
    ComparingHashes,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SimilarVideosMode {
    VisualHash,
    AudioContent,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SimilarVideosStage {
    HidingHardLinks,
    CalculatingHashes,
    CreatingThumbnails,
    LoadingAudioCache(CacheLoadPhase),
    CalculatingAudioFingerprints,
    SavingAudioCache,
    ComparingAudioFingerprints,
    CreatingAudioThumbnails,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ExifRemoverStage {
    LoadingCache(CacheLoadPhase),
    ExtractingTags,
    SavingCache,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VideoOptimizerStage {
    ProcessingVideos,
    CreatingThumbnails,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ToolStage {
    CollectingFiles(CheckingMethod),
    CollectingFolders,

    DeletingFiles,
    RenamingFiles,
    MovingFiles,
    HardlinkingFiles,
    SymlinkingFiles,
    OptimizingVideos,
    CleaningExif,

    Duplicate(DuplicateStage),
    SameMusic(SameMusicMode, SameMusicStage),
    SimilarImages(SimilarImagesStage),
    SimilarVideos(SimilarVideosMode, SimilarVideosStage),
    ExifRemover(ExifRemoverStage),
    VideoOptimizer(VideoOptimizerStage),

    BrokenFilesChecking,
    BadExtensionsChecking,
    BadNamesChecking,
    EmptyFilesCheckingContent,
}

impl ToolStage {
    pub fn is_special_non_tool_stage(self) -> bool {
        matches!(
            self,
            Self::DeletingFiles | Self::RenamingFiles | Self::MovingFiles | Self::HardlinkingFiles | Self::SymlinkingFiles | Self::OptimizingVideos | Self::CleaningExif
        )
    }

    pub fn is_cache_loading_saving(self) -> bool {
        self.is_cache_loading() || self.is_cache_saving()
    }

    pub fn is_cache_loading(self) -> bool {
        matches!(
            self,
            Self::Duplicate(DuplicateStage::LoadingPreHashCache(_) | DuplicateStage::LoadingHashCache(_))
                | Self::SameMusic(_, SameMusicStage::LoadingTagsCache(_) | SameMusicStage::LoadingFingerprintCache(_))
                | Self::ExifRemover(ExifRemoverStage::LoadingCache(_))
                | Self::SimilarVideos(_, SimilarVideosStage::LoadingAudioCache(_))
        )
    }

    pub fn is_cache_saving(self) -> bool {
        matches!(
            self,
            Self::Duplicate(DuplicateStage::SavingPreHashCache | DuplicateStage::SavingHashCache)
                | Self::SameMusic(_, SameMusicStage::SavingTagsCache | SameMusicStage::SavingFingerprintCache)
                | Self::ExifRemover(ExifRemoverStage::SavingCache)
                | Self::SimilarVideos(_, SimilarVideosStage::SavingAudioCache)
        )
    }

    pub fn is_indeterminate(self) -> bool {
        matches!(
            self,
            Self::CollectingFiles(_)
                | Self::CollectingFolders
                | Self::Duplicate(
                    DuplicateStage::LoadingPreHashCache(CacheLoadPhase::Loading)
                        | DuplicateStage::LoadingHashCache(CacheLoadPhase::Loading)
                        | DuplicateStage::SavingPreHashCache
                        | DuplicateStage::SavingHashCache,
                )
                | Self::SameMusic(
                    _,
                    SameMusicStage::LoadingTagsCache(CacheLoadPhase::Loading)
                        | SameMusicStage::LoadingFingerprintCache(CacheLoadPhase::Loading)
                        | SameMusicStage::SavingTagsCache
                        | SameMusicStage::SavingFingerprintCache,
                )
                | Self::ExifRemover(ExifRemoverStage::LoadingCache(CacheLoadPhase::Loading) | ExifRemoverStage::SavingCache)
                | Self::SimilarVideos(_, SimilarVideosStage::LoadingAudioCache(CacheLoadPhase::Loading) | SimilarVideosStage::SavingAudioCache)
        )
    }

    pub fn uses_bytes(self) -> bool {
        matches!(
            self,
            Self::Duplicate(DuplicateStage::PreHashing | DuplicateStage::FullHashing)
                | Self::SimilarImages(SimilarImagesStage::CalculatingHashes)
                | Self::SameMusic(_, SameMusicStage::CalculatingFingerprints)
                | Self::SimilarVideos(_, SimilarVideosStage::CalculatingAudioFingerprints)
                | Self::ExifRemover(ExifRemoverStage::ExtractingTags)
                | Self::BrokenFilesChecking
                | Self::EmptyFilesCheckingContent
                | Self::VideoOptimizer(VideoOptimizerStage::ProcessingVideos)
        )
    }

    pub fn current_stage_idx(self) -> u8 {
        match self {
            Self::CollectingFiles(_)
            | Self::CollectingFolders
            | Self::DeletingFiles
            | Self::RenamingFiles
            | Self::MovingFiles
            | Self::HardlinkingFiles
            | Self::SymlinkingFiles
            | Self::OptimizingVideos
            | Self::CleaningExif => 0,

            Self::Duplicate(s) => match s {
                DuplicateStage::HidingHardLinks => 1,
                DuplicateStage::LoadingPreHashCache(_) => 2,
                DuplicateStage::PreHashing => 3,
                DuplicateStage::SavingPreHashCache => 4,
                DuplicateStage::LoadingHashCache(_) => 5,
                DuplicateStage::FullHashing => 6,
                DuplicateStage::SavingHashCache => 7,
            },

            Self::SameMusic(mode, s) => match (mode, s) {
                (_, SameMusicStage::LoadingTagsCache(_)) => 1,
                (_, SameMusicStage::ReadingTags) => 2,
                (_, SameMusicStage::SavingTagsCache) => 3,
                (_, SameMusicStage::ComparingTags | SameMusicStage::LoadingFingerprintCache(_)) => 4,
                (_, SameMusicStage::CalculatingFingerprints) => 5,
                (_, SameMusicStage::SavingFingerprintCache) => 6,
                (_, SameMusicStage::ComparingFingerprints) => 7,
            },

            Self::SimilarImages(s) => match s {
                SimilarImagesStage::HidingHardLinks => 1,
                SimilarImagesStage::CalculatingHashes => 2,
                SimilarImagesStage::ComparingHashes => 3,
            },

            Self::SimilarVideos(SimilarVideosMode::VisualHash, s) => match s {
                SimilarVideosStage::HidingHardLinks => 1,
                SimilarVideosStage::CalculatingHashes => 2,
                SimilarVideosStage::CreatingThumbnails => 3,
                _ => 0,
            },
            Self::SimilarVideos(SimilarVideosMode::AudioContent, s) => match s {
                SimilarVideosStage::HidingHardLinks => 1,
                SimilarVideosStage::LoadingAudioCache(_) => 2,
                SimilarVideosStage::CalculatingAudioFingerprints => 3,
                SimilarVideosStage::SavingAudioCache => 4,
                SimilarVideosStage::ComparingAudioFingerprints => 5,
                SimilarVideosStage::CreatingAudioThumbnails => 6,
                _ => 0,
            },

            Self::ExifRemover(s) => match s {
                ExifRemoverStage::LoadingCache(_) => 1,
                ExifRemoverStage::ExtractingTags => 2,
                ExifRemoverStage::SavingCache => 3,
            },

            Self::VideoOptimizer(s) => match s {
                VideoOptimizerStage::ProcessingVideos => 1,
                VideoOptimizerStage::CreatingThumbnails => 2,
            },

            Self::BrokenFilesChecking | Self::BadExtensionsChecking | Self::BadNamesChecking | Self::EmptyFilesCheckingContent => 1,
        }
    }

    pub fn max_stage_idx(self) -> u8 {
        match self {
            Self::CollectingFiles(_)
            | Self::CollectingFolders
            | Self::DeletingFiles
            | Self::RenamingFiles
            | Self::MovingFiles
            | Self::HardlinkingFiles
            | Self::SymlinkingFiles
            | Self::OptimizingVideos
            | Self::CleaningExif => 0,
            Self::Duplicate(_) | Self::SameMusic(SameMusicMode::AudioContent, _) => 7,
            Self::SameMusic(SameMusicMode::AudioTags, _) => 4,
            Self::SimilarImages(_) | Self::ExifRemover(_) | Self::SimilarVideos(SimilarVideosMode::VisualHash, _) => 3,
            Self::SimilarVideos(SimilarVideosMode::AudioContent, _) => 6,
            Self::VideoOptimizer(_) => 2,
            Self::BrokenFilesChecking | Self::BadExtensionsChecking | Self::BadNamesChecking | Self::EmptyFilesCheckingContent => 1,
        }
    }
}

/// Ready-to-render progress for a frontend. `current_progress`/`current_progress_size`
/// are `None` when there is nothing meaningful to show (indeterminate stage or no byte
/// data); `all_progress` is `-1` for single-step operations that have no overall bar.
/// All progress values are percentages in `0..=99`.
#[derive(Debug, Clone)]
pub struct ProgressDisplay {
    pub label: String,
    pub all_progress: i32,
    pub current_progress: Option<i32>,
    pub current_progress_size: Option<i32>,
}

#[derive(Debug, Clone, Copy)]
pub struct ProgressData {
    pub stage: ToolStage,
    pub entries_checked: usize,
    pub entries_to_check: usize,
    pub bytes_checked: u64,
    pub bytes_to_check: u64,
}

impl ProgressData {
    pub fn new(stage: ToolStage, entries_to_check: usize, bytes_to_check: u64) -> Self {
        Self {
            stage,
            entries_checked: 0,
            entries_to_check,
            bytes_checked: 0,
            bytes_to_check,
        }
    }

    pub fn item_progress(&self) -> i32 {
        let (checked, total) = if self.bytes_to_check > 0 {
            (self.bytes_checked, self.bytes_to_check)
        } else {
            (self.entries_checked as u64, self.entries_to_check as u64)
        };
        if total == 0 {
            return 0;
        }
        ((checked as f64 / total as f64).min(0.99) * 100.0) as i32
    }

    pub fn item_progress_size(&self) -> i32 {
        if self.bytes_to_check == 0 {
            return -1;
        }
        ((self.bytes_checked as f64 / self.bytes_to_check as f64).min(0.99) * 100.0) as i32
    }

    pub fn all_progress(&self) -> i32 {
        let current_idx = self.stage.current_stage_idx() as f64;
        let max_idx = self.stage.max_stage_idx() as f64;

        let (checked, total) = if self.bytes_to_check > 0 {
            (self.bytes_checked, self.bytes_to_check)
        } else {
            (self.entries_checked as u64, self.entries_to_check as u64)
        };

        let fraction = if total > 0 { (checked as f64 / total as f64).min(1.0) } else { 0.0 };
        let all = (current_idx + fraction) / (max_idx + 1.0);
        (all.min(0.99) * 100.0) as i32
    }

    /// Everything a frontend needs to render this progress update in one call:
    /// a fully translated label (counts included), the overall progress, and the
    /// current-stage progress. Frontends should not branch on the stage themselves.
    pub fn to_display(self) -> ProgressDisplay {
        let all_progress = if self.stage.is_special_non_tool_stage() { -1 } else { self.all_progress() };
        let (current_progress, current_progress_size) = if self.stage.is_indeterminate() {
            (None, None)
        } else {
            let size = (self.bytes_to_check > 0).then(|| self.item_progress_size());
            (Some(self.item_progress()), size)
        };
        ProgressDisplay {
            label: self.label(),
            all_progress,
            current_progress,
            current_progress_size,
        }
    }

    /// Fully translated, ready-to-display label including the live item/byte counts.
    /// One descriptive string per stage (e.g. "Analyzed partial hash of 50/100 files (1 MiB / 4 MiB)").
    pub fn label(&self) -> String {
        let entries_checked = self.entries_checked;
        let items_stats = format!("{}/{}", self.entries_checked, self.entries_to_check);
        let size_stats = format!("{} / {}", format_size(self.bytes_checked, BINARY), format_size(self.bytes_to_check, BINARY));
        let has_size = self.bytes_to_check > 0;

        match self.stage {
            // Collecting / scanning files (no known total yet)
            ToolStage::CollectingFolders => flc!("stage_collecting_folders", entries_checked = entries_checked),
            ToolStage::CollectingFiles(CheckingMethod::Name) => flc!("stage_scanning_name", entries_checked = entries_checked),
            ToolStage::CollectingFiles(CheckingMethod::SizeName) => flc!("stage_scanning_size_name", entries_checked = entries_checked),
            ToolStage::CollectingFiles(CheckingMethod::Size) => flc!("stage_scanning_size", entries_checked = entries_checked),
            ToolStage::CollectingFiles(_) => flc!("stage_collecting_files", entries_checked = entries_checked),

            // Cache load/save (granular per cache kind, indeterminate)
            ToolStage::Duplicate(DuplicateStage::LoadingPreHashCache(CacheLoadPhase::Loading)) => flc!("stage_loading_prehash_cache"),
            ToolStage::Duplicate(DuplicateStage::SavingPreHashCache) => flc!("stage_saving_prehash_cache"),
            ToolStage::Duplicate(DuplicateStage::LoadingHashCache(CacheLoadPhase::Loading)) => flc!("stage_loading_hash_cache"),
            ToolStage::Duplicate(DuplicateStage::SavingHashCache) => flc!("stage_saving_hash_cache"),
            ToolStage::SameMusic(_, SameMusicStage::LoadingTagsCache(CacheLoadPhase::Loading)) => flc!("stage_loading_tags_cache"),
            ToolStage::SameMusic(_, SameMusicStage::SavingTagsCache) => flc!("stage_saving_tags_cache"),
            ToolStage::SameMusic(_, SameMusicStage::LoadingFingerprintCache(CacheLoadPhase::Loading))
            | ToolStage::SimilarVideos(_, SimilarVideosStage::LoadingAudioCache(CacheLoadPhase::Loading)) => flc!("stage_loading_fingerprints_cache"),
            ToolStage::SameMusic(_, SameMusicStage::SavingFingerprintCache) | ToolStage::SimilarVideos(_, SimilarVideosStage::SavingAudioCache) => {
                flc!("stage_saving_fingerprints_cache")
            }
            ToolStage::ExifRemover(ExifRemoverStage::LoadingCache(CacheLoadPhase::Loading)) => flc!("stage_loading_exif_cache"),
            ToolStage::ExifRemover(ExifRemoverStage::SavingCache) => flc!("stage_saving_exif_cache"),

            // Filtering outdated cache entries (determinate sub-phase of any cache load)
            ToolStage::Duplicate(DuplicateStage::LoadingPreHashCache(CacheLoadPhase::FilteringOutdated) | DuplicateStage::LoadingHashCache(CacheLoadPhase::FilteringOutdated))
            | ToolStage::SameMusic(
                _,
                SameMusicStage::LoadingTagsCache(CacheLoadPhase::FilteringOutdated) | SameMusicStage::LoadingFingerprintCache(CacheLoadPhase::FilteringOutdated),
            )
            | ToolStage::SimilarVideos(_, SimilarVideosStage::LoadingAudioCache(CacheLoadPhase::FilteringOutdated))
            | ToolStage::ExifRemover(ExifRemoverStage::LoadingCache(CacheLoadPhase::FilteringOutdated)) => flc!("stage_filtering_outdated_cache"),

            // Per-tool work stages
            ToolStage::Duplicate(DuplicateStage::HidingHardLinks)
            | ToolStage::SimilarImages(SimilarImagesStage::HidingHardLinks)
            | ToolStage::SimilarVideos(_, SimilarVideosStage::HidingHardLinks) => flc!("stage_hiding_links", items_stats = items_stats),
            ToolStage::Duplicate(DuplicateStage::PreHashing) => flc!("stage_analyzed_partial_hash", items_stats = items_stats, size_stats = size_stats),
            ToolStage::Duplicate(DuplicateStage::FullHashing) => flc!("stage_analyzed_full_hash", items_stats = items_stats, size_stats = size_stats),
            ToolStage::SameMusic(_, SameMusicStage::ReadingTags) => flc!("stage_checked_tags", items_stats = items_stats),
            ToolStage::SameMusic(_, SameMusicStage::ComparingTags) => flc!("stage_compared_tags", items_stats = items_stats),
            ToolStage::SameMusic(_, SameMusicStage::CalculatingFingerprints) | ToolStage::SimilarVideos(_, SimilarVideosStage::CalculatingAudioFingerprints) => {
                flc!("stage_checked_content", items_stats = items_stats, size_stats = size_stats)
            }
            ToolStage::SameMusic(_, SameMusicStage::ComparingFingerprints) | ToolStage::SimilarVideos(_, SimilarVideosStage::ComparingAudioFingerprints) => {
                flc!("stage_compared_content", items_stats = items_stats)
            }
            ToolStage::SimilarImages(SimilarImagesStage::CalculatingHashes) => flc!("stage_hashed_images", items_stats = items_stats, size_stats = size_stats),
            ToolStage::SimilarImages(SimilarImagesStage::ComparingHashes) => flc!("stage_compared_image_hashes", items_stats = items_stats),
            ToolStage::SimilarVideos(_, SimilarVideosStage::CalculatingHashes) => flc!("stage_hashed_videos", items_stats = items_stats),
            ToolStage::SimilarVideos(_, SimilarVideosStage::CreatingThumbnails | SimilarVideosStage::CreatingAudioThumbnails)
            | ToolStage::VideoOptimizer(VideoOptimizerStage::CreatingThumbnails) => flc!("stage_created_thumbnails", items_stats = items_stats),
            ToolStage::VideoOptimizer(VideoOptimizerStage::ProcessingVideos) => flc!("stage_checked_videos", items_stats = items_stats, size_stats = size_stats),
            ToolStage::ExifRemover(ExifRemoverStage::ExtractingTags) => flc!("stage_extracted_exif_tags", items_stats = items_stats, size_stats = size_stats),
            ToolStage::BrokenFilesChecking => flc!("stage_checked_files", items_stats = items_stats, size_stats = size_stats),
            ToolStage::BadExtensionsChecking => flc!("stage_checked_files_bad_extensions", items_stats = items_stats),
            ToolStage::BadNamesChecking => flc!("stage_checked_files_bad_names", items_stats = items_stats),
            ToolStage::EmptyFilesCheckingContent => flc!("stage_checking_empty_files_content", items_stats = items_stats, size_stats = size_stats),

            // File operations
            ToolStage::DeletingFiles if has_size => flc!("stage_deleting_files", items_stats = items_stats, size_stats = size_stats),
            ToolStage::DeletingFiles => flc!("stage_deleting_no_size_files", items_stats = items_stats),
            ToolStage::RenamingFiles => flc!("stage_renaming_files", items_stats = items_stats),
            ToolStage::MovingFiles if has_size => flc!("stage_moving_files", items_stats = items_stats, size_stats = size_stats),
            ToolStage::MovingFiles => flc!("stage_moving_no_size_files", items_stats = items_stats),
            ToolStage::HardlinkingFiles if has_size => flc!("stage_hardlinking_files", items_stats = items_stats, size_stats = size_stats),
            ToolStage::HardlinkingFiles => flc!("stage_hardlinking_no_size_files", items_stats = items_stats),
            ToolStage::SymlinkingFiles if has_size => flc!("stage_symlinking_files", items_stats = items_stats, size_stats = size_stats),
            ToolStage::SymlinkingFiles => flc!("stage_symlinking_no_size_files", items_stats = items_stats),
            ToolStage::OptimizingVideos if has_size => flc!("stage_optimizing_videos", items_stats = items_stats, size_stats = size_stats),
            ToolStage::OptimizingVideos => flc!("stage_optimizing_no_size_videos", items_stats = items_stats),
            ToolStage::CleaningExif if has_size => flc!("stage_cleaning_exif", items_stats = items_stats, size_stats = size_stats),
            ToolStage::CleaningExif => flc!("stage_cleaning_no_size_exif", items_stats = items_stats),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_indices() {
        assert_eq!(ToolStage::Duplicate(DuplicateStage::FullHashing).current_stage_idx(), 6);
        assert_eq!(ToolStage::Duplicate(DuplicateStage::FullHashing).max_stage_idx(), 7);
        assert_eq!(
            ToolStage::SameMusic(SameMusicMode::AudioContent, SameMusicStage::ComparingFingerprints).current_stage_idx(),
            7
        );
        assert_eq!(ToolStage::SameMusic(SameMusicMode::AudioTags, SameMusicStage::ComparingTags).current_stage_idx(), 4);
        assert_eq!(ToolStage::SameMusic(SameMusicMode::AudioTags, SameMusicStage::ComparingTags).max_stage_idx(), 4);
        assert_eq!(ToolStage::SimilarImages(SimilarImagesStage::ComparingHashes).current_stage_idx(), 3);
    }

    #[test]
    fn test_is_indeterminate() {
        assert!(ToolStage::CollectingFiles(CheckingMethod::None).is_indeterminate());
        assert!(ToolStage::CollectingFolders.is_indeterminate());
        assert!(ToolStage::Duplicate(DuplicateStage::LoadingPreHashCache(CacheLoadPhase::Loading)).is_indeterminate());
        assert!(!ToolStage::Duplicate(DuplicateStage::LoadingPreHashCache(CacheLoadPhase::FilteringOutdated)).is_indeterminate());
        assert!(!ToolStage::Duplicate(DuplicateStage::PreHashing).is_indeterminate());
    }

    #[test]
    fn test_is_special_non_tool_stage() {
        assert!(ToolStage::DeletingFiles.is_special_non_tool_stage());
        assert!(!ToolStage::CollectingFiles(CheckingMethod::None).is_special_non_tool_stage());
        assert!(!ToolStage::Duplicate(DuplicateStage::FullHashing).is_special_non_tool_stage());
    }

    #[test]
    fn test_cache_detection() {
        assert!(ToolStage::Duplicate(DuplicateStage::LoadingHashCache(CacheLoadPhase::Loading)).is_cache_loading());
        assert!(ToolStage::Duplicate(DuplicateStage::SavingHashCache).is_cache_saving());
        assert!(!ToolStage::Duplicate(DuplicateStage::FullHashing).is_cache_loading_saving());
    }

    #[test]
    fn test_to_display_determinate_stage() {
        let pd = ProgressData {
            stage: ToolStage::Duplicate(DuplicateStage::FullHashing),
            entries_checked: 50,
            entries_to_check: 100,
            bytes_checked: 0,
            bytes_to_check: 0,
        };
        let display = pd.to_display();
        assert!(display.label.contains("50/100"), "label should embed live counts: {}", display.label);
        assert!(!display.label.contains('['), "label should not contain a stage-counter prefix: {}", display.label);
        assert_eq!(display.current_progress, Some(50));
        assert_eq!(display.current_progress_size, None);
        assert!((0..=99).contains(&display.all_progress));
    }

    // Fluent wraps interpolated values in bidi-isolation chars (U+2068/U+2069); strip them for plain comparisons.
    fn strip_isolates(s: &str) -> String {
        s.chars().filter(|c| !matches!(c, '\u{2066}'..='\u{2069}')).collect()
    }

    #[test]
    fn test_label_descriptive_wording() {
        let mut pd = ProgressData::new(ToolStage::Duplicate(DuplicateStage::PreHashing), 100, 4096);
        pd.entries_checked = 50;
        assert!(strip_isolates(&pd.label()).starts_with("Analyzed partial hash of 50/100 files"), "{}", pd.label());

        let pd = ProgressData::new(ToolStage::SameMusic(SameMusicMode::AudioTags, SameMusicStage::ReadingTags), 100, 0);
        assert!(strip_isolates(&pd.label()).starts_with("Checked tags of"), "{}", pd.label());
    }

    #[test]
    fn test_label_granular_cache_and_collecting() {
        assert_eq!(
            ProgressData::new(ToolStage::Duplicate(DuplicateStage::LoadingPreHashCache(CacheLoadPhase::Loading)), 0, 0).label(),
            "Loading prehash cache"
        );
        assert_eq!(
            ProgressData::new(ToolStage::Duplicate(DuplicateStage::LoadingHashCache(CacheLoadPhase::Loading)), 0, 0).label(),
            "Loading hash cache"
        );
        assert_eq!(
            ProgressData::new(ToolStage::SameMusic(SameMusicMode::AudioContent, SameMusicStage::SavingFingerprintCache), 0, 0).label(),
            "Saving fingerprints cache"
        );

        let mut pd = ProgressData::new(ToolStage::CollectingFiles(CheckingMethod::Size), 0, 0);
        pd.entries_checked = 7;
        assert_eq!(strip_isolates(&pd.label()), "Scanning size of 7 file");
    }

    #[test]
    fn test_to_display_indeterminate_hides_current() {
        let pd = ProgressData::new(ToolStage::CollectingFiles(CheckingMethod::None), 0, 0);
        let display = pd.to_display();
        assert_eq!(display.current_progress, None);
        assert_eq!(display.current_progress_size, None);
        assert!(display.label.starts_with("Scanning"), "{}", display.label);
    }

    #[test]
    fn test_to_display_special_stage_has_no_overall_bar() {
        let mut pd = ProgressData::new(ToolStage::DeletingFiles, 10, 0);
        pd.entries_checked = 3;
        let display = pd.to_display();
        assert_eq!(display.all_progress, -1);
        assert_eq!(display.current_progress, Some(30));
        assert!(!display.label.contains('['), "single-step ops have no stage prefix: {}", display.label);
    }

    #[test]
    fn test_label_includes_byte_sizes() {
        let pd = ProgressData {
            stage: ToolStage::Duplicate(DuplicateStage::PreHashing),
            entries_checked: 1,
            entries_to_check: 2,
            bytes_checked: 1024,
            bytes_to_check: 4096,
        };
        let display = pd.to_display();
        assert!(display.label.contains("1/2"), "{}", display.label);
        assert!(display.label.contains("KiB"), "byte stages show sizes: {}", display.label);
        assert_eq!(display.current_progress_size, Some(25));
    }

    #[test]
    fn test_all_progress() {
        let pd = ProgressData {
            stage: ToolStage::Duplicate(DuplicateStage::FullHashing),
            entries_checked: 50,
            entries_to_check: 100,
            bytes_checked: 0,
            bytes_to_check: 0,
        };
        let progress = pd.all_progress();
        assert!((0..=99).contains(&progress));
    }
}
