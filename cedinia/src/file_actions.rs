use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::callbacks::get_model_for_tool;
use crate::callbacks::selection_ops::{full_path_of, get_val_str, vm_of};
use crate::common::{StrDataBadExtensions, StrDataBadNames};
use crate::model::rebuild_similar_images_after_delete;
use crate::{AppState, FileEntry, MainWindow};

#[cfg(not(target_os = "android"))]
pub(crate) fn delete_path(path: &str) -> Result<(), String> {
    trash::delete(path).map_err(|e| e.to_string())
}

// Android has no trash support, so deletion here is permanent. Dispatch on the path type and call
// the matching remove function, so the user sees the real error instead of a spurious directory-removal
// error when removing a regular file fails.
#[cfg(target_os = "android")]
pub(crate) fn delete_path(path: &str) -> Result<(), String> {
    let metadata = std::fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if metadata.is_dir() {
        std::fs::remove_dir_all(path).map_err(|e| e.to_string())
    } else {
        std::fs::remove_file(path).map_err(|e| e.to_string())
    }
}

pub(crate) enum DeleteEvent {
    Progress(usize, usize),

    Finished(Vec<String>, Vec<String>),

    ListDeleteFinished(Vec<String>, Vec<String>),

    ListRenameFinished(usize, Vec<String>),

    ExifCleanFinished(Vec<String>, Vec<String>),
}

pub(crate) fn execute_delete_selected(win: &MainWindow, tx: std::sync::mpsc::Sender<DeleteEvent>) {
    let tool = win.global::<AppState>().get_active_tool();
    let model = get_model_for_tool(win, tool);
    let vm = vm_of(&model);

    let items: Vec<FileEntry> = vm.iter().collect();
    let to_delete: Vec<(usize, String)> = items
        .iter()
        .enumerate()
        .filter(|(_, e)| e.checked && !e.is_header && !e.is_reference)
        .map(|(i, e)| (i, full_path_of(e)))
        .collect();

    if to_delete.is_empty() {
        return;
    }

    let total = to_delete.len();
    win.global::<AppState>().set_delete_running(true);
    win.global::<AppState>().set_delete_progress_text(slint::SharedString::from(format!("0 / {total}")));

    std::thread::spawn(move || {
        let mut deleted_paths: Vec<String> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        for (i, (_idx, path)) in to_delete.iter().enumerate() {
            match delete_path(path) {
                Ok(()) => {
                    deleted_paths.push(path.clone());
                }
                Err(err) => errors.push(format!("{path}\n  {err}")),
            }
            if i % 5 == 4 || i + 1 == total {
                let _ = tx.send(DeleteEvent::Progress(i + 1, total));
            }
        }
        let _ = tx.send(DeleteEvent::ListDeleteFinished(deleted_paths, errors));
    });
}

pub(crate) fn execute_rename_selected(win: &MainWindow, tx: std::sync::mpsc::Sender<DeleteEvent>) {
    let model = win.get_bad_extensions_model();
    let vm = vm_of(&model);

    let items: Vec<FileEntry> = vm.iter().collect();
    let to_rename: Vec<(usize, String, String)> = items
        .iter()
        .enumerate()
        .filter(|(_, e)| e.checked && !e.is_header)
        .filter_map(|(i, e)| {
            let full = full_path_of(e);
            let proper_ext = get_val_str(e, StrDataBadExtensions::ProperExtension as usize);
            if proper_ext.is_empty() {
                return None;
            }
            Some((i, full, proper_ext))
        })
        .collect();

    if to_rename.is_empty() {
        return;
    }

    let total = to_rename.len();
    win.global::<AppState>().set_delete_running(true);
    win.global::<AppState>().set_delete_progress_text(slint::SharedString::from(format!("0 / {total}")));

    std::thread::spawn(move || {
        let mut renamed_indices: Vec<usize> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        for (i, (idx, full, proper_ext)) in to_rename.iter().enumerate() {
            let src = std::path::Path::new(full.as_str());
            let new_path = match src.file_stem() {
                Some(stem) => {
                    let parent = src.parent().unwrap_or(std::path::Path::new(""));
                    parent.join(format!("{}.{}", stem.to_string_lossy(), proper_ext))
                }
                None => {
                    errors.push(format!("{full}\n  {}", crate::flc!("rename_error_read_file_name")));
                    continue;
                }
            };
            match std::fs::rename(full, &new_path) {
                Ok(()) => renamed_indices.push(*idx),
                Err(err) => errors.push(format!("{full}\n  {err}")),
            }
            if i % 5 == 4 || i + 1 == total {
                let _ = tx.send(DeleteEvent::Progress(i + 1, total));
            }
        }
        let renamed = renamed_indices.len();
        let _ = tx.send(DeleteEvent::ListRenameFinished(renamed, errors));
    });
}

pub(crate) fn execute_rename_bad_names(win: &MainWindow, tx: std::sync::mpsc::Sender<DeleteEvent>) {
    let model = win.get_bad_names_model();
    let vm = vm_of(&model);

    let items: Vec<FileEntry> = vm.iter().collect();
    let to_rename: Vec<(usize, String, String)> = items
        .iter()
        .enumerate()
        .filter(|(_, e)| e.checked && !e.is_header)
        .filter_map(|(i, e)| {
            let new_name = get_val_str(e, StrDataBadNames::NewName as usize);
            if new_name.is_empty() {
                return None;
            }
            let full = full_path_of(e);
            Some((i, full, new_name))
        })
        .collect();

    if to_rename.is_empty() {
        return;
    }

    let total = to_rename.len();
    win.global::<AppState>().set_delete_running(true);
    win.global::<AppState>().set_delete_progress_text(slint::SharedString::from(format!("0 / {total}")));

    std::thread::spawn(move || {
        let mut renamed_count = 0usize;
        let mut errors: Vec<String> = Vec::new();

        for (i, (_idx, full, new_name)) in to_rename.iter().enumerate() {
            let src = std::path::Path::new(full.as_str());
            let new_path = match src.parent() {
                Some(parent) => parent.join(new_name),
                None => {
                    errors.push(format!("{full}\n  {}", crate::flc!("rename_error_read_directory")));
                    continue;
                }
            };
            match std::fs::rename(full, &new_path) {
                Ok(()) => renamed_count += 1,
                Err(err) => errors.push(format!("{full}\n  {err}")),
            }
            if i % 5 == 4 || i + 1 == total {
                let _ = tx.send(DeleteEvent::Progress(i + 1, total));
            }
        }
        let _ = tx.send(DeleteEvent::ListRenameFinished(renamed_count, errors));
    });
}
pub(crate) fn execute_clean_exif_selected(win: &MainWindow, tx: std::sync::mpsc::Sender<DeleteEvent>) {
    let model = win.get_exif_remover_model();
    let vm = vm_of(&model);

    let items: Vec<FileEntry> = vm.iter().collect();
    let to_clean: Vec<(usize, String)> = items
        .iter()
        .enumerate()
        .filter(|(_, e)| e.checked && !e.is_header)
        .map(|(i, e)| (i, full_path_of(e)))
        .collect();

    if to_clean.is_empty() {
        return;
    }

    let total = to_clean.len();
    win.global::<AppState>().set_delete_running(true);
    win.global::<AppState>().set_delete_progress_text(slint::SharedString::from(format!("0 / {total}")));

    std::thread::spawn(move || {
        let mut cleaned_paths: Vec<String> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        for (i, (_idx, path)) in to_clean.iter().enumerate() {
            match clean_exif_all_tags(path) {
                Ok(()) => cleaned_paths.push(path.clone()),
                Err(e) => errors.push(format!("{path}\n  {e}")),
            }
            if i % 5 == 4 || i + 1 == total {
                let _ = tx.send(DeleteEvent::Progress(i + 1, total));
            }
        }
        let _ = tx.send(DeleteEvent::ExifCleanFinished(cleaned_paths, errors));
    });
}

fn clean_exif_all_tags(path: &str) -> Result<(), String> {
    use czkawka_core::tools::exif_remover::core::{clean_exif_tags, extract_exif_tags_public};
    let tags = extract_exif_tags_public(std::path::Path::new(path))?;
    clean_exif_tags(path, &tags, true).map(|_| ())
}

fn vm_file_entry(model: &ModelRc<FileEntry>) -> &VecModel<FileEntry> {
    model.as_any().downcast_ref::<VecModel<FileEntry>>().expect("FileEntry model must be backed by a VecModel")
}

fn show_delete_errors(win: &MainWindow, errors: &[String]) {
    let mut msg = errors.iter().take(10).cloned().collect::<Vec<_>>().join("\n\n");
    if errors.len() > 10 {
        msg.push_str(&format!("\n\n{} {} {}", crate::flc!("and_more_prefix"), errors.len() - 10, crate::flc!("and_more_suffix")));
    }
    win.global::<AppState>().set_delete_errors_text(SharedString::from(msg));
    win.global::<AppState>().set_delete_errors_visible(true);
}

pub(crate) fn handle_delete_event(win: &MainWindow, event: DeleteEvent) {
    match event {
        DeleteEvent::Progress(done, total) => {
            win.global::<AppState>().set_delete_progress_text(SharedString::from(format!("{done} / {total}")));
        }
        DeleteEvent::Finished(deleted, errors) => {
            win.global::<AppState>().set_delete_running(false);

            if !deleted.is_empty() {
                let del_set: std::collections::HashSet<String> = deleted.into_iter().collect();
                rebuild_similar_images_after_delete(win, &del_set);
            }

            let status = if errors.is_empty() {
                crate::flc!("status_deleted_selected")
            } else {
                crate::flc!("status_deleted_with_errors")
            };
            win.global::<AppState>().set_status_message(SharedString::from(status));

            if !errors.is_empty() {
                show_delete_errors(win, &errors);
            }
        }
        DeleteEvent::ListDeleteFinished(deleted, errors) => {
            win.global::<AppState>().set_delete_running(false);

            let del_set: std::collections::HashSet<String> = deleted.iter().cloned().collect();
            if !del_set.is_empty() {
                let tool = win.global::<AppState>().get_active_tool();
                let model = get_model_for_tool(win, tool);
                let vm = vm_file_entry(&model);
                let mut items: Vec<FileEntry> = vm.iter().collect();
                items.retain(|e| {
                    if e.is_header {
                        return true;
                    }
                    let name = e
                        .val_str
                        .row_data(0)
                        .map_or_else(|| panic!("Expected name in val_str[0] - {:?}", e.val_str), |s| s.to_string());
                    let path = e
                        .val_str
                        .row_data(1)
                        .map_or_else(|| panic!("Expected path in val_str[1] - {:?}", e.val_str), |s| s.to_string());
                    let full = if path.is_empty() { name } else { format!("{path}/{name}") };
                    !del_set.contains(&full)
                });

                loop {
                    let mut removed = false;
                    let mut i = 0;
                    while i < items.len() {
                        if items[i].is_header {
                            let group_len = items[i + 1..].iter().take_while(|e| !e.is_header).count();
                            if group_len <= 1 {
                                let end = i + 1 + group_len;
                                items.drain(i..end);
                                removed = true;
                                continue;
                            }
                        }
                        i += 1;
                    }
                    if !removed {
                        break;
                    }
                }
                vm.set_vec(items);
                win.global::<AppState>().set_selected_count(0);

                rebuild_similar_images_after_delete(win, &del_set);
            }

            let status = if errors.is_empty() {
                format!("{} {} {}", crate::flc!("deleted_items_prefix"), deleted.len(), crate::flc!("deleted_items_suffix"))
            } else {
                format!(
                    "{} {} {}, {} {}",
                    crate::flc!("deleted_items_prefix"),
                    deleted.len(),
                    crate::flc!("deleted_items_suffix"),
                    errors.len(),
                    crate::flc!("deleted_errors_suffix")
                )
            };
            win.global::<AppState>().set_status_message(SharedString::from(status));

            if !errors.is_empty() {
                show_delete_errors(win, &errors);
            }
        }
        DeleteEvent::ListRenameFinished(renamed, errors) => {
            win.global::<AppState>().set_delete_running(false);

            let model = win.get_bad_extensions_model();
            let vm = vm_file_entry(&model);
            let items: Vec<FileEntry> = vm.iter().filter(|e| !e.checked).collect();
            vm.set_vec(items);
            win.global::<AppState>().set_selected_count(0);

            let status = if errors.is_empty() {
                format!("{} {renamed} {}", crate::flc!("renamed_prefix"), crate::flc!("renamed_files_suffix"))
            } else {
                format!(
                    "{} {} {}, {} {}",
                    crate::flc!("renamed_prefix"),
                    renamed,
                    crate::flc!("renamed_files_suffix"),
                    errors.len(),
                    crate::flc!("renamed_errors_suffix")
                )
            };
            win.global::<AppState>().set_status_message(SharedString::from(status));

            if !errors.is_empty() {
                show_delete_errors(win, &errors);
            }
        }
        DeleteEvent::ExifCleanFinished(cleaned, errors) => {
            win.global::<AppState>().set_delete_running(false);

            let cleaned_set: std::collections::HashSet<String> = cleaned.iter().cloned().collect();
            if !cleaned_set.is_empty() {
                let model = win.get_exif_remover_model();
                let vm = vm_file_entry(&model);
                let items: Vec<FileEntry> = vm
                    .iter()
                    .filter(|e| {
                        if e.is_header {
                            return true;
                        }
                        let name = e.val_str.row_data(0).map(|s| s.to_string()).expect("Expected name in val_str[0]");
                        let path = e.val_str.row_data(1).map(|s| s.to_string()).expect("Expected path in val_str[1]");
                        let full = if path.is_empty() { name } else { format!("{path}/{name}") };
                        !cleaned_set.contains(&full)
                    })
                    .collect();
                vm.set_vec(items);
                win.global::<AppState>().set_selected_count(0);
            }

            let status = if errors.is_empty() {
                format!("{} {} {}", crate::flc!("cleaned_exif_prefix"), cleaned.len(), crate::flc!("cleaned_exif_suffix"))
            } else {
                format!(
                    "{} {} {}, {} {}",
                    crate::flc!("cleaned_exif_prefix"),
                    cleaned.len(),
                    crate::flc!("cleaned_exif_suffix"),
                    errors.len(),
                    crate::flc!("cleaned_exif_errors_suffix")
                )
            };
            win.global::<AppState>().set_status_message(SharedString::from(status));

            if !errors.is_empty() {
                show_delete_errors(win, &errors);
            }
        }
    }
}
