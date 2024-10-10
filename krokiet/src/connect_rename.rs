use std::fs;
use std::path::{Path, MAIN_SEPARATOR};

use czkawka_core::common_messages::Messages;
use slint::{ComponentHandle, ModelRc, VecModel};

use crate::common::{get_is_header_mode, get_tool_model, set_tool_model};
use crate::model_operations::{collect_path_name_and_proper_extension_from_model, deselect_all_items, filter_out_checked_items};
use crate::{Callabler, CurrentTab, GuiState, MainListModel, MainWindow};

pub fn connect_rename(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_rename_files(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = get_tool_model(&app, active_tab);

        let (errors, new_model) = rename_operation(&current_model, active_tab);
        if let Some(new_model) = new_model {
            set_tool_model(&app, active_tab, new_model);
        }
        app.global::<GuiState>().set_info_text(Messages::new_from_errors(errors).create_messages_text().into());
    });
}

fn rename_operation(items: &ModelRc<MainListModel>, active_tab: CurrentTab) -> (Vec<String>, Option<ModelRc<MainListModel>>) {
    assert_eq!(active_tab, CurrentTab::BadExtensions);
    let (entries_to_move, mut entries_left) = filter_out_checked_items(items, get_is_header_mode(active_tab));

    if !entries_to_move.is_empty() {
        let vec_items_to_rename = collect_path_name_and_proper_extension_from_model(&entries_to_move, active_tab);
        let errors = rename_selected_items(vec_items_to_rename);
        deselect_all_items(&mut entries_left);

        let r = ModelRc::new(VecModel::from(entries_left));
        return (errors, Some(r));
    }
    (vec![], None)
}

fn rename_selected_items(files_with_new_extensions: Vec<(String, String, String)>) -> Vec<String> {
    let mut errors = vec![];
    for (folder, file_name, new_extension) in files_with_new_extensions {
        let file_stem = Path::new(&file_name).file_stem().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
        let new_full_path = format!("{}{}{}.{}", folder, MAIN_SEPARATOR, file_stem, new_extension);
        let old_full_path = format!("{}{}{}", folder, MAIN_SEPARATOR, file_name);
        if let Err(e) = fs::rename(&old_full_path, &new_full_path) {
            errors.push(format!("Failed to rename file {} to {} with error {}", old_full_path, new_full_path, e));
        }
    }
    errors
}
