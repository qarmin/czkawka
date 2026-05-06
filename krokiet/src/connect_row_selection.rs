use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem;
use std::sync::{LazyLock, RwLock, RwLockWriteGuard};

use czkawka_core::TOOLS_NUMBER;
use log::{error, trace};
use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::{connect_i32_into_u64, split_u64_into_i32s};
use crate::{ActiveTab, Callabler, GuiState, MainWindow, SingleMainListModel};

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

pub(crate) static TOOLS_SELECTION: LazyLock<RwLock<HashMap<ActiveTab, SelectionData>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

// pub(crate) fn get_selection_data(active_tab: ActiveTab) -> SelectionData {
//     let lock = TOOLS_SELECTION.read().expect("Selection data is not initialized or is poisoned");
//     let keys = lock.keys().cloned().collect::<Vec<_>>();
//     lock.get(&active_tab)
//         .unwrap_or_else(|| panic!("Failed to get selection data for tab {active_tab:?} - {keys:?}"))
//         .clone()
// }

pub(crate) fn reset_selection(app: &MainWindow, active_tab: ActiveTab, reset_all_selection: bool) {
    if reset_all_selection {
        let mut lock = get_write_selection_lock();
        let keys = lock.keys().cloned().collect::<Vec<_>>();
        let selection = lock
            .get_mut(&active_tab)
            .unwrap_or_else(|| panic!("Failed to get selection data for tab {active_tab:?} - {keys:?}"));
        selection.selected_rows.clear();
        selection.number_of_selected_rows = 0;
        selection.exceeded_limit = false;
    }

    app.invoke_reset_selection(active_tab);
}

// E.g. when sorting things, selected rows in vector, may be invalid
// So we need to recalculate them
pub(crate) fn recalculate_small_selection_if_needed(model: &ModelRc<SingleMainListModel>, active_tab: ActiveTab) {
    let mut lock = get_write_selection_lock();
    let keys = lock.keys().cloned().collect::<Vec<_>>();
    let selection = lock
        .get_mut(&active_tab)
        .unwrap_or_else(|| panic!("Failed to get selection data for tab {active_tab:?} - {keys:?}"));

    if selection.exceeded_limit || selection.selected_rows.is_empty() {
        return;
    }

    let selection_not_changed = selection.selected_rows.iter().all(|e| {
        let model_data = model
            .row_data(*e)
            .unwrap_or_else(|| panic!("Failed to get row data with id {}, with model {} items", e, model.row_count()));
        model_data.focused_row
    });

    if selection_not_changed {
        return;
    }

    selection.selected_rows = model.iter().enumerate().filter_map(|(idx, e)| if e.focused_row { Some(idx) } else { None }).collect();
}

pub(crate) fn initialize_selection_struct() {
    let tools: [ActiveTab; TOOLS_NUMBER] = [
        ActiveTab::DuplicateFiles,
        ActiveTab::EmptyFolders,
        ActiveTab::BigFiles,
        ActiveTab::EmptyFiles,
        ActiveTab::TemporaryFiles,
        ActiveTab::SimilarImages,
        ActiveTab::SimilarVideos,
        ActiveTab::SimilarMusic,
        ActiveTab::InvalidSymlinks,
        ActiveTab::BrokenFiles,
        ActiveTab::BadExtensions,
        ActiveTab::BadNames,
        ActiveTab::ExifRemover,
        ActiveTab::VideoOptimizer,
    ];

    let map: HashMap<_, _> = tools.into_iter().map(|tool| (tool, SelectionData::default())).collect();
    let mut tool = TOOLS_SELECTION.write().expect("Failed to get write selection lock");
    if !cfg!(test) {
        let data = mem::replace(&mut *tool, map);
        assert!(data.is_empty(), "Selection data is already initialized, but it should be empty");
    } else {
        let _ = mem::replace(&mut *tool, map);
    }
}

// fn get_read_selection_lock() -> RwLockReadGuard<'static, HashMap<ActiveTab, SelectionData>> {
//     let selection = TOOLS_SELECTION.get().expect("Selection data is not initialized");
//     selection.read().expect("Failed to lock selection data")
// }
fn get_write_selection_lock() -> RwLockWriteGuard<'static, HashMap<ActiveTab, SelectionData>> {
    TOOLS_SELECTION.write().expect("Selection data is not initialized or is poisoned")
}

impl Hash for ActiveTab {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as u8).hash(state);
    }
}
impl Eq for ActiveTab {}

////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
////////////////////
////////////////////

pub(crate) fn connect_row_selections(app: &MainWindow) {
    initialize_selection_struct();

    selection::connect_select_all_rows(app); // CTRL + A
    selection::reverse_single_unique_item(app); // LMB
    selection::reverse_checked_on_selection(app); // Space
    selection::reverse_selection_on_specific_item(app); // CTRL + LMB
    selection::select_items_with_shift(app); // SHIFT + LMB
    opener::open_provided_item(app);
    opener::open_provided_parent_item(app);
    opener::connect_on_open_item(app);
    checker::change_number_of_checked_items(app);
    context_menu::connect_context_menu_actions(app);
}

mod opener {
    use super::{Callabler, ComponentHandle, GuiState, MainWindow, Model, error};

    pub(crate) fn connect_on_open_item(app: &MainWindow) {
        app.global::<Callabler>().on_open_item(move |path| {
            open_item_simple(path.as_str());
        });
        app.global::<Callabler>().on_open_parent(move |path| {
            let Some(parent_path) = std::path::Path::new(&path).parent() else {
                return error!("Failed to get parent path for \"{path}\"");
            };
            open_item_simple(&parent_path.to_string_lossy());
        });
    }

    fn open_item_simple(path_to_open: &str) {
        if let Err(e) = open::that(path_to_open) {
            error!("Failed to open file: {e}");
        }
    }

    fn open_item(app: &MainWindow, items_path_str: &[usize], id: usize) {
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(app);
        let model_data = model
            .row_data(id)
            .unwrap_or_else(|| panic!("Failed to get row data with id {id}, with model {} items", model.row_count()));

        let get_debug_crash_data = || {
            format!(
                "Model data str - {} - cannot find path/name at index/es - {:?}, active tab - {active_tab:?}",
                model_data.val_str.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "),
                items_path_str
            )
        };

        let path_to_open = if items_path_str.len() == 1 {
            format!(
                "{}",
                model_data.val_str.iter().nth(items_path_str[0]).unwrap_or_else(|| panic!("{}", get_debug_crash_data()))
            )
        } else {
            format!(
                "{}/{}",
                model_data.val_str.iter().nth(items_path_str[0]).unwrap_or_else(|| panic!("{}", get_debug_crash_data())),
                model_data.val_str.iter().nth(items_path_str[1]).unwrap_or_else(|| panic!("{}", get_debug_crash_data()))
            )
        };
        open_item_simple(&path_to_open);
    }

    pub(crate) fn open_provided_item(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_open_item_with_index(move |idx| {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();

            open_item(&app, &[active_tab.get_str_path_idx(), active_tab.get_str_name_idx()], idx as usize);
        });
    }

    pub(crate) fn open_provided_parent_item(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_open_parent_item_with_index(move |idx| {
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();

            open_item(&app, &[active_tab.get_str_path_idx()], idx as usize);
        });
    }
}
mod selection {
    use slint::ModelRc;

    use super::{
        Callabler, ComponentHandle, GuiState, MainWindow, Model, get_write_selection_lock, reverse_selection_of_item_with_id, row_select_items_with_shift,
        rows_deselect_all_by_mode, rows_reverse_checked_selection, rows_select_all_by_mode, trace,
    };
    use crate::SingleMainListModel;
    use crate::connect_row_selection::SelectionData;
    use crate::connect_row_selection::checker::change_number_of_enabled_items;

    fn validate_selection_and_model(selection: &SelectionData, model: &ModelRc<SingleMainListModel>) {
        assert!(
            selection.number_of_selected_rows == selection.selected_rows.len() || selection.exceeded_limit,
            "Number of selected rows {} should be equal to length of selected rows vector {} if not exceeded limit",
            selection.number_of_selected_rows,
            selection.selected_rows.len()
        );
        assert!(
            model.row_count() >= selection.number_of_selected_rows,
            "Number of model items {} should be bigger than number of selected items {}",
            model.row_count(),
            selection.number_of_selected_rows
        );
    }

    pub(crate) fn connect_select_all_rows(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_select_all(move || {
            trace!("Clicked select all");
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();

            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
            let model = active_tab.get_tool_model(&app);

            validate_selection_and_model(selection, &model);
            if let Some(new_model) = rows_select_all_by_mode(selection, &model) {
                validate_selection_and_model(selection, &new_model);
                active_tab.set_tool_model(&app, new_model);
            }
        });
    }

    pub(crate) fn reverse_single_unique_item(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_reverse_single_unique_item(move |id| {
            trace!("Clicked reverse single unique item");
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");

            {
                let model = active_tab.get_tool_model(&app);
                validate_selection_and_model(selection, &model);

                if let Some(new_model) = rows_deselect_all_by_mode(selection, &model) {
                    active_tab.set_tool_model(&app, new_model);
                }
            }

            // needs to get model again, because it could be replaced
            let model = active_tab.get_tool_model(&app);
            reverse_selection_of_item_with_id(selection, &model, id as usize);
            validate_selection_and_model(selection, &model);
        });
    }

    pub(crate) fn reverse_checked_on_selection(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_reverse_checked_selection(move || {
            trace!("Clicked reverse checked on selection");
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
            let model = active_tab.get_tool_model(&app);

            validate_selection_and_model(selection, &model);

            let (checked_items, unchecked_items, new_model) = rows_reverse_checked_selection(selection, &model);
            if let Some(new_model) = new_model {
                active_tab.set_tool_model(&app, new_model);
            }

            change_number_of_enabled_items(&app, active_tab, checked_items as i64 - unchecked_items as i64);
            validate_selection_and_model(selection, &active_tab.get_tool_model(&app));
        });
    }
    pub(crate) fn reverse_selection_on_specific_item(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_reverse_item_selection(move |id| {
            trace!("Clicked reverse selection on specific item");
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
            let model = active_tab.get_tool_model(&app);
            validate_selection_and_model(selection, &model);
            reverse_selection_of_item_with_id(selection, &model, id as usize);
            validate_selection_and_model(selection, &model);
        });
    }

    pub(crate) fn select_items_with_shift(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_select_items_with_shift(move |first_idx, second_idx| {
            trace!("Clicked select items with shift");
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let mut lock = get_write_selection_lock();
            let selection = lock.get_mut(&active_tab).expect("Failed to get selection data");
            let model = active_tab.get_tool_model(&app);

            assert!(first_idx >= 0);
            assert!(second_idx >= 0);
            assert!((first_idx as usize) < model.row_count());
            assert!((second_idx as usize) < model.row_count());

            validate_selection_and_model(selection, &model);
            if let Some(new_model) = row_select_items_with_shift(selection, &model, (first_idx as usize, second_idx as usize)) {
                validate_selection_and_model(selection, &new_model);
                active_tab.set_tool_model(&app, new_model);
            }
        });
    }
}

pub(crate) mod checker {
    use super::{ActiveTab, Callabler, ComponentHandle, GuiState, MainWindow, connect_i32_into_u64, split_u64_into_i32s, trace};

    pub(crate) fn change_number_of_checked_items(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_change_number_of_checked_items(move |number_of_changed_items| {
            trace!("Changing number of checked items with {number_of_changed_items}");
            let app = a.upgrade().expect("Failed to upgrade app :(");
            let active_tab = app.global::<GuiState>().get_active_tab();
            change_number_of_enabled_items(&app, active_tab, number_of_changed_items as i64);
        });
    }

    // TODO - sad day for code readability, because slint not supports i64 - https://github.com/slint-ui/slint/issues/6589
    pub(crate) fn set_number_of_enabled_items(app: &MainWindow, active_tab: ActiveTab, items_number: u64) {
        let (it1, it2) = split_u64_into_i32s(items_number);
        match active_tab {
            ActiveTab::DuplicateFiles => {
                app.global::<GuiState>().set_selected_results_duplicates(it1);
                app.global::<GuiState>().set_selected_results_duplicates2(it2);
            }
            ActiveTab::EmptyFolders => {
                app.global::<GuiState>().set_selected_results_empty_folders(it1);
                app.global::<GuiState>().set_selected_results_empty_folders2(it2);
            }
            ActiveTab::BigFiles => {
                app.global::<GuiState>().set_selected_results_big_files(it1);
                app.global::<GuiState>().set_selected_results_big_files2(it2);
            }
            ActiveTab::EmptyFiles => {
                app.global::<GuiState>().set_selected_results_empty_files(it1);
                app.global::<GuiState>().set_selected_results_empty_files2(it2);
            }
            ActiveTab::TemporaryFiles => {
                app.global::<GuiState>().set_selected_results_temporary_files(it1);
                app.global::<GuiState>().set_selected_results_temporary_files2(it2);
            }
            ActiveTab::SimilarImages => {
                app.global::<GuiState>().set_selected_results_similar_images(it1);
                app.global::<GuiState>().set_selected_results_similar_images2(it2);
            }
            ActiveTab::SimilarVideos => {
                app.global::<GuiState>().set_selected_results_similar_videos(it1);
                app.global::<GuiState>().set_selected_results_similar_videos2(it2);
            }
            ActiveTab::SimilarMusic => {
                app.global::<GuiState>().set_selected_results_similar_music(it1);
                app.global::<GuiState>().set_selected_results_similar_music2(it2);
            }
            ActiveTab::InvalidSymlinks => {
                app.global::<GuiState>().set_selected_results_invalid_symlinks(it1);
                app.global::<GuiState>().set_selected_results_invalid_symlinks2(it2);
            }
            ActiveTab::BrokenFiles => {
                app.global::<GuiState>().set_selected_results_broken_files(it1);
                app.global::<GuiState>().set_selected_results_broken_files2(it2);
            }
            ActiveTab::BadExtensions => {
                app.global::<GuiState>().set_selected_results_bad_extensions(it1);
                app.global::<GuiState>().set_selected_results_bad_extensions2(it2);
            }
            ActiveTab::BadNames => {
                app.global::<GuiState>().set_selected_results_bad_names(it1);
                app.global::<GuiState>().set_selected_results_bad_names2(it2);
            }
            ActiveTab::ExifRemover => {
                app.global::<GuiState>().set_selected_results_exif_remover(it1);
                app.global::<GuiState>().set_selected_results_exif_remover2(it2);
            }
            ActiveTab::VideoOptimizer => {
                app.global::<GuiState>().set_selected_results_video_optimizer(it1);
                app.global::<GuiState>().set_selected_results_video_optimizer2(it2);
            }
            _ => unreachable!("Current tab is not a tool that has enabled items"),
        }
    }

    pub(crate) fn change_number_of_enabled_items(app: &MainWindow, active_tab: ActiveTab, additions: i64) {
        let before_number_of_items = get_number_of_enabled_items(app, active_tab);
        let after_number_of_items = before_number_of_items.checked_add_signed(additions).unwrap_or_else(|| {
            panic!("Counter desync: before_number_of_items = {before_number_of_items}, additions = {additions}, tab = {active_tab:?}");
        });
        set_number_of_enabled_items(app, active_tab, after_number_of_items);
    }

    pub(crate) fn get_number_of_enabled_items(app: &MainWindow, active_tab: ActiveTab) -> u64 {
        let (it1, it2) = match active_tab {
            ActiveTab::DuplicateFiles => (
                app.global::<GuiState>().get_selected_results_duplicates(),
                app.global::<GuiState>().get_selected_results_duplicates2(),
            ),
            ActiveTab::EmptyFolders => (
                app.global::<GuiState>().get_selected_results_empty_folders(),
                app.global::<GuiState>().get_selected_results_empty_folders2(),
            ),
            ActiveTab::BigFiles => (
                app.global::<GuiState>().get_selected_results_big_files(),
                app.global::<GuiState>().get_selected_results_big_files2(),
            ),
            ActiveTab::EmptyFiles => (
                app.global::<GuiState>().get_selected_results_empty_files(),
                app.global::<GuiState>().get_selected_results_empty_files2(),
            ),
            ActiveTab::TemporaryFiles => (
                app.global::<GuiState>().get_selected_results_temporary_files(),
                app.global::<GuiState>().get_selected_results_temporary_files2(),
            ),
            ActiveTab::SimilarImages => (
                app.global::<GuiState>().get_selected_results_similar_images(),
                app.global::<GuiState>().get_selected_results_similar_images2(),
            ),
            ActiveTab::SimilarVideos => (
                app.global::<GuiState>().get_selected_results_similar_videos(),
                app.global::<GuiState>().get_selected_results_similar_videos2(),
            ),
            ActiveTab::SimilarMusic => (
                app.global::<GuiState>().get_selected_results_similar_music(),
                app.global::<GuiState>().get_selected_results_similar_music2(),
            ),
            ActiveTab::InvalidSymlinks => (
                app.global::<GuiState>().get_selected_results_invalid_symlinks(),
                app.global::<GuiState>().get_selected_results_invalid_symlinks2(),
            ),
            ActiveTab::BrokenFiles => (
                app.global::<GuiState>().get_selected_results_broken_files(),
                app.global::<GuiState>().get_selected_results_broken_files2(),
            ),
            ActiveTab::BadExtensions => (
                app.global::<GuiState>().get_selected_results_bad_extensions(),
                app.global::<GuiState>().get_selected_results_bad_extensions2(),
            ),
            ActiveTab::BadNames => (
                app.global::<GuiState>().get_selected_results_bad_names(),
                app.global::<GuiState>().get_selected_results_bad_names2(),
            ),
            ActiveTab::ExifRemover => (
                app.global::<GuiState>().get_selected_results_exif_remover(),
                app.global::<GuiState>().get_selected_results_exif_remover2(),
            ),
            ActiveTab::VideoOptimizer => (
                app.global::<GuiState>().get_selected_results_video_optimizer(),
                app.global::<GuiState>().get_selected_results_video_optimizer2(),
            ),
            _ => unreachable!("Current tab is not a tool that has enabled items"),
        };
        connect_i32_into_u64(it1, it2)
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

//
// Deselect
//

fn rows_deselect_all_by_mode(selection: &mut SelectionData, model: &ModelRc<SingleMainListModel>) -> Option<ModelRc<SingleMainListModel>> {
    let new_model = if selection.exceeded_limit {
        Some(rows_deselect_all_selected_by_replacing_models(model))
    } else if !selection.selected_rows.is_empty() {
        rows_deselect_all_selected_one_by_one(model, selection);
        None
    } else {
        assert_ne!(model.row_count(), 0);
        None
    };

    selection.selected_rows.clear();
    selection.exceeded_limit = false;
    selection.number_of_selected_rows = 0;

    new_model
}

fn rows_deselect_all_selected_one_by_one(model: &ModelRc<SingleMainListModel>, selection: &SelectionData) {
    for id in &selection.selected_rows {
        let mut model_data = model
            .row_data(*id)
            .unwrap_or_else(|| panic!("Failed to get row data with id {id}, with model {} items", model.row_count()));
        assert!(model_data.focused_row);
        model_data.focused_row = false;
        model.set_row_data(*id, model_data);
    }
}

fn rows_deselect_all_selected_by_replacing_models(model: &ModelRc<SingleMainListModel>) -> ModelRc<SingleMainListModel> {
    let new_model = model
        .iter()
        .map(|mut row| {
            row.focused_row = false;
            row
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(new_model))
}

//
// Select All
//
fn rows_select_all_by_mode(selection: &mut SelectionData, model: &ModelRc<SingleMainListModel>) -> Option<ModelRc<SingleMainListModel>> {
    assert!(
        model.row_count() >= selection.number_of_selected_rows,
        "Number of model items {} should be bigger than number of selected items {}",
        model.row_count(),
        selection.number_of_selected_rows
    );
    let new_model = if model.row_count() - selection.number_of_selected_rows > 100 {
        rows_select_all_by_replacing_models(selection, model)
    } else {
        rows_select_all_one_by_one(model);
        None
    };

    if model.row_count() > SELECTED_ROWS_LIMIT || selection.exceeded_limit {
        selection.exceeded_limit = true;
        selection.selected_rows.clear();
        selection.number_of_selected_rows = new_model.as_ref().unwrap_or(model).iter().filter(|e| e.focused_row).count();
    } else {
        selection.selected_rows = new_model
            .as_ref()
            .unwrap_or(model)
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| if item.focused_row { Some(idx) } else { None })
            .collect();
        selection.number_of_selected_rows = selection.selected_rows.len();
    }

    new_model
}

fn rows_select_all_one_by_one(model: &ModelRc<SingleMainListModel>) {
    let items_to_update = model.iter().filter(|e| !e.focused_row && !e.header_row).count();
    trace!("[FAST][ONE_BY_ONE] select all {}/{} items", items_to_update, model.row_count());
    for id in 0..model.row_count() {
        let mut model_data = model
            .row_data(id)
            .unwrap_or_else(|| panic!("Failed to get row data with id {id}, with model {} items", model.row_count()));

        if model_data.header_row {
            continue;
        }

        if model_data.focused_row {
            continue;
        }

        model_data.focused_row = true;
        model.set_row_data(id, model_data);
    }
}

fn rows_select_all_by_replacing_models(selection: &SelectionData, model: &ModelRc<SingleMainListModel>) -> Option<ModelRc<SingleMainListModel>> {
    // May happen with simple models, but for more advanced with header rows, we need something like "selection.all_items_selected"
    if selection.number_of_selected_rows == model.row_count() {
        trace!(
            "[SLOW][REPLACE_MODEL], but no need to replace it - {} items both exists and selected",
            selection.number_of_selected_rows
        );
        return None;
    }
    trace!("[SLOW][REPLACE_MODEL] select all {} items", model.row_count());

    let new_model = model
        .iter()
        .map(|mut row| {
            row.focused_row = !row.header_row;
            row
        })
        .collect::<Vec<_>>();
    Some(ModelRc::new(VecModel::from(new_model)))
}

//
// Reverse selection and selecting
//
fn reverse_selection_of_item_with_id(selection: &mut SelectionData, model: &ModelRc<SingleMainListModel>, id: usize) {
    let mut model_data = model
        .row_data(id)
        .unwrap_or_else(|| panic!("Failed to get row data with id {id}, with model {} items", model.row_count()));

    if model_data.header_row {
        assert!(!model_data.focused_row);
        return;
    }

    let was_selected = model_data.focused_row;
    model_data.focused_row = !model_data.focused_row;
    model.set_row_data(id, model_data);

    if was_selected {
        assert!(selection.number_of_selected_rows > 0);
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

fn row_select_items_with_shift(selection: &mut SelectionData, model: &ModelRc<SingleMainListModel>, indexes: (usize, usize)) -> Option<ModelRc<SingleMainListModel>> {
    let (smaller_idx, bigger_idx) = if indexes.0 < indexes.1 { (indexes.0, indexes.1) } else { (indexes.1, indexes.0) };

    if bigger_idx - smaller_idx > SELECTED_ROWS_LIMIT || selection.exceeded_limit {
        trace!("[SLOW][REPLACE_MODEL] selecting from {} items", model.row_count());
        // To not iterate twice over the same model, which may be slow, we check if we exceeded limit
        // This may not be 100% correct, because we may select only 501 items and 500 headers
        // But gains are bigger than selecting
        selection.exceeded_limit = bigger_idx - smaller_idx > SELECTED_ROWS_LIMIT;
        selection.selected_rows.clear();
        selection.number_of_selected_rows = 0;

        let new_model: Vec<_> = model
            .iter()
            .enumerate()
            .map(|(idx, mut row)| {
                row.focused_row = !row.header_row && (smaller_idx..=bigger_idx).contains(&idx);
                if row.focused_row {
                    selection.number_of_selected_rows += 1;
                    if !selection.exceeded_limit {
                        selection.selected_rows.push(idx);
                    }
                }
                row
            })
            .collect();

        Some(ModelRc::new(VecModel::from(new_model)))
    } else {
        trace!(
            "[FAST][ONE_BY_ONE] deselecting {} items, and later selecting, maybe {}/{} items",
            selection.selected_rows.len(),
            bigger_idx - smaller_idx,
            model.row_count()
        );
        // Deselect all previously selected rows, that are not in the range
        for idx in &selection.selected_rows {
            if !(smaller_idx..=bigger_idx).contains(idx) {
                let mut model_data = model
                    .row_data(*idx)
                    .unwrap_or_else(|| panic!("Failed to get row data with id {idx}, with model {} items", model.row_count()));
                assert!(model_data.focused_row); // Probably can be removed in future
                model_data.focused_row = false;
                model.set_row_data(*idx, model_data);
            }
        }

        // select new rows
        selection.number_of_selected_rows = 0;
        selection.selected_rows.clear();
        selection.exceeded_limit = false;

        for idx in smaller_idx..=bigger_idx {
            let mut model_data = model
                .row_data(idx)
                .unwrap_or_else(|| panic!("Failed to get row data with id {idx}, with model {} items", model.row_count()));

            // Every item in range is selected
            // We don't set this in if below, because this doesn't take in to account,
            // already selected items, that we don't deselect in above for loop
            if !model_data.header_row {
                selection.selected_rows.push(idx);
                selection.number_of_selected_rows += 1;
            }

            if !model_data.focused_row && !model_data.header_row {
                model_data.focused_row = true;
                model.set_row_data(idx, model_data);
            }
        }

        None
    }
}

fn rows_reverse_checked_selection(selection: &SelectionData, model: &ModelRc<SingleMainListModel>) -> (u64, u64, Option<ModelRc<SingleMainListModel>>) {
    let (mut checked_items, mut unchecked_items) = (0, 0);

    if selection.exceeded_limit {
        trace!("[SLOW][REPLACE_MODEL] reverse checked selection(SPACE)");
        let new_model = model
            .iter()
            .map(|mut row| {
                if row.focused_row {
                    assert!(!row.header_row); // Header row should not be selected
                    row.checked = !row.checked;
                    if row.checked {
                        checked_items += 1;
                    } else {
                        unchecked_items += 1;
                    }
                }
                row
            })
            .collect::<Vec<_>>();
        return (checked_items, unchecked_items, Some(ModelRc::new(VecModel::from(new_model))));
    } else if !selection.selected_rows.is_empty() {
        trace!("[FAST][ONE_BY_ONE] reverse selection(SPACE)");
        for id in &selection.selected_rows {
            let mut model_data = model
                .row_data(*id)
                .unwrap_or_else(|| panic!("Failed to get row data with id {id}, with model {} items", model.row_count()));
            assert!(model_data.focused_row);
            assert!(!model_data.header_row);
            model_data.checked = !model_data.checked;
            if model_data.checked {
                checked_items += 1;
            } else {
                unchecked_items += 1;
            }
            model.set_row_data(*id, model_data);
        }
    }
    (checked_items, unchecked_items, None)
}

mod context_menu {
    use log::warn;
    use slint::{ComponentHandle, Model, ModelRc, VecModel};

    use super::reset_selection;
    use crate::connect_directories_changes::add_excluded_paths;
    use crate::connect_row_selection::checker::set_number_of_enabled_items;
    use crate::model_operations::remove_single_items_in_groups;
    use crate::{Callabler, GuiState, MainWindow, Settings, SingleMainListModel};

    pub(crate) fn connect_context_menu_actions(app: &MainWindow) {
        connect_remove_from_results(app);
        connect_remove_all_from_folder(app);
        connect_remove_all_from_folder_recursive(app);
        connect_select_all_from_folder(app);
        connect_select_all_from_folder_recursive(app);
        connect_exclude_parent_folder(app);
        connect_exclude_item(app);
        connect_copy_file_name(app);
        connect_copy_parent_folder_path(app);
        connect_copy_full_path(app);
    }

    fn connect_remove_from_results(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_remove_from_results(move |idx| {
            let app = a.upgrade().expect("Failed to upgrade app");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let model = active_tab.get_tool_model(&app);
            let idx = idx as usize;

            let Some(row) = model.row_data(idx) else { return };
            if row.header_row {
                return;
            }

            let new_items: Vec<SingleMainListModel> = model.iter().enumerate().filter_map(|(i, r)| if i == idx { None } else { Some(r) }).collect();
            let cleaned = remove_single_items_in_groups(new_items, active_tab.get_is_header_mode());
            active_tab.set_tool_model(&app, ModelRc::new(VecModel::from(cleaned)));
            reset_selection(&app, active_tab, true);
        });
    }

    fn connect_remove_all_from_folder(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_remove_all_from_folder(move |idx| {
            remove_all_from_folder_impl(&a.upgrade().expect("Failed to upgrade app"), idx as usize, false);
        });
    }

    fn connect_remove_all_from_folder_recursive(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_remove_all_from_folder_recursive(move |idx| {
            remove_all_from_folder_impl(&a.upgrade().expect("Failed to upgrade app"), idx as usize, true);
        });
    }

    fn remove_all_from_folder_impl(app: &MainWindow, idx: usize, recursive: bool) {
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(app);
        let path_idx = active_tab.get_str_path_idx();

        let Some(clicked_row) = model.row_data(idx) else { return };
        if clicked_row.header_row {
            return;
        }
        let target_path = clicked_row.val_str.iter().nth(path_idx).map(|s| s.to_string()).unwrap_or_default();
        let target_prefix = format!("{target_path}{}", std::path::MAIN_SEPARATOR);

        let mut in_reference_group = false;
        let new_items: Vec<SingleMainListModel> = model
            .iter()
            .filter(|row| {
                if row.header_row {
                    in_reference_group = row.filled_header_row;
                    true
                } else if in_reference_group {
                    true // never remove items from a reference-folder group
                } else {
                    let p = row.val_str.iter().nth(path_idx).map(|s| s.as_str().to_owned()).unwrap_or_default();
                    if recursive {
                        p != target_path && !p.starts_with(&target_prefix)
                    } else {
                        p != target_path
                    }
                }
            })
            .collect();
        let cleaned = remove_single_items_in_groups(new_items, active_tab.get_is_header_mode());
        active_tab.set_tool_model(app, ModelRc::new(VecModel::from(cleaned)));
        reset_selection(app, active_tab, true);
    }

    fn connect_select_all_from_folder(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_select_all_from_folder(move |idx| {
            select_all_from_folder_impl(&a.upgrade().expect("Failed to upgrade app"), idx as usize, false);
        });
    }

    fn connect_select_all_from_folder_recursive(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_select_all_from_folder_recursive(move |idx| {
            select_all_from_folder_impl(&a.upgrade().expect("Failed to upgrade app"), idx as usize, true);
        });
    }

    fn select_all_from_folder_impl(app: &MainWindow, idx: usize, recursive: bool) {
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = active_tab.get_tool_model(app);
        let path_idx = active_tab.get_str_path_idx();
        let is_header_mode = active_tab.get_is_header_mode();

        let Some(clicked_row) = model.row_data(idx) else { return };
        if clicked_row.header_row {
            return;
        }
        let target_path = clicked_row.val_str.iter().nth(path_idx).map(|s| s.to_string()).unwrap_or_default();
        let target_prefix = format!("{target_path}{}", std::path::MAIN_SEPARATOR);

        let items: Vec<SingleMainListModel> = model.iter().collect();
        let n = items.len();

        let path_matches = |i: usize| -> bool {
            if items[i].header_row {
                return false;
            }
            let p = items[i].val_str.iter().nth(path_idx).map(|s| s.as_str().to_owned()).unwrap_or_default();
            if recursive {
                p == target_path || p.starts_with(&target_prefix)
            } else {
                p == target_path
            }
        };

        // should_check[i] = true  → set checked=true for that item (never uncheck)
        let mut should_check = vec![false; n];

        if !is_header_mode {
            // Flat list: simply check all matching items.
            for i in 0..n {
                if path_matches(i) && !items[i].checked {
                    should_check[i] = true;
                }
            }
        } else {
            // Group-aware: iterate group by group.
            let mut i = 0;
            while i < n {
                if !items[i].header_row {
                    i += 1;
                    continue;
                }
                let is_reference = items[i].filled_header_row;
                i += 1; // skip the header row

                let group_start = i;
                while i < n && !items[i].header_row {
                    i += 1;
                }
                let group_end = i; // exclusive
                let total_in_group = group_end - group_start;
                if total_in_group == 0 {
                    continue;
                }

                let currently_checked = (group_start..group_end).filter(|&j| items[j].checked).count();
                let matching_unchecked: Vec<usize> = (group_start..group_end).filter(|&j| path_matches(j) && !items[j].checked).collect();
                let would_be_checked = currently_checked + matching_unchecked.len();

                if is_reference {
                    // The reference header itself is permanently unchecked, so there is always
                    // at least one "uncheckable" item in the group → no restriction needed.
                    for j in matching_unchecked {
                        should_check[j] = true;
                    }
                } else if would_be_checked == total_in_group && currently_checked < total_in_group {
                    // Checking all matching items would make the whole group checked, and the
                    // group is not already full → check all but skip the last matching unchecked
                    // so one item remains unchecked.
                    let count = matching_unchecked.len();
                    if count > 1 {
                        for &j in &matching_unchecked[..count - 1] {
                            should_check[j] = true;
                        }
                    }
                    // count == 1: checking that single item would fill the group → skip it.
                    // count == 0: nothing to check (impossible to reach here, but harmless).
                } else {
                    // Either the group won't be fully checked, or it is already fully checked
                    // (currently_checked == total_in_group → matching_unchecked is empty).
                    // In both cases just check all matching unchecked items.
                    for j in matching_unchecked {
                        should_check[j] = true;
                    }
                }
            }
        }

        let new_items: Vec<SingleMainListModel> = items
            .into_iter()
            .enumerate()
            .map(|(i, mut row)| {
                if should_check[i] {
                    row.checked = true;
                }
                row.focused_row = false; // keep in sync with TOOLS_SELECTION reset below
                row
            })
            .collect();

        let checked_count = new_items.iter().filter(|r| r.checked).count() as u64;
        active_tab.set_tool_model(app, ModelRc::new(VecModel::from(new_items)));
        reset_selection(app, active_tab, true);
        set_number_of_enabled_items(app, active_tab, checked_count);
    }

    fn connect_exclude_parent_folder(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_exclude_parent_folder(move |idx| {
            let app = a.upgrade().expect("Failed to upgrade app");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let model = active_tab.get_tool_model(&app);
            let path_idx = active_tab.get_str_path_idx();

            let row = model
                .row_data(idx as usize)
                .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
            if row.header_row {
                return;
            }
            let path = row
                .val_str
                .iter()
                .nth(path_idx)
                .unwrap_or_else(|| panic!("path_idx={path_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
                .to_string();
            add_excluded_paths(&app.global::<Settings>(), std::slice::from_ref(&path));

            // Also remove matching rows from results, keeping reference-group items.
            let mut in_reference_group = false;
            let new_items: Vec<SingleMainListModel> = model
                .iter()
                .filter(|r| {
                    if r.header_row {
                        in_reference_group = r.filled_header_row;
                        true
                    } else if in_reference_group {
                        true
                    } else {
                        r.val_str.iter().nth(path_idx).is_none_or(|p| p.as_str() != path)
                    }
                })
                .collect();
            let cleaned = remove_single_items_in_groups(new_items, active_tab.get_is_header_mode());
            active_tab.set_tool_model(&app, ModelRc::new(VecModel::from(cleaned)));
            reset_selection(&app, active_tab, true);
        });
    }

    fn connect_exclude_item(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_exclude_item(move |idx| {
            let app = a.upgrade().expect("Failed to upgrade app");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let model = active_tab.get_tool_model(&app);
            let path_idx = active_tab.get_str_path_idx();
            let name_idx = active_tab.get_str_name_idx();
            let idx = idx as usize;

            let row = model
                .row_data(idx)
                .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
            if row.header_row {
                return;
            }
            let path = row
                .val_str
                .iter()
                .nth(path_idx)
                .unwrap_or_else(|| panic!("path_idx={path_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
                .to_string();
            let name = row
                .val_str
                .iter()
                .nth(name_idx)
                .unwrap_or_else(|| panic!("name_idx={name_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
                .to_string();
            let full_path = std::path::PathBuf::from(&path).join(&name).to_string_lossy().to_string();
            add_excluded_paths(&app.global::<Settings>(), &[full_path]);

            // Remove the specific item from results.
            let new_items: Vec<SingleMainListModel> = model.iter().enumerate().filter_map(|(i, r)| if i == idx { None } else { Some(r) }).collect();
            let cleaned = remove_single_items_in_groups(new_items, active_tab.get_is_header_mode());
            active_tab.set_tool_model(&app, ModelRc::new(VecModel::from(cleaned)));
            reset_selection(&app, active_tab, true);
        });
    }

    fn set_clipboard(text: String) {
        // X11/Wayland clipboard ownership must be held until another app reads the content.
        // SetExtLinux::wait() blocks the thread until that happens, so we run it in the background.
        std::thread::spawn(move || {
            use arboard::Clipboard;
            #[cfg(target_os = "linux")]
            use arboard::SetExtLinux as _;
            match Clipboard::new() {
                Ok(mut ctx) => {
                    #[cfg(target_os = "linux")]
                    let result = ctx.set().wait().text(text);
                    #[cfg(not(target_os = "linux"))]
                    let result = ctx.set_text(text);
                    if let Err(e) = result {
                        warn!("Failed to set clipboard contents: {e}");
                    }
                }
                Err(e) => warn!("Failed to create clipboard context: {e}"),
            }
        });
    }

    fn connect_copy_file_name(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_copy_file_name(move |idx| {
            let app = a.upgrade().expect("Failed to upgrade app");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let model = active_tab.get_tool_model(&app);
            let name_idx = active_tab.get_str_name_idx();

            let row = model
                .row_data(idx as usize)
                .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
            if row.header_row {
                return;
            }
            let name = row
                .val_str
                .iter()
                .nth(name_idx)
                .unwrap_or_else(|| panic!("name_idx={name_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
                .to_string();
            set_clipboard(name);
        });
    }

    fn connect_copy_parent_folder_path(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_copy_parent_folder_path(move |idx| {
            let app = a.upgrade().expect("Failed to upgrade app");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let model = active_tab.get_tool_model(&app);
            let path_idx = active_tab.get_str_path_idx();

            let row = model
                .row_data(idx as usize)
                .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
            if row.header_row {
                return;
            }
            let path = row
                .val_str
                .iter()
                .nth(path_idx)
                .unwrap_or_else(|| panic!("path_idx={path_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
                .to_string();
            set_clipboard(path);
        });
    }

    fn connect_copy_full_path(app: &MainWindow) {
        let a = app.as_weak();
        app.global::<Callabler>().on_row_copy_full_path(move |idx| {
            let app = a.upgrade().expect("Failed to upgrade app");
            let active_tab = app.global::<GuiState>().get_active_tab();
            let model = active_tab.get_tool_model(&app);
            let name_idx = active_tab.get_str_name_idx();
            let path_idx = active_tab.get_str_path_idx();

            let row = model
                .row_data(idx as usize)
                .unwrap_or_else(|| panic!("Row idx={idx} out of bounds (row_count={})", model.row_count()));
            if row.header_row {
                return;
            }
            let name = row
                .val_str
                .iter()
                .nth(name_idx)
                .unwrap_or_else(|| panic!("name_idx={name_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
                .to_string();
            let path = row
                .val_str
                .iter()
                .nth(path_idx)
                .unwrap_or_else(|| panic!("path_idx={path_idx} out of bounds, full val_str={:?}", row.val_str.iter().collect::<Vec<_>>()))
                .to_string();
            let full_path = if path.is_empty() { name } else { format!("{path}/{name}") };
            set_clipboard(full_path);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::create_model_from_model_vec;
    use crate::test_common::get_model_vec;

    #[test]
    fn rows_deselect_all_by_mode_with_exceeded_limit() {
        let mut model = get_model_vec(3);
        model[0].focused_row = true;
        model[1].focused_row = true;
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 2,
            selected_rows: vec![0, 1],
            exceeded_limit: true,
        };

        let new_model = rows_deselect_all_by_mode(&mut selection, &model);

        assert!(new_model.is_some());
        let new_model = new_model.unwrap();
        assert!(!new_model.row_data(0).unwrap().focused_row);
        assert!(!new_model.row_data(1).unwrap().focused_row);
        assert!(!new_model.row_data(2).unwrap().focused_row);
        assert!(selection.selected_rows.is_empty());
        assert!(!selection.exceeded_limit);
        assert_eq!(selection.number_of_selected_rows, 0);
    }

    #[test]
    fn rows_deselect_all_by_mode_with_selected_rows() {
        let mut model = get_model_vec(3);
        model[0].focused_row = true;
        model[1].focused_row = true;
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 2,
            selected_rows: vec![0, 1],
            exceeded_limit: false,
        };

        let new_model = rows_deselect_all_by_mode(&mut selection, &model);

        assert!(new_model.is_none());
        assert!(!model.row_data(0).unwrap().focused_row);
        assert!(!model.row_data(1).unwrap().focused_row);
        assert!(!model.row_data(2).unwrap().focused_row);
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
            selected_rows: Vec::new(),
            exceeded_limit: false,
        };

        let new_model = rows_deselect_all_by_mode(&mut selection, &model);

        assert!(new_model.is_none());
        assert!(!model.row_data(0).unwrap().focused_row);
        assert!(!model.row_data(1).unwrap().focused_row);
        assert!(!model.row_data(2).unwrap().focused_row);
        assert!(selection.selected_rows.is_empty());
        assert!(!selection.exceeded_limit);
        assert_eq!(selection.number_of_selected_rows, 0);
    }

    #[test]
    fn rows_select_all_by_mode_with_few_selected_rows() {
        let mut model = get_model_vec(3);
        model[0].focused_row = true;

        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 1,
            selected_rows: vec![0],
            exceeded_limit: false,
        };

        let new_model = rows_select_all_by_mode(&mut selection, &model);

        assert!(new_model.is_none());
        assert!(model.row_data(0).unwrap().focused_row);
        assert!(model.row_data(1).unwrap().focused_row);
        assert!(model.row_data(2).unwrap().focused_row);
        assert_eq!(selection.selected_rows, vec![0, 1, 2]);
        assert!(!selection.exceeded_limit);
        assert_eq!(selection.number_of_selected_rows, 3);
    }

    #[test]
    fn rows_select_all_by_mode_with_header_rows() {
        let mut model = get_model_vec(5);
        model[0].header_row = true;
        model[3].header_row = true;
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 0,
            selected_rows: Vec::new(),
            exceeded_limit: false,
        };

        let new_model = rows_select_all_by_mode(&mut selection, &model);

        assert!(new_model.is_none());
        assert!(!model.row_data(0).unwrap().focused_row); // header row
        assert!(model.row_data(1).unwrap().focused_row);
        assert!(model.row_data(2).unwrap().focused_row);
        assert!(!model.row_data(3).unwrap().focused_row); // header row
        assert!(model.row_data(4).unwrap().focused_row);
        assert_eq!(selection.selected_rows, vec![1, 2, 4]);
        assert!(!selection.exceeded_limit);
        assert_eq!(selection.number_of_selected_rows, 3);
    }

    #[test]
    fn rows_select_all_by_mode_with_exceeded_limit() {
        let mut model = get_model_vec(500);
        model[11].header_row = true;
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 0,
            selected_rows: Vec::new(),
            exceeded_limit: true,
        };

        let new_model = rows_select_all_by_mode(&mut selection, &model);

        assert!(new_model.is_some());
        let new_model = new_model.unwrap();
        for idx in 0..new_model.row_count() {
            if idx == 11 {
                assert!(!new_model.row_data(idx).unwrap().focused_row, "idx: {idx}");
            } else {
                assert!(new_model.row_data(idx).unwrap().focused_row, "idx: {idx}");
            }
        }

        assert!(selection.selected_rows.is_empty());
        assert!(selection.exceeded_limit);
        assert_eq!(selection.number_of_selected_rows, 499);
    }

    #[test]
    fn reverse_selection_of_item_with_id_select_item() {
        let model = get_model_vec(3);
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 0,
            selected_rows: Vec::new(),
            exceeded_limit: false,
        };

        reverse_selection_of_item_with_id(&mut selection, &model, 1);

        assert!(!model.row_data(0).unwrap().focused_row);
        assert!(model.row_data(1).unwrap().focused_row);
        assert!(!model.row_data(2).unwrap().focused_row);
        assert_eq!(selection.selected_rows, vec![1]);
        assert_eq!(selection.number_of_selected_rows, 1);
    }

    #[test]
    fn reverse_selection_of_item_with_id_deselect_item() {
        let mut model = get_model_vec(3);
        model[1].header_row = true;
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 1,
            selected_rows: vec![2],
            exceeded_limit: false,
        };

        reverse_selection_of_item_with_id(&mut selection, &model, 1);

        assert!(!model.row_data(1).unwrap().focused_row);
        assert_eq!(selection.selected_rows, vec![2]);
        assert_eq!(selection.number_of_selected_rows, 1);
    }
    #[test]
    fn row_select_items_with_shift_simple() {
        let model = get_model_vec(5);
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 0,
            selected_rows: Vec::new(),
            exceeded_limit: false,
        };

        let new_model = row_select_items_with_shift(&mut selection, &model, (1, 3));

        assert!(new_model.is_none());
        assert!(!model.row_data(0).unwrap().focused_row);
        assert!(model.row_data(1).unwrap().focused_row);
        assert!(model.row_data(2).unwrap().focused_row);
        assert!(model.row_data(3).unwrap().focused_row);
        assert!(!model.row_data(4).unwrap().focused_row);
        assert_eq!(selection.selected_rows, vec![1, 2, 3]);
        assert_eq!(selection.number_of_selected_rows, 3);
    }

    #[test]
    fn row_select_items_with_shift_with_header_rows() {
        let mut model = get_model_vec(5);
        model[1].header_row = true;
        model[3].header_row = true;
        let model = create_model_from_model_vec(&model);

        let mut selection = SelectionData {
            number_of_selected_rows: 0,
            selected_rows: Vec::new(),
            exceeded_limit: false,
        };

        let new_model = row_select_items_with_shift(&mut selection, &model, (0, 4));

        assert!(new_model.is_none());
        assert!(model.row_data(0).unwrap().focused_row);
        assert!(!model.row_data(1).unwrap().focused_row); // header row
        assert!(model.row_data(2).unwrap().focused_row);
        assert!(!model.row_data(3).unwrap().focused_row); // header row
        assert!(model.row_data(4).unwrap().focused_row);
        assert_eq!(selection.selected_rows, vec![0, 2, 4]);
        assert_eq!(selection.number_of_selected_rows, 3);
    }

    #[test]
    fn rows_reverse_checked_selection_with_selected_rows() {
        let mut model = get_model_vec(4);
        model[0].focused_row = true;
        model[1].focused_row = true;
        model[2].focused_row = true;
        model[2].checked = true;
        let model = create_model_from_model_vec(&model);

        let selection = SelectionData {
            number_of_selected_rows: 3,
            selected_rows: vec![0, 1, 2],
            exceeded_limit: false,
        };

        let (checked_items, unchecked_items, new_model) = rows_reverse_checked_selection(&selection, &model);

        assert!(new_model.is_none());
        assert!(model.row_data(0).unwrap().checked);
        assert!(model.row_data(1).unwrap().checked);
        assert!(!model.row_data(2).unwrap().checked);
        assert_eq!(checked_items, 2);
        assert_eq!(unchecked_items, 1);
    }

    #[test]
    fn rows_reverse_checked_selection_with_exceeded_limit() {
        let mut model = get_model_vec(3);
        model[0].focused_row = true;
        model[1].focused_row = true;
        let model = create_model_from_model_vec(&model);

        let selection = SelectionData {
            number_of_selected_rows: 2,
            selected_rows: Vec::new(),
            exceeded_limit: true,
        };

        let (checked_items, unchecked_items, new_model) = rows_reverse_checked_selection(&selection, &model);

        assert!(new_model.is_some());
        let new_model = new_model.unwrap();
        assert!(new_model.row_data(0).unwrap().checked);
        assert!(new_model.row_data(1).unwrap().checked);
        assert!(!new_model.row_data(2).unwrap().checked);
        assert_eq!(checked_items, 2);
        assert_eq!(unchecked_items, 0);
    }
}
