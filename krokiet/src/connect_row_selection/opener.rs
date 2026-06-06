use log::error;
use slint::{ComponentHandle, Model};

use crate::{Callabler, GuiState, MainWindow};

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
