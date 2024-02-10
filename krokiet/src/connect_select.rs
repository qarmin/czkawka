use crate::common::{get_tool_model, set_tool_model};
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
        };
        set_tool_model(&app, active_tab, new_model);
    });
}

pub fn connect_showing_proper_select_buttons(app: &MainWindow) {
    set_select_buttons(&app);
    let a = app.as_weak();
    app.global::<Callabler>().on_tab_changed(move || {
        let app = a.upgrade().unwrap();
        set_select_buttons(&app);
    });
}

fn set_select_buttons(app: &MainWindow) {
    // let active_tab = app.global::<GuiState>().get_active_tab();
    let base_buttons = vec![SelectMode::SelectAll, SelectMode::UnselectAll, SelectMode::InvertSelection];

    // TODO Here needs to be put logic to set custom buttons depending on tab

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
    }
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
