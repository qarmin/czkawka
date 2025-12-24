#![allow(dead_code)]

use std::path::PathBuf;

use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::{ActiveTab, ExcludedDirectoriesModel, IncludedDirectoriesModel, MainListModel, MainWindow, Settings};

// Int model is used to store data in unchanged(* except that we need to split u64 into two i32) form and is used to sort/select data
// Str model is used to display data in gui

// Duplicates
#[repr(u8)]
pub enum IntDataDuplicateFiles {
    ModificationDatePart1,
    ModificationDatePart2,
    SizePart1,
    SizePart2,
}
#[repr(u8)]
pub enum StrDataDuplicateFiles {
    Size,
    Name,
    Path,
    ModificationDate,
}

// Empty Folders
#[repr(u8)]
pub enum IntDataEmptyFolders {
    ModificationDatePart1,
    ModificationDatePart2,
}

#[repr(u8)]
pub enum StrDataEmptyFolders {
    Name,
    Path,
    ModificationDate,
}
// Big Files
#[repr(u8)]
pub enum IntDataBigFiles {
    ModificationDatePart1,
    ModificationDatePart2,
    SizePart1,
    SizePart2,
}

#[repr(u8)]
pub enum StrDataBigFiles {
    Size,
    Name,
    Path,
    ModificationDate,
}

// Empty files
#[repr(u8)]
pub enum IntDataEmptyFiles {
    ModificationDatePart1,
    ModificationDatePart2,
    SizePart1,
    SizePart2,
}

#[repr(u8)]
pub enum StrDataEmptyFiles {
    Name,
    Path,
    ModificationDate,
}
// Temporary Files
#[repr(u8)]
pub enum IntDataTemporaryFiles {
    ModificationDatePart1,
    ModificationDatePart2,
    SizePart1,
    SizePart2,
}
#[repr(u8)]
pub enum StrDataTemporaryFiles {
    Name,
    Path,
    ModificationDate,
}

// Similar Images
#[repr(u8)]
pub enum IntDataSimilarImages {
    ModificationDatePart1,
    ModificationDatePart2,
    SizePart1,
    SizePart2,
    Width,
    Height,
}

#[repr(u8)]
pub enum StrDataSimilarImages {
    Similarity,
    Size,
    Resolution,
    Name,
    Path,
    ModificationDate,
}

// Similar Videos
#[repr(u8)]
pub enum IntDataSimilarVideos {
    ModificationDatePart1,
    ModificationDatePart2,
    SizePart1,
    SizePart2,
}

#[repr(u8)]
pub enum StrDataSimilarVideos {
    Size,
    Name,
    Path,
    Dimensions,
    Duration,
    Bitrate,
    Fps,
    Codec,
    ModificationDate,
    PreviewPath,
}

// Similar Music
#[repr(u8)]
pub enum IntDataSimilarMusic {
    ModificationDatePart1,
    ModificationDatePart2,
    SizePart1,
    SizePart2,
}

#[repr(u8)]
pub enum StrDataSimilarMusic {
    Size,
    Name,
    Title,
    Artist,
    Year,
    Bitrate,
    Length,
    Genre,
    Path,
    ModificationDate,
}

// Invalid Symlinks
#[repr(u8)]
pub enum IntDataInvalidSymlinks {
    ModificationDatePart1,
    ModificationDatePart2,
}

#[repr(u8)]
pub enum StrDataInvalidSymlinks {
    SymlinkName,
    SymlinkFolder,
    DestinationPath,
    TypeOfError,
    ModificationDate,
}

// Broken Files
#[repr(u8)]
pub enum IntDataBrokenFiles {
    ModificationDatePart1,
    ModificationDatePart2,
    SizePart1,
    SizePart2,
}

#[repr(u8)]
pub enum StrDataBrokenFiles {
    Name,
    Path,
    TypeOfError,
    Size,
    ModificationDate,
}
// Bad Extensions
#[repr(u8)]
pub enum IntDataBadExtensions {
    ModificationDatePart1,
    ModificationDatePart2,
    SizePart1,
    SizePart2,
}

#[repr(u8)]
pub enum StrDataBadExtensions {
    Name,
    Path,
    CurrentExtension,
    ProperExtensionsGroup,
    ProperExtension,
}

impl ActiveTab {
    // Remember to match updated this according to ui/main_lists.slint and connect_scan.rs files
    pub(crate) fn get_str_path_idx(self) -> usize {
        match self {
            Self::EmptyFolders => StrDataEmptyFolders::Path as usize,
            Self::EmptyFiles => StrDataEmptyFiles::Path as usize,
            Self::SimilarImages => StrDataSimilarImages::Path as usize,
            Self::DuplicateFiles => StrDataDuplicateFiles::Path as usize,
            Self::BigFiles => StrDataBigFiles::Path as usize,
            Self::TemporaryFiles => StrDataTemporaryFiles::Path as usize,
            Self::SimilarVideos => StrDataSimilarVideos::Path as usize,
            Self::SimilarMusic => StrDataSimilarMusic::Path as usize,
            Self::InvalidSymlinks => StrDataInvalidSymlinks::SymlinkFolder as usize,
            Self::BrokenFiles => StrDataBrokenFiles::Path as usize,
            Self::BadExtensions => StrDataBadExtensions::Path as usize,
            Self::Settings | Self::About => panic!("Button should be disabled"),
        }
    }

    pub(crate) fn get_str_name_idx(self) -> usize {
        match self {
            Self::EmptyFolders => StrDataEmptyFolders::Name as usize,
            Self::EmptyFiles => StrDataEmptyFiles::Name as usize,
            Self::SimilarImages => StrDataSimilarImages::Name as usize,
            Self::DuplicateFiles => StrDataDuplicateFiles::Name as usize,
            Self::BigFiles => StrDataBigFiles::Name as usize,
            Self::TemporaryFiles => StrDataTemporaryFiles::Name as usize,
            Self::SimilarVideos => StrDataSimilarVideos::Name as usize,
            Self::SimilarMusic => StrDataSimilarMusic::Name as usize,
            Self::InvalidSymlinks => StrDataInvalidSymlinks::SymlinkName as usize,
            Self::BrokenFiles => StrDataBrokenFiles::Name as usize,
            Self::BadExtensions => StrDataBadExtensions::Name as usize,
            Self::Settings | Self::About => panic!("Button should be disabled"),
        }
    }

    pub(crate) fn get_str_proper_extension(self) -> usize {
        match self {
            Self::BadExtensions => StrDataBadExtensions::ProperExtension as usize,
            Self::Settings | Self::About => panic!("Button should be disabled"),
            _ => panic!("Unable to get proper extension from this tab"),
        }
    }
    pub(crate) fn get_int_modification_date_idx(self) -> usize {
        match self {
            Self::EmptyFiles => IntDataEmptyFiles::ModificationDatePart1 as usize,
            Self::EmptyFolders => IntDataEmptyFolders::ModificationDatePart1 as usize,
            Self::SimilarImages => IntDataSimilarImages::ModificationDatePart1 as usize,
            Self::DuplicateFiles => IntDataDuplicateFiles::ModificationDatePart1 as usize,
            Self::BigFiles => IntDataBigFiles::ModificationDatePart1 as usize,
            Self::TemporaryFiles => IntDataTemporaryFiles::ModificationDatePart1 as usize,
            Self::SimilarVideos => IntDataSimilarVideos::ModificationDatePart1 as usize,
            Self::SimilarMusic => IntDataSimilarMusic::ModificationDatePart1 as usize,
            Self::InvalidSymlinks => IntDataInvalidSymlinks::ModificationDatePart1 as usize,
            Self::BrokenFiles => IntDataBrokenFiles::ModificationDatePart1 as usize,
            Self::BadExtensions => IntDataBadExtensions::ModificationDatePart1 as usize,
            Self::Settings | Self::About => panic!("Button should be disabled"),
        }
    }
    pub(crate) fn get_int_size_opt_idx(self) -> Option<usize> {
        let res = match self {
            Self::EmptyFiles => IntDataEmptyFiles::SizePart1 as usize,
            Self::SimilarImages => IntDataSimilarImages::SizePart1 as usize,
            Self::DuplicateFiles => IntDataDuplicateFiles::SizePart1 as usize,
            Self::BigFiles => IntDataBigFiles::SizePart1 as usize,
            Self::SimilarVideos => IntDataSimilarVideos::SizePart1 as usize,
            Self::SimilarMusic => IntDataSimilarMusic::SizePart1 as usize,
            Self::BrokenFiles => IntDataBrokenFiles::SizePart1 as usize,
            Self::BadExtensions => IntDataBadExtensions::SizePart1 as usize,
            Self::TemporaryFiles => IntDataTemporaryFiles::SizePart1 as usize,
            Self::Settings | Self::About | Self::EmptyFolders | Self::InvalidSymlinks => return None,
        };
        Some(res)
    }
    pub(crate) fn get_int_size_idx(self) -> usize {
        self.get_int_size_opt_idx().unwrap_or_else(|| panic!("Unable to get size index for tab: {self:?}"))
    }
    pub(crate) fn get_int_width_idx(self) -> usize {
        match self {
            Self::SimilarImages => IntDataSimilarImages::Width as usize,
            Self::Settings | Self::About => panic!("Button should be disabled"),
            _ => panic!("Unable to get height from this tab"),
        }
    }

    pub(crate) fn get_int_height_idx(self) -> usize {
        match self {
            Self::SimilarImages => IntDataSimilarImages::Height as usize,
            Self::Settings | Self::About => panic!("Button should be disabled"),
            _ => panic!("Unable to get height from this tab"),
        }
    }

    pub(crate) fn get_is_header_mode(self) -> bool {
        match self {
            Self::EmptyFolders | Self::EmptyFiles | Self::BrokenFiles | Self::BigFiles | Self::TemporaryFiles | Self::InvalidSymlinks | Self::BadExtensions => false,
            Self::SimilarImages | Self::DuplicateFiles | Self::SimilarVideos | Self::SimilarMusic => true,
            Self::Settings | Self::About => panic!("Button should be disabled"),
        }
    }
    pub(crate) fn get_tool_model(self, app: &MainWindow) -> ModelRc<MainListModel> {
        match self {
            Self::EmptyFolders => app.get_empty_folder_model(),
            Self::SimilarImages => app.get_similar_images_model(),
            Self::EmptyFiles => app.get_empty_files_model(),
            Self::DuplicateFiles => app.get_duplicate_files_model(),
            Self::BigFiles => app.get_big_files_model(),
            Self::TemporaryFiles => app.get_temporary_files_model(),
            Self::SimilarVideos => app.get_similar_videos_model(),
            Self::SimilarMusic => app.get_similar_music_model(),
            Self::InvalidSymlinks => app.get_invalid_symlinks_model(),
            Self::BrokenFiles => app.get_broken_files_model(),
            Self::BadExtensions => app.get_bad_extensions_model(),
            Self::Settings | Self::About => panic!("Button should be disabled"),
        }
    }

    pub(crate) fn set_tool_model(self, app: &MainWindow, model: ModelRc<MainListModel>) {
        match self {
            Self::EmptyFolders => app.set_empty_folder_model(model),
            Self::SimilarImages => app.set_similar_images_model(model),
            Self::EmptyFiles => app.set_empty_files_model(model),
            Self::DuplicateFiles => app.set_duplicate_files_model(model),
            Self::BigFiles => app.set_big_files_model(model),
            Self::TemporaryFiles => app.set_temporary_files_model(model),
            Self::SimilarVideos => app.set_similar_videos_model(model),
            Self::SimilarMusic => app.set_similar_music_model(model),
            Self::InvalidSymlinks => app.set_invalid_symlinks_model(model),
            Self::BrokenFiles => app.set_broken_files_model(model),
            Self::BadExtensions => app.set_bad_extensions_model(model),
            Self::Settings | Self::About => panic!("Button should be disabled"),
        }
    }
}

pub(crate) fn create_included_directories_model_from_pathbuf(items: &[PathBuf], referenced: &[PathBuf]) -> ModelRc<IncludedDirectoriesModel> {
    let referenced_as_string = referenced.iter().map(|x| x.to_string_lossy().to_string()).collect::<Vec<_>>();
    let converted = items
        .iter()
        .map(|x| {
            let path_as_string = x.to_string_lossy().to_string();
            IncludedDirectoriesModel {
                path: x.to_string_lossy().to_string().into(),
                referenced_folder: referenced_as_string.contains(&path_as_string),
                selected_row: false,
            }
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(converted))
}

pub(crate) fn create_excluded_directories_model_from_pathbuf(items: &[PathBuf]) -> ModelRc<ExcludedDirectoriesModel> {
    let converted = items
        .iter()
        .map(|x| ExcludedDirectoriesModel {
            path: x.to_string_lossy().to_string().into(),
            selected_row: false,
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(converted))
}

pub(crate) fn check_if_there_are_any_included_folders(app: &MainWindow) -> bool {
    let included = app.global::<Settings>().get_included_directories_model();
    included.iter().count() > 0
}

pub(crate) fn check_if_all_included_dirs_are_referenced(app: &MainWindow) -> bool {
    let included = app.global::<Settings>().get_included_directories_model();
    included.iter().all(|x| x.referenced_folder)
}

pub(crate) fn create_vec_model_from_vec_string(items: Vec<String>) -> VecModel<SharedString> {
    VecModel::from(items.into_iter().map(SharedString::from).collect::<Vec<_>>())
}

// Workaround for https://github.com/slint-ui/slint/discussions/4596
// Currently there is no way to save u64 in slint, so we need to split it into two i32
pub(crate) fn split_u64_into_i32s(value: u64) -> (i32, i32) {
    let part1: i32 = (value >> 32) as i32;
    let part2: i32 = value as i32;
    (part1, part2)
}

pub(crate) fn connect_i32_into_u64(part1: i32, part2: i32) -> u64 {
    ((part1 as u64) << 32) | (part2 as u64 & 0xFFFF_FFFF)
}

#[cfg(test)]
mod test {
    use crate::common::split_u64_into_i32s;

    #[test]
    fn test_split_u64_into_i32s_small() {
        let value = 1;
        let (part1, part2) = split_u64_into_i32s(value);
        assert_eq!(part1, 0);
        assert_eq!(part2, 1);
    }

    #[test]
    fn test_split_u64_into_i32s_big() {
        let value = u64::MAX;
        let (part1, part2) = split_u64_into_i32s(value);
        assert_eq!(part1, -1);
        assert_eq!(part2, -1);
    }

    #[test]
    fn test_connect_i32_into_u64_small() {
        let part1 = 0;
        let part2 = 1;
        let value = super::connect_i32_into_u64(part1, part2);
        assert_eq!(value, 1);
    }

    #[test]
    fn test_connect_i32_into_u64_big() {
        let part1 = -1;
        let part2 = -1;
        let value = super::connect_i32_into_u64(part1, part2);
        assert_eq!(value, u64::MAX);
    }

    #[test]
    fn test_connect_split_zero() {
        for start_value in [0, 1, 10, u32::MAX as u64, i32::MAX as u64, u64::MAX] {
            let (part1, part2) = split_u64_into_i32s(start_value);
            let end_value = super::connect_i32_into_u64(part1, part2);
            assert_eq!(start_value, end_value);
        }
    }
}
