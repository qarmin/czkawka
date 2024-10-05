#![allow(dead_code)]
use std::path::PathBuf;

use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::{CurrentTab, ExcludedDirectoriesModel, IncludedDirectoriesModel, MainListModel, MainWindow, Settings};

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
    ModificationDate,
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
    ProperExtension,
}

// Remember to match updated this according to ui/main_lists.slint and connect_scan.rs files
pub fn get_str_path_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFolders => StrDataEmptyFolders::Path as usize,
        CurrentTab::EmptyFiles => StrDataEmptyFiles::Path as usize,
        CurrentTab::SimilarImages => StrDataSimilarImages::Path as usize,
        CurrentTab::DuplicateFiles => StrDataDuplicateFiles::Path as usize,
        CurrentTab::BigFiles => StrDataBigFiles::Path as usize,
        CurrentTab::TemporaryFiles => StrDataTemporaryFiles::Path as usize,
        CurrentTab::SimilarVideos => StrDataSimilarVideos::Path as usize,
        CurrentTab::SimilarMusic => StrDataSimilarMusic::Path as usize,
        CurrentTab::InvalidSymlinks => StrDataInvalidSymlinks::SymlinkFolder as usize,
        CurrentTab::BrokenFiles => StrDataBrokenFiles::Path as usize,
        CurrentTab::BadExtensions => StrDataBadExtensions::Path as usize,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}

pub fn get_str_name_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFolders => StrDataEmptyFolders::Name as usize,
        CurrentTab::EmptyFiles => StrDataEmptyFiles::Name as usize,
        CurrentTab::SimilarImages => StrDataSimilarImages::Name as usize,
        CurrentTab::DuplicateFiles => StrDataDuplicateFiles::Name as usize,
        CurrentTab::BigFiles => StrDataBigFiles::Name as usize,
        CurrentTab::TemporaryFiles => StrDataTemporaryFiles::Name as usize,
        CurrentTab::SimilarVideos => StrDataSimilarVideos::Name as usize,
        CurrentTab::SimilarMusic => StrDataSimilarMusic::Name as usize,
        CurrentTab::InvalidSymlinks => StrDataInvalidSymlinks::SymlinkName as usize,
        CurrentTab::BrokenFiles => StrDataBrokenFiles::Name as usize,
        CurrentTab::BadExtensions => StrDataBadExtensions::Name as usize,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}

pub fn get_int_modification_date_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFiles => IntDataEmptyFiles::ModificationDatePart1 as usize,
        CurrentTab::EmptyFolders => IntDataEmptyFolders::ModificationDatePart1 as usize,
        CurrentTab::SimilarImages => IntDataSimilarImages::ModificationDatePart1 as usize,
        CurrentTab::DuplicateFiles => IntDataDuplicateFiles::ModificationDatePart1 as usize,
        CurrentTab::BigFiles => IntDataBigFiles::ModificationDatePart1 as usize,
        CurrentTab::TemporaryFiles => IntDataTemporaryFiles::ModificationDatePart1 as usize,
        CurrentTab::SimilarVideos => IntDataSimilarVideos::ModificationDatePart1 as usize,
        CurrentTab::SimilarMusic => IntDataSimilarMusic::ModificationDatePart1 as usize,
        CurrentTab::InvalidSymlinks => IntDataInvalidSymlinks::ModificationDatePart1 as usize,
        CurrentTab::BrokenFiles => IntDataBrokenFiles::ModificationDatePart1 as usize,
        CurrentTab::BadExtensions => IntDataBadExtensions::ModificationDatePart1 as usize,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}

pub fn get_int_size_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFiles => IntDataEmptyFiles::SizePart1 as usize,
        CurrentTab::SimilarImages => IntDataSimilarImages::SizePart1 as usize,
        CurrentTab::DuplicateFiles => IntDataDuplicateFiles::SizePart1 as usize,
        CurrentTab::BigFiles => IntDataBigFiles::SizePart1 as usize,
        CurrentTab::SimilarVideos => IntDataSimilarVideos::SizePart1 as usize,
        CurrentTab::SimilarMusic => IntDataSimilarMusic::SizePart1 as usize,
        CurrentTab::BrokenFiles => IntDataBrokenFiles::SizePart1 as usize,
        CurrentTab::BadExtensions => IntDataBadExtensions::SizePart1 as usize,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
        CurrentTab::EmptyFolders | CurrentTab::InvalidSymlinks | CurrentTab::TemporaryFiles => panic!("Unable to get size from this tab"),
    }
}

pub fn get_int_width_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::SimilarImages => IntDataSimilarImages::Width as usize,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
        _ => panic!("Unable to get height from this tab"),
    }
}

pub fn get_int_height_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::SimilarImages => IntDataSimilarImages::Height as usize,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
        _ => panic!("Unable to get height from this tab"),
    }
}

pub fn get_is_header_mode(active_tab: CurrentTab) -> bool {
    match active_tab {
        CurrentTab::EmptyFolders
        | CurrentTab::EmptyFiles
        | CurrentTab::BrokenFiles
        | CurrentTab::BigFiles
        | CurrentTab::TemporaryFiles
        | CurrentTab::InvalidSymlinks
        | CurrentTab::BadExtensions => false,
        CurrentTab::SimilarImages | CurrentTab::DuplicateFiles | CurrentTab::SimilarVideos | CurrentTab::SimilarMusic => true,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}

pub fn get_tool_model(app: &MainWindow, tab: CurrentTab) -> ModelRc<MainListModel> {
    match tab {
        CurrentTab::EmptyFolders => app.get_empty_folder_model(),
        CurrentTab::SimilarImages => app.get_similar_images_model(),
        CurrentTab::EmptyFiles => app.get_empty_files_model(),
        CurrentTab::DuplicateFiles => app.get_duplicate_files_model(),
        CurrentTab::BigFiles => app.get_big_files_model(),
        CurrentTab::TemporaryFiles => app.get_temporary_files_model(),
        CurrentTab::SimilarVideos => app.get_similar_videos_model(),
        CurrentTab::SimilarMusic => app.get_similar_music_model(),
        CurrentTab::InvalidSymlinks => app.get_invalid_symlinks_model(),
        CurrentTab::BrokenFiles => app.get_broken_files_model(),
        CurrentTab::BadExtensions => app.get_bad_extensions_model(),
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}

pub fn set_tool_model(app: &MainWindow, tab: CurrentTab, model: ModelRc<MainListModel>) {
    match tab {
        CurrentTab::EmptyFolders => app.set_empty_folder_model(model),
        CurrentTab::SimilarImages => app.set_similar_images_model(model),
        CurrentTab::EmptyFiles => app.set_empty_files_model(model),
        CurrentTab::DuplicateFiles => app.set_duplicate_files_model(model),
        CurrentTab::BigFiles => app.set_big_files_model(model),
        CurrentTab::TemporaryFiles => app.set_temporary_files_model(model),
        CurrentTab::SimilarVideos => app.set_similar_videos_model(model),
        CurrentTab::SimilarMusic => app.set_similar_music_model(model),
        CurrentTab::InvalidSymlinks => app.set_invalid_symlinks_model(model),
        CurrentTab::BrokenFiles => app.set_broken_files_model(model),
        CurrentTab::BadExtensions => app.set_bad_extensions_model(model),
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}

// pub fn create_string_standard_list_view(items: &[String]) -> ModelRc<StandardListViewItem> {
//     let new_folders_standard_list_view = items
//         .iter()
//         .map(|x| {
//             let mut element = StandardListViewItem::default();
//             element.text = x.into();
//             element
//         })
//         .collect::<Vec<_>>();
//     ModelRc::new(VecModel::from(new_folders_standard_list_view))
// }
// pub fn create_string_standard_list_view_from_pathbuf(items: &[PathBuf]) -> ModelRc<StandardListViewItem> {
//     let new_folders_standard_list_view = items
//         .iter()
//         .map(|x| {
//             let mut element = StandardListViewItem::default();
//             element.text = x.to_string_lossy().to_string().into();
//             element
//         })
//         .collect::<Vec<_>>();
//     ModelRc::new(VecModel::from(new_folders_standard_list_view))
// }

pub fn create_included_directories_model_from_pathbuf(items: &[PathBuf], referenced: &[PathBuf]) -> ModelRc<IncludedDirectoriesModel> {
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

pub fn create_excluded_directories_model_from_pathbuf(items: &[PathBuf]) -> ModelRc<ExcludedDirectoriesModel> {
    let converted = items
        .iter()
        .map(|x| ExcludedDirectoriesModel {
            path: x.to_string_lossy().to_string().into(),
            selected_row: false,
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(converted))
}

pub fn check_if_there_are_any_included_folders(app: &MainWindow) -> bool {
    let included = app.global::<Settings>().get_included_directories_model();
    included.iter().count() > 0
}

pub fn check_if_all_included_dirs_are_referenced(app: &MainWindow) -> bool {
    let included = app.global::<Settings>().get_included_directories_model();
    included.iter().all(|x| x.referenced_folder)
}

pub fn create_vec_model_from_vec_string(items: Vec<String>) -> VecModel<SharedString> {
    VecModel::from(items.into_iter().map(SharedString::from).collect::<Vec<_>>())
}

// Workaround for https://github.com/slint-ui/slint/discussions/4596
// Currently there is no way to save u64 in slint, so we need to split it into two i32
pub fn split_u64_into_i32s(value: u64) -> (i32, i32) {
    let part1: i32 = (value >> 32) as i32;
    let part2: i32 = value as i32;
    (part1, part2)
}

pub fn connect_i32_into_u64(part1: i32, part2: i32) -> u64 {
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
