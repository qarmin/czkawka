use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{fs, path, thread};

use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use slint::{ComponentHandle, Weak};

use crate::model_operations::model_processor::{MessageType, ModelProcessor, ProcessFunction};
use crate::simpler_model::{SimplerSingleMainListModel, ToSimplerVec};
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
        let rename_on_conflict = app.global::<Settings>().get_popup_move_rename_on_conflict();

        let processor = ModelProcessor::new(active_tab);
        processor.move_selected_items(progress_sender, weak_app, stop_flag, preserve_structure, copy_mode, rename_on_conflict, &output_folder);
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
        rename_on_conflict: bool,
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

            let claimed_paths = Mutex::new(HashSet::new());
            let mlt_fnc = move |data: &SimplerSingleMainListModel| {
                move_single_item(data, path_idx, name_idx, &output_folder, preserve_structure, copy_mode, rename_on_conflict, &claimed_paths)
            };

            self.process_and_update_gui_state(
                &weak_app,
                stop_flag,
                &progress_sender,
                simpler_model,
                &ProcessFunction::Simple(Box::new(mlt_fnc)),
                MessageType::Move,
                false,
            );
        });
    }
}

#[expect(clippy::too_many_arguments)]
fn move_single_item(
    data: &SimplerSingleMainListModel,
    path_idx: usize,
    name_idx: usize,
    output_folder: &str,
    preserve_structure: bool,
    copy_mode: bool,
    rename_on_conflict: bool,
    claimed_paths: &Mutex<HashSet<PathBuf>>,
) -> Result<(), String> {
    let path = &data.val_str[path_idx];
    let name = &data.val_str[name_idx];

    let (input_file, desired_output_file) = collect_path_and_create_folders(path, name, output_folder, preserve_structure);

    let output_file = if rename_on_conflict {
        let mut claimed = claimed_paths.lock().expect("Mutex poisoned");
        let free = find_free_output_path(&desired_output_file, &claimed);
        claimed.insert(free.clone());
        free
    } else {
        if desired_output_file.exists() {
            return Err(flk!("rust_file_already_exists", file = desired_output_file.to_string_lossy().to_string()));
        }
        desired_output_file
    };

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

// Returns the first destination path that is neither already on disk nor already reserved by
// another item in the current move. If `desired` is free it is returned unchanged, otherwise a
// counter is inserted before the extension: `file.txt` -> `file(1).txt`, `file(2).txt`, ...
fn find_free_output_path(desired: &Path, claimed: &HashSet<PathBuf>) -> PathBuf {
    if !claimed.contains(desired) && !desired.exists() {
        return desired.to_path_buf();
    }

    let parent = desired.parent().map(Path::to_path_buf).unwrap_or_default();
    let stem = desired.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let extension = desired.extension().map(|s| s.to_string_lossy().into_owned());

    let mut counter: u64 = 1;
    loop {
        let candidate_name = match &extension {
            Some(ext) => format!("{stem}({counter}).{ext}"),
            None => format!("{stem}({counter})"),
        };
        let candidate = parent.join(candidate_name);
        if !claimed.contains(&candidate) && !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::path::PathBuf;
    use std::{env, fs};

    use super::find_free_output_path;

    // A directory path that does not exist on disk, so `Path::exists()` is always false and only
    // the `claimed` set drives the result (no filesystem side effects needed).
    fn nonexistent_dir() -> PathBuf {
        env::temp_dir().join("krokiet_move_conflict_nonexistent_dir_1304")
    }

    #[test]
    fn test_find_free_output_path_returns_desired_when_free() {
        let desired = nonexistent_dir().join("file.txt");
        assert_eq!(find_free_output_path(&desired, &HashSet::new()), desired);
    }

    #[test]
    fn test_find_free_output_path_skips_claimed_paths() {
        // Mirrors issue #1304: two items in a group share a name and are moved together.
        let dir = nonexistent_dir();
        let desired = dir.join("file.txt");
        let mut claimed: HashSet<PathBuf> = HashSet::new();

        let first = find_free_output_path(&desired, &claimed);
        assert_eq!(first, desired);
        claimed.insert(first);

        let second = find_free_output_path(&desired, &claimed);
        assert_eq!(second, dir.join("file(1).txt"));
        claimed.insert(second);

        let third = find_free_output_path(&desired, &claimed);
        assert_eq!(third, dir.join("file(2).txt"));
    }

    #[test]
    fn test_find_free_output_path_handles_no_extension() {
        let dir = nonexistent_dir();
        let desired = dir.join("file");
        let mut claimed = HashSet::new();
        claimed.insert(desired.clone());
        assert_eq!(find_free_output_path(&desired, &claimed), dir.join("file(1)"));
    }

    #[test]
    fn test_find_free_output_path_skips_existing_file_on_disk() {
        let dir = env::temp_dir();
        let desired = dir.join("krokiet_move_conflict_test_1304.txt");
        let candidate = dir.join("krokiet_move_conflict_test_1304(1).txt");
        let _ = fs::remove_file(&desired);
        let _ = fs::remove_file(&candidate);

        fs::write(&desired, "x").unwrap();
        let got = find_free_output_path(&desired, &HashSet::new());
        let _ = fs::remove_file(&desired);

        assert_eq!(got, candidate);
    }
}
