use crate::fl;
use crate::help_functions::get_custom_label_from_label_with_image;
use gtk::prelude::*;
use gtk::{Bin, Builder, EventControllerKey, Window};

#[derive(Clone)]
pub struct GuiProgressDialog {
    pub window_progress: gtk::Dialog,

    pub progress_bar_current_stage: gtk::ProgressBar,
    pub progress_bar_all_stages: gtk::ProgressBar,

    pub label_stage: gtk::Label,
    pub label_progress_current_stage: gtk::Label,
    pub label_progress_all_stages: gtk::Label,

    pub grid_progress_stages: gtk::Grid,

    pub button_stop_in_dialog: gtk::Button,
    pub evk_button_stop_in_dialog: gtk::EventControllerKey,
}

impl GuiProgressDialog {
    pub fn create_from_builder(window_main: &Window) -> Self {
        let glade_src = include_str!("../ui/progress.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_progress: gtk::Dialog = builder.object("window_progress").unwrap();
        window_progress.set_transient_for(Some(window_main));
        window_progress.set_modal(true);
        window_progress.set_title("Czkawka");

        let progress_bar_current_stage: gtk::ProgressBar = builder.object("progress_bar_current_stage").unwrap();
        let progress_bar_all_stages: gtk::ProgressBar = builder.object("progress_bar_all_stages").unwrap();

        let label_stage: gtk::Label = builder.object("label_stage").unwrap();
        let label_progress_current_stage: gtk::Label = builder.object("label_progress_current_stage").unwrap();
        let label_progress_all_stages: gtk::Label = builder.object("label_progress_all_stages").unwrap();

        let grid_progress_stages: gtk::Grid = builder.object("grid_progress_stages").unwrap();

        let button_stop_in_dialog: gtk::Button = builder.object("button_stop_in_dialog").unwrap();
        let evk_button_stop_in_dialog = EventControllerKey::new(&button_stop_in_dialog);

        Self {
            window_progress,
            progress_bar_current_stage,
            progress_bar_all_stages,
            label_stage,
            label_progress_current_stage,
            label_progress_all_stages,
            grid_progress_stages,
            button_stop_in_dialog,
            evk_button_stop_in_dialog,
        }
    }
    pub fn update_language(&self) {
        get_custom_label_from_label_with_image(&self.button_stop_in_dialog.clone().upcast::<Bin>()).set_text(&fl!("progress_stop_button"));

        self.label_progress_current_stage.set_label(&fl!("progress_current_stage"));
        self.label_progress_all_stages.set_label(&fl!("progress_all_stages"));
    }
}
