use std::path::MAIN_SEPARATOR;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use slint::{ComponentHandle, Weak};

use crate::model_operations::model_processor::{MessageType, ModelProcessor};
use crate::simpler_model::{SimplerMainListModel, ToSimplerVec};
use crate::{Callabler, GuiState, MainWindow, Settings};

pub(crate) fn connect_clean(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_clean_exif_items(move || {
        let weak_app = a.clone();
        let progress_sender = progress_sender.clone();
        let stop_flag = stop_flag.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        let override_file = app.global::<Settings>().get_popup_clean_exif_overwrite_files();

        let processor = ModelProcessor::new(active_tab);
        processor.clean_exif_selected_files(progress_sender, weak_app, stop_flag, override_file);
    });
}

impl ModelProcessor {
    fn clean_exif_selected_files(self, progress_sender: Sender<ProgressData>, weak_app: Weak<MainWindow>, stop_flag: Arc<AtomicBool>, override_file: bool) {
        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();
            let tag_groups_idx = self.active_tab.get_exif_tag_groups_idx();
            let tag_u16_idx = self.active_tab.get_exif_tag_u16_idx();

            let clean_fnc = move |data: &SimplerMainListModel| {
                clean_exif_single_file(
                    &format!("{}{MAIN_SEPARATOR}{}", data.val_str[path_idx], data.val_str[name_idx]),
                    &data.val_str[tag_groups_idx].split(',').map(|s| s.to_string()).collect::<Vec<_>>(),
                    &data.val_str[tag_u16_idx].split(',').map(|s| s.to_string()).collect::<Vec<_>>(),
                    override_file,
                )
            };

            self.process_and_update_gui_state(&weak_app, stop_flag, &progress_sender, simpler_model, clean_fnc, MessageType::CleanExif, false);
        });
    }
}

#[cfg(not(test))]
fn clean_exif_single_file(file_path: &str, tag_groups: &[String], tags_u16: &[String], override_file: bool) -> Result<(), String> {
    // Such data are split into multiple vectors, but in Krokiet they are not changed
    assert_eq!(tags_u16.len(), tag_groups.len());
    let connected_tags = tag_groups
        .iter()
        .zip(tags_u16.iter())
        .map(|(group, code)| (code.parse::<u16>().unwrap_or(0), group.clone()))
        .collect::<Vec<(u16, String)>>();
    let _ = czkawka_core::tools::exif_remover::core::clean_exif_tags(file_path, &connected_tags, override_file)
        .map_err(|e| format!("Failed to clean EXIF for file {file_path:?}, reason: {e}"))?;
    Ok(())
}

#[cfg(test)]
fn clean_exif_single_file(full_path: &str, _tag_groups: &[String], _tags_u16: &[String], _override_file: bool) -> Result<(), String> {
    if full_path.contains("test_error") {
        return Err(format!("Test error for item: {full_path}"));
    }
    Ok(())
}
