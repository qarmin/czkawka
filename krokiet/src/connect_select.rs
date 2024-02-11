use crate::common::{connect_i32_into_u64, get_int_size_idx, get_is_header_mode, get_tool_model, set_tool_model};
use crate::{Callabler, GuiState, MainListModel, MainWindow, SelectMode};
use crate::{CurrentTab, SelectModel};
use slint::{ComponentHandle, Model, ModelRc, VecModel};

// TODO optimize this, not sure if it is possible to not copy entire model to just select item
// https://github.com/slint-ui/slint/discussions/4595
pub fn connect_select(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_select_items(move |select_mode| {
        let app = a.upgrade().unwrap();
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = get_tool_model(&app, active_tab);

        let new_model = match select_mode {
            SelectMode::SelectAll => select_all(current_model),
            SelectMode::UnselectAll => deselect_all(current_model),
            SelectMode::InvertSelection => invert_selection(current_model),
            SelectMode::SelectTheBiggestSize => select_the_biggest_size(current_model, active_tab),
            SelectMode::SelectTheSmallestSize => select_the_small_size(current_model, active_tab),
            _ => unimplemented!(),
        };
        set_tool_model(&app, active_tab, new_model);
    });
}

pub fn connect_showing_proper_select_buttons(app: &MainWindow) {
    set_select_buttons(app);
    let a = app.as_weak();
    app.global::<Callabler>().on_tab_changed(move || {
        let app = a.upgrade().unwrap();
        set_select_buttons(&app);
    });
}

fn set_select_buttons(app: &MainWindow) {
    let active_tab = app.global::<GuiState>().get_active_tab();
    let mut base_buttons = vec![SelectMode::SelectAll, SelectMode::UnselectAll, SelectMode::InvertSelection];

    let additional_buttons = match active_tab {
        CurrentTab::SimilarImages => vec![
            SelectMode::SelectOldest,
            SelectMode::SelectNewest,
            SelectMode::SelectTheSmallestSize,
            SelectMode::SelectTheBiggestSize,
            SelectMode::SelectTheSmallestResolution,
            SelectMode::SelectTheBiggestResolution,
        ],
        _ => vec![],
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

fn select_the_biggest_size(model: ModelRc<MainListModel>, active_tab: CurrentTab) -> ModelRc<MainListModel> {
    let is_header_mode = get_is_header_mode(active_tab);
    assert!(is_header_mode); // non header modes not really have reasont to use this function

    let mut old_data = model.iter().collect::<Vec<_>>();
    let headers_idx = find_header_idx_and_deselect_all(&mut old_data);
    let size_idx = get_int_size_idx(active_tab);

    for i in 0..(headers_idx.len() - 1) {
        let mut max_size = 0;
        let mut max_size_idx = 0;
        for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
            let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
            let size = connect_i32_into_u64(int_data[size_idx], int_data[size_idx + 1]);
            if size > max_size {
                max_size = size;
                max_size_idx = j;
            }
        }
        old_data[max_size_idx].checked = true;
    }

    ModelRc::new(VecModel::from(old_data))
}

fn select_the_small_size(model: ModelRc<MainListModel>, active_tab: CurrentTab) -> ModelRc<MainListModel> {
    let is_header_mode = get_is_header_mode(active_tab);
    assert!(is_header_mode); // non header modes not really have reasont to use this function

    let mut old_data = model.iter().collect::<Vec<_>>();
    let headers_idx = find_header_idx_and_deselect_all(&mut old_data);
    let size_idx = get_int_size_idx(active_tab);

    for i in 0..(headers_idx.len() - 1) {
        let mut min_size = u64::MAX;
        let mut min_size_idx = 0;
        for j in (headers_idx[i] + 1)..headers_idx[i + 1] {
            let int_data = old_data[j].val_int.iter().collect::<Vec<_>>();
            let size = connect_i32_into_u64(int_data[size_idx], int_data[size_idx + 1]);
            if size < min_size {
                min_size = size;
                min_size_idx = j;
            }
        }
        old_data[min_size_idx].checked = true;
    }

    ModelRc::new(VecModel::from(old_data))
}

fn select_all(model: ModelRc<MainListModel>) -> ModelRc<MainListModel> {
    let mut old_data = model.iter().collect::<Vec<_>>();
    old_data.iter_mut().for_each(|x| {
        if !x.header_row {
            x.checked = true
        }
    });
    ModelRc::new(VecModel::from(old_data))
}

fn deselect_all(model: ModelRc<MainListModel>) -> ModelRc<MainListModel> {
    let mut old_data = model.iter().collect::<Vec<_>>();
    old_data.iter_mut().for_each(|x| x.checked = false);
    ModelRc::new(VecModel::from(old_data))
}

fn invert_selection(model: ModelRc<MainListModel>) -> ModelRc<MainListModel> {
    let mut old_data = model.iter().collect::<Vec<_>>();
    old_data.iter_mut().for_each(|x| {
        if !x.header_row {
            x.checked = !x.checked
        }
    });
    ModelRc::new(VecModel::from(old_data))
}

fn find_header_idx_and_deselect_all(old_data: &mut Vec<MainListModel>) -> Vec<usize> {
    let mut header_idx = old_data
        .iter()
        .enumerate()
        .filter_map(|(idx, m)| if m.header_row { Some(idx) } else { None })
        .collect::<Vec<_>>();
    header_idx.push(old_data.len());

    old_data.iter_mut().for_each(|x| {
        if !x.header_row {
            x.checked = false;
        }
    });
    header_idx
}

#[cfg(test)]
mod test {
    use crate::{MainListModel, SelectMode};
    use slint::ModelRc;

    // #[test]
    // pub fn test_select_all() {
    //     let model = ModelRc::new(VecModel::from(vec![SelectModel {
    //         name: "test".into(),
    //         data: SelectMode::SelectAll,
    //     }]));
    //     let new_model = select_all(model);
    //     let new_data = new_model.iter().collect::<Vec<_>>();
    //     assert_eq!(new_data[0].checked, true);
    // }
    //
    // fn prepare_simple_model() -> ModelRc<MainListModel> {
    //     ModelRc::new(VecModel::from(vec![
    //         MainListModel {
    //             header_row: false,
    //             checked: false,
    //             selected_row: false,
    //             val_str: [],
    //             val_int: [0, 0, 0, 0, 0, 0],
    //         },
    //         MainListModel {
    //             header_row: false,
    //             checked: true,
    //             text: "test".into(),
    //             size: 0,
    //             resolution: (0, 0),
    //             date: 0,
    //         },
    //         MainListModel {
    //             header_row: false,
    //             checked: false,
    //             text: "test".into(),
    //             size: 0,
    //             resolution: (0, 0),
    //             date: 0,
    //         },
    //     ]))
    // }
}
