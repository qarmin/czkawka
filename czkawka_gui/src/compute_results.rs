use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use chrono::DateTime;
use crossbeam_channel::Receiver;
use czkawka_core::common::model::CheckingMethod;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::ResultEntry;
use czkawka_core::common::{format_time, split_path, split_path_compare};
use czkawka_core::tools::bad_extensions::BadExtensions;
use czkawka_core::tools::big_file::BigFile;
use czkawka_core::tools::broken_files::BrokenFiles;
use czkawka_core::tools::duplicate::DuplicateFinder;
use czkawka_core::tools::empty_files::EmptyFiles;
use czkawka_core::tools::empty_folder::EmptyFolder;
use czkawka_core::tools::invalid_symlinks::InvalidSymlinks;
use czkawka_core::tools::same_music::core::format_audio_duration;
use czkawka_core::tools::same_music::{MusicSimilarity, SameMusic};
use czkawka_core::tools::similar_images::core::get_string_from_similarity;
use czkawka_core::tools::similar_images::{ImagesEntry, SimilarImages};
use czkawka_core::tools::similar_videos::SimilarVideos;
use czkawka_core::tools::similar_videos::core::{format_bitrate_opt, format_duration_opt};
use czkawka_core::tools::temporary::Temporary;
use fun_time::fun_time;
use gtk4::prelude::*;
use gtk4::{Entry, ListStore, TextView};
use humansize::{BINARY, format_size};
use rayon::prelude::*;

use crate::flg;
use crate::gui_structs::common_tree_view::{SharedModelEnum, SubView, TreeViewListStoreTrait};
use crate::gui_structs::gui_data::GuiData;
use crate::help_combo_box::IMAGES_HASH_SIZE_COMBO_BOX;
use crate::help_functions::{HEADER_ROW_COLOR, MAIN_ROW_COLOR, TEXT_COLOR, print_text_messages_to_text_view, set_buttons};
use crate::helpers::enums::{
    BottomButtonsEnum, ColumnsBadExtensions, ColumnsBigFiles, ColumnsBrokenFiles, ColumnsDuplicates, ColumnsEmptyFiles, ColumnsEmptyFolders, ColumnsInvalidSymlinks,
    ColumnsSameMusic, ColumnsSimilarImages, ColumnsSimilarVideos, ColumnsTemporaryFiles, Message,
};
use crate::helpers::list_store_operations::append_row_to_list_store;
use crate::notebook_enums::NotebookMainEnum;
use crate::notebook_info::NOTEBOOKS_INFO;
use crate::opening_selecting_records::{
    select_function_always_true, select_function_duplicates, select_function_same_music, select_function_similar_images, select_function_similar_videos,
};

// Helper functions for deduplication

fn handle_stopped_search<T: CommonData>(tool: &T, entry_info: &Entry) -> bool {
    if tool.get_stopped_search() {
        entry_info.set_text(&flg!("compute_stopped_by_user"));
        true
    } else {
        false
    }
}

#[expect(clippy::unnecessary_wraps)]
fn finalize_compute<T: Into<SharedModelEnum>>(subview: &SubView, tool: T, items_found: usize) -> Option<bool> {
    subview.shared_model_enum.replace(tool.into());
    Some(items_found > 0)
}

fn conditional_sort_vector<T>(vector: &[T]) -> Vec<T>
where
    T: ResultEntry + Clone + Send,
{
    if vector.len() >= 2 {
        let mut vector = vector.to_vec();
        vector.par_sort_unstable_by(|a, b| split_path_compare(a.get_path(), b.get_path()));
        vector
    } else {
        vector.to_vec()
    }
}

fn format_size_and_date(size: u64, modified_date: u64, is_header: bool, is_reference_folder: bool) -> (String, String) {
    if is_header && !is_reference_folder {
        (String::new(), String::new())
    } else {
        (format_size(size, BINARY), get_dt_timestamp_string(modified_date))
    }
}

fn get_row_color(is_header: bool) -> &'static str {
    if is_header { HEADER_ROW_COLOR } else { MAIN_ROW_COLOR }
}

pub(crate) fn connect_compute_results(gui_data: &GuiData, result_receiver: Receiver<Message>) {
    let combo_box_image_hash_size = gui_data.main_notebook.combo_box_image_hash_size.clone();
    let buttons_search = gui_data.bottom_buttons.buttons_search.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let entry_info = gui_data.entry_info.clone();
    let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
    let buttons_names = gui_data.bottom_buttons.buttons_names;
    let window_progress = gui_data.progress_window.window_progress.clone();
    let taskbar_state = gui_data.taskbar_state.clone();
    let notebook_upper = gui_data.upper_notebook.notebook_upper.clone();
    let button_settings = gui_data.header.button_settings.clone();
    let button_app_info = gui_data.header.button_app_info.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();

    let main_context = glib::MainContext::default();
    let _guard = main_context.acquire().expect("Failed to acquire main context");

    glib::spawn_future_local(async move {
        loop {
            while let Ok(msg) = result_receiver.try_recv() {
                buttons_search.set_visible(true);

                notebook_main.set_sensitive(true);
                notebook_upper.set_sensitive(true);
                button_settings.set_sensitive(true);
                button_app_info.set_sensitive(true);

                window_progress.set_visible(false);

                taskbar_state.borrow().hide();

                let hash_size_index = combo_box_image_hash_size.active().expect("Failed to get active item") as usize;
                let hash_size = IMAGES_HASH_SIZE_COMBO_BOX[hash_size_index] as u8;

                let msg_type = msg.get_message_type();
                let subview = common_tree_views.get_subview(msg_type);

                let found_duplicates: Option<bool> = match msg {
                    Message::Duplicates(df) => compute_duplicate_finder(df, &entry_info, &text_view_errors, subview),
                    Message::EmptyFolders(ef) => compute_empty_folders(ef, &entry_info, &text_view_errors, subview),
                    Message::EmptyFiles(vf) => compute_empty_files(vf, &entry_info, &text_view_errors, subview),
                    Message::BigFiles(bf) => compute_big_files(bf, &entry_info, &text_view_errors, subview),
                    Message::Temporary(tf) => compute_temporary_files(tf, &entry_info, &text_view_errors, subview),
                    Message::SimilarImages(sf) => compute_similar_images(sf, &entry_info, &text_view_errors, subview, hash_size),
                    Message::SimilarVideos(ff) => compute_similar_videos(ff, &entry_info, &text_view_errors, subview),
                    Message::SameMusic(mf) => compute_same_music(mf, &entry_info, &text_view_errors, subview),
                    Message::InvalidSymlinks(ifs) => compute_invalid_symlinks(ifs, &entry_info, &text_view_errors, subview),
                    Message::BrokenFiles(br) => compute_broken_files(br, &entry_info, &text_view_errors, subview),
                    Message::BadExtensions(be) => compute_bad_extensions(be, &entry_info, &text_view_errors, subview),
                };

                if let Some(found_duplicates) = found_duplicates {
                    set_specific_buttons_as_active(&shared_buttons, msg_type, found_duplicates);

                    set_buttons(
                        &mut *shared_buttons.borrow_mut().get_mut(&msg_type).expect("Failed to borrow buttons"),
                        &buttons_array,
                        &buttons_names,
                    );
                }
            }
            glib::timeout_future(Duration::from_millis(300)).await;
        }
    });
}

#[fun_time(message = "compute_bad_extensions", level = "debug")]
fn compute_bad_extensions(be: BadExtensions, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&be, entry_info) {
        return None;
    }
    let information = be.get_information();
    let text_messages = be.get_text_messages();
    let bad_extensions_number = information.number_of_files_with_bad_extension;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(flg!("compute_found_bad_extensions", number_files = bad_extensions_number, time = scanning_time_str).as_str());
    }

    let list_store = subview.tree_view.get_model();
    let mut vector = be.get_bad_extensions_files().clone();
    vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

    for file_entry in vector {
        let (directory, file) = split_path(&file_entry.path);
        let values: [(u32, &dyn ToValue); 7] = [
            (ColumnsBadExtensions::SelectionButton as u32, &false),
            (ColumnsBadExtensions::Name as u32, &file),
            (ColumnsBadExtensions::Path as u32, &directory),
            (ColumnsBadExtensions::CurrentExtension as u32, &file_entry.current_extension),
            (ColumnsBadExtensions::ValidExtensions as u32, &file_entry.proper_extensions_group),
            (ColumnsBadExtensions::Modification as u32, &(get_dt_timestamp_string(file_entry.modified_date))),
            (ColumnsBadExtensions::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
        ];
        append_row_to_list_store(&list_store, &values);
    }
    print_text_messages_to_text_view(text_messages, text_view_errors);
    finalize_compute(subview, be, bad_extensions_number)
}

#[fun_time(message = "compute_broken_files", level = "debug")]
fn compute_broken_files(br: BrokenFiles, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&br, entry_info) {
        return None;
    }
    let information = br.get_information();
    let text_messages = br.get_text_messages();
    let broken_files_number = information.number_of_broken_files;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(flg!("compute_found_broken_files", number_files = broken_files_number, time = scanning_time_str).as_str());
    }

    let list_store = subview.tree_view.get_model();
    let mut vector = br.get_broken_files().clone();
    vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

    for file_entry in vector {
        let (directory, file) = split_path(&file_entry.path);
        let values: [(u32, &dyn ToValue); 6] = [
            (ColumnsBrokenFiles::SelectionButton as u32, &false),
            (ColumnsBrokenFiles::Name as u32, &file),
            (ColumnsBrokenFiles::Path as u32, &directory),
            (ColumnsBrokenFiles::ErrorType as u32, &file_entry.error_string),
            (ColumnsBrokenFiles::Modification as u32, &(get_dt_timestamp_string(file_entry.modified_date))),
            (ColumnsBrokenFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
        ];
        append_row_to_list_store(&list_store, &values);
    }
    print_text_messages_to_text_view(text_messages, text_view_errors);
    finalize_compute(subview, br, broken_files_number)
}

#[fun_time(message = "compute_invalid_symlinks", level = "debug")]
fn compute_invalid_symlinks(ifs: InvalidSymlinks, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&ifs, entry_info) {
        return None;
    }
    let information = ifs.get_information();
    let text_messages = ifs.get_text_messages();
    let invalid_symlinks = information.number_of_invalid_symlinks;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(&flg!("compute_found_invalid_symlinks", number_files = invalid_symlinks, time = scanning_time_str));
    }

    let list_store = subview.tree_view.get_model();
    let vector = conditional_sort_vector(ifs.get_invalid_symlinks());

    for file_entry in vector {
        let (directory, file) = split_path(&file_entry.path);
        let symlink_info = file_entry.symlink_info;
        let values: [(u32, &dyn ToValue); 7] = [
            (ColumnsInvalidSymlinks::SelectionButton as u32, &false),
            (ColumnsInvalidSymlinks::Name as u32, &file),
            (ColumnsInvalidSymlinks::Path as u32, &directory),
            (ColumnsInvalidSymlinks::DestinationPath as u32, &symlink_info.destination_path.to_string_lossy().to_string()),
            (ColumnsInvalidSymlinks::TypeOfError as u32, &symlink_info.type_of_error.translate()),
            (ColumnsInvalidSymlinks::Modification as u32, &(get_dt_timestamp_string(file_entry.modified_date))),
            (ColumnsInvalidSymlinks::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
        ];
        append_row_to_list_store(&list_store, &values);
    }
    print_text_messages_to_text_view(text_messages, text_view_errors);
    finalize_compute(subview, ifs, invalid_symlinks)
}

#[fun_time(message = "compute_same_music", level = "debug")]
fn compute_same_music(mf: SameMusic, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&mf, entry_info) {
        return None;
    }
    if mf.get_use_reference() {
        subview.tree_view.selection().set_select_function(select_function_always_true);
    } else {
        subview.tree_view.selection().set_select_function(select_function_same_music);
    }

    let information = mf.get_information();
    let text_messages = mf.get_text_messages();

    let same_music_number: usize = information.number_of_duplicates;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(&flg!(
            "compute_found_music",
            number_files = information.number_of_duplicates,
            number_groups = information.number_of_groups,
            time = scanning_time_str
        ));
    }

    // Create GUI
    {
        let list_store = subview.tree_view.get_model();

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
                let vec_file_entry = vector_sort_unstable_entry_by_path(vec_file_entry);

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
                    &format_audio_duration(base_file_entry.length),
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
                        &format_audio_duration(file_entry.length),
                        false,
                        true,
                    );
                }
            }
        } else {
            let vector = mf.get_duplicated_music_entries();

            let text: &str = if mf.get_params().check_type == CheckingMethod::AudioTags { "-----" } else { "" };

            for vec_file_entry in vector {
                let vec_file_entry = vector_sort_unstable_entry_by_path(vec_file_entry);

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
                        &format_audio_duration(file_entry.length),
                        false,
                        false,
                    );
                }
            }
        }
        print_text_messages_to_text_view(text_messages, text_view_errors);
    }

    finalize_compute(subview, mf, same_music_number)
}

#[fun_time(message = "compute_similar_videos", level = "debug")]
fn compute_similar_videos(ff: SimilarVideos, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&ff, entry_info) {
        return None;
    }
    if ff.get_use_reference() {
        subview.tree_view.selection().set_select_function(select_function_always_true);
    } else {
        subview.tree_view.selection().set_select_function(select_function_similar_videos);
    }
    let information = ff.get_information();
    let text_messages = ff.get_text_messages();
    let found_any_duplicates = information.number_of_duplicates > 0;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(&flg!(
            "compute_found_videos",
            number_files = information.number_of_duplicates,
            number_groups = information.number_of_groups,
            time = scanning_time_str
        ));
    }

    // Create GUI
    {
        let list_store = subview.tree_view.get_model();

        if ff.get_use_reference() {
            let vec_struct_similar = ff.get_similar_videos_referenced();

            for (base_file_entry, vec_file_entry) in vec_struct_similar {
                let vec_file_entry = vector_sort_unstable_entry_by_path(vec_file_entry);

                let (directory, file) = split_path(&base_file_entry.path);
                similar_videos_add_to_list_store(
                    &list_store,
                    &file,
                    &directory,
                    base_file_entry.size,
                    base_file_entry.modified_date,
                    true,
                    true,
                    base_file_entry.fps,
                    base_file_entry.codec.as_deref(),
                    base_file_entry.bitrate,
                    base_file_entry.width,
                    base_file_entry.height,
                    base_file_entry.duration,
                );
                for file_entry in &vec_file_entry {
                    let (directory, file) = split_path(&file_entry.path);
                    similar_videos_add_to_list_store(
                        &list_store,
                        &file,
                        &directory,
                        file_entry.size,
                        file_entry.modified_date,
                        false,
                        true,
                        file_entry.fps,
                        file_entry.codec.as_deref(),
                        file_entry.bitrate,
                        file_entry.width,
                        file_entry.height,
                        file_entry.duration,
                    );
                }
            }
        } else {
            let vec_struct_similar = ff.get_similar_videos();

            for vec_file_entry in vec_struct_similar {
                let vec_file_entry = vector_sort_unstable_entry_by_path(vec_file_entry);

                similar_videos_add_to_list_store(&list_store, "", "", 0, 0, true, false, None, None, None, None, None, None);
                for file_entry in &vec_file_entry {
                    let (directory, file) = split_path(&file_entry.path);
                    similar_videos_add_to_list_store(
                        &list_store,
                        &file,
                        &directory,
                        file_entry.size,
                        file_entry.modified_date,
                        false,
                        false,
                        file_entry.fps,
                        file_entry.codec.as_deref(),
                        file_entry.bitrate,
                        file_entry.width,
                        file_entry.height,
                        file_entry.duration,
                    );
                }
            }
        }

        print_text_messages_to_text_view(text_messages, text_view_errors);
    }

    finalize_compute(subview, ff, found_any_duplicates as usize)
}

#[fun_time(message = "compute_similar_images", level = "debug")]
fn compute_similar_images(sf: SimilarImages, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView, hash_size: u8) -> Option<bool> {
    if handle_stopped_search(&sf, entry_info) {
        return None;
    }
    if sf.get_use_reference() {
        subview.tree_view.selection().set_select_function(select_function_always_true);
    } else {
        subview.tree_view.selection().set_select_function(select_function_similar_images);
    }
    let information = sf.get_information();
    let text_messages = sf.get_text_messages();

    let found_any_duplicates = information.number_of_duplicates > 0;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(&flg!(
            "compute_found_images",
            number_files = information.number_of_duplicates,
            number_groups = information.number_of_groups,
            time = scanning_time_str
        ));
    }

    // Create GUI
    {
        let list_store = subview.tree_view.get_model();

        if sf.get_use_reference() {
            let vec_struct_similar: Vec<(ImagesEntry, Vec<ImagesEntry>)> = sf.get_similar_images_referenced().clone();
            for (base_file_entry, mut vec_file_entry) in vec_struct_similar {
                vec_file_entry.sort_by_key(|e| e.similarity);

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
            let vec_struct_similar = sf.get_similar_images().clone();
            for mut vec_file_entry in vec_struct_similar {
                vec_file_entry.sort_by_key(|e| e.similarity);

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

    finalize_compute(subview, sf, found_any_duplicates as usize)
}

#[fun_time(message = "compute_temporary_files", level = "debug")]
fn compute_temporary_files(tf: Temporary, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&tf, entry_info) {
        return None;
    }
    let information = tf.get_information();
    let text_messages = tf.get_text_messages();
    let temporary_files_number = information.number_of_temporary_files;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(&flg!("compute_found_temporary_files", number_files = temporary_files_number, time = scanning_time_str));
    }

    let list_store = subview.tree_view.get_model();
    let mut vector = tf.get_temporary_files().clone();
    vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

    for file_entry in vector {
        let (directory, file) = split_path(&file_entry.path);
        let values: [(u32, &dyn ToValue); 5] = [
            (ColumnsTemporaryFiles::SelectionButton as u32, &false),
            (ColumnsTemporaryFiles::Name as u32, &file),
            (ColumnsTemporaryFiles::Path as u32, &directory),
            (ColumnsTemporaryFiles::Modification as u32, &(get_dt_timestamp_string(file_entry.modified_date))),
            (ColumnsTemporaryFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
        ];
        append_row_to_list_store(&list_store, &values);
    }
    print_text_messages_to_text_view(text_messages, text_view_errors);
    finalize_compute(subview, tf, temporary_files_number)
}

#[fun_time(message = "compute_big_files", level = "debug")]
fn compute_big_files(bf: BigFile, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&bf, entry_info) {
        return None;
    }
    let information = bf.get_information();
    let text_messages = bf.get_text_messages();
    let biggest_files_number = information.number_of_real_files;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(&flg!("compute_found_big_files", number_files = biggest_files_number, time = scanning_time_str));
    }

    let list_store = subview.tree_view.get_model();
    let vector = bf.get_big_files();

    for file_entry in vector {
        let (directory, file) = split_path(&file_entry.path);
        let values: [(u32, &dyn ToValue); 7] = [
            (ColumnsBigFiles::SelectionButton as u32, &false),
            (ColumnsBigFiles::Size as u32, &(format_size(file_entry.size, BINARY))),
            (ColumnsBigFiles::Name as u32, &file),
            (ColumnsBigFiles::Path as u32, &directory),
            (ColumnsBigFiles::Modification as u32, &(get_dt_timestamp_string(file_entry.modified_date))),
            (ColumnsBigFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
            (ColumnsBigFiles::SizeAsBytes as u32, &(file_entry.size)),
        ];
        append_row_to_list_store(&list_store, &values);
    }
    print_text_messages_to_text_view(text_messages, text_view_errors);
    finalize_compute(subview, bf, biggest_files_number)
}

#[fun_time(message = "compute_empty_files", level = "debug")]
fn compute_empty_files(vf: EmptyFiles, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&vf, entry_info) {
        return None;
    }
    let information = vf.get_information();
    let text_messages = vf.get_text_messages();
    let empty_files_number = information.number_of_empty_files;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(&flg!("compute_found_empty_files", number_files = empty_files_number, time = scanning_time_str));
    }

    let list_store = subview.tree_view.get_model();
    let vector = conditional_sort_vector(vf.get_empty_files());

    for file_entry in vector {
        let (directory, file) = split_path(&file_entry.path);
        let values: [(u32, &dyn ToValue); 5] = [
            (ColumnsEmptyFiles::SelectionButton as u32, &false),
            (ColumnsEmptyFiles::Name as u32, &file),
            (ColumnsEmptyFiles::Path as u32, &directory),
            (ColumnsEmptyFiles::Modification as u32, &(get_dt_timestamp_string(file_entry.modified_date))),
            (ColumnsEmptyFiles::ModificationAsSecs as u32, &(file_entry.modified_date as i64)),
        ];
        append_row_to_list_store(&list_store, &values);
    }
    print_text_messages_to_text_view(text_messages, text_view_errors);
    finalize_compute(subview, vf, empty_files_number)
}

#[fun_time(message = "compute_empty_folders", level = "debug")]
fn compute_empty_folders(ef: EmptyFolder, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&ef, entry_info) {
        return None;
    }
    let information = ef.get_information();
    let text_messages = ef.get_text_messages();
    let empty_folder_number = information.number_of_empty_folders;
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        entry_info.set_text(&flg!("compute_found_empty_folders", number_files = empty_folder_number, time = scanning_time_str));
    }

    let list_store = subview.tree_view.get_model();
    let hashmap = ef.get_empty_folder_list();
    let mut vector = hashmap.values().collect::<Vec<_>>();
    vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

    for fe in vector {
        let (directory, file) = split_path(&fe.path);
        let values: [(u32, &dyn ToValue); 5] = [
            (ColumnsEmptyFolders::SelectionButton as u32, &false),
            (ColumnsEmptyFolders::Name as u32, &file),
            (ColumnsEmptyFolders::Path as u32, &directory),
            (ColumnsEmptyFolders::Modification as u32, &(get_dt_timestamp_string(fe.modified_date))),
            (ColumnsEmptyFolders::ModificationAsSecs as u32, &(fe.modified_date)),
        ];
        append_row_to_list_store(&list_store, &values);
    }
    print_text_messages_to_text_view(text_messages, text_view_errors);
    finalize_compute(subview, ef, empty_folder_number)
}

#[fun_time(message = "compute_duplicate_finder", level = "debug")]
fn compute_duplicate_finder(df: DuplicateFinder, entry_info: &Entry, text_view_errors: &TextView, subview: &SubView) -> Option<bool> {
    if handle_stopped_search(&df, entry_info) {
        return None;
    }

    if df.get_use_reference() {
        subview.tree_view.selection().set_select_function(select_function_always_true);
    } else {
        subview.tree_view.selection().set_select_function(select_function_duplicates);
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
        _ => unreachable!(),
    }
    let scanning_time_str = format_time(information.scanning_time);

    if let Some(critical) = text_messages.critical.clone() {
        entry_info.set_text(&critical);
    } else {
        if duplicates_size == 0 {
            entry_info.set_text(
                flg!(
                    "compute_found_duplicates_name",
                    number_files = duplicates_number,
                    number_groups = duplicates_group,
                    time = scanning_time_str
                )
                .as_str(),
            );
        } else {
            entry_info.set_text(
                flg!(
                    "compute_found_duplicates_hash_size",
                    number_files = duplicates_number,
                    number_groups = duplicates_group,
                    size = format_size(duplicates_size, BINARY),
                    time = scanning_time_str
                )
                .as_str(),
            );
        }
    }

    // Create GUI
    {
        let list_store = subview.tree_view.get_model();

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

    finalize_compute(subview, df, duplicates_number)
}

fn vector_sort_unstable_entry_by_path<T>(vector: &[T]) -> Vec<T>
where
    T: ResultEntry + Clone + Send,
{
    if vector.len() >= 2 {
        let mut vector = vector.to_vec();
        vector.par_sort_unstable_by(|a, b| split_path_compare(a.get_path(), b.get_path()));
        vector
    } else {
        vector.to_vec()
    }
}

fn duplicates_add_to_list_store(list_store: &ListStore, file: &str, directory: &str, size: u64, modified_date: u64, is_header: bool, is_reference_folder: bool) {
    const COLUMNS_NUMBER: usize = 11;
    let (size_str, string_date) = format_size_and_date(size, modified_date, is_header, is_reference_folder);
    let color = get_row_color(is_header);

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
    append_row_to_list_store(list_store, &values);
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
    let (size_str, string_date) = format_size_and_date(size, modified_date, is_header, is_reference_folder);
    let color = get_row_color(is_header);
    let similarity_string = if is_header { String::new() } else { get_string_from_similarity(similarity, hash_size) };

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
    append_row_to_list_store(list_store, &values);
}

fn similar_videos_add_to_list_store(
    list_store: &ListStore,
    file: &str,
    directory: &str,
    size: u64,
    modified_date: u64,
    is_header: bool,
    is_reference_folder: bool,
    fps: Option<f64>,
    codec: Option<&str>,
    bitrate: Option<u64>,
    width: Option<u32>,
    height: Option<u32>,
    duration: Option<f64>,
) {
    const COLUMNS_NUMBER: usize = 16;
    let (size_str, string_date) = format_size_and_date(size, modified_date, is_header, is_reference_folder);
    let color = get_row_color(is_header);

    let fps_str = fps.map(|f| format!("{f:.2}")).unwrap_or_default();
    let bitrate_str = format_bitrate_opt(bitrate);
    let codec_str = codec.unwrap_or_default();
    let dimensions = match (width, height) {
        (Some(w), Some(h)) => format!("{w}x{h}"),
        _ => "".to_string(),
    };
    let duration_str = format_duration_opt(duration);

    let values: [(u32, &dyn ToValue); COLUMNS_NUMBER] = [
        (ColumnsSimilarVideos::ActivatableSelectButton as u32, &(!is_header)),
        (ColumnsSimilarVideos::SelectionButton as u32, &false),
        (ColumnsSimilarVideos::Size as u32, &size_str),
        (ColumnsSimilarVideos::SizeAsBytes as u32, &size),
        (ColumnsSimilarVideos::Fps as u32, &fps_str),
        (ColumnsSimilarVideos::Codec as u32, &codec_str),
        (ColumnsSimilarVideos::Bitrate as u32, &bitrate_str),
        (ColumnsSimilarVideos::Dimensions as u32, &dimensions),
        (ColumnsSimilarVideos::Duration as u32, &duration_str),
        (ColumnsSimilarVideos::Name as u32, &file),
        (ColumnsSimilarVideos::Path as u32, &directory),
        (ColumnsSimilarVideos::Modification as u32, &string_date),
        (ColumnsSimilarVideos::ModificationAsSecs as u32, &modified_date),
        (ColumnsSimilarVideos::Color as u32, &color),
        (ColumnsSimilarVideos::IsHeader as u32, &is_header),
        (ColumnsSimilarVideos::TextColor as u32, &TEXT_COLOR),
    ];

    append_row_to_list_store(list_store, &values);
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
    let (size_str, string_date) = format_size_and_date(size, modified_date, is_header, is_reference_folder);
    let color = get_row_color(is_header);

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

    append_row_to_list_store(list_store, &values);
}

fn get_dt_timestamp_string(timestamp: u64) -> String {
    DateTime::from_timestamp(timestamp as i64, 0)
        .expect("Modified date always should be in valid range")
        .to_string()
}

fn set_specific_buttons_as_active(buttons_array: &Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>, notebook_enum: NotebookMainEnum, value_to_set: bool) {
    let mut b_mut = buttons_array.borrow_mut();
    let butt = b_mut.get_mut(&notebook_enum).expect("Failed to borrow buttons");
    let allowed_buttons = NOTEBOOKS_INFO[notebook_enum as usize].bottom_buttons;
    for i in allowed_buttons {
        *butt.get_mut(i).expect("Failed to borrow buttons") = value_to_set;
    }
}
