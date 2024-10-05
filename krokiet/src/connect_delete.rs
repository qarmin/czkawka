use czkawka_core::common::remove_folder_if_contains_only_empty_folders;
use czkawka_core::common_messages::Messages;
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, VecModel};

use crate::common::{get_is_header_mode, get_tool_model, set_tool_model};
use crate::model_operations::{collect_full_path_from_model, deselect_all_items, filter_out_checked_items};
use crate::{Callabler, CurrentTab, GuiState, MainListModel, MainWindow, Settings};

pub fn connect_delete_button(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_delete_selected_items(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let active_tab = app.global::<GuiState>().get_active_tab();

        let model = get_tool_model(&app, active_tab);

        let settings = app.global::<Settings>();

        let (errors, new_model) = handle_delete_items(&app, &model, active_tab, settings.get_move_to_trash());

        if let Some(new_model) = new_model {
            set_tool_model(&app, active_tab, new_model);
        }

        app.global::<GuiState>().set_info_text(Messages::new_from_errors(errors).create_messages_text().into());

        app.global::<GuiState>().set_preview_visible(false);
    });
}

fn handle_delete_items(app: &MainWindow, items: &ModelRc<MainListModel>, active_tab: CurrentTab, remove_to_trash: bool) -> (Vec<String>, Option<ModelRc<MainListModel>>) {
    let (entries_to_delete, mut entries_left) = filter_out_checked_items(items, get_is_header_mode(active_tab));

    if !entries_to_delete.is_empty() {
        let vec_items_to_remove = collect_full_path_from_model(&entries_to_delete, active_tab);
        let errors = remove_selected_items(vec_items_to_remove, active_tab, remove_to_trash);
        deselect_all_items(&mut entries_left); // TODO - this now probably is not needed, because selected items were removed
        app.set_text_summary_text(format!("Deleted {} items, failed to remove {} items", entries_to_delete.len() - errors.len(), errors.len()).into());

        let r = ModelRc::new(VecModel::from(entries_left)); // TODO here maybe should also stay old model if entries cannot be removed
        return (errors, Some(r));
    }
    (vec![], None)
}

// TODO delete in parallel items, consider to add progress bar
// For empty folders double check if folders are really empty - this function probably should be run in thread
// and at the end should be send signal to main thread to update model
fn remove_selected_items(items_to_remove: Vec<String>, active_tab: CurrentTab, remove_to_trash: bool) -> Vec<String> {
    // Iterate over empty folders and not delete them if they are not empty
    if active_tab == CurrentTab::EmptyFolders {
        items_to_remove
            .into_par_iter()
            .filter_map(|item| remove_folder_if_contains_only_empty_folders(item, remove_to_trash).err())
            .collect()
    } else {
        items_to_remove
            .into_par_iter()
            .filter_map(|item| {
                if remove_to_trash {
                    if let Err(e) = trash::delete(item) {
                        return Some(format!("Error while moving to trash: {e}"));
                    }
                } else {
                    if let Err(e) = std::fs::remove_file(item) {
                        return Some(format!("Error while removing file: {e}"));
                    }
                }
                None
            })
            .collect()
    }
}
