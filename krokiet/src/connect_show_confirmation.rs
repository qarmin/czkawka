use std::sync::{Arc, Mutex};
use rfd::FileDialog;
use slint::ComponentHandle;

use crate::model_operations::get_checked_info_from_app;
use crate::{MainWindow, PopupRequest, Translations, flk};
use crate::shared_models::SharedModels;

pub(crate) fn connect_show_confirmation(app: &MainWindow,
                                        shared_models: Arc<Mutex<SharedModels>>,) {
    let a = app.as_weak();
    app.on_request_setup_action_popup(move |popup_request: PopupRequest| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let translation = app.global::<Translations>();
        let res = get_checked_info_from_app(&app);
        let mut data = "".to_string();

        match popup_request {
            PopupRequest::Delete => {
                let mut base = flk!("rust_delete_confirmation");
                if let Some(group_res) = res.groups_with_checked_items {
                    base.push_str(
                        format!(
                            "\n{}",
                            flk!(
                                "rust_delete_confirmation_number_groups",
                                items = res.checked_items_number,
                                groups = group_res.groups_with_checked_items
                            )
                        )
                        .as_str(),
                    );
                    if group_res.number_of_groups_with_all_items_checked > 0 {
                        base.push_str(
                            format!(
                                "\n{}",
                                flk!("rust_delete_confirmation_selected_all_in_group", groups = group_res.number_of_groups_with_all_items_checked)
                            )
                            .as_str(),
                        );
                    }
                } else {
                    base.push_str(format!("\n{}", flk!("rust_delete_confirmation_number_simple", items = res.checked_items_number)).as_str());
                }
                translation.set_delete_confirmation_text(base.into());
            }
            PopupRequest::Move => {
                let file_dialog = FileDialog::new();
                let Some(folder) = file_dialog.pick_folder() else {
                    return;
                };
                data = folder.to_string_lossy().to_string();

                let mut base = flk!("rust_move_confirmation");
                base.push_str(format!("\n{}", flk!("rust_move_confirmation_number_simple", items = res.checked_items_number)).as_str());
                translation.set_move_confirmation_text(base.into());
            }
            PopupRequest::OptimizeVideo => {
                let mut base = flk!("rust_optimize_video_confirmation");
                base.push_str(format!("\n{}", flk!("rust_optimize_video_confirmation_number_simple", items = res.checked_items_number)).as_str());
                translation.set_optimize_confirmation_text(base.into());

                let shared_model = shared_models.lock();
                let shared_model = shared_model.as_ref().expect("Failed to lock shared models");
                let shared_model = shared_model.shared_video_optimizer_state.as_ref().expect("Item should be present for video optimizer");
                data = if matches!(shared_model.get_params(), VideoOptimizerParameters::VideoCrop(_)) {
                    "crop".to_string()
                } else {
                    "transcode".to_string()
                }
            }
            PopupRequest::CleanExif => {
                let mut base = flk!("rust_clean_exif_confirmation");
                base.push_str(format!("\n{}", flk!("rust_clean_exif_confirmation_number_simple", items = res.checked_items_number)).as_str());
                translation.set_clean_confirmation_text(base.into());
            }
            PopupRequest::Symlink => {
                let mut base = flk!("rust_symlink_confirmation");
                base.push_str(format!("\n{}", flk!("rust_symlink_confirmation_number_simple", items = res.checked_items_number)).as_str());
                translation.set_softlink_confirmation_text(base.into());
            }
            PopupRequest::Hardlink => {
                let mut base = flk!("rust_hardlink_confirmation");
                base.push_str(format!("\n{}", flk!("rust_hardlink_confirmation_number_simple", items = res.checked_items_number)).as_str());
                translation.set_hardlink_confirmation_text(base.into());
            }
            PopupRequest::Rename => {
                let mut base = flk!("rust_rename_confirmation");
                base.push_str(format!("\n{}", flk!("rust_rename_confirmation_number_simple", items = res.checked_items_number)).as_str());
                translation.set_rename_confirmation_text(base.into());
            }
            PopupRequest::Save => {
                // There is no confirmation saving
            }
        }

        app.invoke_show_action_popup(popup_request, data.into());
    });
}
