use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::common::{INT_IDX_SIZE_HI, INT_IDX_SIZE_LO, IntDataSimilarImages, STR_IDX_NAME, STR_IDX_PATH, StrDataBadExtensions, StrDataBadNames};
use crate::model::{count_checked, toggle_row};
use crate::{ActiveTool, AppState, ConfirmPopupAction, FileEntry, MainWindow, SimilarGroupCard, SimilarImageItem};

#[cfg(not(target_os = "android"))]
fn delete_path(path: &str) -> Result<(), String> {
    trash::delete(path).map_err(|e| e.to_string())
}

#[cfg(target_os = "android")]
fn delete_path(path: &str) -> Result<(), String> {
    std::fs::remove_file(path).or_else(|_| std::fs::remove_dir_all(path)).map_err(|e| e.to_string())
}

pub(crate) enum DeleteEvent {
    Progress(usize, usize),

    Finished(Vec<String>, Vec<String>),

    ListDeleteFinished(Vec<String>, Vec<String>),

    ListRenameFinished(usize, Vec<String>),

    ExifCleanFinished(Vec<String>, Vec<String>),
}

fn vm_of(model: &ModelRc<FileEntry>) -> Option<&VecModel<FileEntry>> {
    model.as_any().downcast_ref::<VecModel<FileEntry>>()
}

fn size_from_entry(e: &FileEntry) -> u64 {
    let hi = get_val_int(e, INT_IDX_SIZE_HI) as u64;
    let lo = get_val_int(e, INT_IDX_SIZE_LO) as u64;
    (hi << 32) | (lo & 0xFFFF_FFFF)
}

fn get_val_str(e: &FileEntry, idx: usize) -> String {
    e.val_str
        .row_data(idx)
        .unwrap_or_else(|| panic!("get_val_str: val_str[{idx}] missing, full val_str={:?}", e.val_str.iter().collect::<Vec<_>>()))
        .to_string()
}

fn get_val_int(e: &FileEntry, idx: usize) -> i32 {
    e.val_int
        .row_data(idx)
        .unwrap_or_else(|| panic!("get_val_int: val_int[{idx}] missing, full val_int={:?}", e.val_int.iter().collect::<Vec<_>>()))
}

fn full_path_of(e: &FileEntry) -> String {
    let name = get_val_str(e, STR_IDX_NAME);
    let path = get_val_str(e, STR_IDX_PATH);
    if path.is_empty() { name } else { format!("{path}/{name}") }
}

fn execute_delete_selected(win: &MainWindow, tx: std::sync::mpsc::Sender<DeleteEvent>) {
    let tool = win.global::<AppState>().get_active_tool();
    let model = get_model_for_tool(win, tool);
    let Some(vm) = vm_of(&model) else { return };

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

fn execute_rename_selected(win: &MainWindow, tx: std::sync::mpsc::Sender<DeleteEvent>) {
    let model = win.get_bad_extensions_model();
    let Some(vm) = vm_of(&model) else { return };

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
                    errors.push(format!("{full}\n  Nie można odczytać nazwy pliku"));
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

fn execute_rename_bad_names(win: &MainWindow, tx: std::sync::mpsc::Sender<DeleteEvent>) {
    let model = win.get_bad_names_model();
    let Some(vm) = vm_of(&model) else { return };

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
                    errors.push(format!("{full}\n  Nie można odczytać katalogu"));
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
fn execute_clean_exif_selected(win: &MainWindow, tx: std::sync::mpsc::Sender<DeleteEvent>) {
    let model = win.get_exif_remover_model();
    let Some(vm) = vm_of(&model) else { return };

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

pub(crate) fn wire_selection(window: &MainWindow, delete_tx: std::sync::mpsc::Sender<DeleteEvent>, delete_stop: Rc<std::cell::RefCell<Arc<AtomicBool>>>) {
    {
        let weak = window.as_weak();
        let tx = delete_tx.clone();
        window.global::<AppState>().on_clean_exif_selected(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_clean_exif_selected");
            let model = win.get_exif_remover_model();
            let n = count_checked(&model);
            if n == 0 {
                return;
            }
            let state = win.global::<AppState>();
            state.set_confirm_popup_message(slint::SharedString::from(crate::flc!("confirm_clean_exif", n = n)));
            state.set_confirm_popup_action(ConfirmPopupAction::CleanExif);
            state.set_confirm_popup_visible(true);
            let _ = tx.clone();
        });
    }
    {
        let weak = window.as_weak();
        let tx = delete_tx.clone();
        window.global::<AppState>().on_delete_selected(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_delete_selected");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            let n = count_checked(&model);
            if n == 0 {
                return;
            }
            let state = win.global::<AppState>();
            state.set_confirm_popup_message(slint::SharedString::from(crate::flc!("confirm_delete_items", n = n)));
            state.set_confirm_popup_action(ConfirmPopupAction::Delete);
            state.set_confirm_popup_visible(true);
            let _ = tx.clone();
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_all(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_all");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            set_all_checked(&model, true);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_deselect_all(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_deselect_all");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            set_all_checked(&model, false);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(0);
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_all_except_one(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_all_except_one");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_except_one_per_group(&model, true);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_deselect_all_except_one(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_deselect_all_except_one");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_except_one_per_group(&model, false);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_invert_selection(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_invert_selection");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            if let Some(vm) = vm_of(&model) {
                let mut items: Vec<FileEntry> = vm.iter().collect::<Vec<_>>();
                for e in &mut items {
                    if !e.is_header && !e.is_reference {
                        e.checked = !e.checked;
                    }
                }
                vm.set_vec(items);
            }
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_largest_per_group(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_largest_per_group");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_largest_per_group(&model);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_all_except_largest(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_all_except_largest");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_all_except_largest(&model);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_smallest_per_group(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_smallest_per_group");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_smallest_per_group(&model);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_all_except_smallest(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_all_except_smallest");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_all_except_smallest(&model);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_highest_resolution_per_group(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_highest_resolution_per_group");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_highest_resolution_per_group(&model);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_all_except_highest_resolution(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_all_except_highest_resolution");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_all_except_highest_resolution(&model);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_lowest_resolution_per_group(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_lowest_resolution_per_group");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_lowest_resolution_per_group(&model);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_all_except_lowest_resolution(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_select_all_except_lowest_resolution");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            select_all_except_lowest_resolution(&model);
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_toggle_file_checked(move |idx| {
            let win = weak.upgrade().expect("MainWindow dropped in on_toggle_file_checked");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            toggle_row(&model, idx as usize);
            if tool == ActiveTool::SimilarImages {
                sync_gallery_checked_from_flat(&win);
            }
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_request_gallery_delete(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_request_gallery_delete");
            let groups: Vec<SimilarGroupCard> = win.get_similar_images_groups().iter().collect::<Vec<_>>();

            let mut total_images = 0i32;
            let mut total_groups = 0i32;
            let mut unsafe_groups = 0i32;

            for group in &groups {
                let items: Vec<SimilarImageItem> = group.items.iter().collect::<Vec<_>>();
                let checked = items.iter().filter(|it| it.checked).count();
                if checked > 0 {
                    total_groups += 1;
                    total_images += checked as i32;
                    if checked == items.len() {
                        unsafe_groups += 1;
                    }
                }
            }

            let msg = slint::SharedString::from(crate::flc!("gallery_confirm_delete_msg", total_images = total_images, total_groups = total_groups));
            let warn = if unsafe_groups > 0 {
                slint::SharedString::from(crate::flc!("gallery_confirm_delete_warning", unsafe_groups = unsafe_groups))
            } else {
                slint::SharedString::default()
            };

            let state = win.global::<AppState>();
            state.set_gallery_delete_message(msg);
            state.set_gallery_delete_warning(warn);
            state.set_gallery_delete_popup_visible(true);
        });
    }
    {
        let weak = window.as_weak();
        let tx = delete_tx.clone();
        let stop_cell = Rc::clone(&delete_stop);
        window.global::<AppState>().on_confirm_gallery_delete(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_confirm_gallery_delete");

            let files: Vec<String> = win
                .get_similar_images_groups()
                .iter()
                .flat_map(|g: SimilarGroupCard| {
                    g.items
                        .iter()
                        .filter(|it: &SimilarImageItem| it.checked)
                        .map(|it: SimilarImageItem| it.full_path.to_string())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            if files.is_empty() {
                win.global::<AppState>().set_gallery_delete_popup_visible(false);
                return;
            }

            let new_stop = Arc::new(AtomicBool::new(false));
            *stop_cell.borrow_mut() = new_stop.clone();

            let state = win.global::<AppState>();
            state.set_gallery_delete_popup_visible(false);
            state.set_delete_running(true);
            state.set_delete_progress_text(slint::SharedString::from(format!("0 / {}", files.len())));

            let tx = tx.clone();
            let total = files.len();
            std::thread::spawn(move || {
                let mut deleted: Vec<String> = Vec::new();
                let mut errors: Vec<String> = Vec::new();

                for (i, path) in files.iter().enumerate() {
                    if new_stop.load(Ordering::Relaxed) {
                        break;
                    }
                    match delete_path(path) {
                        Ok(()) => deleted.push(path.clone()),
                        Err(e) => errors.push(format!("{path}\n  {e}")),
                    }
                    if i % 5 == 4 || i + 1 == total {
                        let _ = tx.send(DeleteEvent::Progress(i + 1, total));
                    }
                }
                let _ = tx.send(DeleteEvent::Finished(deleted, errors));
            });
        });
    }
    {
        window.global::<AppState>().on_delete_stop_requested(move || {
            delete_stop.borrow().store(true, Ordering::Relaxed);
        });
    }
    {
        let weak = window.as_weak();
        let tx = delete_tx.clone();
        window.global::<AppState>().on_rename_selected(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_rename_selected");
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            let n = count_checked(&model);
            if n == 0 {
                return;
            }
            let state = win.global::<AppState>();
            state.set_confirm_popup_message(slint::SharedString::from(crate::flc!("confirm_rename_items", n = n)));
            let action = if tool == ActiveTool::BadNames {
                ConfirmPopupAction::RenameBadNames
            } else {
                ConfirmPopupAction::Rename
            };
            state.set_confirm_popup_action(action);
            state.set_confirm_popup_visible(true);
            let _ = tx.clone();
        });
    }
    {
        let weak = window.as_weak();
        let tx_confirm = delete_tx;
        window.global::<AppState>().on_confirm_popup_ok(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_confirm_popup_ok");
            let action = win.global::<AppState>().get_confirm_popup_action();
            win.global::<AppState>().set_confirm_popup_visible(false);
            match action {
                ConfirmPopupAction::Delete => execute_delete_selected(&win, tx_confirm.clone()),
                ConfirmPopupAction::Rename => execute_rename_selected(&win, tx_confirm.clone()),
                ConfirmPopupAction::RenameBadNames => execute_rename_bad_names(&win, tx_confirm.clone()),
                ConfirmPopupAction::CleanExif => execute_clean_exif_selected(&win, tx_confirm.clone()),
                ConfirmPopupAction::None => {}
            }
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_confirm_popup_cancel(move || {
            weak.upgrade()
                .expect("MainWindow dropped in on_confirm_popup_cancel")
                .global::<AppState>()
                .set_confirm_popup_visible(false);
        });
    }
}

pub(crate) fn get_model_for_tool(win: &MainWindow, tool: ActiveTool) -> ModelRc<FileEntry> {
    match tool {
        ActiveTool::DuplicateFiles => win.get_duplicate_files_model(),
        ActiveTool::EmptyFolders => win.get_empty_folder_model(),
        ActiveTool::SimilarImages => win.get_similar_images_model(),
        ActiveTool::EmptyFiles => win.get_empty_files_model(),
        ActiveTool::TemporaryFiles => win.get_temporary_files_model(),
        ActiveTool::BigFiles => win.get_big_files_model(),
        ActiveTool::BrokenFiles => win.get_broken_files_model(),
        ActiveTool::BadExtensions => win.get_bad_extensions_model(),
        ActiveTool::SameMusic => win.get_same_music_model(),
        ActiveTool::BadNames => win.get_bad_names_model(),
        ActiveTool::ExifRemover => win.get_exif_remover_model(),
        ActiveTool::SimilarVideos => win.get_similar_videos_model(),
        ActiveTool::Home | ActiveTool::Directories | ActiveTool::Settings => ModelRc::new(VecModel::from(Vec::new())),
    }
}

pub(crate) fn set_all_checked(model: &ModelRc<FileEntry>, state: bool) {
    if let Some(vm) = vm_of(model) {
        let mut items: Vec<FileEntry> = vm.iter().collect::<Vec<_>>();
        for e in &mut items {
            if !e.is_header && !e.is_reference {
                e.checked = state;
            }
        }
        vm.set_vec(items);
    }
}

pub(crate) fn select_except_one_per_group(model: &ModelRc<FileEntry>, select: bool) {
    let Some(vm) = vm_of(model) else { return };
    let mut items: Vec<FileEntry> = vm.iter().collect::<Vec<_>>();
    let has_headers = items.iter().any(|e| e.is_header);

    if !has_headers {
        for e in &mut items {
            if !e.is_header {
                e.checked = select;
            }
        }
        vm.set_vec(items);
        return;
    }

    if select {
        let mut first_non_ref_in_group = false;
        for e in &mut items {
            if e.is_header {
                first_non_ref_in_group = true;
                continue;
            }
            if e.is_reference {
                continue;
            }
            e.checked = !std::mem::take(&mut first_non_ref_in_group);
        }
    } else {
        let mut i = 0;
        while i < items.len() {
            if items[i].is_header {
                let group_end = items[i + 1..].iter().position(|e| e.is_header).map_or(items.len(), |p| i + 1 + p);
                let checked_count = items[i + 1..group_end].iter().filter(|e| e.checked).count();
                if checked_count >= 2 {
                    let mut kept = false;
                    for j in i + 1..group_end {
                        if items[j].checked {
                            if kept {
                                items[j].checked = false;
                            } else {
                                kept = true;
                            }
                        }
                    }
                }
                i = group_end;
                continue;
            }
            i += 1;
        }
    }

    vm.set_vec(items);
}

fn sync_gallery_if_similar(win: &MainWindow, tool: ActiveTool) {
    if tool == ActiveTool::SimilarImages {
        sync_gallery_checked_from_flat(win);
    }
}

pub(crate) fn select_largest_per_group(model: &ModelRc<FileEntry>) {
    select_by_size_per_group(model, true, true);
}

pub(crate) fn select_all_except_largest(model: &ModelRc<FileEntry>) {
    select_by_size_per_group(model, true, false);
}

pub(crate) fn select_smallest_per_group(model: &ModelRc<FileEntry>) {
    select_by_size_per_group(model, false, true);
}

pub(crate) fn select_all_except_smallest(model: &ModelRc<FileEntry>) {
    select_by_size_per_group(model, false, false);
}

fn select_by_size_per_group(model: &ModelRc<FileEntry>, largest: bool, select_target: bool) {
    let Some(vm) = vm_of(model) else { return };
    let mut items: Vec<FileEntry> = vm.iter().collect();

    let mut i = 0;
    while i < items.len() {
        if items[i].is_header {
            let group_end = items[i + 1..].iter().position(|e| e.is_header).map_or(items.len(), |p| i + 1 + p);

            let target_idx = if largest {
                items[i + 1..group_end]
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| !e.is_reference)
                    .max_by_key(|(_, e)| size_from_entry(e))
                    .map(|(j, _)| i + 1 + j)
            } else {
                items[i + 1..group_end]
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| !e.is_reference)
                    .min_by_key(|(_, e)| size_from_entry(e))
                    .map(|(j, _)| i + 1 + j)
            };

            for j in i + 1..group_end {
                if items[j].is_reference {
                    continue;
                }
                let is_target = target_idx == Some(j);
                items[j].checked = if select_target { is_target } else { !is_target };
            }

            i = group_end;
            continue;
        }
        i += 1;
    }

    vm.set_vec(items);
}

fn resolution_from_entry(e: &FileEntry) -> u64 {
    let w = get_val_int(e, IntDataSimilarImages::Width as usize) as u64;
    let h = get_val_int(e, IntDataSimilarImages::Height as usize) as u64;
    w * h
}

fn select_by_resolution_per_group(model: &ModelRc<FileEntry>, highest: bool, select_target: bool) {
    let Some(vm) = vm_of(model) else { return };
    let mut items: Vec<FileEntry> = vm.iter().collect();

    let mut i = 0;
    while i < items.len() {
        if items[i].is_header {
            let group_end = items[i + 1..].iter().position(|e| e.is_header).map_or(items.len(), |p| i + 1 + p);

            let target_idx = if highest {
                items[i + 1..group_end]
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| !e.is_reference)
                    .max_by_key(|(_, e)| resolution_from_entry(e))
                    .map(|(j, _)| i + 1 + j)
            } else {
                items[i + 1..group_end]
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| !e.is_reference)
                    .min_by_key(|(_, e)| resolution_from_entry(e))
                    .map(|(j, _)| i + 1 + j)
            };

            for j in i + 1..group_end {
                if items[j].is_reference {
                    continue;
                }
                let is_target = target_idx == Some(j);
                items[j].checked = if select_target { is_target } else { !is_target };
            }

            i = group_end;
            continue;
        }
        i += 1;
    }
    vm.set_vec(items);
}

pub(crate) fn select_highest_resolution_per_group(model: &ModelRc<FileEntry>) {
    select_by_resolution_per_group(model, true, true);
}
pub(crate) fn select_all_except_highest_resolution(model: &ModelRc<FileEntry>) {
    select_by_resolution_per_group(model, true, false);
}
pub(crate) fn select_lowest_resolution_per_group(model: &ModelRc<FileEntry>) {
    select_by_resolution_per_group(model, false, true);
}
pub(crate) fn select_all_except_lowest_resolution(model: &ModelRc<FileEntry>) {
    select_by_resolution_per_group(model, false, false);
}
pub(crate) fn sync_gallery_checked_from_flat(win: &MainWindow) {
    let flat: Vec<FileEntry> = win.get_similar_images_model().iter().collect::<Vec<_>>();
    let groups: Vec<SimilarGroupCard> = win.get_similar_images_groups().iter().collect::<Vec<_>>();

    for group in &groups {
        let mut items: Vec<SimilarImageItem> = group.items.iter().collect::<Vec<_>>();
        let mut changed = false;
        for item in &mut items {
            if let Some(entry) = flat.get(item.flat_idx as usize)
                && item.checked != entry.checked
            {
                item.checked = entry.checked;
                changed = true;
            }
        }
        if changed && let Some(vm) = group.items.as_any().downcast_ref::<VecModel<SimilarImageItem>>() {
            vm.set_vec(items);
        }
    }
}
