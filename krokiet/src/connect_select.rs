use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::{
    connect_i32_into_u64, get_int_height_idx, get_int_modification_date_idx, get_int_size_idx, get_int_width_idx, get_is_header_mode, get_tool_model, set_tool_model,
};
use crate::{Callabler, CurrentTab, GuiState, MainListModel, MainWindow, SelectMode, SelectModel};

// TODO optimize this, not sure if it is possible to not copy entire model to just select item
// https://github.com/slint-ui/slint/discussions/4595
pub fn connect_select(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_select_items(move |select_mode| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = get_tool_model(&app, active_tab);

        let new_model = match select_mode {
            SelectMode::SelectAll => select_all(current_model),
            SelectMode::UnselectAll => deselect_all(current_model),
            SelectMode::InvertSelection => invert_selection(current_model),
            SelectMode::SelectTheBiggestSize => select_by_size_date(current_model, active_tab, true, true),
            SelectMode::SelectTheSmallestSize => select_by_size_date(current_model, active_tab, false, true),
            SelectMode::SelectTheBiggestResolution => select_by_resolution(current_model, active_tab, true),
            SelectMode::SelectTheSmallestResolution => select_by_resolution(current_model, active_tab, false),
            SelectMode::SelectNewest => select_by_size_date(current_model, active_tab, true, false),
            SelectMode::SelectOldest => select_by_size_date(current_model, active_tab, false, false),
        };
        set_tool_model(&app, active_tab, new_model);
    });
}

pub fn connect_showing_proper_select_buttons(app: &MainWindow) {
    set_select_buttons(app);
    let a = app.as_weak();
    app.global::<Callabler>().on_tab_changed(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        set_select_buttons(&app);
    });
}

fn set_select_buttons(app: &MainWindow) {
    let active_tab = app.global::<GuiState>().get_active_tab();
    let mut base_buttons = vec![SelectMode::SelectAll, SelectMode::UnselectAll, SelectMode::InvertSelection];

    let additional_buttons = match active_tab {
        CurrentTab::DuplicateFiles | CurrentTab::SimilarVideos | CurrentTab::SimilarMusic => vec![
            SelectMode::SelectOldest,
            SelectMode::SelectNewest,
            SelectMode::SelectTheSmallestSize,
            SelectMode::SelectTheBiggestSize,
        ],
        CurrentTab::SimilarImages => vec![
            SelectMode::SelectOldest,
            SelectMode::SelectNewest,
            SelectMode::SelectTheSmallestSize,
            SelectMode::SelectTheBiggestSize,
            SelectMode::SelectTheSmallestResolution,
            SelectMode::SelectTheBiggestResolution,
        ],
        CurrentTab::EmptyFolders
        | CurrentTab::BigFiles
        | CurrentTab::EmptyFiles
        | CurrentTab::TemporaryFiles
        | CurrentTab::InvalidSymlinks
        | CurrentTab::BrokenFiles
        | CurrentTab::BadExtensions => vec![],
        CurrentTab::Settings | CurrentTab::About => vec![], // Not available in settings and about, so may be set any value here
    };

    base_buttons.extend(additional_buttons);
    base_buttons.reverse();

    let new_select_model = base_buttons
        .into_iter()
        .map(|e| SelectModel {
            name: translate_select_mode(e).into(),
            data: e,
        })
        .collect::<Vec<_>>();

    app.global::<GuiState>().set_select_results_list(ModelRc::new(VecModel::from(new_select_model)));
}

fn translate_select_mode(select_mode: SelectMode) -> String {
    match select_mode {
        SelectMode::SelectAll => "Select all".into(),
        SelectMode::UnselectAll => "Unselect all".into(),
        SelectMode::InvertSelection => "Invert selection".into(),
        SelectMode::SelectTheBiggestSize => "Select the biggest size".into(),
        SelectMode::SelectTheBiggestResolution => "Select the biggest resolution".into(),
        SelectMode::SelectTheSmallestSize => "Select the smallest size".into(),
        SelectMode::SelectTheSmallestResolution => "Select the smallest resolution".into(),
        SelectMode::SelectNewest => "Select newest".into(),
        SelectMode::SelectOldest => "Select oldest".into(),
    }
}

// TODO, when model will be able to contain i64 instead two i32, this function could be merged with select_by_size_date
fn select_by_resolution(model: ModelRc<MainListModel>, active_tab: CurrentTab, biggest: bool) -> ModelRc<MainListModel> {
    let is_header_mode = get_is_header_mode(active_tab);
    assert!(is_header_mode); // non header modes not really have reason to use this function

    let mut old_data = model.iter().collect::<Vec<_>>();
    let headers_idx = find_header_idx_and_deselect_all(&mut old_data);
    let width_idx = get_int_width_idx(active_tab);
    let height_idx = get_int_height_idx(active_tab);

    if biggest {
        for i in 0..(headers_idx.len() - 1) {
            let mut max_item = 0;
            let mut max_item_idx = 1;
            #[allow(clippy::needless_range_loop)]
            for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
                let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
                let item = int_data[width_idx] * int_data[height_idx];
                if item > max_item {
                    max_item = item;
                    max_item_idx = j;
                }
            }
            old_data[max_item_idx].checked = true;
        }
    } else {
        for i in 0..(headers_idx.len() - 1) {
            let mut min_item = u64::MAX;
            let mut min_item_idx = 1;
            #[allow(clippy::needless_range_loop)]
            for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
                let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
                let item = (int_data[width_idx] * int_data[height_idx]) as u64;
                if item < min_item {
                    min_item = item;
                    min_item_idx = j;
                }
            }
            old_data[min_item_idx].checked = true;
        }
    }

    ModelRc::new(VecModel::from(old_data))
}

fn select_by_size_date(model: ModelRc<MainListModel>, active_tab: CurrentTab, biggest_newest: bool, size: bool) -> ModelRc<MainListModel> {
    let is_header_mode = get_is_header_mode(active_tab);
    assert!(is_header_mode); // non header modes not really have reason to use this function

    let mut old_data = model.iter().collect::<Vec<_>>();
    let headers_idx = find_header_idx_and_deselect_all(&mut old_data);
    let item_idx = if size {
        get_int_size_idx(active_tab)
    } else {
        get_int_modification_date_idx(active_tab)
    };

    if biggest_newest {
        for i in 0..(headers_idx.len() - 1) {
            let mut max_item = 0;
            let mut max_item_idx = 1;
            #[allow(clippy::needless_range_loop)]
            for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
                let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
                let item = connect_i32_into_u64(int_data[item_idx], int_data[item_idx + 1]);
                if item > max_item {
                    max_item = item;
                    max_item_idx = j;
                }
            }
            old_data[max_item_idx].checked = true;
        }
    } else {
        for i in 0..(headers_idx.len() - 1) {
            let mut min_item = u64::MAX;
            let mut min_item_idx = 1;
            #[allow(clippy::needless_range_loop)]
            for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
                let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
                let item = connect_i32_into_u64(int_data[item_idx], int_data[item_idx + 1]);
                if item < min_item {
                    min_item = item;
                    min_item_idx = j;
                }
            }
            old_data[min_item_idx].checked = true;
        }
    }

    ModelRc::new(VecModel::from(old_data))
}

fn select_all(model: ModelRc<MainListModel>) -> ModelRc<MainListModel> {
    let mut old_data = model.iter().collect::<Vec<_>>();
    for x in &mut old_data {
        if !x.header_row {
            x.checked = true;
        }
    }
    ModelRc::new(VecModel::from(old_data))
}

fn deselect_all(model: ModelRc<MainListModel>) -> ModelRc<MainListModel> {
    let mut old_data = model.iter().collect::<Vec<_>>();
    old_data.iter_mut().for_each(|x| x.checked = false);
    ModelRc::new(VecModel::from(old_data))
}

fn invert_selection(model: ModelRc<MainListModel>) -> ModelRc<MainListModel> {
    let mut old_data = model.iter().collect::<Vec<_>>();
    for x in &mut old_data {
        if !x.header_row {
            x.checked = !x.checked;
        }
    }
    ModelRc::new(VecModel::from(old_data))
}

fn find_header_idx_and_deselect_all(old_data: &mut [MainListModel]) -> Vec<usize> {
    let mut header_idx = old_data
        .iter()
        .enumerate()
        .filter_map(|(idx, m)| if m.header_row { Some(idx) } else { None })
        .collect::<Vec<_>>();
    header_idx.push(old_data.len());

    for x in old_data.iter_mut() {
        if !x.header_row {
            x.checked = false;
        }
    }
    header_idx
}
