use std::path::MAIN_SEPARATOR;

use czkawka_core::common_messages::Messages;
use rayon::prelude::*;
use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::{get_str_name_idx, get_str_path_idx, get_tool_model, set_tool_model};
use crate::connect_row_selection::reset_selection;
use crate::model_operations::{ModelProcessor, get_shared_str_item};
use crate::{Callabler, CurrentTab, GuiState, MainListModel, MainWindow, Settings, flk};

type DeleteModel = Option<(Vec<MainListModel>, Vec<(usize, Option<String>)>, usize)>;

pub fn connect_delete_button(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_delete_selected_items(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let active_tab = app.global::<GuiState>().get_active_tab();

        let model = get_tool_model(&app, active_tab);

        let settings = app.global::<Settings>();

        let processor = ModelProcessor::new(&model, active_tab);
        let Some((new_model, errors, _items_queued_to_delete, items_deleted)) = processor.delete_selected_items(settings.get_move_to_trash()) else {
            app.global::<GuiState>().set_info_text("".into());
            return;
        };

        app.set_text_summary_text(flk!("rust_delete_summary", deleted = items_deleted, failed = errors.len()).into());

        set_tool_model(&app, active_tab, ModelRc::new(VecModel::from(new_model)));

        app.global::<GuiState>().set_info_text(Messages::new_from_errors(errors).create_messages_text().into());

        app.global::<GuiState>().set_preview_visible(false);

        reset_selection(&app, true);
    });
}

impl<'a> ModelProcessor<'a> {
    fn delete_selected_items(&self, remove_to_trash: bool) -> Option<(Vec<MainListModel>, Vec<String>, usize, usize)> {
        let is_empty_folder_tab = self.active_tab == CurrentTab::EmptyFolders;

        let (items, items_simplified, items_queued_to_delete) = self.prepare_delete_models()?;

        let output = self.delete_items(items_simplified, is_empty_folder_tab, remove_to_trash);

        let (new_model, errors, items_deleted) = self.collect_delete_model(output, items);

        Some((new_model, errors, items_queued_to_delete, items_deleted))
    }

    fn delete_items(&self, items_simplified: Vec<(usize, Option<String>)>, is_empty_folder_tab: bool, remove_to_trash: bool) -> Vec<(usize, Option<Result<(), String>>)> {
        let mut output: Vec<_> = items_simplified
            .into_par_iter()
            .map(|(idx, data)| {
                let Some(data) = data else {
                    return (idx, None);
                };

                let res = remove_single_item(&data, is_empty_folder_tab, remove_to_trash);
                (idx, Some(res))
            })
            .collect();
        output.sort_by_key(|(idx, _)| *idx);

        output
    }

    fn collect_delete_model(&self, output: Vec<(usize, Option<Result<(), String>>)>, items: Vec<MainListModel>) -> (Vec<MainListModel>, Vec<String>, usize) {
        let mut errors = vec![];
        let mut items_deleted = 0;

        let new_model = output
            .into_iter()
            .map(|(_idx, res)| res)
            .zip(items)
            .filter_map(|(res, model)| match res {
                Some(Ok(())) => {
                    items_deleted += 1;
                    None
                }
                Some(Err(err)) => {
                    errors.push(err);
                    Some(model)
                }
                None => Some(model),
            })
            .collect();

        let new_model = self.remove_single_items_in_groups(new_model);

        (new_model, errors, items_deleted)
    }

    fn prepare_delete_models(&self) -> DeleteModel {
        let path_idx = get_str_path_idx(self.active_tab);
        let name_idx = get_str_name_idx(self.active_tab);

        let items = self.items.iter().collect::<Vec<_>>();
        let items_queued_to_delete = items.iter().filter(|item| item.checked).count();
        if items_queued_to_delete == 0 {
            return None;
        }

        // Due to slint limitation in sending MainListModel to Rayon, we need to extract Sized arguments, that are used
        // and then put them to rayon and then sort, to be able to remove items that were deleted
        // It is quite inefficient, but it is the only way to do it()
        let items_simplified = items
            .iter()
            .map(|model| {
                if model.checked {
                    Some(format!("{}{MAIN_SEPARATOR}{}", get_shared_str_item(model, path_idx), get_shared_str_item(model, name_idx)))
                } else {
                    None
                }
            })
            .enumerate()
            .collect::<Vec<_>>();
        Some((items, items_simplified, items_queued_to_delete))
    }
}

#[cfg(not(test))]
fn remove_single_item(full_path: &str, is_folder_tab: bool, remove_to_trash: bool) -> Result<(), String> {
    if is_folder_tab {
        return czkawka_core::common::remove_folder_if_contains_only_empty_folders(full_path, remove_to_trash);
    }
    if remove_to_trash {
        if let Err(e) = trash::delete(full_path) {
            return Err(flk!("rust_error_moving_to_trash", error = e.to_string()));
        }
    } else {
        if let Err(e) = std::fs::remove_file(full_path) {
            return Err(flk!("rust_error_removing_file", error = e.to_string()));
        }
    }
    Ok(())
}

#[cfg(test)]
fn remove_single_item(full_path: &str, _is_folder_tab: bool, _remove_to_trash: bool) -> Result<(), String> {
    if full_path.contains("test_error") {
        return Err(format!("Test error for item: {full_path}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common::{create_model_from_model_vec, get_model_vec};

    #[test]
    fn test_no_delete_items() {
        let model = get_model_vec(10);
        let model = create_model_from_model_vec(&model);
        let processor = ModelProcessor::new(&model, CurrentTab::EmptyFolders);
        assert!(processor.delete_selected_items(false).is_none());
    }

    #[test]
    fn test_delete_selected_items() {
        let mut model = get_model_vec(10);
        model[0].checked = true;
        model[0].val_str = ModelRc::new(VecModel::from(vec!["normal1".to_string().into(); 10]));
        model[1].checked = true;
        model[1].val_str = ModelRc::new(VecModel::from(vec!["normal2".to_string().into(); 10]));
        model[3].checked = true;
        model[3].val_str = ModelRc::new(VecModel::from(vec!["test_error".to_string().into(); 10]));
        let model = create_model_from_model_vec(&model);
        let processor = ModelProcessor::new(&model, CurrentTab::EmptyFolders);
        let (new_model, errors, items_queued_to_delete, items_deleted) = processor.delete_selected_items(false).unwrap();

        assert_eq!(new_model.len(), 8);
        assert_eq!(errors.len(), 1);
        assert_eq!(items_queued_to_delete, 3);
        assert_eq!(items_deleted, 2);

        assert!(new_model[1].checked);
        assert!(new_model[1].val_str.iter().all(|s| s == "test_error"));
        assert!(!new_model[0].checked);
        assert!(new_model.iter().skip(2).all(|model| !model.checked));
    }
}
