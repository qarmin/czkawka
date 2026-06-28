use std::time::Duration;

use crossbeam_channel::Receiver;
use czkawka_core::common::progress_data::{ProgressData, ProgressDisplay};
use glib::MainContext;
use gtk4::prelude::*;

use crate::gui_structs::gui_data::GuiData;
use crate::taskbar_progress::tbp_flags::TBPF_INDETERMINATE;

pub(crate) fn connect_progress_window(gui_data: &GuiData, progress_receiver: Receiver<ProgressData>) {
    let main_context = MainContext::default();
    let _guard = main_context.acquire().expect("Failed to acquire main context");

    let gui_data = gui_data.clone();

    let future = async move {
        loop {
            while let Ok(item) = progress_receiver.try_recv() {
                set_progress(&gui_data, &item.to_display());
            }
            glib::timeout_future(Duration::from_millis(300)).await;
        }
    };

    main_context.spawn_local(future);
}

fn set_progress(gui_data: &GuiData, display: &ProgressDisplay) {
    let label_stage = &gui_data.progress_window.label_stage;
    let progress_bar_current_stage = &gui_data.progress_window.progress_bar_current_stage;
    let progress_bar_all_stages = &gui_data.progress_window.progress_bar_all_stages;
    let taskbar_state = gui_data.taskbar_state.borrow();

    label_stage.set_text(&display.label);

    if let Some(current) = display.current_progress {
        progress_bar_current_stage.set_visible(true);
        let fraction = display.current_progress_size.unwrap_or(current) as f64 / 100.0;
        progress_bar_current_stage.set_fraction(fraction);
        let overall = if display.all_progress >= 0 { display.all_progress } else { current };
        taskbar_state.set_progress_value(overall as u64, 100);
    } else {
        progress_bar_current_stage.set_visible(false);
        taskbar_state.set_progress_state(TBPF_INDETERMINATE);
    }

    if display.all_progress >= 0 {
        progress_bar_all_stages.set_visible(true);
        progress_bar_all_stages.set_fraction(display.all_progress as f64 / 100.0);
    } else {
        progress_bar_all_stages.set_visible(false);
    }
}
