use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::callbacks::selection_ops::{
    select_all_except_highest_resolution, select_all_except_largest, select_all_except_lowest_resolution, select_all_except_smallest, select_except_one_per_group,
    select_highest_resolution_per_group, select_largest_per_group, select_lowest_resolution_per_group, select_smallest_per_group, set_all_checked, vm_of,
};
use crate::file_actions::{DeleteEvent, delete_path, execute_clean_exif_selected, execute_delete_selected, execute_rename_bad_names, execute_rename_selected};
use crate::model::{count_checked, toggle_row};
use crate::{ActiveTool, AppState, ConfirmPopupAction, FileEntry, MainWindow, SimilarGroupCard, SimilarImageItem};

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
            let vm = vm_of(&model);
            let mut items: Vec<FileEntry> = vm.iter().collect::<Vec<_>>();
            for e in &mut items {
                if !e.is_header && !e.is_reference {
                    e.checked = !e.checked;
                }
            }
            vm.set_vec(items);
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

fn sync_gallery_if_similar(win: &MainWindow, tool: ActiveTool) {
    if tool == ActiveTool::SimilarImages {
        sync_gallery_checked_from_flat(win);
    }
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
        if changed {
            let vm = group
                .items
                .as_any()
                .downcast_ref::<VecModel<SimilarImageItem>>()
                .expect("SimilarImageItem model must be backed by a VecModel");
            vm.set_vec(items);
        }
    }
}
