use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fs, path, thread};

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use slint::{ComponentHandle, Weak};

use crate::model_operations::model_processor::{MessageType, ModelProcessor};
use crate::simpler_model::{SimplerMainListModel, ToSimplerVec};
use crate::{Callabler, GuiState, MainWindow, Settings, flk};

pub(crate) fn connect_move(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>) {
    let a = app.as_weak();
    app.global::<Callabler>().on_move_items(move |output_folder| {
        let weak_app = a.clone();
        let progress_sender = progress_sender.clone();
        let stop_flag = stop_flag.clone();
        stop_flag.store(false, Ordering::Relaxed);
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();

        let preserve_structure = app.global::<Settings>().get_popup_move_preserve_folder_structure();
        let copy_mode = app.global::<Settings>().get_popup_move_copy_mode();

        let processor = ModelProcessor::new(active_tab);
        processor.move_selected_items(progress_sender, weak_app, stop_flag, preserve_structure, copy_mode, &output_folder);
    });
}

impl ModelProcessor {
    fn move_selected_items(
        self,
        progress_sender: Sender<ProgressData>,
        weak_app: Weak<MainWindow>,
        stop_flag: Arc<AtomicBool>,
        preserve_structure: bool,
        copy_mode: bool,
        output_folder: &str,
    ) {
        if let Err(err) = fs::create_dir_all(output_folder) {
            let app = weak_app.upgrade().expect("Failed to upgrade app :(");
            app.global::<GuiState>()
                .set_info_text(flk!("rust_cannot_create_output_folder", output_folder = output_folder, error = err.to_string()).into());
            return;
        }

        let model = self.active_tab.get_tool_model(&weak_app.upgrade().expect("Failed to upgrade app :("));
        let simpler_model = model.to_simpler_enumerated_vec();
        let output_folder = output_folder.to_string();
        thread::spawn(move || {
            let path_idx = self.active_tab.get_str_path_idx();
            let name_idx = self.active_tab.get_str_name_idx();

            let mlt_fnc = move |data: &SimplerMainListModel| move_single_item(data, path_idx, name_idx, &output_folder, preserve_structure, copy_mode);

            self.process_and_update_gui_state(&weak_app, stop_flag, &progress_sender, simpler_model, mlt_fnc, MessageType::Move, false);
        });
    }
}

fn move_single_item(data: &SimplerMainListModel, path_idx: usize, name_idx: usize, output_folder: &str, preserve_structure: bool, copy_mode: bool) -> Result<(), String> {
    let path = &data.val_str[path_idx];
    let name = &data.val_str[name_idx];

    let (input_file, output_file) = collect_path_and_create_folders(path, name, output_folder, preserve_structure);
    if output_file.exists() {
        return Err(flk!("rust_file_already_exists", file = output_file.to_string_lossy().to_string()));
    }

    if copy_mode {
        try_to_copy_item(&input_file, &output_file)
    } else {
        // Try to rename file, may fail due various reasons
        // It is the easiest way to move file, but only on same partition
        if fs::rename(&input_file, &output_file).is_ok() {
            return Ok(());
        }

        // It is possible that this failed, because file is on different partition, so
        // we need to copy file and then remove old
        try_to_copy_item(&input_file, &output_file)?;

        if let Err(e) = fs::remove_file(&input_file) {
            return Err(flk!(
                "rust_error_removing_file_after_copy",
                file = input_file.to_string_lossy().to_string(),
                reason = e.to_string()
            ));
        }
        Ok(())
    }
}

// Tries to copy file/folder, and returns error if it fails
fn try_to_copy_item(input_file: &Path, output_file: &Path) -> Result<(), String> {
    let res = if input_file.is_dir() {
        let options = fs_extra::dir::CopyOptions::new();
        fs_extra::dir::copy(input_file, output_file, &options) // TODO consider to use less buggy library
    } else {
        let options = fs_extra::file::CopyOptions::new();
        fs_extra::file::copy(input_file, output_file, &options)
    };
    if let Err(e) = res {
        return Err(flk!(
            "rust_error_copying_file",
            input = input_file.to_string_lossy().to_string(),
            output = output_file.to_string_lossy().to_string(),
            reason = e.to_string()
        ));
    }
    Ok(())
}

// Create input/output paths, and create output folder
fn collect_path_and_create_folders(input_path: &str, input_file: &str, output_path: &str, preserve_structure: bool) -> (PathBuf, PathBuf) {
    let input_full_path = PathBuf::from(input_path).join(input_file);

    let mut output_full_path = PathBuf::from(output_path);
    if preserve_structure {
        output_full_path.extend(Path::new(input_path).components().filter(|c| matches!(c, path::Component::Normal(_))));
    }
    let _ = fs::create_dir_all(&output_full_path);
    output_full_path.push(input_file);

    (input_full_path, output_full_path)
}
