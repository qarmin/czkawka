use gtk4::prelude::*;
use gtk4::{Builder, EventControllerKey, Window};

use crate::help_functions::{get_custom_label_from_widget, set_icon_of_button};
use crate::{flg, CZK_ICON_STOP};

#[derive(Clone)]
pub struct GuiProgressDialog {
    pub window_progress: gtk4::Dialog,

    pub progress_bar_current_stage: gtk4::ProgressBar,
    pub progress_bar_all_stages: gtk4::ProgressBar,

    pub label_stage: gtk4::Label,
    pub label_progress_current_stage: gtk4::Label,
    pub label_progress_all_stages: gtk4::Label,

    pub grid_progress: gtk4::Grid,

    pub button_stop_in_dialog: gtk4::Button,
    pub evk_button_stop_in_dialog: EventControllerKey,
}

impl GuiProgressDialog {
    pub fn create_from_builder(window_main: &Window) -> Self {
        let glade_src = include_str!("../../ui/progress.ui").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        let window_progress: gtk4::Dialog = builder.object("window_progress").expect("Cambalache");
        window_progress.set_title(Some(&flg!("window_progress_title")));
        window_progress.set_transient_for(Some(window_main));
        window_progress.set_modal(true);

        let progress_bar_current_stage: gtk4::ProgressBar = builder.object("progress_bar_current_stage").expect("Cambalache");
        let progress_bar_all_stages: gtk4::ProgressBar = builder.object("progress_bar_all_stages").expect("Cambalache");

        let label_stage: gtk4::Label = builder.object("label_stage").expect("Cambalache");
        let label_progress_current_stage: gtk4::Label = builder.object("label_progress_current_stage").expect("Cambalache");
        let label_progress_all_stages: gtk4::Label = builder.object("label_progress_all_stages").expect("Cambalache");

        let grid_progress: gtk4::Grid = builder.object("grid_progress").expect("Cambalache");

        let button_stop_in_dialog: gtk4::Button = builder.object("button_stop_in_dialog").expect("Cambalache");
        let evk_button_stop_in_dialog = EventControllerKey::new();
        button_stop_in_dialog.add_controller(evk_button_stop_in_dialog.clone());

        set_icon_of_button(&button_stop_in_dialog, CZK_ICON_STOP);

        Self {
            window_progress,
            progress_bar_current_stage,
            progress_bar_all_stages,
            label_stage,
            label_progress_current_stage,
            label_progress_all_stages,
            grid_progress,
            button_stop_in_dialog,
            evk_button_stop_in_dialog,
        }
    }
    pub fn update_language(&self) {
        self.window_progress.set_title(Some(&flg!("window_progress_title")));

        get_custom_label_from_widget(&self.button_stop_in_dialog.clone()).set_text(&flg!("progress_stop_button"));

        self.label_progress_current_stage.set_label(&flg!("progress_current_stage"));
        self.label_progress_all_stages.set_label(&flg!("progress_all_stages"));
    }
}
