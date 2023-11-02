use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};

pub fn create_string_standard_list_view(items: &[String]) -> ModelRc<StandardListViewItem> {
    let new_folders_standard_list_view = items
        .iter()
        .map(|x| {
            let mut element = StandardListViewItem::default();
            element.text = SharedString::from(x.to_string());
            element
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(new_folders_standard_list_view))
}
