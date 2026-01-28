use std::sync::mpsc;

use slint::ComponentHandle;

use crate::create_calculate_task_size::SizeCountResult;
use crate::{ActiveTab, Callabler, GuiState, MainWindow};
pub(crate) fn connect_tab_changed(app: &MainWindow, cache_task_sender: mpsc::Sender<std::sync::mpsc::Sender<SizeCountResult>>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_tab_changed(move || {
        let cache_task_sender = cache_task_sender.clone();
        let app = a.upgrade().expect("Failed to upgrade app :(");
        crate::connect_select::set_select_buttons(&app);
        crate::connect_sort::set_sort_buttons(&app);

        let active_tab = app.global::<GuiState>().get_active_tab();
        if active_tab != ActiveTab::Settings {
            return;
        }

        crate::create_calculate_task_size::request_and_update_cache_sizes(a.clone(), cache_task_sender);
    });
}
