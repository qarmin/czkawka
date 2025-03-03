use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use gtk4::prelude::*;

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::KEY_ENTER;

fn send_stop_message(stop_flag: &Arc<AtomicBool>) {
    stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
}

pub fn connect_button_stop(gui_data: &GuiData) {
    let evk_button_stop_in_dialog = gui_data.progress_window.evk_button_stop_in_dialog.clone();
    let stop_dialog = gui_data.progress_window.window_progress.clone();
    let stop_flag = gui_data.stop_flag.clone();
    evk_button_stop_in_dialog.connect_key_released(move |_, _, key_code, _| {
        if key_code == KEY_ENTER {
            stop_dialog.set_title(Some(&format!("{} ({})", flg!("window_progress_title"), flg!("progress_stop_additional_message"))));
            send_stop_message(&stop_flag);
        }
    });

    let button_stop_in_dialog = gui_data.progress_window.button_stop_in_dialog.clone();
    let stop_dialog = gui_data.progress_window.window_progress.clone();
    let stop_flag = gui_data.stop_flag.clone();

    button_stop_in_dialog.connect_clicked(move |_a| {
        stop_dialog.set_title(Some(&format!("{} ({})", flg!("window_progress_title"), flg!("progress_stop_additional_message"))));
        send_stop_message(&stop_flag);
    });
}
