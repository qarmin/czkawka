use gtk::prelude::*;

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
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let window_progress: gtk::Window = builder.get_object("window_progress").unwrap();

        let progress_bar_current_stage: gtk::ProgressBar = builder.get_object("progress_bar_current_stage").unwrap();
        let progress_bar_all_stages: gtk::ProgressBar = builder.get_object("progress_bar_all_stages").unwrap();

        let label_stage: gtk::Label = builder.get_object("label_stage").unwrap();

        let grid_progress_stages: gtk::Grid = builder.get_object("grid_progress_stages").unwrap();

        let button_stop_in_dialog: gtk::Button = builder.get_object("button_stop_in_dialog").unwrap();

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
