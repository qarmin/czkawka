use rfd::FileDialog;
use slint::ComponentHandle;

use crate::model_operations::get_checked_info_from_app;
use crate::{MainWindow, PopupRequest, Translations, flk};

pub(crate) fn connect_show_confirmation(app: &MainWindow) {
    let a = app.as_weak();
    app.on_request_setup_action_popup(move |popup_request: PopupRequest| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let translation = app.global::<Translations>();
        let res = get_checked_info_from_app(&app);
        let mut folder_path = "".to_string();

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
                folder_path = folder.to_string_lossy().to_string();
                // TODO missing info about moving items
            }
            PopupRequest::OptimizeVideo => {
                // TODO missing info about optimizing items
            }
            PopupRequest::CleanExif => {
                // TODO missing info about cleaning EXIF items
            }
            PopupRequest::Symlink => {
                // TODO missing info about symlinking items
            }
            PopupRequest::Hardlink => {
                // TODO missing info about hardlinking items
            }
            PopupRequest::Rename => {
                // TODO missing info about renaming items
            }
            PopupRequest::Save => {
                // TODO missing info about saving items
            }
        }

        app.invoke_show_action_popup(popup_request, folder_path.into());
    });
}
