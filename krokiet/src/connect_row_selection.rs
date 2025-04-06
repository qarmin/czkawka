use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{RwLock, RwLockWriteGuard};

use czkawka_core::TOOLS_NUMBER;
use once_cell::sync::OnceCell;
use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::{CurrentTab, GuiState, MainListModel, MainWindow};

const SELECTED_ROWS_LIMIT: usize = 1000;

#[derive(Debug, Default, Clone)]
pub(crate) struct SelectionData {
    // Should be always valid
    number_of_selected_rows: usize,
    // Needs to be empty, when exceeded limit
    selected_rows: Vec<usize>,
    // If exceeded limit, then we need to reload entire model, because it should be faster that changing each row
    exceeded_limit: bool,
}

pub(crate) static TOOLS_SELECTION: OnceCell<RwLock<HashMap<CurrentTab, SelectionData>>> = OnceCell::new();

pub(crate) fn reset_selection(app: &MainWindow, reset_all_selection: bool) {
    if reset_all_selection {
        let active_tab = app.global::<GuiState>().get_active_tab();
        let mut lock = get_write_selection_lock();
        let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
        selection.selected_rows.clear();
        selection.exceeded_limit = false;
    }

    app.invoke_reset_selection();
}

fn initialize_selection_struct() {
    let tools: [CurrentTab; TOOLS_NUMBER] = [
        CurrentTab::DuplicateFiles,
        CurrentTab::EmptyFolders,
        CurrentTab::BigFiles,
        CurrentTab::EmptyFiles,
        CurrentTab::TemporaryFiles,
        CurrentTab::SimilarImages,
        CurrentTab::SimilarVideos,
        CurrentTab::SimilarMusic,
        CurrentTab::InvalidSymlinks,
        CurrentTab::BrokenFiles,
        CurrentTab::BadExtensions,
    ];

    let map: HashMap<_, _> = tools.into_iter().map(|tool| (tool, SelectionData::default())).collect();
    TOOLS_SELECTION.set(RwLock::new(map)).expect("Failed to set selection data, it was already set");
}

// fn get_read_selection_lock() -> RwLockReadGuard<'static, HashMap<CurrentTab, SelectionData>> {
//     let selection = TOOLS_SELECTION.get().expect("Selection data is not initialized");
//     selection.read().expect("Failed to lock selection data")
// }
fn get_write_selection_lock() -> RwLockWriteGuard<'static, HashMap<CurrentTab, SelectionData>> {
    let selection = TOOLS_SELECTION.get().expect("Selection data is not initialized");
    selection.write().expect("Failed to lock selection data")
}

impl Hash for CurrentTab {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as u8).hash(state);
    }
}
impl Eq for CurrentTab {}

////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
////////////////////

pub fn connect_row_selections(app: &MainWindow) {
    initialize_selection_struct();

    selection::connect_select_all_rows(app); // CTRL + A
    selection::reverse_single_unique_item(app); // LMB
    selection::reverse_checked_on_selection(app); // Space
    selection::reverse_selection_on_specific_item(app); // CTRL + LMB
    selection::select_items_with_shift(app); // SHIFT + LMB
    opener::open_selected_item(app);
    opener::open_parent_of_selected_item(app);
}

mod opener {
    use log::error;
    use slint::{ComponentHandle, Model};

    use crate::common::{get_str_name_idx, get_str_path_idx, get_tool_model};
    use crate::connect_row_selection::get_write_selection_lock;
    use crate::{Callabler, GuiState, MainWindow};

    fn open_selected_items(app: &MainWindow, items_path_str: &[usize]) {
        let active_tab = app.global::<GuiState>().get_active_tab();
        let mut lock = get_write_selection_lock();
        let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
        let model = get_tool_model(app, active_tab);

        if selection.selected_rows.len() == 1 {
            let id = selection.selected_rows[0];
            let model_data = model
                .row_data(id)
                .unwrap_or_else(|| panic!("Failed to get row data with id {id}, in tab {active_tab:?}, with model {} items", model.row_count()));

            let path_to_open = if items_path_str.len() == 1 {
                format!("{}", model_data.val_str.iter().nth(items_path_str[0]).expect("Cannot find path"))
            } else {
                format!(
                    "{}/{}",
                    model_data.val_str.iter().nth(items_path_str[0]).expect("Cannot find path"),
                    model_data.val_str.iter().nth(items_path_str[1]).expect("Cannot find name")
                )
            };

            if let Err(e) = open::that(&path_to_open) {
                error!("Failed to open file: {e}");
            };
        } else {
            if selection.selected_rows.is_empty() {
                error!("Failed to open selected item, because there is no selected item");
            } else {
                error!("Failed to open selected item, because there is more than one selected item");
            }
        }
    }

    pub(crate) fn open_selected_item(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_open_selected_item(move || {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            open_selected_items(&app, &[get_str_path_idx(active_tab), get_str_name_idx(active_tab)]);
        });
    }

    pub(crate) fn open_parent_of_selected_item(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_open_parent_of_selected_item(move || {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            open_selected_items(&app, &[get_str_path_idx(active_tab)]);
        });
    }
}
mod selection {
    use slint::{ComponentHandle, Model};

    use crate::common::{get_tool_model, set_tool_model};
    use crate::connect_row_selection::{
        get_write_selection_lock, reverse_selection_of_item_with_id, row_select_items_with_shift, rows_deselect_all_by_mode, rows_reverse_checked_selection,
        rows_select_all_by_mode,
    };
    use crate::{Callabler, GuiState, MainWindow};

    pub(crate) fn connect_select_all_rows(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_select_all(move || {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();

            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
            let model = get_tool_model(&app, active_tab);

            if let Some(new_model) = rows_select_all_by_mode(selection, &model) {
                set_tool_model(&app, active_tab, new_model);
            };
        });
    }

    pub(crate) fn reverse_single_unique_item(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_reverse_single_unique_item(move |id| {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
            let model = get_tool_model(&app, active_tab);

            if let Some(new_model) = rows_deselect_all_by_mode(selection, &model) {
                set_tool_model(&app, active_tab, new_model);
            }
            reverse_selection_of_item_with_id(selection, &model, id as usize, active_tab);
        });
    }

    pub(crate) fn reverse_checked_on_selection(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_reverse_checked_selection(move || {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
            let model = get_tool_model(&app, active_tab);

            let new_model = rows_reverse_checked_selection(selection, &model);
            if let Some(new_model) = new_model {
                set_tool_model(&app, active_tab, new_model);
            }
        });
    }
    pub(crate) fn reverse_selection_on_specific_item(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_reverse_item_selection(move |id| {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
            let model = get_tool_model(&app, active_tab);

            reverse_selection_of_item_with_id(selection, &model, id as usize, active_tab);
        });
    }

    pub(crate) fn select_items_with_shift(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_select_items_with_shift(move |first_idx, second_idx| {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
            let model = get_tool_model(&app, active_tab);

            assert!(first_idx >= 0);
            assert!(second_idx >= 0);
            assert!((first_idx as usize) < model.row_count());
            assert!((second_idx as usize) < model.row_count());

            if let Some(new_model) = row_select_items_with_shift(selection, &model, (first_idx as usize, second_idx as usize)) {
                set_tool_model(&app, active_tab, new_model);
            };
        });
    }
}

////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
// Deselect
fn rows_deselect_all_by_mode(selection: &mut SelectionData, model: &ModelRc<MainListModel>) -> Option<ModelRc<MainListModel>> {
    let new_model = if selection.exceeded_limit {
        Some(rows_deselect_all_selected_by_replacing_models(model))
    } else if !selection.selected_rows.is_empty() {
        rows_deselect_all_selected_one_by_one(model, selection);
        None
    } else {
        None
    };

    selection.selected_rows.clear();
    selection.exceeded_limit = false;
    selection.number_of_selected_rows = 0;

    new_model
}

fn rows_deselect_all_selected_one_by_one(model: &ModelRc<MainListModel>, selection: &SelectionData) {
    for id in &selection.selected_rows {
        let mut model_data = model
            .row_data(*id)
            .unwrap_or_else(|| panic!("Failed to get row data with id {id}, with model {} items", model.row_count()));
        assert!(model_data.selected_row); // Probably can be removed in future
        model_data.selected_row = false;
        model.set_row_data(*id, model_data);
    }
}

fn rows_deselect_all_selected_by_replacing_models(model: &ModelRc<MainListModel>) -> ModelRc<MainListModel> {
    let new_model = model
        .iter()
        .map(|mut row| {
            row.selected_row = false;
            row
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(new_model))
}

// Select
fn rows_select_all_by_mode(selection: &mut SelectionData, model: &ModelRc<MainListModel>) -> Option<ModelRc<MainListModel>> {
    let new_model = if model.row_count() - selection.number_of_selected_rows > 100 {
        rows_select_all_by_replacing_models(selection, model)
    } else {
        rows_select_all_one_by_one(model);
        None
    };

    if model.row_count() > SELECTED_ROWS_LIMIT || selection.exceeded_limit {
        selection.exceeded_limit = true;
        selection.selected_rows.clear();
    } else {
        selection.selected_rows = (0..model.row_count()).collect();
    }
    selection.number_of_selected_rows = model.row_count();

    new_model
}

fn rows_select_all_one_by_one(model: &ModelRc<MainListModel>) {
    for id in 0..model.row_count() {
        let mut model_data = model
            .row_data(id)
            .unwrap_or_else(|| panic!("Failed to get row data with id {id}, with model {} items", model.row_count()));

        if model_data.selected_row {
            continue;
        }

        model_data.selected_row = true;
        model.set_row_data(id, model_data);
    }
}

fn rows_select_all_by_replacing_models(selection: &SelectionData, model: &ModelRc<MainListModel>) -> Option<ModelRc<MainListModel>> {
    if selection.number_of_selected_rows == model.row_count() {
        return None;
    }

    let new_model = model
        .iter()
        .map(|mut row| {
            row.selected_row = true;
            row
        })
        .collect::<Vec<_>>();
    Some(ModelRc::new(VecModel::from(new_model)))
}

fn reverse_selection_of_item_with_id(selection: &mut SelectionData, model: &ModelRc<MainListModel>, id: usize, active_tab: CurrentTab) {
    let mut model_data = model
        .row_data(id)
        .unwrap_or_else(|| panic!("Failed to get row data with id {id}, in tab {active_tab:?}, with model {} items", model.row_count()));

    let was_selected = model_data.selected_row;
    model_data.selected_row = !model_data.selected_row;
    model.set_row_data(id, model_data);

    if was_selected {
        if !selection.exceeded_limit {
            selection.selected_rows.retain(|&x| x != id);
        }
        selection.number_of_selected_rows -= 1;
    } else {
        if !selection.exceeded_limit {
            selection.selected_rows.push(id);
            selection.selected_rows.sort_unstable();
        }
        selection.number_of_selected_rows += 1;
    }
}
fn row_select_items_with_shift(selection: &mut SelectionData, model: &ModelRc<MainListModel>, indexes: (usize, usize)) -> Option<ModelRc<MainListModel>> {
    let (smaller_idx, bigger_idx) = if indexes.0 < indexes.1 { (indexes.0, indexes.1) } else { (indexes.1, indexes.0) };

    let new_model = if bigger_idx - smaller_idx > SELECTED_ROWS_LIMIT || selection.exceeded_limit {
        let new_model: Vec<_> = model
            .iter()
            .enumerate()
            .map(|(idx, mut row)| {
                row.selected_row = (smaller_idx..=bigger_idx).contains(&idx);
                row
            })
            .collect();
        Some(ModelRc::new(VecModel::from(new_model)))
    } else {
        for idx in &selection.selected_rows {
            if !(smaller_idx..=bigger_idx).contains(idx) {
                let mut model_data = model
                    .row_data(*idx)
                    .unwrap_or_else(|| panic!("Failed to get row data with id {idx}, with model {} items", model.row_count()));
                assert!(model_data.selected_row); // Probably can be removed in future
                model_data.selected_row = false;
                model.set_row_data(*idx, model_data);
            }
        }
        for idx in smaller_idx..=bigger_idx {
            let mut model_data = model
                .row_data(idx)
                .unwrap_or_else(|| panic!("Failed to get row data with id {idx}, with model {} items", model.row_count()));
            if !model_data.selected_row {
                model_data.selected_row = true;
                model.set_row_data(idx, model_data);
            }
        }
        None
    };

    if bigger_idx - smaller_idx > SELECTED_ROWS_LIMIT {
        selection.exceeded_limit = true;
        selection.selected_rows.clear();
    } else {
        selection.selected_rows = (smaller_idx..=bigger_idx).collect();
    }
    selection.number_of_selected_rows = bigger_idx - smaller_idx + 1;

    new_model
}

fn rows_reverse_checked_selection(selection: &mut SelectionData, model: &ModelRc<MainListModel>) -> Option<ModelRc<MainListModel>> {
    if selection.exceeded_limit {
        let new_model = model
            .iter()
            .map(|mut row| {
                if row.selected_row {
                    row.checked = !row.checked;
                }
                row
            })
            .collect::<Vec<_>>();
        return Some(ModelRc::new(VecModel::from(new_model)));
    } else if !selection.selected_rows.is_empty() {
        let ids = model
            .iter()
            .enumerate()
            .filter_map(|(idx, e)| if e.selected_row { Some(idx) } else { None })
            .collect::<Vec<_>>();
        for id in ids {
            let mut model_data = model
                .row_data(id)
                .unwrap_or_else(|| panic!("Failed to get row data with id {id}, with model {} items", model.row_count()));
            assert!(model_data.selected_row); // Probably can be removed in future
            model_data.checked = !model_data.checked;
            model.set_row_data(id, model_data);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use slint::VecModel;

    fn get_main_list_model() -> MainListModel {
        MainListModel {
            selected_row: false,
            val_int: Default::default(),
            checked: false,
            filled_header_row: false,
            header_row: false,
            val_str: Default::default(),
        }
    }
    fn get_model_vec(items: usize) -> Vec<MainListModel> {
        (0..items).map(|_| get_main_list_model()).collect::<Vec<_>>()
    }
    fn create_model_from_model_vec(model_vec: &Vec<MainListModel>) -> ModelRc<MainListModel> {
        ModelRc::new(VecModel::from(model_vec.clone()))
    }

    #[test]
    fn rows_deselect_all_by_mode_with_exceeded_limit() {
        let mut model = get_model_vec(3);
        model[0].selected_row = true;
        model[1].selected_row = true;
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 2,
            selected_rows: vec![0, 1],
            exceeded_limit: true,
        };

        let new_model = rows_deselect_all_by_mode(&mut selection, &model);

        assert!(new_model.is_some());
        let new_model = new_model.unwrap();
        assert!(!new_model.row_data(0).unwrap().selected_row);
        assert!(!new_model.row_data(1).unwrap().selected_row);
        assert!(!new_model.row_data(2).unwrap().selected_row);
        assert!(selection.selected_rows.is_empty());
        assert!(!selection.exceeded_limit);
        assert_eq!(selection.number_of_selected_rows, 0);
    }

    #[test]
    fn rows_deselect_all_by_mode_with_selected_rows() {
        let mut model = get_model_vec(3);
        model[0].selected_row = true;
        model[1].selected_row = true;
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 2,
            selected_rows: vec![0, 1],
            exceeded_limit: false,
        };

        let new_model = rows_deselect_all_by_mode(&mut selection, &model);

        assert!(new_model.is_none());
        assert!(!model.row_data(0).unwrap().selected_row);
        assert!(!model.row_data(1).unwrap().selected_row);
        assert!(!model.row_data(2).unwrap().selected_row);
        assert!(selection.selected_rows.is_empty());
        assert!(!selection.exceeded_limit);
        assert_eq!(selection.number_of_selected_rows, 0);
    }

    #[test]
    fn rows_deselect_all_by_mode_with_no_selected_rows() {
        let model = get_model_vec(3);
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 0,
            selected_rows: vec![],
            exceeded_limit: false,
        };

        let new_model = rows_deselect_all_by_mode(&mut selection, &model);

        assert!(new_model.is_none());
        assert!(!model.row_data(0).unwrap().selected_row);
        assert!(!model.row_data(1).unwrap().selected_row);
        assert!(!model.row_data(2).unwrap().selected_row);
        assert!(selection.selected_rows.is_empty());
        assert!(!selection.exceeded_limit);
        assert_eq!(selection.number_of_selected_rows, 0);
    }
}
