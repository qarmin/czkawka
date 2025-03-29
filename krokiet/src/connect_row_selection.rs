use log::error;
use slint::{ComponentHandle, Model};

use crate::common::get_tool_model;
use crate::{Callabler, GuiState, MainWindow};

pub fn connect_row_selections(app: &MainWindow) {
    let a = app.as_weak();

    app.global::<Callabler>().on_row_select_all(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let _active_tab = app.global::<GuiState>().get_active_tab();
        error!("Clicked on select all rows");
    });
    let a = app.as_weak();
    app.global::<Callabler>().on_row_deselect_all(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let _active_tab = app.global::<GuiState>().get_active_tab();
        error!("Clicked on deselect all rows");
    });
    let a = app.as_weak();
    app.global::<Callabler>().on_row_select_single_item(move |id| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let model = get_tool_model(&app, active_tab);
        let mut model_data = model
            .row_data(id as usize)
            .expect(&format!("Failed to get row data with id {id}, in tab ??, with model {} items", model.row_count())); // TODO add tab name
        model_data.selected_row = true;
        model.set_row_data(id as usize, model_data);
        error!("Clicked on select single item rows, with id {id}");
    });

    // Opening
    let a = app.as_weak();
    app.global::<Callabler>().on_row_open_selected_item(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let _active_tab = app.global::<GuiState>().get_active_tab();
        error!("Clicked on open selected item rows");
    });
    let a = app.as_weak();
    app.global::<Callabler>().on_row_open_parent_of_selected_item(move || {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let _active_tab = app.global::<GuiState>().get_active_tab();
        error!("Clicked on select single item rows");
    });
}
