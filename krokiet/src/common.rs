use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use std::path::PathBuf;

// pub fn create_string_standard_list_view(items: &[String]) -> ModelRc<StandardListViewItem> {
//     let new_folders_standard_list_view = items
//         .iter()
//         .map(|x| {
//             let mut element = StandardListViewItem::default();
//             element.text = SharedString::from(x.to_string());
//             element
//         })
//         .collect::<Vec<_>>();
//     ModelRc::new(VecModel::from(new_folders_standard_list_view))
// }
pub fn create_string_standard_list_view_from_pathbuf(items: &[PathBuf]) -> ModelRc<StandardListViewItem> {
    let new_folders_standard_list_view = items
        .iter()
        .map(|x| {
            let mut element = StandardListViewItem::default();
            element.text = SharedString::from(x.to_string_lossy().to_string());
            element
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(new_folders_standard_list_view))
}

pub fn create_vec_model_from_vec_string(items: Vec<String>) -> VecModel<SharedString> {
    VecModel::from(items.into_iter().map(SharedString::from).collect::<Vec<_>>())
}
