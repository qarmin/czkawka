use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{Button, Entry};

use czkawka_core::common_traits::PrintResults;

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::BottomButtonsEnum;
use crate::notebook_enums::*;

pub fn connect_button_save(gui_data: &GuiData) {
    let buttons_save = gui_data.bottom_buttons.buttons_save.clone();
    let buttons_save_clone = gui_data.bottom_buttons.buttons_save.clone();
    let shared_duplication_state = gui_data.shared_duplication_state.clone();
    let shared_empty_folders_state = gui_data.shared_empty_folders_state.clone();
    let shared_big_files_state = gui_data.shared_big_files_state.clone();
    let shared_temporary_files_state = gui_data.shared_temporary_files_state.clone();
    let shared_empty_files_state = gui_data.shared_empty_files_state.clone();
    let shared_similar_images_state = gui_data.shared_similar_images_state.clone();
    let shared_similar_videos_state = gui_data.shared_similar_videos_state.clone();
    let shared_same_music_state = gui_data.shared_same_music_state.clone();
    let shared_same_invalid_symlinks = gui_data.shared_same_invalid_symlinks.clone();
    let shared_broken_files_state = gui_data.shared_broken_files_state.clone();
    let shared_bad_extensions_state = gui_data.shared_bad_extensions_state.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
    let entry_info = gui_data.entry_info.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    buttons_save.connect_clicked(move |_| {
        let result = match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
            NotebookMainEnum::Duplicate => shared_duplication_state.borrow().save_all_in_one("results_duplicates"),
            NotebookMainEnum::EmptyDirectories => shared_empty_folders_state.borrow().save_all_in_one("results_empty_directories"),
            NotebookMainEnum::EmptyFiles => shared_empty_files_state.borrow().save_all_in_one("results_empty_files"),
            NotebookMainEnum::Temporary => shared_temporary_files_state.borrow().save_all_in_one("results_temporary_files"),
            NotebookMainEnum::BigFiles => shared_big_files_state.borrow().save_all_in_one("results_big_files"),
            NotebookMainEnum::SimilarImages => shared_similar_images_state.borrow().save_all_in_one("results_similar_images"),
            NotebookMainEnum::SimilarVideos => shared_similar_videos_state.borrow().save_all_in_one("results_similar_videos"),
            NotebookMainEnum::SameMusic => shared_same_music_state.borrow().save_all_in_one("results_same_music"),
            NotebookMainEnum::Symlinks => shared_same_invalid_symlinks.borrow().save_all_in_one("results_invalid_symlinks"),
            NotebookMainEnum::BrokenFiles => shared_broken_files_state.borrow().save_all_in_one("results_broken_files"),
            NotebookMainEnum::BadExtensions => shared_bad_extensions_state.borrow().save_all_in_one("results_bad_extensions"),
        };

        match result {
            Ok(()) => (),
            Err(e) => {
                entry_info.set_text(&format!("Failed to save results to file {e}"));
                return;
            }
        }

        post_save_things(
            &to_notebook_main_enum(notebook_main.current_page().unwrap()),
            &shared_buttons,
            &entry_info,
            &buttons_save_clone,
        );
    });
}

fn post_save_things(
    type_of_tab: &NotebookMainEnum,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    entry_info: &Entry,
    buttons_save: &Button,
) {
    entry_info.set_text(&flg!("save_results_to_file"));
    // Set state
    {
        buttons_save.hide();
        *shared_buttons.borrow_mut().get_mut(type_of_tab).unwrap().get_mut(&BottomButtonsEnum::Save).unwrap() = false;
    }
}
