use std::thread;

use crossbeam_channel::Receiver;
use czkawka_core::common::progress_data::ProgressData;
use slint::ComponentHandle;

use crate::{MainWindow, ProgressToSend};

pub(crate) fn connect_progress_gathering(app: &MainWindow, progress_receiver: Receiver<ProgressData>) {
    let a = app.as_weak();

    thread::spawn(move || {
        loop {
            let Ok(progress_data) = progress_receiver.recv() else {
                return; // Channel closed, so exit the thread since app closing
            };

            a.upgrade_in_event_loop(move |app| {
                let display = progress_data.to_display();
                app.set_progress_datas(ProgressToSend {
                    all_progress: display.all_progress,
                    current_progress: display.current_progress.unwrap_or(-1),
                    current_progress_size: display.current_progress_size.unwrap_or(-1),
                    step_name: display.label.into(),
                });
            })
            .expect("Failed to spawn thread for progress gathering");
        }
    });
}
