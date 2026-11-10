use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::{ActiveTab, Callabler, GuiState, MainWindow, Settings, ToolVisibilityModel};

pub(crate) fn connect_tools_visibility(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_tool_visibility_toggled(move |idx, visible| {
        let app = a.upgrade().expect("MainWindow dropped");
        set_tool_visibility(&app, idx as usize, visible);
    });
}

pub(crate) fn set_tools_names(app: &MainWindow, tools: Vec<(SharedString, ActiveTab)>) {
    let previous = app.global::<Settings>().get_tools_visibility();
    let rows = tools
        .into_iter()
        .map(|(name, tab)| {
            let visible = previous.iter().find(|row| row.tab == tab).is_none_or(|row| row.visible);
            ToolVisibilityModel { name, tab, visible }
        })
        .collect::<Vec<_>>();
    app.global::<Settings>().set_tools_visibility(ModelRc::new(VecModel::from(rows)));
    refresh_left_panel(app);
}

pub(crate) fn apply_hidden_tools(app: &MainWindow, hidden_tools: &[String]) {
    let settings = app.global::<Settings>();
    let mut rows = settings.get_tools_visibility().iter().collect::<Vec<_>>();
    for row in &mut rows {
        row.visible = !hidden_tools.contains(&tool_id(row.tab));
    }
    if rows.iter().all(|row| !row.visible) {
        for row in &mut rows {
            row.visible = true;
        }
    }
    settings.set_tools_visibility(ModelRc::new(VecModel::from(rows)));
    refresh_left_panel(app);
}

pub(crate) fn collect_hidden_tools(app: &MainWindow) -> Vec<String> {
    app.global::<Settings>()
        .get_tools_visibility()
        .iter()
        .filter(|row| !row.visible)
        .map(|row| tool_id(row.tab))
        .collect()
}

fn tool_id(tab: ActiveTab) -> String {
    format!("{tab:?}")
}

fn set_tool_visibility(app: &MainWindow, idx: usize, visible: bool) {
    let rows = app.global::<Settings>().get_tools_visibility();
    let Some(mut row) = rows.row_data(idx) else {
        return;
    };
    let visible_count = rows.iter().filter(|row| row.visible).count();
    if !visible && visible_count <= 1 {
        rows.set_row_data(idx, row);
        return;
    }
    row.visible = visible;
    rows.set_row_data(idx, row);
    refresh_left_panel(app);
}

fn refresh_left_panel(app: &MainWindow) {
    let settings = app.global::<Settings>();
    let gui_state = app.global::<GuiState>();
    let visible_tools = settings
        .get_tools_visibility()
        .iter()
        .filter(|row| row.visible)
        .map(|row| (row.name, row.tab))
        .collect::<Vec<_>>();
    settings.set_visible_tools_count(visible_tools.len() as i32);

    let active_tab = gui_state.get_active_tab();
    let active_is_hidden_tool = !matches!(active_tab, ActiveTab::Settings | ActiveTab::About) && !visible_tools.iter().any(|(_, tab)| *tab == active_tab);
    let first_visible = visible_tools.first().map(|(_, tab)| *tab);
    gui_state.set_tools_model(ModelRc::new(VecModel::from(visible_tools)));

    if let (true, Some(tab)) = (active_is_hidden_tool, first_visible) {
        gui_state.set_active_tab(tab);
        app.global::<Callabler>().invoke_tab_changed();
    }
}
