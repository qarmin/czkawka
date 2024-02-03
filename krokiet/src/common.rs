use std::path::PathBuf;

use crate::{CurrentTab, ExcludedDirectoriesModel, IncludedDirectoriesModel};
use slint::{ModelRc, SharedString, VecModel};

// Remember to match updated this according to ui/main_lists.slint and connect_scan.rs files
pub fn get_path_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFolders => 1,
        CurrentTab::EmptyFiles => 1,
        CurrentTab::SimilarImages => 4,
        CurrentTab::Settings => panic!("Button should be disabled"),
    }
}
pub fn get_name_idx(active_tab: CurrentTab) -> usize {
    match active_tab {
        CurrentTab::EmptyFolders => 0,
        CurrentTab::EmptyFiles => 0,
        CurrentTab::SimilarImages => 3,
        CurrentTab::Settings => panic!("Button should be disabled"),
    }
}
pub fn get_is_header_mode(active_tab: CurrentTab) -> bool {
    match active_tab {
        CurrentTab::EmptyFolders | CurrentTab::EmptyFiles => false,
        CurrentTab::SimilarImages => true,
        CurrentTab::Settings => panic!("Button should be disabled"),
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
