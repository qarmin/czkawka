use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::model::{count_checked, toggle_row};
use crate::{ActiveTool, AppState, FileEntry, MainWindow};

pub(crate) enum DeleteEvent {
    Progress(usize, usize),
    Finished(Vec<String>, Vec<String>),
}

pub(crate) fn wire_selection(window: &MainWindow, delete_tx: std::sync::mpsc::Sender<DeleteEvent>, delete_stop: Rc<std::cell::RefCell<Arc<AtomicBool>>>) {
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_delete_selected(move || {
            let win = weak.unwrap();
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);

            let mut deleted_indices: Vec<usize> = Vec::new();
            let mut errors: Vec<String> = Vec::new();

            for i in 0..model.row_count() {
                let Some(e) = model.row_data(i) else { continue };
                if !e.checked {
                    continue;
                }
                let full = if e.path.is_empty() { e.name.to_string() } else { format!("{}/{}", e.path, e.name) };

                let result = std::fs::remove_file(&full).or_else(|_| std::fs::remove_dir_all(&full));

                match result {
                    Ok(()) => deleted_indices.push(i),
                    Err(err) => errors.push(format!("{full}\n  {err}")),
                }
            }

            let model_vec = model.as_any().downcast_ref::<VecModel<FileEntry>>();
            if let Some(vm) = model_vec {
                for &idx in deleted_indices.iter().rev() {
                    vm.remove(idx);
                }

                let mut to_remove_headers: Vec<usize> = Vec::new();
                let n = vm.row_count();
                let mut i = 0;
                while i < n {
                    if let Some(entry) = vm.row_data(i) {
                        if entry.is_header {
                            let next_is_non_header = vm.row_data(i + 1).map(|e| !e.is_header).unwrap_or(false);
                            if !next_is_non_header {
                                to_remove_headers.push(i);
                            }
                        }
                    }
                    i += 1;
                }
                for &idx in to_remove_headers.iter().rev() {
                    vm.remove(idx);
                }
            }

            win.global::<AppState>().set_selected_count(count_checked(&model));

            let deleted = deleted_indices.len();
            let status = if errors.is_empty() {
                format!("Usunięto {} elementów", deleted)
            } else {
                format!("Usunięto {deleted} elementów, {} błędów", errors.len())
            };
            win.global::<AppState>().set_status_message(slint::SharedString::from(status));

            if !errors.is_empty() {
                let displayed: Vec<&String> = errors.iter().take(10).collect();
                let mut msg = displayed.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n\n");
                if errors.len() > 10 {
                    msg.push_str(&format!("\n\n…i {} więcej", errors.len() - 10));
                }
                win.global::<AppState>().set_delete_errors_text(slint::SharedString::from(msg));
                win.global::<AppState>().set_delete_errors_visible(true);
            }
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_select_all(move || {
            let win = weak.unwrap();
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
            let win = weak.unwrap();
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
            let win = weak.unwrap();
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
            let win = weak.unwrap();
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
            let win = weak.unwrap();
            let tool = win.global::<AppState>().get_active_tool();
            let model = get_model_for_tool(&win, tool);
            for i in 0..model.row_count() {
                toggle_row(&model, i);
            }
            sync_gallery_if_similar(&win, tool);
            win.global::<AppState>().set_selected_count(count_checked(&model));
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_toggle_file_checked(move |idx| {
            let win = weak.unwrap();
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
            let win = weak.unwrap();
            let groups = win.get_similar_images_groups();
            let mut total_images = 0i32;
            let mut total_groups = 0i32;
            let mut unsafe_groups = 0i32;

            for gi in 0..groups.row_count() {
                if let Some(group) = groups.row_data(gi) {
                    let n = group.items.row_count();
                    let checked = (0..n).filter(|&ii| group.items.row_data(ii).map(|it| it.checked).unwrap_or(false)).count();
                    if checked > 0 {
                        total_groups += 1;
                        total_images += checked as i32;
                        if checked == n {
                            unsafe_groups += 1;
                        }
                    }
                }
            }

            let msg = slint::SharedString::from(format!("Zamierzasz usunąć {} obrazów w {} grupach?", total_images, total_groups));
            let warn = if unsafe_groups > 0 {
                slint::SharedString::from(format!("⚠ W {} grupach zaznaczono wszystkie elementy!", unsafe_groups))
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
        let tx = delete_tx;
        let stop_cell = Rc::clone(&delete_stop);
        window.global::<AppState>().on_confirm_gallery_delete(move || {
            let win = weak.unwrap();

            let groups = win.get_similar_images_groups();
            let mut files: Vec<String> = Vec::new();
            for gi in 0..groups.row_count() {
                if let Some(group) = groups.row_data(gi) {
                    for ii in 0..group.items.row_count() {
                        if let Some(item) = group.items.row_data(ii) {
                            if item.checked {
                                files.push(item.full_path.to_string());
                            }
                        }
                    }
                }
            }
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

                    match std::fs::remove_file(path) {
                        Ok(()) => deleted.push(path.clone()),
                        Err(e) => errors.push(format!("{}\n  {}", path, e)),
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
        ActiveTool::InvalidSymlinks => win.get_invalid_symlinks_model(),
        ActiveTool::BadExtensions => win.get_bad_extensions_model(),
        ActiveTool::SameMusic => win.get_same_music_model(),
        ActiveTool::Home | ActiveTool::Directories | ActiveTool::Settings => ModelRc::new(VecModel::from(vec![])),
    }
}

pub(crate) fn set_all_checked(model: &ModelRc<FileEntry>, state: bool) {
    for i in 0..model.row_count() {
        if let Some(mut entry) = model.row_data(i) {
            if !entry.is_header {
                entry.checked = state;
                model.set_row_data(i, entry);
            }
        }
    }
}

pub(crate) fn select_except_one_per_group(model: &ModelRc<FileEntry>, select: bool) {
    let has_headers = (0..model.row_count()).any(|i| model.row_data(i).map(|e| e.is_header).unwrap_or(false));

    if !has_headers {
        set_all_checked(model, select);
        return;
    }

    let mut first_in_group = false;
    for i in 0..model.row_count() {
        if let Some(mut entry) = model.row_data(i) {
            if entry.is_header {
                first_in_group = true;
                continue;
            }
            let new_checked = if first_in_group {
                first_in_group = false;
                !select
            } else {
                select
            };
            if entry.checked != new_checked {
                entry.checked = new_checked;
                model.set_row_data(i, entry);
            }
        }
    }
}

fn sync_gallery_if_similar(win: &MainWindow, tool: ActiveTool) {
    if tool == ActiveTool::SimilarImages {
        sync_gallery_checked_from_flat(win);
    }
}

pub(crate) fn sync_gallery_checked_from_flat(win: &MainWindow) {
    let flat = win.get_similar_images_model();
    let groups = win.get_similar_images_groups();
    for gi in 0..groups.row_count() {
        if let Some(group) = groups.row_data(gi) {
            for ii in 0..group.items.row_count() {
                if let Some(mut item) = group.items.row_data(ii) {
                    if let Some(entry) = flat.row_data(item.flat_idx as usize) {
                        if item.checked != entry.checked {
                            item.checked = entry.checked;
                            group.items.set_row_data(ii, item);
                        }
                    }
                }
            }
        }
    }
}
