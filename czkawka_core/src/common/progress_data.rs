use log::error;

use crate::common::model::{CheckingMethod, ToolType};
// Empty files
// 0 - Collecting files

// Empty folders
// 0 - Collecting folders

// Big files
// 0 - Collecting files

// Same music
// 0 - Collecting files
// 1 - Loading cache
// 2 - Checking tags
// 3 - Saving cache
// 4 - TAGS - Comparing tags
// 4 - CONTENT - Loading cache
// 5 - CONTENT - Calculating fingerprints
// 6 - CONTENT - Saving cache
// 7 - CONTENT - Comparing fingerprints

// Similar images
// 0 - Collecting files
// 1 - Scanning images
// 2 - Comparing hashes

// Similar videos
// 0 - Collecting files
// 1 - Scanning videos
// 2 - Creating thumbnails

// Temporary files
// 0 - Collecting files

// Invalid symlinks
// 0 - Collecting files

// Broken files
// 0 - Collecting files
// 1 - Scanning files

// Bad extensions
// 0 - Collecting files
// 1 - Scanning files

// Exif Remover
// 0 - Collecting files
// 1 - Loading cache
// 2 - Extracting tags
// 3 - Saving cache

// Duplicates - Hash
// 0 - Collecting files
// 1 - Loading cache
// 2 - Hash - first 1KB file
// 3 - Saving cache
// 4 - Loading cache
// 5 - Hash - normal hash
// 6 - Saving cache

// Duplicates - Name or SizeName or Size
// 0 - Collecting files

// Deleting files
// Renaming files

#[derive(Debug, Clone, Copy)]
pub struct ProgressData {
    pub sstage: CurrentStage,
    pub checking_method: CheckingMethod,
    pub current_stage_idx: u8,
    pub max_stage_idx: u8,
    pub entries_checked: usize,
    pub entries_to_check: usize,
    pub bytes_checked: u64,
    pub bytes_to_check: u64,
    pub tool_type: ToolType,
}

impl ProgressData {
    pub fn get_empty_state(current_stage: CurrentStage) -> Self {
        Self {
            sstage: current_stage,
            checking_method: CheckingMethod::None,
            current_stage_idx: 0,
            max_stage_idx: 0,
            entries_checked: 0,
            entries_to_check: 0,
            bytes_checked: 0,
            bytes_to_check: 0,
            tool_type: ToolType::None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CurrentStage {
    DeletingFiles,
    RenamingFiles,
    MovingFiles,
    HardlinkingFiles,
    SymlinkingFiles,
    OptimizingVideos,
    CleaningExif,

    CollectingFiles,
    DuplicateCacheSaving,
    DuplicateCacheLoading,
    DuplicatePreHashCacheSaving,
    DuplicatePreHashCacheLoading,
    DuplicateScanningName,
    DuplicateScanningSizeName,
    DuplicateScanningSize,
    DuplicatePreHashing,
    DuplicateFullHashing,

    SameMusicCacheSavingTags,
    SameMusicCacheLoadingTags,
    SameMusicCacheSavingFingerprints,
    SameMusicCacheLoadingFingerprints,
    SameMusicReadingTags,
    SameMusicCalculatingFingerprints,
    SameMusicComparingTags,
    SameMusicComparingFingerprints,

    SimilarImagesCalculatingHashes,
    SimilarImagesComparingHashes,
    SimilarVideosCalculatingHashes,
    SimilarVideosCreatingThumbnails,
    BrokenFilesChecking,
    BadExtensionsChecking,
    ExifRemoverCacheLoading,
    ExifRemoverExtractingTags,
    ExifRemoverCacheSaving,
    VideoOptimizerCreatingThumbnails,
    VideoOptimizerProcessingVideos,
}

impl ProgressData {
    pub(crate) fn validate(&self) {
        assert!(
            self.current_stage_idx <= self.max_stage_idx,
            "Current stage index: {}, max stage index: {}, stage {:?}",
            self.current_stage_idx,
            self.max_stage_idx,
            self.sstage
        );
        assert_eq!(
            self.max_stage_idx,
            self.tool_type.get_max_stage(self.checking_method),
            "Max stage index: {}, tool type: {:?}, checking method: {:?}",
            self.max_stage_idx,
            self.tool_type,
            self.checking_method
        );

        if self.sstage != CurrentStage::CollectingFiles {
            assert!(
                self.entries_checked <= self.entries_to_check,
                "Entries checked: {}, entries to check: {}, stage {:?}",
                self.entries_checked,
                self.entries_to_check,
                self.sstage
            );
        }

        // This could be an assert, but it is possible that in duplicate finder, file that will
        // be checked, will increase the size of the file between collecting file to scan and
        // scanning it. So it is better to just log it
        if self.bytes_checked > self.bytes_to_check {
            error!("Bytes checked: {}, bytes to check: {}, stage {:?}", self.bytes_checked, self.bytes_to_check, self.sstage);
        }

        let tool_type_checking_method: Option<ToolType> = match self.checking_method {
            CheckingMethod::AudioTags | CheckingMethod::AudioContent => Some(ToolType::SameMusic),
            CheckingMethod::Name | CheckingMethod::SizeName | CheckingMethod::Size | CheckingMethod::Hash => Some(ToolType::Duplicate),
            CheckingMethod::None => None,
        };
        if let Some(tool_type) = tool_type_checking_method {
            assert_eq!(self.tool_type, tool_type, "Tool type: {:?}, checking method: {:?}", self.tool_type, self.checking_method);
        }
        let tool_type_current_stage: Option<ToolType> = match self.sstage {
            CurrentStage::CollectingFiles
            | CurrentStage::DeletingFiles
            | CurrentStage::RenamingFiles
            | CurrentStage::MovingFiles
            | CurrentStage::HardlinkingFiles
            | CurrentStage::SymlinkingFiles
            | CurrentStage::OptimizingVideos
            | CurrentStage::CleaningExif => None,
            CurrentStage::DuplicateCacheSaving | CurrentStage::DuplicateCacheLoading | CurrentStage::DuplicatePreHashCacheSaving | CurrentStage::DuplicatePreHashCacheLoading => {
                Some(ToolType::Duplicate)
            }
            CurrentStage::DuplicateScanningName
            | CurrentStage::DuplicateScanningSizeName
            | CurrentStage::DuplicateScanningSize
            | CurrentStage::DuplicatePreHashing
            | CurrentStage::DuplicateFullHashing => Some(ToolType::Duplicate),
            CurrentStage::SameMusicCacheLoadingTags
            | CurrentStage::SameMusicCacheSavingTags
            | CurrentStage::SameMusicCacheLoadingFingerprints
            | CurrentStage::SameMusicCacheSavingFingerprints
            | CurrentStage::SameMusicComparingTags
            | CurrentStage::SameMusicReadingTags
            | CurrentStage::SameMusicComparingFingerprints
            | CurrentStage::SameMusicCalculatingFingerprints => Some(ToolType::SameMusic),
            CurrentStage::SimilarImagesCalculatingHashes | CurrentStage::SimilarImagesComparingHashes => Some(ToolType::SimilarImages),
            CurrentStage::SimilarVideosCalculatingHashes | CurrentStage::SimilarVideosCreatingThumbnails => Some(ToolType::SimilarVideos),
            CurrentStage::BrokenFilesChecking => Some(ToolType::BrokenFiles),
            CurrentStage::BadExtensionsChecking => Some(ToolType::BadExtensions),
            CurrentStage::ExifRemoverCacheLoading | CurrentStage::ExifRemoverExtractingTags | CurrentStage::ExifRemoverCacheSaving => Some(ToolType::ExifRemover),
            CurrentStage::VideoOptimizerCreatingThumbnails | CurrentStage::VideoOptimizerProcessingVideos => Some(ToolType::VideoOptimizer),
        };
        if let Some(tool_type) = tool_type_current_stage {
            assert_eq!(self.tool_type, tool_type, "Tool type: {:?}, stage {:?}", self.tool_type, self.sstage);
        }
    }
}

impl ToolType {
    pub(crate) fn get_max_stage(self, checking_method: CheckingMethod) -> u8 {
        match self {
            Self::Duplicate => 6,
            Self::EmptyFolders | Self::EmptyFiles | Self::InvalidSymlinks | Self::BigFile | Self::TemporaryFiles => 0,
            Self::BrokenFiles | Self::BadExtensions => 1,
            Self::SimilarImages | Self::SimilarVideos | Self::VideoOptimizer => 2,
            Self::ExifRemover => 3,
            Self::None => unreachable!("ToolType::None is not allowed"),
            Self::SameMusic => match checking_method {
                CheckingMethod::AudioTags => 4,
                CheckingMethod::AudioContent => 7,
                _ => unreachable!("CheckingMethod {checking_method:?} in same music mode is not allowed"),
            },
        }
    }
}

impl CurrentStage {
    pub fn is_special_non_tool_stage(self) -> bool {
        matches!(
            self,
            Self::DeletingFiles | Self::RenamingFiles | Self::MovingFiles | Self::HardlinkingFiles | Self::SymlinkingFiles | Self::OptimizingVideos | Self::CleaningExif
        )
    }

    pub fn get_current_stage(self) -> u8 {
        #[expect(clippy::match_same_arms)] // Now it is easier to read
        match self {
            Self::DeletingFiles => 0,
            Self::RenamingFiles => 0,
            Self::MovingFiles => 0,
            Self::HardlinkingFiles => 0,
            Self::SymlinkingFiles => 0,
            Self::OptimizingVideos => 0,
            Self::CleaningExif => 0,
            Self::CollectingFiles => 0,
            Self::DuplicateScanningName => 0,
            Self::DuplicateScanningSizeName => 0,
            Self::DuplicateScanningSize => 0,
            Self::DuplicatePreHashCacheLoading => 1,
            Self::DuplicatePreHashing => 2,
            Self::DuplicatePreHashCacheSaving => 3,
            Self::DuplicateCacheLoading => 4,
            Self::DuplicateFullHashing => 5,
            Self::DuplicateCacheSaving => 6,
            Self::SimilarImagesCalculatingHashes => 1,
            Self::SimilarImagesComparingHashes => 2,
            Self::SimilarVideosCalculatingHashes => 1,
            Self::SimilarVideosCreatingThumbnails => 2,
            Self::BrokenFilesChecking => 1,
            Self::BadExtensionsChecking => 1,
            Self::VideoOptimizerCreatingThumbnails => 2,
            Self::VideoOptimizerProcessingVideos => 1,
            Self::SameMusicCacheLoadingTags => 1,
            Self::SameMusicReadingTags => 2,
            Self::SameMusicCacheSavingTags => 3,
            Self::SameMusicComparingTags => 4,
            Self::SameMusicCacheLoadingFingerprints => 4,
            Self::SameMusicCalculatingFingerprints => 5,
            Self::SameMusicCacheSavingFingerprints => 6,
            Self::SameMusicComparingFingerprints => 7,
            Self::ExifRemoverCacheLoading => 1,
            Self::ExifRemoverExtractingTags => 2,
            Self::ExifRemoverCacheSaving => 3,
        }
    }
    pub fn check_if_loading_saving_cache(self) -> bool {
        self.check_if_saving_cache() || self.check_if_loading_cache()
    }
    pub fn check_if_loading_cache(self) -> bool {
        matches!(
            self,
            Self::SameMusicCacheLoadingFingerprints
                | Self::SameMusicCacheLoadingTags
                | Self::DuplicateCacheLoading
                | Self::DuplicatePreHashCacheLoading
                | Self::ExifRemoverCacheLoading
        )
    }
    pub fn check_if_saving_cache(self) -> bool {
        matches!(
            self,
            Self::SameMusicCacheSavingFingerprints | Self::SameMusicCacheSavingTags | Self::DuplicateCacheSaving | Self::DuplicatePreHashCacheSaving | Self::ExifRemoverCacheSaving
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_type_and_current_stage_integration() {
        assert_eq!(ToolType::Duplicate.get_max_stage(CheckingMethod::Hash), 6);
        assert_eq!(ToolType::SameMusic.get_max_stage(CheckingMethod::AudioTags), 4);
        assert_eq!(ToolType::SameMusic.get_max_stage(CheckingMethod::AudioContent), 7);
        assert_eq!(ToolType::SimilarImages.get_max_stage(CheckingMethod::None), 2);
        assert_eq!(ToolType::BrokenFiles.get_max_stage(CheckingMethod::None), 1);

        assert_eq!(CurrentStage::DuplicateFullHashing.get_current_stage(), 5);
        assert_eq!(CurrentStage::SameMusicComparingFingerprints.get_current_stage(), 7);
        assert!(CurrentStage::DeletingFiles.is_special_non_tool_stage());
        assert!(!CurrentStage::CollectingFiles.is_special_non_tool_stage());
    }

    #[test]
    fn test_cache_operations_detection() {
        assert!(CurrentStage::DuplicateCacheLoading.check_if_loading_cache());
        assert!(CurrentStage::DuplicateCacheSaving.check_if_saving_cache());
        assert!(CurrentStage::SameMusicCacheLoadingTags.check_if_loading_saving_cache());
        assert!(!CurrentStage::DuplicateFullHashing.check_if_loading_saving_cache());
    }

    #[test]
    fn test_progress_data_validation_and_empty_state() {
        let empty = ProgressData::get_empty_state(CurrentStage::CollectingFiles);
        assert_eq!(empty.entries_checked, 0);
        assert_eq!(empty.tool_type, ToolType::None);

        let valid = ProgressData {
            sstage: CurrentStage::DuplicateFullHashing,
            checking_method: CheckingMethod::Hash,
            current_stage_idx: 5,
            max_stage_idx: 6,
            entries_checked: 50,
            entries_to_check: 100,
            bytes_checked: 1000,
            bytes_to_check: 2000,
            tool_type: ToolType::Duplicate,
        };
        valid.validate();
    }

    #[test]
    #[should_panic(expected = "Current stage index")]
    fn test_validation_invalid_stage_idx() {
        ProgressData {
            sstage: CurrentStage::DuplicateFullHashing,
            checking_method: CheckingMethod::Hash,
            current_stage_idx: 7,
            max_stage_idx: 6,
            entries_checked: 0,
            entries_to_check: 100,
            bytes_checked: 0,
            bytes_to_check: 1000,
            tool_type: ToolType::Duplicate,
        }
        .validate();
    }

    #[test]
    #[should_panic(expected = "Entries checked")]
    fn test_validation_too_many_entries() {
        ProgressData {
            sstage: CurrentStage::DuplicateFullHashing,
            checking_method: CheckingMethod::Hash,
            current_stage_idx: 5,
            max_stage_idx: 6,
            entries_checked: 150,
            entries_to_check: 100,
            bytes_checked: 0,
            bytes_to_check: 1000,
            tool_type: ToolType::Duplicate,
        }
        .validate();
    }
}
