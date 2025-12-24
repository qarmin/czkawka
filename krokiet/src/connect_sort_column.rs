use std::mem;

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::connect_i32_into_u64;
use crate::connect_row_selection::{recalculate_small_selection_if_needed, reset_selection};
use crate::connect_translation::translate_sort_mode;
use crate::{ActiveTab, Callabler, GuiState, MainListModel, MainWindow, SortMode, SortModel};

pub(crate) fn connect_sort_column(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_change_sort_column_mode(move |sort_column_mode, column_idx| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        let current_model = active_tab.get_tool_model(&app);

        // TODO sorting

        dbg!(&sort_column_mode, &column_idx);


        recalculate_small_selection_if_needed(&current_model, active_tab);
        active_tab.set_tool_model(&app, current_model);
    });
}
