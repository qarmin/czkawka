use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use chrono::DateTime;
use crossbeam_channel::Receiver;
use czkawka_core::bad_extensions::BadExtensions;
use czkawka_core::big_file::BigFile;
use czkawka_core::broken_files::BrokenFiles;
use czkawka_core::common::{split_path, split_path_compare};
use czkawka_core::common_dir_traversal::CheckingMethod;
use czkawka_core::common_tool::CommonData;
use czkawka_core::common_traits::ResultEntry;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::{MusicSimilarity, SameMusic};
use czkawka_core::similar_images;
use czkawka_core::similar_images::{ImagesEntry, SimilarImages};
use czkawka_core::similar_videos::SimilarVideos;
use czkawka_core::temporary::Temporary;
use fun_time::fun_time;
use gtk4::prelude::*;
use gtk4::{Entry, ListStore, TextView, TreeView, Widget};
use humansize::{format_size, BINARY};
use rayon::prelude::*;

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_combo_box::IMAGES_HASH_SIZE_COMBO_BOX;
use crate::help_functions::*;
use crate::notebook_enums::*;
use crate::notebook_info::NOTEBOOKS_INFO;
use crate::opening_selecting_records::*;

pub fn connect_compute_results(gui_data: &GuiData, result_receiver: Receiver<Message>) {
    let combo_box_image_hash_size = gui_data.main_notebook.combo_box_image_hash_size.clone();
    let buttons_search = gui_data.bottom_buttons.buttons_search.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let entry_info = gui_data.entry_info.clone();
    let tree_view_empty_folder_finder = gui_data.main_notebook.tree_view_empty_folder_finder.clone();
    let tree_view_empty_files_finder = gui_data.main_notebook.tree_view_empty_files_finder.clone();
    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_similar_videos_finder = gui_data.main_notebook.tree_view_similar_videos_finder.clone();
    let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let shared_duplication_state = gui_data.shared_duplication_state.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
    let shared_empty_folders_state = gui_data.shared_empty_folders_state.clone();
    let shared_empty_files_state = gui_data.shared_empty_files_state.clone();
    let shared_broken_files_state = gui_data.shared_broken_files_state.clone();
    let tree_view_big_files_finder = gui_data.main_notebook.tree_view_big_files_finder.clone();
    let tree_view_broken_files = gui_data.main_notebook.tree_view_broken_files.clone();
    let tree_view_invalid_symlinks = gui_data.main_notebook.tree_view_invalid_symlinks.clone();
    let shared_big_files_state = gui_data.shared_big_files_state.clone();
    let shared_same_invalid_symlinks = gui_data.shared_same_invalid_symlinks.clone();
    let tree_view_temporary_files_finder = gui_data.main_notebook.tree_view_temporary_files_finder.clone();
    let tree_view_bad_extensions = gui_data.main_notebook.tree_view_bad_extensions.clone();
    let shared_temporary_files_state = gui_data.shared_temporary_files_state.clone();
    let shared_similar_images_state = gui_data.shared_similar_images_state.clone();
    let shared_similar_videos_state = gui_data.shared_similar_videos_state.clone();
    let shared_bad_extensions_state = gui_data.shared_bad_extensions_state.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let shared_same_music_state = gui_data.shared_same_music_state.clone();
    let buttons_names = gui_data.bottom_buttons.buttons_names;
    let window_progress = gui_data.progress_window.window_progress.clone();
    let taskbar_state = gui_data.taskbar_state.clone();
    let notebook_upper = gui_data.upper_notebook.notebook_upper.clone();
    let button_settings = gui_data.header.button_settings.clone();
    let button_app_info = gui_data.header.button_app_info.clone();

    let main_context = glib::MainContext::default();
    let _guard = main_context.acquire().expect("Failed to acquire main context");

    glib::spawn_future_local(async move {
        loop {
            loop {
                let msg = result_receiver.try_recv();
                if let Ok(msg) = msg {
                    buttons_search.show();

                    notebook_main.set_sensitive(true);
                    notebook_upper.set_sensitive(true);
                    button_settings.set_sensitive(true);
                    button_app_info.set_sensitive(true);

                    window_progress.hide();

                    taskbar_state.borrow().hide();

                    let hash_size_index = combo_box_image_hash_size.active().expect("Failed to get active item") as usize;
                    let hash_size = IMAGES_HASH_SIZE_COMBO_BOX[hash_size_index] as u8;

                    match msg {
                        Message::Duplicates(df) => {
                            compute_duplicate_finder(
                                df,
                                &entry_info,
                                &tree_view_duplicate_finder,
                                &text_view_errors,
                                &shared_duplication_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                        Message::EmptyFolders(ef) => {
                            compute_empty_folders(
                                ef,
                                &entry_info,
                                &tree_view_empty_folder_finder,
                                &text_view_errors,
                                &shared_empty_folders_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                        Message::EmptyFiles(vf) => {
                            compute_empty_files(
                                vf,
                                &entry_info,
                                &tree_view_empty_files_finder,
                                &text_view_errors,
                                &shared_empty_files_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                        Message::BigFiles(bf) => {
                            compute_big_files(
                                bf,
                                &entry_info,
                                &tree_view_big_files_finder,
                                &text_view_errors,
                                &shared_big_files_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                        Message::Temporary(tf) => {
                            compute_temporary_files(
                                tf,
                                &entry_info,
                                &tree_view_temporary_files_finder,
                                &text_view_errors,
                                &shared_temporary_files_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                        Message::SimilarImages(sf) => {
                            compute_similar_images(
                                sf,
                                &entry_info,
                                &tree_view_similar_images_finder,
                                &text_view_errors,
                                &shared_similar_images_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                                hash_size,
                            );
                        }
                        Message::SimilarVideos(ff) => {
                            compute_similar_videos(
                                ff,
                                &entry_info,
                                &tree_view_similar_videos_finder,
                                &text_view_errors,
                                &shared_similar_videos_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                        Message::SameMusic(mf) => {
                            compute_same_music(
                                mf,
                                &entry_info,
                                &tree_view_same_music_finder,
                                &text_view_errors,
                                &shared_same_music_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                        Message::InvalidSymlinks(ifs) => {
                            compute_invalid_symlinks(
                                ifs,
                                &entry_info,
                                &tree_view_invalid_symlinks,
                                &text_view_errors,
                                &shared_same_invalid_symlinks,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                        Message::BrokenFiles(br) => {
                            compute_broken_files(
                                br,
                                &entry_info,
                                &tree_view_broken_files,
                                &text_view_errors,
                                &shared_broken_files_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                        Message::BadExtensions(be) => {
                            compute_bad_extensions(
                                be,
                                &entry_info,
                                &tree_view_bad_extensions,
                                &text_view_errors,
                                &shared_bad_extensions_state,
                                &shared_buttons,
                                &buttons_array,
                                &buttons_names,
                            );
                        }
                    }
                } else {
                    break;
                }
            }
            glib::timeout_future(Duration::from_millis(300)).await;
        }
    });
}

#[fun_time(message = "compute_bad_extensions", level = "debug")]
fn compute_bad_extensions(
    be: BadExtensions,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<BadExtensions>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    const COLUMNS_NUMBER: usize = 7;
    if be.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        let information = be.get_information();
        let text_messages = be.get_text_messages();

        let bad_extensions_number: usize = information.number_of_files_with_bad_extension;
        entry_info.set_text(flg!("compute_found_bad_extensions", number_files = bad_extensions_number).as_str());

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            let vector = be.get_bad_extensions_files();

            // Sort
            let mut vector = vector.clone();
            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            for file_entry in vector {
                let (directory, file) = split_path(&file_entry.path);
                let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
                    (ColumnsBadExtensions::SelectionButton as u32, &false),
                    (ColumnsBadExtensions::Name as u32, &file),
                    (ColumnsBadExtensions::Path as u32, &directory),
                    (ColumnsBadExtensions::CurrentExtension as u32, &file_entry.current_extension),
                    (ColumnsBadExtensions::ValidExtensions as u32, &file_entry.proper_extensions),
                    (
                        ColumnsBadExtensions::Modification as u32,
                        &(DateTime::from_timestamp(file_entry.modified_date as i64, 0)
                            .expect("Modified date always should be in valid range")
                            .to_string()),
                    ),
                    (ColumnsBadExtensions::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                ];
                list_store.set(&list_store.append(), &values);
            }
            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(be);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::Temporary, bad_extensions_number > 0);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_broken_files", level = "debug")]
fn compute_broken_files(
    br: BrokenFiles,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<BrokenFiles>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    const COLUMNS_NUMBER: usize = 6;
    if br.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        let information = br.get_information();
        let text_messages = br.get_text_messages();

        let broken_files_number: usize = information.number_of_broken_files;

        entry_info.set_text(flg!("compute_found_broken_files", number_files = broken_files_number).as_str());

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            let vector = br.get_broken_files();

            // Sort
            let mut vector = vector.clone();
            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            for file_entry in vector {
                let (directory, file) = split_path(&file_entry.path);
                let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
                    (ColumnsBrokenFiles::SelectionButton as u32, &false),
                    (ColumnsBrokenFiles::Name as u32, &file),
                    (ColumnsBrokenFiles::Path as u32, &directory),
                    (ColumnsBrokenFiles::ErrorType as u32, &file_entry.error_string),
                    (
                        ColumnsBrokenFiles::Modification as u32,
                        &(DateTime::from_timestamp(file_entry.modified_date as i64, 0)
                            .expect("Modified date always should be in valid range")
                            .to_string()),
                    ),
                    (ColumnsBrokenFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                ];
                list_store.set(&list_store.append(), &values);
            }
            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(br);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::BrokenFiles, broken_files_number > 0);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_invalid_symlinks", level = "debug")]
fn compute_invalid_symlinks(
    ifs: InvalidSymlinks,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<InvalidSymlinks>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    const COLUMNS_NUMBER: usize = 7;
    if ifs.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        let information = ifs.get_information();
        let text_messages = ifs.get_text_messages();

        let invalid_symlinks: usize = information.number_of_invalid_symlinks;

        entry_info.set_text(flg!("compute_found_invalid_symlinks", number_files = invalid_symlinks).as_str());

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            let vector = vector_sort_simple_unstable_entry_by_path(ifs.get_invalid_symlinks());

            for file_entry in vector {
                let (directory, file) = split_path(&file_entry.path);
                let symlink_info = file_entry.symlink_info;
                let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
                    (ColumnsInvalidSymlinks::SelectionButton as u32, &false),
                    (ColumnsInvalidSymlinks::Name as u32, &file),
                    (ColumnsInvalidSymlinks::Path as u32, &directory),
                    (ColumnsInvalidSymlinks::DestinationPath as u32, &symlink_info.destination_path.to_string_lossy().to_string()),
                    (ColumnsInvalidSymlinks::TypeOfError as u32, &get_text_from_invalid_symlink_cause(symlink_info.type_of_error)),
                    (
                        ColumnsInvalidSymlinks::Modification as u32,
                        &(DateTime::from_timestamp(file_entry.modified_date as i64, 0)
                            .expect("Modified date always should be in valid range")
                            .to_string()),
                    ),
                    (ColumnsInvalidSymlinks::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                ];
                list_store.set(&list_store.append(), &values);
            }
            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(ifs);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::Symlinks, invalid_symlinks > 0);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Symlinks).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_same_music", level = "debug")]
fn compute_same_music(
    mf: SameMusic,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<SameMusic>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    if mf.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        if mf.get_use_reference() {
            tree_view.selection().set_select_function(select_function_always_true);
        } else {
            tree_view.selection().set_select_function(select_function_same_music);
        }

        let information = mf.get_information();
        let text_messages = mf.get_text_messages();

        let same_music_number: usize = information.number_of_duplicates;

        entry_info.set_text(
            flg!(
                "compute_found_music",
                number_files = information.number_of_duplicates,
                number_groups = information.number_of_groups
            )
            .as_str(),
        );

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            let music_similarity = mf.get_params().music_similarity;

            let is_track_title = (MusicSimilarity::TRACK_TITLE & music_similarity) != MusicSimilarity::NONE;
            let is_track_artist = (MusicSimilarity::TRACK_ARTIST & music_similarity) != MusicSimilarity::NONE;
            let is_year = (MusicSimilarity::YEAR & music_similarity) != MusicSimilarity::NONE;
            let is_bitrate = (MusicSimilarity::BITRATE & music_similarity) != MusicSimilarity::NONE;
            let is_length = (MusicSimilarity::LENGTH & music_similarity) != MusicSimilarity::NONE;
            let is_genre = (MusicSimilarity::GENRE & music_similarity) != MusicSimilarity::NONE;

            if mf.get_use_reference() {
                let vector = mf.get_similar_music_referenced();

                for (base_file_entry, vec_file_entry) in vector {
                    // Sort
                    let vec_file_entry = if vec_file_entry.len() >= 2 {
                        let mut vec_file_entry = vec_file_entry.clone();
                        vec_file_entry.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));
                        vec_file_entry
                    } else {
                        vec_file_entry.clone()
                    };

                    let (directory, file) = split_path(&base_file_entry.path);
                    same_music_add_to_list_store(
                        &list_store,
                        &file,
                        &directory,
                        base_file_entry.size,
                        base_file_entry.modified_date,
                        &base_file_entry.track_title,
                        &base_file_entry.track_artist,
                        &base_file_entry.year,
                        base_file_entry.bitrate,
                        &format!("{} kbps", base_file_entry.bitrate),
                        &base_file_entry.genre,
                        &base_file_entry.length,
                        true,
                        true,
                    );
                    for file_entry in vec_file_entry {
                        let (directory, file) = split_path(&file_entry.path);
                        same_music_add_to_list_store(
                            &list_store,
                            &file,
                            &directory,
                            file_entry.size,
                            file_entry.modified_date,
                            &file_entry.track_title,
                            &file_entry.track_artist,
                            &file_entry.year,
                            file_entry.bitrate,
                            &format!("{} kbps", file_entry.bitrate),
                            &file_entry.genre,
                            &file_entry.length,
                            false,
                            true,
                        );
                    }
                }
            } else {
                let vector = mf.get_duplicated_music_entries();

                let text: &str = if mf.get_params().check_type == CheckingMethod::AudioTags { "-----" } else { "" };

                for vec_file_entry in vector {
                    // Sort
                    let vec_file_entry = if vec_file_entry.len() >= 2 {
                        let mut vec_file_entry = vec_file_entry.clone();
                        vec_file_entry.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));
                        vec_file_entry
                    } else {
                        vec_file_entry.clone()
                    };

                    same_music_add_to_list_store(
                        &list_store,
                        "",
                        "",
                        0,
                        0,
                        if is_track_title { text } else { "" },
                        if is_track_artist { text } else { "" },
                        if is_year { text } else { "" },
                        0,
                        if is_bitrate { text } else { "" },
                        if is_genre { text } else { "" },
                        if is_length { text } else { "" },
                        true,
                        false,
                    );
                    for file_entry in vec_file_entry {
                        let (directory, file) = split_path(&file_entry.path);
                        same_music_add_to_list_store(
                            &list_store,
                            &file,
                            &directory,
                            file_entry.size,
                            file_entry.modified_date,
                            &file_entry.track_title,
                            &file_entry.track_artist,
                            &file_entry.year,
                            file_entry.bitrate,
                            &format!("{} kbps", file_entry.bitrate),
                            &file_entry.genre,
                            &file_entry.length,
                            false,
                            false,
                        );
                    }
                }
            }
            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(mf);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::SameMusic, same_music_number > 0);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_similar_videos", level = "debug")]
fn compute_similar_videos(
    ff: SimilarVideos,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<SimilarVideos>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    if ff.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        if ff.get_use_reference() {
            tree_view.selection().set_select_function(select_function_always_true);
        } else {
            tree_view.selection().set_select_function(select_function_similar_videos);
        }
        let information = ff.get_information();
        let text_messages = ff.get_text_messages();
        let found_any_duplicates = information.number_of_duplicates > 0;

        entry_info.set_text(
            flg!(
                "compute_found_videos",
                number_files = information.number_of_duplicates,
                number_groups = information.number_of_groups
            )
            .as_str(),
        );

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            if ff.get_use_reference() {
                let vec_struct_similar = ff.get_similar_videos_referenced();

                for (base_file_entry, vec_file_entry) in vec_struct_similar {
                    // Sort
                    let vec_file_entry = if vec_file_entry.len() >= 2 {
                        let mut vec_file_entry = vec_file_entry.clone();
                        vec_file_entry.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));
                        vec_file_entry
                    } else {
                        vec_file_entry.clone()
                    };

                    similar_videos_add_to_list_store(&list_store, "", "", base_file_entry.size, base_file_entry.modified_date, true, true);
                    for file_entry in &vec_file_entry {
                        let (directory, file) = split_path(&file_entry.path);
                        similar_videos_add_to_list_store(&list_store, &file, &directory, file_entry.size, file_entry.modified_date, false, true);
                    }
                }
            } else {
                let vec_struct_similar = ff.get_similar_videos();

                for vec_file_entry in vec_struct_similar {
                    // Sort
                    let vec_file_entry = if vec_file_entry.len() >= 2 {
                        let mut vec_file_entry = vec_file_entry.clone();
                        vec_file_entry.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));
                        vec_file_entry
                    } else {
                        vec_file_entry.clone()
                    };

                    similar_videos_add_to_list_store(&list_store, "", "", 0, 0, true, false);
                    for file_entry in &vec_file_entry {
                        let (directory, file) = split_path(&file_entry.path);
                        similar_videos_add_to_list_store(&list_store, &file, &directory, file_entry.size, file_entry.modified_date, false, false);
                    }
                }
            }

            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(ff);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::SimilarVideos, found_any_duplicates);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarVideos).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_similar_images", level = "debug")]
fn compute_similar_images(
    sf: SimilarImages,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<SimilarImages>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
    hash_size: u8,
) {
    if sf.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        if sf.get_use_reference() {
            tree_view.selection().set_select_function(select_function_always_true);
        } else {
            tree_view.selection().set_select_function(select_function_similar_images);
        }
        let information = sf.get_information();
        let text_messages = sf.get_text_messages();

        let found_any_duplicates = information.number_of_duplicates > 0;

        entry_info.set_text(
            flg!(
                "compute_found_images",
                number_files = information.number_of_duplicates,
                number_groups = information.number_of_groups
            )
            .as_str(),
        );

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            if sf.get_use_reference() {
                let vec_struct_similar: &Vec<(ImagesEntry, Vec<ImagesEntry>)> = sf.get_similar_images_referenced();
                for (base_file_entry, vec_file_entry) in vec_struct_similar {
                    // Sort
                    let vec_file_entry = if vec_file_entry.len() >= 2 {
                        let mut vec_file_entry = vec_file_entry.clone();
                        // Use comparison by similarity, because it is more important that path here
                        vec_file_entry.par_sort_unstable_by_key(|e| e.similarity);
                        vec_file_entry
                    } else {
                        vec_file_entry.clone()
                    };

                    // Header
                    let (directory, file) = split_path(&base_file_entry.path);
                    similar_images_add_to_list_store(
                        &list_store,
                        &file,
                        &directory,
                        base_file_entry.size,
                        base_file_entry.modified_date,
                        &format!("{}x{}", base_file_entry.width, base_file_entry.height),
                        0,
                        hash_size,
                        true,
                        true,
                    );
                    for file_entry in &vec_file_entry {
                        let (directory, file) = split_path(&file_entry.path);
                        similar_images_add_to_list_store(
                            &list_store,
                            &file,
                            &directory,
                            file_entry.size,
                            file_entry.modified_date,
                            &format!("{}x{}", file_entry.width, file_entry.height),
                            file_entry.similarity,
                            hash_size,
                            false,
                            true,
                        );
                    }
                }
            } else {
                let vec_struct_similar = sf.get_similar_images();
                for vec_file_entry in vec_struct_similar {
                    // Sort
                    let vec_file_entry = if vec_file_entry.len() >= 2 {
                        let mut vec_file_entry = vec_file_entry.clone();
                        // Use comparison by similarity, because it is more important that path here
                        vec_file_entry.par_sort_unstable_by_key(|e| e.similarity);
                        vec_file_entry
                    } else {
                        vec_file_entry.clone()
                    };

                    similar_images_add_to_list_store(&list_store, "", "", 0, 0, "", 0, 0, true, false);
                    for file_entry in &vec_file_entry {
                        let (directory, file) = split_path(&file_entry.path);
                        similar_images_add_to_list_store(
                            &list_store,
                            &file,
                            &directory,
                            file_entry.size,
                            file_entry.modified_date,
                            &format!("{}x{}", file_entry.width, file_entry.height),
                            file_entry.similarity,
                            hash_size,
                            false,
                            false,
                        );
                    }
                }
            }

            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(sf);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::SimilarImages, found_any_duplicates);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SimilarImages).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_temporary_files", level = "debug")]
fn compute_temporary_files(
    tf: Temporary,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<Temporary>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    const COLUMNS_NUMBER: usize = 5;
    if tf.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        let information = tf.get_information();
        let text_messages = tf.get_text_messages();

        let temporary_files_number: usize = information.number_of_temporary_files;
        entry_info.set_text(flg!("compute_found_temporary_files", number_files = temporary_files_number).as_str());

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            let vector = tf.get_temporary_files();

            // Sort // TODO maybe simplify this via common file entry
            let mut vector = vector.clone();
            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            for file_entry in vector {
                let (directory, file) = split_path(&file_entry.path);
                let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
                    (ColumnsTemporaryFiles::SelectionButton as u32, &false),
                    (ColumnsTemporaryFiles::Name as u32, &file),
                    (ColumnsTemporaryFiles::Path as u32, &directory),
                    (
                        ColumnsTemporaryFiles::Modification as u32,
                        &(DateTime::from_timestamp(file_entry.modified_date as i64, 0)
                            .expect("Modified date always should be in valid range")
                            .to_string()),
                    ),
                    (ColumnsTemporaryFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                ];
                list_store.set(&list_store.append(), &values);
            }
            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(tf);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::Temporary, temporary_files_number > 0);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Temporary).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_big_files", level = "debug")]
fn compute_big_files(
    bf: BigFile,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<BigFile>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    const COLUMNS_NUMBER: usize = 7;
    if bf.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        let information = bf.get_information();
        let text_messages = bf.get_text_messages();

        let biggest_files_number: usize = information.number_of_real_files;

        entry_info.set_text(flg!("compute_found_big_files", number_files = biggest_files_number).as_str());

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            let vector = bf.get_big_files();

            for file_entry in vector {
                let (directory, file) = split_path(&file_entry.path);
                let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
                    (ColumnsBigFiles::SelectionButton as u32, &false),
                    (ColumnsBigFiles::Size as u32, &(format_size(file_entry.size, BINARY))),
                    (ColumnsBigFiles::Name as u32, &file),
                    (ColumnsBigFiles::Path as u32, &directory),
                    (
                        ColumnsBigFiles::Modification as u32,
                        &(DateTime::from_timestamp(file_entry.modified_date as i64, 0)
                            .expect("Modified date always should be in valid range")
                            .to_string()),
                    ),
                    (ColumnsBigFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                    (ColumnsBigFiles::SizeAsBytes as u32, &(file_entry.size)),
                ];
                list_store.set(&list_store.append(), &values);
            }
            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(bf);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::BigFiles, biggest_files_number > 0);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BigFiles).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_empty_files", level = "debug")]
fn compute_empty_files(
    vf: EmptyFiles,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<EmptyFiles>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    const COLUMNS_NUMBER: usize = 5;
    if vf.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        let information = vf.get_information();
        let text_messages = vf.get_text_messages();

        let empty_files_number: usize = information.number_of_empty_files;

        entry_info.set_text(flg!("compute_found_empty_files", number_files = empty_files_number).as_str());

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            let vector = vf.get_empty_files();
            let vector = vector_sort_simple_unstable_entry_by_path(vector);

            for file_entry in vector {
                let (directory, file) = split_path(&file_entry.path);
                let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
                    (ColumnsEmptyFiles::SelectionButton as u32, &false),
                    (ColumnsEmptyFiles::Name as u32, &file),
                    (ColumnsEmptyFiles::Path as u32, &directory),
                    (
                        ColumnsEmptyFiles::Modification as u32,
                        &(DateTime::from_timestamp(file_entry.modified_date as i64, 0)
                            .expect("Modified date always should be in valid range")
                            .to_string()),
                    ),
                    (ColumnsEmptyFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
                ];
                list_store.set(&list_store.append(), &values);
            }
            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(vf);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::EmptyFiles, empty_files_number > 0);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyFiles).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_empty_folders", level = "debug")]
fn compute_empty_folders(
    ef: EmptyFolder,
    entry_info: &Entry,
    tree_view: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<EmptyFolder>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    const COLUMNS_NUMBER: usize = 5;
    if ef.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        let information = ef.get_information();
        let text_messages = ef.get_text_messages();

        let empty_folder_number: usize = information.number_of_empty_folders;

        entry_info.set_text(flg!("compute_found_empty_folders", number_files = empty_folder_number).as_str());

        // Create GUI
        {
            let list_store = get_list_store(tree_view);

            let hashmap = ef.get_empty_folder_list();
            let mut vector = hashmap.values().collect::<Vec<_>>();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            for fe in vector {
                let (directory, file) = split_path(&fe.path);
                let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
                    (ColumnsEmptyFolders::SelectionButton as u32, &false),
                    (ColumnsEmptyFolders::Name as u32, &file),
                    (ColumnsEmptyFolders::Path as u32, &directory),
                    (
                        ColumnsEmptyFolders::Modification as u32,
                        &(DateTime::from_timestamp(fe.modified_date as i64, 0)
                            .expect("Modified date always should be in valid range")
                            .to_string()),
                    ),
                    (ColumnsEmptyFolders::ModificationAsSecs as u32, &(fe.modified_date)),
                ];
                list_store.set(&list_store.append(), &values);
            }
            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(ef);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::EmptyDirectories, empty_folder_number > 0);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::EmptyDirectories).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

#[fun_time(message = "compute_duplicate_finder", level = "debug")]
fn compute_duplicate_finder(
    df: DuplicateFinder,
    entry_info: &Entry,
    tree_view_duplicate_finder: &TreeView,
    text_view_errors: &TextView,
    shared_state: &SharedState<DuplicateFinder>,
    shared_buttons: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,
    buttons_array: &[Widget; 9],
    buttons_names: &[BottomButtonsEnum; 9],
) {
    if df.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
    } else {
        if df.get_use_reference() {
            tree_view_duplicate_finder.selection().set_select_function(select_function_always_true);
        } else {
            tree_view_duplicate_finder.selection().set_select_function(select_function_duplicates);
        }

        let information = df.get_information();
        let text_messages = df.get_text_messages();

        let duplicates_number: usize;
        let duplicates_size: u64;
        let duplicates_group: usize;

        match df.get_params().check_method {
            CheckingMethod::Name => {
                duplicates_number = information.number_of_duplicated_files_by_name;
                duplicates_size = 0;
                duplicates_group = information.number_of_groups_by_name;
            }
            CheckingMethod::Hash => {
                duplicates_number = information.number_of_duplicated_files_by_hash;
                duplicates_size = information.lost_space_by_hash;
                duplicates_group = information.number_of_groups_by_hash;
            }
            CheckingMethod::Size => {
                duplicates_number = information.number_of_duplicated_files_by_size;
                duplicates_size = information.lost_space_by_size;
                duplicates_group = information.number_of_groups_by_size;
            }
            CheckingMethod::SizeName => {
                duplicates_number = information.number_of_duplicated_files_by_size_name;
                duplicates_size = information.lost_space_by_size;
                duplicates_group = information.number_of_groups_by_size_name;
            }
            _ => panic!(),
        }
        if duplicates_size == 0 {
            entry_info.set_text(flg!("compute_found_duplicates_name", number_files = duplicates_number, number_groups = duplicates_group).as_str());
        } else {
            entry_info.set_text(
                flg!(
                    "compute_found_duplicates_hash_size",
                    number_files = duplicates_number,
                    number_groups = duplicates_group,
                    size = format_size(duplicates_size, BINARY)
                )
                .as_str(),
            );
        }

        // Create GUI
        {
            let list_store = get_list_store(tree_view_duplicate_finder);

            if df.get_use_reference() {
                match df.get_params().check_method {
                    CheckingMethod::Name => {
                        let btreemap = df.get_files_with_identical_name_referenced();

                        for (_name, (base_file_entry, vector)) in btreemap.iter().rev() {
                            let vector = vector_sort_unstable_entry_by_path(vector);
                            let (directory, file) = split_path(&base_file_entry.path);
                            duplicates_add_to_list_store(&list_store, &file, &directory, base_file_entry.size, base_file_entry.modified_date, true, true);

                            for entry in vector {
                                let (directory, file) = split_path(&entry.path);
                                duplicates_add_to_list_store(&list_store, &file, &directory, entry.size, entry.modified_date, false, true);
                            }
                        }
                    }
                    CheckingMethod::Hash => {
                        let btreemap = df.get_files_with_identical_hashes_referenced();

                        for (_size, vectors_vector) in btreemap.iter().rev() {
                            for (base_file_entry, vector) in vectors_vector {
                                let vector = vector_sort_unstable_entry_by_path(vector);
                                let (directory, file) = split_path(&base_file_entry.path);
                                duplicates_add_to_list_store(&list_store, &file, &directory, base_file_entry.size, base_file_entry.modified_date, true, true);
                                for entry in vector {
                                    let (directory, file) = split_path(&entry.path);
                                    duplicates_add_to_list_store(&list_store, &file, &directory, entry.size, entry.modified_date, false, true);
                                }
                            }
                        }
                    }
                    CheckingMethod::Size => {
                        let btreemap = df.get_files_with_identical_size_referenced();

                        for (_size, (base_file_entry, vector)) in btreemap.iter().rev() {
                            let vector = vector_sort_unstable_entry_by_path(vector);
                            let (directory, file) = split_path(&base_file_entry.path);
                            duplicates_add_to_list_store(&list_store, &file, &directory, base_file_entry.size, base_file_entry.modified_date, true, true);
                            for entry in vector {
                                let (directory, file) = split_path(&entry.path);
                                duplicates_add_to_list_store(&list_store, &file, &directory, entry.size, entry.modified_date, false, true);
                            }
                        }
                    }
                    CheckingMethod::SizeName => {
                        let btreemap = df.get_files_with_identical_size_names_referenced();

                        for (_size, (base_file_entry, vector)) in btreemap.iter().rev() {
                            let vector = vector_sort_unstable_entry_by_path(vector);
                            let (directory, file) = split_path(&base_file_entry.path);
                            duplicates_add_to_list_store(&list_store, &file, &directory, base_file_entry.size, base_file_entry.modified_date, true, true);
                            for entry in vector {
                                let (directory, file) = split_path(&entry.path);
                                duplicates_add_to_list_store(&list_store, &file, &directory, entry.size, entry.modified_date, false, true);
                            }
                        }
                    }
                    _ => panic!(),
                }
            } else {
                match df.get_params().check_method {
                    CheckingMethod::Name => {
                        let btreemap = df.get_files_sorted_by_names();

                        for (_name, vector) in btreemap.iter().rev() {
                            let vector = vector_sort_unstable_entry_by_path(vector);
                            duplicates_add_to_list_store(&list_store, "", "", 0, 0, true, false);
                            for entry in vector {
                                let (directory, file) = split_path(&entry.path);
                                duplicates_add_to_list_store(&list_store, &file, &directory, entry.size, entry.modified_date, false, false);
                            }
                        }
                    }
                    CheckingMethod::Hash => {
                        let btreemap = df.get_files_sorted_by_hash();

                        for (_size, vectors_vector) in btreemap.iter().rev() {
                            for vector in vectors_vector {
                                let vector = vector_sort_unstable_entry_by_path(vector);
                                duplicates_add_to_list_store(&list_store, "", "", 0, 0, true, false);

                                for entry in vector {
                                    let (directory, file) = split_path(&entry.path);
                                    duplicates_add_to_list_store(&list_store, &file, &directory, entry.size, entry.modified_date, false, false);
                                }
                            }
                        }
                    }
                    CheckingMethod::Size => {
                        let btreemap = df.get_files_sorted_by_size();

                        for (_size, vector) in btreemap.iter().rev() {
                            let vector = vector_sort_unstable_entry_by_path(vector);
                            duplicates_add_to_list_store(&list_store, "", "", 0, 0, true, false);

                            for entry in vector {
                                let (directory, file) = split_path(&entry.path);
                                duplicates_add_to_list_store(&list_store, &file, &directory, entry.size, entry.modified_date, false, false);
                            }
                        }
                    }
                    CheckingMethod::SizeName => {
                        let btreemap = df.get_files_sorted_by_size_name();

                        for (_size, vector) in btreemap.iter().rev() {
                            let vector = vector_sort_unstable_entry_by_path(vector);
                            duplicates_add_to_list_store(&list_store, "", "", 0, 0, true, false);

                            for entry in vector {
                                let (directory, file) = split_path(&entry.path);
                                duplicates_add_to_list_store(&list_store, &file, &directory, entry.size, entry.modified_date, false, false);
                            }
                        }
                    }
                    _ => panic!(),
                }
            }
            print_text_messages_to_text_view(text_messages, text_view_errors);
        }

        // Set state
        {
            *shared_state.borrow_mut() = Some(df);

            set_specific_buttons_as_active(shared_buttons, NotebookMainEnum::Duplicate, duplicates_number > 0);

            set_buttons(
                &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::Duplicate).expect("Failed to borrow buttons"),
                buttons_array,
                buttons_names,
            );
        }
    }
}

fn vector_sort_unstable_entry_by_path<T>(vector: &[T]) -> Vec<T>
where
    T: ResultEntry + Clone,
    T: Send,
{
    if vector.len() >= 2 {
        let mut vector = vector.to_vec();
        vector.par_sort_unstable_by(|a, b| split_path_compare(a.get_path(), b.get_path()));
        vector
    } else {
        vector.to_vec()
    }
}

fn vector_sort_simple_unstable_entry_by_path<T>(vector: &[T]) -> Vec<T>
where
    T: ResultEntry + Clone,
    T: Send,
{
    let mut vector = vector.to_vec();
    vector.par_sort_unstable_by(|a, b| split_path_compare(a.get_path(), b.get_path()));
    vector
}

fn duplicates_add_to_list_store(list_store: &ListStore, file: &str, directory: &str, size: u64, modified_date: u64, is_header: bool, is_reference_folder: bool) {
    const COLUMNS_NUMBER: usize = 11;
    let size_str;
    let string_date;
    let color = if is_header { HEADER_ROW_COLOR } else { MAIN_ROW_COLOR };

    if is_header && !is_reference_folder {
        size_str = String::new();
        string_date = String::new();
    } else {
        size_str = format_size(size, BINARY);
        string_date = DateTime::from_timestamp(modified_date as i64, 0)
            .expect("Modified date always should be in valid range")
            .to_string();
    };

    let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
        (ColumnsDuplicates::ActivatableSelectButton as u32, &(!is_header)),
        (ColumnsDuplicates::SelectionButton as u32, &false),
        (ColumnsDuplicates::Size as u32, &size_str),
        (ColumnsDuplicates::SizeAsBytes as u32, &size),
        (ColumnsDuplicates::Name as u32, &file),
        (ColumnsDuplicates::Path as u32, &directory),
        (ColumnsDuplicates::Modification as u32, &string_date),
        (ColumnsDuplicates::ModificationAsSecs as u32, &modified_date),
        (ColumnsDuplicates::Color as u32, &color),
        (ColumnsDuplicates::IsHeader as u32, &is_header),
        (ColumnsDuplicates::TextColor as u32, &TEXT_COLOR),
    ];
    list_store.set(&list_store.append(), &values);
}

fn similar_images_add_to_list_store(
    list_store: &ListStore,
    file: &str,
    directory: &str,
    size: u64,
    modified_date: u64,
    dimensions: &str,
    similarity: u32,
    hash_size: u8,
    is_header: bool,
    is_reference_folder: bool,
) {
    const COLUMNS_NUMBER: usize = 13;
    let size_str;
    let string_date;
    let similarity_string;
    let color = if is_header { HEADER_ROW_COLOR } else { MAIN_ROW_COLOR };

    if is_header {
        similarity_string = String::new();
    } else {
        similarity_string = similar_images::get_string_from_similarity(&similarity, hash_size);
    };

    if is_header && !is_reference_folder {
        size_str = String::new();
        string_date = String::new();
    } else {
        size_str = format_size(size, BINARY);
        string_date = DateTime::from_timestamp(modified_date as i64, 0)
            .expect("Modified date always should be in valid range")
            .to_string();
    }

    let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
        (ColumnsSimilarImages::ActivatableSelectButton as u32, &(!is_header)),
        (ColumnsSimilarImages::SelectionButton as u32, &false),
        (ColumnsSimilarImages::Similarity as u32, &similarity_string),
        (ColumnsSimilarImages::Size as u32, &size_str),
        (ColumnsSimilarImages::SizeAsBytes as u32, &size),
        (ColumnsSimilarImages::Dimensions as u32, &dimensions),
        (ColumnsSimilarImages::Name as u32, &file),
        (ColumnsSimilarImages::Path as u32, &directory),
        (ColumnsSimilarImages::Modification as u32, &string_date),
        (ColumnsSimilarImages::ModificationAsSecs as u32, &modified_date),
        (ColumnsSimilarImages::Color as u32, &color),
        (ColumnsSimilarImages::IsHeader as u32, &is_header),
        (ColumnsSimilarImages::TextColor as u32, &TEXT_COLOR),
    ];
    list_store.set(&list_store.append(), &values);
}

fn similar_videos_add_to_list_store(list_store: &ListStore, file: &str, directory: &str, size: u64, modified_date: u64, is_header: bool, is_reference_folder: bool) {
    const COLUMNS_NUMBER: usize = 11;
    let size_str;
    let string_date;
    let color = if is_header { HEADER_ROW_COLOR } else { MAIN_ROW_COLOR };
    if is_header && !is_reference_folder {
        size_str = String::new();
        string_date = String::new();
    } else {
        size_str = format_size(size, BINARY);
        string_date = DateTime::from_timestamp(modified_date as i64, 0)
            .expect("Modified date always should be in valid range")
            .to_string();
    };

    let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
        (ColumnsSimilarVideos::ActivatableSelectButton as u32, &(!is_header)),
        (ColumnsSimilarVideos::SelectionButton as u32, &false),
        (ColumnsSimilarVideos::Size as u32, &size_str),
        (ColumnsSimilarVideos::SizeAsBytes as u32, &size),
        (ColumnsSimilarVideos::Name as u32, &file),
        (ColumnsSimilarVideos::Path as u32, &directory),
        (ColumnsSimilarVideos::Modification as u32, &string_date),
        (ColumnsSimilarVideos::ModificationAsSecs as u32, &modified_date),
        (ColumnsSimilarVideos::Color as u32, &color),
        (ColumnsSimilarVideos::IsHeader as u32, &is_header),
        (ColumnsSimilarVideos::TextColor as u32, &TEXT_COLOR),
    ];

    list_store.set(&list_store.append(), &values);
}

fn same_music_add_to_list_store(
    list_store: &ListStore,
    file: &str,
    directory: &str,
    size: u64,
    modified_date: u64,
    track_title: &str,
    track_artist: &str,
    track_year: &str,
    track_bitrate: u32,
    bitrate_string: &str,
    track_genre: &str,
    track_length: &str,
    is_header: bool,
    is_reference_folder: bool,
) {
    const COLUMNS_NUMBER: usize = 18;
    let size_str;
    let string_date;
    let color = if is_header { HEADER_ROW_COLOR } else { MAIN_ROW_COLOR };
    if is_header && !is_reference_folder {
        size_str = String::new();
        string_date = String::new();
    } else {
        size_str = format_size(size, BINARY);
        string_date = DateTime::from_timestamp(modified_date as i64, 0)
            .expect("Modified date always should be in valid range")
            .to_string();
    };

    let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
        (ColumnsSameMusic::ActivatableSelectButton as u32, &(!is_header)),
        (ColumnsSameMusic::SelectionButton as u32, &false),
        (ColumnsSameMusic::Size as u32, &size_str),
        (ColumnsSameMusic::SizeAsBytes as u32, &size),
        (ColumnsSameMusic::Name as u32, &file),
        (ColumnsSameMusic::Path as u32, &directory),
        (ColumnsSameMusic::Title as u32, &track_title),
        (ColumnsSameMusic::Artist as u32, &track_artist),
        (ColumnsSameMusic::Year as u32, &track_year),
        (ColumnsSameMusic::Genre as u32, &track_genre),
        (ColumnsSameMusic::Bitrate as u32, &bitrate_string),
        (ColumnsSameMusic::BitrateAsNumber as u32, &track_bitrate),
        (ColumnsSameMusic::Length as u32, &track_length),
        (ColumnsSameMusic::Modification as u32, &string_date),
        (ColumnsSameMusic::ModificationAsSecs as u32, &modified_date),
        (ColumnsSameMusic::Color as u32, &color),
        (ColumnsSameMusic::IsHeader as u32, &is_header),
        (ColumnsSameMusic::TextColor as u32, &TEXT_COLOR),
    ];

    list_store.set(&list_store.append(), &values);
}

fn set_specific_buttons_as_active(buttons_array: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>, notebook_enum: NotebookMainEnum, value_to_set: bool) {
    let mut b_mut = buttons_array.borrow_mut();
    let butt = b_mut.get_mut(&notebook_enum).expect("Failed to borrow buttons");
    let allowed_buttons = NOTEBOOKS_INFO[notebook_enum as usize].bottom_buttons;
    for i in allowed_buttons {
        *butt.get_mut(i).expect("Failed to borrow buttons") = value_to_set;
    }
}
