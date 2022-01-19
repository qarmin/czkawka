use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use glib::Sender;
use gtk::prelude::*;

use czkawka_core::big_file::BigFile;
use czkawka_core::broken_files::BrokenFiles;
use czkawka_core::common_dir_traversal;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::{MusicSimilarity, SameMusic};
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::similar_videos::SimilarVideos;
use czkawka_core::temporary::Temporary;
use czkawka_core::*;

use crate::gui_structs::gui_data::GuiData;
use crate::help_combo_box::{
    DUPLICATES_CHECK_METHOD_COMBO_BOX, DUPLICATES_HASH_TYPE_COMBO_BOX, IMAGES_HASH_SIZE_COMBO_BOX, IMAGES_HASH_TYPE_COMBO_BOX, IMAGES_RESIZE_ALGORITHM_COMBO_BOX,
};
use crate::help_functions::*;
use crate::notebook_enums::*;
use crate::taskbar_progress::tbp_flags::TBPF_NOPROGRESS;
use crate::{flg, DEFAULT_MAXIMAL_FILE_SIZE, DEFAULT_MINIMAL_CACHE_SIZE, DEFAULT_MINIMAL_FILE_SIZE};

#[allow(clippy::too_many_arguments)]
pub fn connect_button_search(
    gui_data: &GuiData,
    glib_stop_sender: Sender<Message>,
    futures_sender_duplicate_files: futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
    futures_sender_empty_files: futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
    futures_sender_empty_folder: futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
    futures_sender_big_file: futures::channel::mpsc::UnboundedSender<big_file::ProgressData>,
    futures_sender_same_music: futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
    futures_sender_similar_images: futures::channel::mpsc::UnboundedSender<similar_images::ProgressData>,
    futures_sender_similar_videos: futures::channel::mpsc::UnboundedSender<similar_videos::ProgressData>,
    futures_sender_temporary: futures::channel::mpsc::UnboundedSender<temporary::ProgressData>,
    futures_sender_invalid_symlinks: futures::channel::mpsc::UnboundedSender<common_dir_traversal::ProgressData>,
    futures_sender_broken_files: futures::channel::mpsc::UnboundedSender<broken_files::ProgressData>,
) {
    let combo_box_image_hash_size = gui_data.main_notebook.combo_box_image_hash_size.clone();
    let combo_box_image_hash_algorithm = gui_data.main_notebook.combo_box_image_hash_algorithm.clone();
    let combo_box_image_resize_algorithm = gui_data.main_notebook.combo_box_image_resize_algorithm.clone();
    let combo_box_duplicate_check_method = gui_data.main_notebook.combo_box_duplicate_check_method.clone();
    let combo_box_duplicate_hash_type = gui_data.main_notebook.combo_box_duplicate_hash_type.clone();
    let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
    let check_button_image_ignore_same_size = gui_data.main_notebook.check_button_image_ignore_same_size.clone();
    let check_button_video_ignore_same_size = gui_data.main_notebook.check_button_video_ignore_same_size.clone();
    let buttons_names = gui_data.bottom_buttons.buttons_names;
    let buttons_search_clone = gui_data.bottom_buttons.buttons_search.clone();
    let check_button_duplicates_use_prehash_cache = gui_data.settings.check_button_duplicates_use_prehash_cache.clone();
    let check_button_music_album_artist: gtk::CheckButton = gui_data.main_notebook.check_button_music_album_artist.clone();
    let check_button_music_album_title: gtk::CheckButton = gui_data.main_notebook.check_button_music_album_title.clone();
    let check_button_music_artist: gtk::CheckButton = gui_data.main_notebook.check_button_music_artist.clone();
    let check_button_music_title: gtk::CheckButton = gui_data.main_notebook.check_button_music_title.clone();
    let check_button_music_year: gtk::CheckButton = gui_data.main_notebook.check_button_music_year.clone();
    let check_button_recursive = gui_data.upper_notebook.check_button_recursive.clone();
    let check_button_settings_duplicates_delete_outdated_cache = gui_data.settings.check_button_settings_duplicates_delete_outdated_cache.clone();
    let check_button_settings_hide_hard_links = gui_data.settings.check_button_settings_hide_hard_links.clone();
    let check_button_settings_similar_images_delete_outdated_cache = gui_data.settings.check_button_settings_similar_images_delete_outdated_cache.clone();
    let check_button_settings_similar_videos_delete_outdated_cache = gui_data.settings.check_button_settings_similar_videos_delete_outdated_cache.clone();
    let check_button_settings_use_cache = gui_data.settings.check_button_settings_use_cache.clone();
    let entry_allowed_extensions = gui_data.upper_notebook.entry_allowed_extensions.clone();
    let entry_big_files_number = gui_data.main_notebook.entry_big_files_number.clone();
    let entry_excluded_items = gui_data.upper_notebook.entry_excluded_items.clone();
    let entry_general_maximal_size = gui_data.upper_notebook.entry_general_maximal_size.clone();
    let entry_general_minimal_size = gui_data.upper_notebook.entry_general_minimal_size.clone();
    let entry_settings_cache_file_minimal_size = gui_data.settings.entry_settings_cache_file_minimal_size.clone();
    let entry_settings_prehash_cache_file_minimal_size = gui_data.settings.entry_settings_prehash_cache_file_minimal_size.clone();
    let grid_progress_stages = gui_data.progress_window.grid_progress_stages.clone();
    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();
    let label_stage = gui_data.progress_window.label_stage.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let notebook_upper = gui_data.upper_notebook.notebook_upper.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
    let scale_similarity_similar_videos = gui_data.main_notebook.scale_similarity_similar_videos.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
    let stop_receiver = gui_data.stop_receiver.clone();
    let taskbar_state = gui_data.taskbar_state.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let tree_view_big_files_finder = gui_data.main_notebook.tree_view_big_files_finder.clone();
    let tree_view_broken_files = gui_data.main_notebook.tree_view_broken_files.clone();
    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_empty_files_finder = gui_data.main_notebook.tree_view_empty_files_finder.clone();
    let tree_view_empty_folder_finder = gui_data.main_notebook.tree_view_empty_folder_finder.clone();
    let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
    let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
    let tree_view_invalid_symlinks = gui_data.main_notebook.tree_view_invalid_symlinks.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_similar_videos_finder = gui_data.main_notebook.tree_view_similar_videos_finder.clone();
    let tree_view_temporary_files_finder = gui_data.main_notebook.tree_view_temporary_files_finder.clone();
    let window_progress = gui_data.progress_window.window_progress.clone();
    let entry_info = gui_data.entry_info.clone();
    let button_settings = gui_data.header.button_settings.clone();
    let button_app_info = gui_data.header.button_app_info.clone();
    let check_button_music_approximate_comparison = gui_data.main_notebook.check_button_music_approximate_comparison.clone();
    let check_button_image_fast_compare = gui_data.main_notebook.check_button_image_fast_compare.clone();
    let check_button_settings_save_also_json = gui_data.settings.check_button_settings_save_also_json.clone();

    buttons_search_clone.connect_clicked(move |_| {
        let included_directories = get_path_buf_from_vector_of_strings(get_string_from_list_store(&tree_view_included_directories, ColumnsIncludedDirectory::Path as i32, None));
        let excluded_directories = get_path_buf_from_vector_of_strings(get_string_from_list_store(&tree_view_excluded_directories, ColumnsExcludedDirectory::Path as i32, None));
        let reference_directories = get_path_buf_from_vector_of_strings(get_string_from_list_store(
            &tree_view_included_directories,
            ColumnsIncludedDirectory::Path as i32,
            Some(ColumnsIncludedDirectory::ReferenceButton as i32),
        ));
        let recursive_search = check_button_recursive.is_active();
        let excluded_items = entry_excluded_items.text().as_str().to_string().split(',').map(|e| e.to_string()).collect::<Vec<String>>();
        let allowed_extensions = entry_allowed_extensions.text().as_str().to_string();
        let hide_hard_links = check_button_settings_hide_hard_links.is_active();
        let use_cache = check_button_settings_use_cache.is_active();
        let save_also_as_json = check_button_settings_save_also_json.is_active();
        let minimal_cache_file_size = entry_settings_cache_file_minimal_size
            .text()
            .as_str()
            .parse::<u64>()
            .unwrap_or_else(|_| DEFAULT_MINIMAL_CACHE_SIZE.parse::<u64>().unwrap());

        let minimal_file_size = entry_general_minimal_size
            .text()
            .as_str()
            .parse::<u64>()
            .unwrap_or_else(|_| DEFAULT_MINIMAL_FILE_SIZE.parse::<u64>().unwrap());
        let maximal_file_size = entry_general_maximal_size
            .text()
            .as_str()
            .parse::<u64>()
            .unwrap_or_else(|_| DEFAULT_MAXIMAL_FILE_SIZE.parse::<u64>().unwrap());

        let show_dialog = Arc::new(AtomicBool::new(true));

        hide_all_buttons(&buttons_array);

        notebook_main.set_sensitive(false);
        notebook_upper.set_sensitive(false);
        button_settings.set_sensitive(false);
        button_app_info.set_sensitive(false);

        entry_info.set_text(&flg!("searching_for_data"));

        // Resets progress bars
        progress_bar_all_stages.set_fraction(0 as f64);
        progress_bar_current_stage.set_fraction(0 as f64);

        reset_text_view(&text_view_errors);

        let glib_stop_sender = glib_stop_sender.clone();
        let stop_receiver = stop_receiver.clone();

        match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
            NotebookMainEnum::Duplicate => {
                image_preview_duplicates.hide();

                label_stage.show();
                grid_progress_stages.show_all();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_duplicate_finder).clear();

                let check_method_index = combo_box_duplicate_check_method.active().unwrap() as usize;
                let check_method = DUPLICATES_CHECK_METHOD_COMBO_BOX[check_method_index].check_method;

                let hash_type_index = combo_box_duplicate_hash_type.active().unwrap() as usize;
                let hash_type = DUPLICATES_HASH_TYPE_COMBO_BOX[hash_type_index].hash_type;

                let use_prehash_cache = check_button_duplicates_use_prehash_cache.is_active();
                let minimal_prehash_cache_file_size = entry_settings_prehash_cache_file_minimal_size.text().as_str().parse::<u64>().unwrap_or(0);

                let delete_outdated_cache = check_button_settings_duplicates_delete_outdated_cache.is_active();

                let futures_sender_duplicate_files = futures_sender_duplicate_files.clone();
                // Find duplicates
                thread::spawn(move || {
                    let mut df = DuplicateFinder::new();
                    df.set_included_directory(included_directories);
                    df.set_excluded_directory(excluded_directories);
                    df.set_reference_directory(reference_directories);
                    df.set_recursive_search(recursive_search);
                    df.set_excluded_items(excluded_items);
                    df.set_allowed_extensions(allowed_extensions);
                    df.set_minimal_file_size(minimal_file_size);
                    df.set_maximal_file_size(maximal_file_size);
                    df.set_minimal_cache_file_size(minimal_cache_file_size);
                    df.set_minimal_prehash_cache_file_size(minimal_prehash_cache_file_size);
                    df.set_check_method(check_method);
                    df.set_hash_type(hash_type);
                    df.set_ignore_hard_links(hide_hard_links);
                    df.set_use_cache(use_cache);
                    df.set_use_prehash_cache(use_prehash_cache);
                    df.set_delete_outdated_cache(delete_outdated_cache);
                    df.find_duplicates(Some(&stop_receiver), Some(&futures_sender_duplicate_files));
                    let _ = glib_stop_sender.send(Message::Duplicates(df));
                });
            }
            NotebookMainEnum::EmptyFiles => {
                label_stage.show();
                grid_progress_stages.hide();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_empty_files_finder).clear();

                let futures_sender_empty_files = futures_sender_empty_files.clone();
                // Find empty files
                thread::spawn(move || {
                    let mut vf = EmptyFiles::new();

                    vf.set_included_directory(included_directories);
                    vf.set_excluded_directory(excluded_directories);
                    vf.set_recursive_search(recursive_search);
                    vf.set_excluded_items(excluded_items);
                    vf.set_allowed_extensions(allowed_extensions);
                    vf.find_empty_files(Some(&stop_receiver), Some(&futures_sender_empty_files));
                    let _ = glib_stop_sender.send(Message::EmptyFiles(vf));
                });
            }
            NotebookMainEnum::EmptyDirectories => {
                label_stage.show();
                grid_progress_stages.hide();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_empty_folder_finder).clear();

                let futures_sender_empty_folder = futures_sender_empty_folder.clone();
                // Find empty folders
                thread::spawn(move || {
                    let mut ef = EmptyFolder::new();
                    ef.set_included_directory(included_directories);
                    ef.set_excluded_directory(excluded_directories);
                    ef.set_excluded_items(excluded_items);
                    ef.find_empty_folders(Some(&stop_receiver), Some(&futures_sender_empty_folder));
                    let _ = glib_stop_sender.send(Message::EmptyFolders(ef));
                });
            }
            NotebookMainEnum::BigFiles => {
                label_stage.show();
                grid_progress_stages.hide();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_big_files_finder).clear();

                let numbers_of_files_to_check = entry_big_files_number.text().as_str().parse::<usize>().unwrap_or(50);

                let futures_sender_big_file = futures_sender_big_file.clone();
                // Find big files
                thread::spawn(move || {
                    let mut bf = BigFile::new();

                    bf.set_included_directory(included_directories);
                    bf.set_excluded_directory(excluded_directories);
                    bf.set_recursive_search(recursive_search);
                    bf.set_excluded_items(excluded_items);
                    bf.set_allowed_extensions(allowed_extensions);
                    bf.set_number_of_files_to_check(numbers_of_files_to_check);
                    bf.find_big_files(Some(&stop_receiver), Some(&futures_sender_big_file));
                    let _ = glib_stop_sender.send(Message::BigFiles(bf));
                });
            }
            NotebookMainEnum::Temporary => {
                label_stage.show();
                grid_progress_stages.hide();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_temporary_files_finder).clear();

                let futures_sender_temporary = futures_sender_temporary.clone();
                // Find temporary files
                thread::spawn(move || {
                    let mut tf = Temporary::new();

                    tf.set_included_directory(included_directories);
                    tf.set_excluded_directory(excluded_directories);
                    tf.set_recursive_search(recursive_search);
                    tf.set_excluded_items(excluded_items);
                    tf.find_temporary_files(Some(&stop_receiver), Some(&futures_sender_temporary));
                    let _ = glib_stop_sender.send(Message::Temporary(tf));
                });
            }
            NotebookMainEnum::SimilarImages => {
                image_preview_similar_images.hide();

                label_stage.show();
                grid_progress_stages.show_all();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_similar_images_finder).clear();

                let hash_size_index = combo_box_image_hash_size.active().unwrap() as usize;
                let hash_size = IMAGES_HASH_SIZE_COMBO_BOX[hash_size_index] as u8;

                let image_filter_index = combo_box_image_resize_algorithm.active().unwrap() as usize;
                let image_filter = IMAGES_RESIZE_ALGORITHM_COMBO_BOX[image_filter_index].filter;

                let hash_alg_index = combo_box_image_hash_algorithm.active().unwrap() as usize;
                let hash_alg = IMAGES_HASH_TYPE_COMBO_BOX[hash_alg_index].hash_alg;

                let ignore_same_size = check_button_image_ignore_same_size.is_active();

                let similarity = similar_images::Similarity::Similar(scale_similarity_similar_images.value() as u32);

                let delete_outdated_cache = check_button_settings_similar_images_delete_outdated_cache.is_active();

                let fast_compare = check_button_image_fast_compare.is_active();

                let futures_sender_similar_images = futures_sender_similar_images.clone();
                // Find similar images
                thread::spawn(move || {
                    let mut sf = SimilarImages::new();

                    sf.set_included_directory(included_directories);
                    sf.set_excluded_directory(excluded_directories);
                    sf.set_reference_directory(reference_directories);
                    sf.set_recursive_search(recursive_search);
                    sf.set_excluded_items(excluded_items);
                    sf.set_minimal_file_size(minimal_file_size);
                    sf.set_maximal_file_size(maximal_file_size);
                    sf.set_similarity(similarity);
                    sf.set_use_cache(use_cache);
                    sf.set_hash_alg(hash_alg);
                    sf.set_hash_size(hash_size);
                    sf.set_image_filter(image_filter);
                    sf.set_allowed_extensions(allowed_extensions);
                    sf.set_delete_outdated_cache(delete_outdated_cache);
                    sf.set_exclude_images_with_same_size(ignore_same_size);
                    sf.set_fast_comparing(fast_compare);
                    sf.set_save_also_as_json(save_also_as_json);
                    sf.find_similar_images(Some(&stop_receiver), Some(&futures_sender_similar_images));
                    let _ = glib_stop_sender.send(Message::SimilarImages(sf));
                });
            }
            NotebookMainEnum::SimilarVideos => {
                label_stage.show();
                grid_progress_stages.show_all();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_similar_videos_finder).clear();

                let tolerance = scale_similarity_similar_videos.value() as i32;

                let delete_outdated_cache = check_button_settings_similar_videos_delete_outdated_cache.is_active();

                let ignore_same_size = check_button_video_ignore_same_size.is_active();

                let futures_sender_similar_videos = futures_sender_similar_videos.clone();
                // Find similar videos
                thread::spawn(move || {
                    let mut sf = SimilarVideos::new();

                    sf.set_included_directory(included_directories);
                    sf.set_excluded_directory(excluded_directories);
                    sf.set_reference_directory(reference_directories);
                    sf.set_recursive_search(recursive_search);
                    sf.set_excluded_items(excluded_items);
                    sf.set_minimal_file_size(minimal_file_size);
                    sf.set_maximal_file_size(maximal_file_size);
                    sf.set_allowed_extensions(allowed_extensions);
                    sf.set_use_cache(use_cache);
                    sf.set_tolerance(tolerance);
                    sf.set_delete_outdated_cache(delete_outdated_cache);
                    sf.set_exclude_videos_with_same_size(ignore_same_size);
                    sf.set_save_also_as_json(save_also_as_json);
                    sf.find_similar_videos(Some(&stop_receiver), Some(&futures_sender_similar_videos));
                    let _ = glib_stop_sender.send(Message::SimilarVideos(sf));
                });
            }
            NotebookMainEnum::SameMusic => {
                label_stage.show();
                grid_progress_stages.show_all();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_same_music_finder).clear();

                let approximate_comparison = check_button_music_approximate_comparison.is_active();

                let mut music_similarity: MusicSimilarity = MusicSimilarity::NONE;

                if check_button_music_title.is_active() {
                    music_similarity |= MusicSimilarity::TITLE;
                }
                if check_button_music_artist.is_active() {
                    music_similarity |= MusicSimilarity::ARTIST;
                }
                if check_button_music_album_title.is_active() {
                    music_similarity |= MusicSimilarity::ALBUM_TITLE;
                }
                if check_button_music_album_artist.is_active() {
                    music_similarity |= MusicSimilarity::ALBUM_ARTIST;
                }
                if check_button_music_year.is_active() {
                    music_similarity |= MusicSimilarity::YEAR;
                }

                if music_similarity != MusicSimilarity::NONE {
                    let futures_sender_same_music = futures_sender_same_music.clone();
                    // Find Similar music
                    thread::spawn(move || {
                        let mut mf = SameMusic::new();

                        mf.set_included_directory(included_directories);
                        mf.set_excluded_directory(excluded_directories);
                        mf.set_reference_directory(reference_directories);
                        mf.set_excluded_items(excluded_items);
                        mf.set_minimal_file_size(minimal_file_size);
                        mf.set_maximal_file_size(maximal_file_size);
                        mf.set_allowed_extensions(allowed_extensions);
                        mf.set_recursive_search(recursive_search);
                        mf.set_music_similarity(music_similarity);
                        mf.set_approximate_comparison(approximate_comparison);
                        mf.set_save_also_as_json(save_also_as_json);
                        mf.find_same_music(Some(&stop_receiver), Some(&futures_sender_same_music));
                        let _ = glib_stop_sender.send(Message::SameMusic(mf));
                    });
                } else {
                    set_buttons(
                        &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap(),
                        &buttons_array,
                        &buttons_names,
                    );
                    entry_info.set_text(&flg!("search_not_choosing_any_music"));
                    show_dialog.store(false, Ordering::Relaxed);
                }
            }
            NotebookMainEnum::Symlinks => {
                label_stage.show();
                grid_progress_stages.hide();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_invalid_symlinks).clear();

                let futures_sender_invalid_symlinks = futures_sender_invalid_symlinks.clone();

                thread::spawn(move || {
                    let mut isf = InvalidSymlinks::new();

                    isf.set_included_directory(included_directories);
                    isf.set_excluded_directory(excluded_directories);
                    isf.set_recursive_search(recursive_search);
                    isf.set_excluded_items(excluded_items);
                    isf.set_allowed_extensions(allowed_extensions);
                    isf.find_invalid_links(Some(&stop_receiver), Some(&futures_sender_invalid_symlinks));
                    let _ = glib_stop_sender.send(Message::InvalidSymlinks(isf));
                });
            }
            NotebookMainEnum::BrokenFiles => {
                label_stage.show();
                grid_progress_stages.show();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_broken_files).clear();

                let futures_sender_broken_files = futures_sender_broken_files.clone();

                thread::spawn(move || {
                    let mut br = BrokenFiles::new();

                    br.set_included_directory(included_directories);
                    br.set_excluded_directory(excluded_directories);
                    br.set_recursive_search(recursive_search);
                    br.set_excluded_items(excluded_items);
                    br.set_use_cache(use_cache);
                    br.set_allowed_extensions(allowed_extensions);
                    br.set_save_also_as_json(save_also_as_json);
                    br.find_broken_files(Some(&stop_receiver), Some(&futures_sender_broken_files));
                    let _ = glib_stop_sender.send(Message::BrokenFiles(br));
                });
            }
        }

        // Show progress dialog
        if show_dialog.load(Ordering::Relaxed) {
            window_progress.show();
            taskbar_state.borrow().show();
            taskbar_state.borrow().set_progress_state(TBPF_NOPROGRESS);
        }
    });
}
