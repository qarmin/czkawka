use gtk::prelude::*;
use gtk::Builder;

#[derive(Clone)]
pub struct GuiProgressDialog {
    pub window_progress: gtk::Window,

    pub progress_bar_current_stage: gtk::ProgressBar,
    pub progress_bar_all_stages: gtk::ProgressBar,

    pub label_stage: gtk::Label,

    pub grid_progress_stages: gtk::Grid,

    pub button_stop_in_dialog: gtk::Button,
}

impl GuiProgressDialog {
    pub fn create_from_builder() -> Self {
        let glade_src = include_str!("../ui/progress.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_progress: gtk::Window = builder.object("window_progress").unwrap();

        let progress_bar_current_stage: gtk::ProgressBar = builder.object("progress_bar_current_stage").unwrap();
        let progress_bar_all_stages: gtk::ProgressBar = builder.object("progress_bar_all_stages").unwrap();

        let label_stage: gtk::Label = builder.object("label_stage").unwrap();

        let grid_progress_stages: gtk::Grid = builder.object("grid_progress_stages").unwrap();

        let button_stop_in_dialog: gtk::Button = builder.object("button_stop_in_dialog").unwrap();

        Self {
            window_progress,
            progress_bar_current_stage,
            progress_bar_all_stages,
            label_stage,
            grid_progress_stages,
            button_stop_in_dialog,
        }
    }
}
