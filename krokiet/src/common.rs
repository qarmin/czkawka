use std::path::PathBuf;

use crate::{CurrentTab, ExcludedDirectoriesModel, IncludedDirectoriesModel, MainListModel, MainWindow};
use slint::{ModelRc, SharedString, VecModel};

// Remember to match updated this according to ui/main_lists.slint and connect_scan.rs files
pub fn get_str_path_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFolders => 1,
        CurrentTab::EmptyFiles => 1,
        CurrentTab::SimilarImages => 4,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}
pub fn get_str_name_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFolders => 0,
        CurrentTab::EmptyFiles => 0,
        CurrentTab::SimilarImages => 3,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}

pub fn get_int_modification_date_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFiles => 0,
        CurrentTab::SimilarImages => 0,
        CurrentTab::EmptyFolders => 0,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}
pub fn get_int_size_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFiles => 2,
        CurrentTab::SimilarImages => 2,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
        CurrentTab::EmptyFolders => panic!("Unable to get size from this tab"),
    }
}

pub fn get_int_width_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::SimilarImages => 4,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
        _ => panic!("Unable to get height from this tab"),
    }
}
pub fn get_int_height_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::SimilarImages => 5,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
        _ => panic!("Unable to get height from this tab"),
    }
}

pub fn get_is_header_mode(active_tab: CurrentTab) -> bool {
    match active_tab {
        CurrentTab::EmptyFolders | CurrentTab::EmptyFiles => false,
        CurrentTab::SimilarImages => true,
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}

pub fn get_tool_model(app: &MainWindow, tab: CurrentTab) -> ModelRc<MainListModel> {
    match tab {
        CurrentTab::EmptyFolders => app.get_empty_folder_model(),
        CurrentTab::SimilarImages => app.get_similar_images_model(),
        CurrentTab::EmptyFiles => app.get_empty_files_model(),
        CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
    }
}

pub fn set_tool_model(app: &MainWindow, tab: CurrentTab, model: ModelRc<MainListModel>) {
    match tab {
        CurrentTab::EmptyFolders => app.set_empty_folder_model(model),
        CurrentTab::SimilarImages => app.set_similar_images_model(model),
        CurrentTab::EmptyFiles => app.set_empty_files_model(model),
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
    ((part1 as u64) << 32) | (part2 as u64 & 0xFFFFFFFF)
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
