use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use crossbeam_channel::Receiver;
use fun_time::fun_time;
use futures::channel::mpsc::UnboundedSender;
use glib::Sender;
use gtk4::prelude::*;
use gtk4::Grid;

use czkawka_core::bad_extensions::BadExtensions;
use czkawka_core::big_file::BigFile;
use czkawka_core::broken_files::{BrokenFiles, CheckedTypes};
use czkawka_core::common_dir_traversal::{CheckingMethod, ProgressData};
use czkawka_core::common_tool::CommonData;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::{MusicSimilarity, SameMusic};
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::similar_videos::SimilarVideos;
use czkawka_core::temporary::Temporary;

use crate::gui_structs::gui_data::GuiData;
use crate::help_combo_box::{
    AUDIO_TYPE_CHECK_METHOD_COMBO_BOX, BIG_FILES_CHECK_METHOD_COMBO_BOX, DUPLICATES_CHECK_METHOD_COMBO_BOX, DUPLICATES_HASH_TYPE_COMBO_BOX, IMAGES_HASH_SIZE_COMBO_BOX,
    IMAGES_HASH_TYPE_COMBO_BOX, IMAGES_RESIZE_ALGORITHM_COMBO_BOX,
};
use crate::help_functions::*;
use crate::notebook_enums::*;
use crate::taskbar_progress::tbp_flags::TBPF_NOPROGRESS;
use crate::{flg, DEFAULT_MAXIMAL_FILE_SIZE, DEFAULT_MINIMAL_CACHE_SIZE, DEFAULT_MINIMAL_FILE_SIZE};

#[allow(clippy::too_many_arguments)]
pub fn connect_button_search(gui_data: &GuiData, glib_stop_sender: Sender<Message>, progress_sender: UnboundedSender<ProgressData>) {
    let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
    let buttons_search_clone = gui_data.bottom_buttons.buttons_search.clone();
    let grid_progress_stages = gui_data.progress_window.grid_progress_stages.clone();
    let label_stage = gui_data.progress_window.label_stage.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let notebook_upper = gui_data.upper_notebook.notebook_upper.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let stop_receiver = gui_data.stop_receiver.clone();
    let taskbar_state = gui_data.taskbar_state.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
    let window_progress = gui_data.progress_window.window_progress.clone();
    let entry_info = gui_data.entry_info.clone();
    let button_settings = gui_data.header.button_settings.clone();
    let button_app_info = gui_data.header.button_app_info.clone();

    let gui_data = gui_data.clone();
    buttons_search_clone.connect_clicked(move |_| {
        let loaded_common_items = LoadedCommonItems::load_items(&gui_data);

        // Check if user selected all referenced folders
        let list_store_included_directories = get_list_store(&tree_view_included_directories);
        if check_if_list_store_column_have_all_same_values(&list_store_included_directories, ColumnsIncludedDirectory::ReferenceButton as i32, true) {
            entry_info.set_text(&flg!("selected_all_reference_folders"));
            return;
        }

        let show_dialog = Arc::new(AtomicBool::new(true));

        window_progress.set_title(Some(&flg!("window_progress_title")));

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
        // Consume any stale stop messages.
        stop_receiver.try_iter().for_each(|()| ());

        label_stage.show();

        match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
            NotebookMainEnum::Duplicate => duplicate_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
            ),
            NotebookMainEnum::EmptyFiles => empty_files_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
            ),
            NotebookMainEnum::EmptyDirectories => empty_directories_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
            ),
            NotebookMainEnum::BigFiles => big_files_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
            ),
            NotebookMainEnum::Temporary => temporary_files_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
            ),
            NotebookMainEnum::SimilarImages => similar_image_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
            ),
            NotebookMainEnum::SimilarVideos => similar_video_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
            ),
            NotebookMainEnum::SameMusic => same_music_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
                &show_dialog,
            ),
            NotebookMainEnum::Symlinks => bad_symlinks_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
            ),
            NotebookMainEnum::BrokenFiles => broken_files_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
                &show_dialog,
            ),
            NotebookMainEnum::BadExtensions => bad_extensions_search(
                &gui_data,
                loaded_common_items,
                stop_receiver,
                glib_stop_sender,
                &grid_progress_stages,
                progress_sender.clone(),
            ),
        }

        window_progress.set_default_size(1, 1);

        // Show progress dialog
        if show_dialog.load(Ordering::Relaxed) {
            window_progress.show();
            taskbar_state.borrow().show();
            taskbar_state.borrow().set_progress_state(TBPF_NOPROGRESS);
        }
    });
}

struct LoadedCommonItems {
    included_directories: Vec<PathBuf>,
    excluded_directories: Vec<PathBuf>,
    reference_directories: Vec<PathBuf>,
    recursive_search: bool,
    excluded_items: Vec<String>,
    allowed_extensions: String,
    hide_hard_links: bool,
    use_cache: bool,
    save_also_as_json: bool,
    minimal_cache_file_size: u64,
    minimal_file_size: u64,
    maximal_file_size: u64,
    ignore_other_filesystems: bool,
}

impl LoadedCommonItems {
    fn load_items(gui_data: &GuiData) -> Self {
        let check_button_settings_one_filesystem = gui_data.settings.check_button_settings_one_filesystem.clone();
        let check_button_recursive = gui_data.upper_notebook.check_button_recursive.clone();
        let check_button_settings_hide_hard_links = gui_data.settings.check_button_settings_hide_hard_links.clone();
        let check_button_settings_use_cache = gui_data.settings.check_button_settings_use_cache.clone();
        let entry_allowed_extensions = gui_data.upper_notebook.entry_allowed_extensions.clone();
        let entry_excluded_items = gui_data.upper_notebook.entry_excluded_items.clone();
        let entry_general_maximal_size = gui_data.upper_notebook.entry_general_maximal_size.clone();
        let entry_general_minimal_size = gui_data.upper_notebook.entry_general_minimal_size.clone();
        let entry_settings_cache_file_minimal_size = gui_data.settings.entry_settings_cache_file_minimal_size.clone();
        let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
        let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
        let check_button_settings_save_also_json = gui_data.settings.check_button_settings_save_also_json.clone();

        let included_directories = get_path_buf_from_vector_of_strings(&get_string_from_list_store(&tree_view_included_directories, ColumnsIncludedDirectory::Path as i32, None));
        let excluded_directories = get_path_buf_from_vector_of_strings(&get_string_from_list_store(&tree_view_excluded_directories, ColumnsExcludedDirectory::Path as i32, None));
        let reference_directories = get_path_buf_from_vector_of_strings(&get_string_from_list_store(
            &tree_view_included_directories,
            ColumnsIncludedDirectory::Path as i32,
            Some(ColumnsIncludedDirectory::ReferenceButton as i32),
        ));
        let recursive_search = check_button_recursive.is_active();
        let excluded_items = entry_excluded_items
            .text()
            .as_str()
            .to_string()
            .split(',')
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();
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
        let ignore_other_filesystems = check_button_settings_one_filesystem.is_active();

        LoadedCommonItems {
            included_directories,
            excluded_directories,
            reference_directories,
            recursive_search,
            excluded_items,
            allowed_extensions,
            hide_hard_links,
            use_cache,
            save_also_as_json,
            minimal_cache_file_size,
            minimal_file_size,
            maximal_file_size,
            ignore_other_filesystems,
        }
    }
}

fn duplicate_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
) {
    grid_progress_stages.show();

    let combo_box_duplicate_check_method = gui_data.main_notebook.combo_box_duplicate_check_method.clone();
    let combo_box_duplicate_hash_type = gui_data.main_notebook.combo_box_duplicate_hash_type.clone();
    let check_button_duplicates_use_prehash_cache = gui_data.settings.check_button_duplicates_use_prehash_cache.clone();
    let check_button_duplicate_case_sensitive_name: gtk4::CheckButton = gui_data.main_notebook.check_button_duplicate_case_sensitive_name.clone();
    let check_button_settings_duplicates_delete_outdated_cache = gui_data.settings.check_button_settings_duplicates_delete_outdated_cache.clone();
    let entry_settings_prehash_cache_file_minimal_size = gui_data.settings.entry_settings_prehash_cache_file_minimal_size.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();
    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();

    image_preview_duplicates.hide();
    clean_tree_view(&tree_view_duplicate_finder);

    let check_method_index = combo_box_duplicate_check_method.active().unwrap() as usize;
    let check_method = DUPLICATES_CHECK_METHOD_COMBO_BOX[check_method_index].check_method;

    let hash_type_index = combo_box_duplicate_hash_type.active().unwrap() as usize;
    let hash_type = DUPLICATES_HASH_TYPE_COMBO_BOX[hash_type_index].hash_type;

    let use_prehash_cache = check_button_duplicates_use_prehash_cache.is_active();
    let minimal_prehash_cache_file_size = entry_settings_prehash_cache_file_minimal_size.text().as_str().parse::<u64>().unwrap_or(0);

    let case_sensitive_name_comparison = check_button_duplicate_case_sensitive_name.is_active();

    let delete_outdated_cache = check_button_settings_duplicates_delete_outdated_cache.is_active();

    // Find duplicates
    thread::spawn(move || {
        let mut df = DuplicateFinder::new();
        df.set_included_directory(loaded_common_items.included_directories);
        df.set_excluded_directory(loaded_common_items.excluded_directories);
        df.set_reference_directory(loaded_common_items.reference_directories);
        df.set_recursive_search(loaded_common_items.recursive_search);
        df.set_excluded_items(loaded_common_items.excluded_items);
        df.set_allowed_extensions(loaded_common_items.allowed_extensions);
        df.set_minimal_file_size(loaded_common_items.minimal_file_size);
        df.set_maximal_file_size(loaded_common_items.maximal_file_size);
        df.set_minimal_cache_file_size(loaded_common_items.minimal_cache_file_size);
        df.set_minimal_prehash_cache_file_size(minimal_prehash_cache_file_size);
        df.set_check_method(check_method);
        df.set_hash_type(hash_type);
        df.set_save_also_as_json(loaded_common_items.save_also_as_json);
        df.set_ignore_hard_links(loaded_common_items.hide_hard_links);
        df.set_use_cache(loaded_common_items.use_cache);
        df.set_use_prehash_cache(use_prehash_cache);
        df.set_delete_outdated_cache(delete_outdated_cache);
        df.set_case_sensitive_name_comparison(case_sensitive_name_comparison);
        df.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
        df.find_duplicates(Some(&stop_receiver), Some(&progress_data_sender));
        glib_stop_sender.send(Message::Duplicates(df)).unwrap();
    });
}

fn empty_files_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
) {
    grid_progress_stages.hide();

    let tree_view_empty_files_finder = gui_data.main_notebook.tree_view_empty_files_finder.clone();
    clean_tree_view(&tree_view_empty_files_finder);
    // Find empty files
    thread::spawn(move || {
        let mut vf = EmptyFiles::new();

        vf.set_included_directory(loaded_common_items.included_directories);
        vf.set_excluded_directory(loaded_common_items.excluded_directories);
        vf.set_recursive_search(loaded_common_items.recursive_search);
        vf.set_excluded_items(loaded_common_items.excluded_items);
        vf.set_allowed_extensions(loaded_common_items.allowed_extensions);
        vf.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
        vf.find_empty_files(Some(&stop_receiver), Some(&progress_data_sender));
        glib_stop_sender.send(Message::EmptyFiles(vf)).unwrap();
    });
}

fn empty_directories_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
) {
    grid_progress_stages.hide();

    let tree_view_empty_folder_finder = gui_data.main_notebook.tree_view_empty_folder_finder.clone();
    clean_tree_view(&tree_view_empty_folder_finder);

    thread::spawn(move || {
        let mut ef = EmptyFolder::new();
        ef.set_included_directory(loaded_common_items.included_directories);
        ef.set_excluded_directory(loaded_common_items.excluded_directories);
        ef.set_excluded_items(loaded_common_items.excluded_items);
        ef.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
        ef.find_empty_folders(Some(&stop_receiver), Some(&progress_data_sender));
        glib_stop_sender.send(Message::EmptyFolders(ef)).unwrap();
    });
}

fn big_files_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
) {
    grid_progress_stages.hide();

    let combo_box_big_files_mode = gui_data.main_notebook.combo_box_big_files_mode.clone();
    let entry_big_files_number = gui_data.main_notebook.entry_big_files_number.clone();
    let tree_view_big_files_finder = gui_data.main_notebook.tree_view_big_files_finder.clone();
    clean_tree_view(&tree_view_big_files_finder);

    let big_files_mode_index = combo_box_big_files_mode.active().unwrap() as usize;
    let big_files_mode = BIG_FILES_CHECK_METHOD_COMBO_BOX[big_files_mode_index].check_method;

    let numbers_of_files_to_check = entry_big_files_number.text().as_str().parse::<usize>().unwrap_or(50);

    thread::spawn(move || {
        let mut bf = BigFile::new();

        bf.set_included_directory(loaded_common_items.included_directories);
        bf.set_excluded_directory(loaded_common_items.excluded_directories);
        bf.set_recursive_search(loaded_common_items.recursive_search);
        bf.set_excluded_items(loaded_common_items.excluded_items);
        bf.set_allowed_extensions(loaded_common_items.allowed_extensions);
        bf.set_number_of_files_to_check(numbers_of_files_to_check);
        bf.set_search_mode(big_files_mode);
        bf.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
        bf.find_big_files(Some(&stop_receiver), Some(&progress_data_sender));
        glib_stop_sender.send(Message::BigFiles(bf)).unwrap();
    });
}

fn temporary_files_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
) {
    grid_progress_stages.hide();

    let tree_view_temporary_files_finder = gui_data.main_notebook.tree_view_temporary_files_finder.clone();
    clean_tree_view(&tree_view_temporary_files_finder);

    thread::spawn(move || {
        let mut tf = Temporary::new();

        tf.set_included_directory(loaded_common_items.included_directories);
        tf.set_excluded_directory(loaded_common_items.excluded_directories);
        tf.set_recursive_search(loaded_common_items.recursive_search);
        tf.set_excluded_items(loaded_common_items.excluded_items);
        tf.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
        tf.find_temporary_files(Some(&stop_receiver), Some(&progress_data_sender));
        glib_stop_sender.send(Message::Temporary(tf)).unwrap();
    });
}

fn same_music_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
    show_dialog: &Arc<AtomicBool>,
) {
    grid_progress_stages.show();

    let check_button_music_artist: gtk4::CheckButton = gui_data.main_notebook.check_button_music_artist.clone();
    let check_button_music_title: gtk4::CheckButton = gui_data.main_notebook.check_button_music_title.clone();
    let check_button_music_year: gtk4::CheckButton = gui_data.main_notebook.check_button_music_year.clone();
    let check_button_music_genre: gtk4::CheckButton = gui_data.main_notebook.check_button_music_genre.clone();
    let check_button_music_length: gtk4::CheckButton = gui_data.main_notebook.check_button_music_length.clone();
    let check_button_music_bitrate: gtk4::CheckButton = gui_data.main_notebook.check_button_music_bitrate.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let combo_box_audio_check_type = gui_data.main_notebook.combo_box_audio_check_type.clone();
    let check_button_music_approximate_comparison = gui_data.main_notebook.check_button_music_approximate_comparison.clone();
    let scale_seconds_same_music = gui_data.main_notebook.scale_seconds_same_music.clone();
    let scale_similarity_same_music = gui_data.main_notebook.scale_similarity_same_music.clone();

    clean_tree_view(&tree_view_same_music_finder);

    let approximate_comparison = check_button_music_approximate_comparison.is_active();

    let mut music_similarity: MusicSimilarity = MusicSimilarity::NONE;

    if check_button_music_title.is_active() {
        music_similarity |= MusicSimilarity::TRACK_TITLE;
    }
    if check_button_music_artist.is_active() {
        music_similarity |= MusicSimilarity::TRACK_ARTIST;
    }
    if check_button_music_year.is_active() {
        music_similarity |= MusicSimilarity::YEAR;
    }
    if check_button_music_bitrate.is_active() {
        music_similarity |= MusicSimilarity::BITRATE;
    }
    if check_button_music_genre.is_active() {
        music_similarity |= MusicSimilarity::GENRE;
    }
    if check_button_music_length.is_active() {
        music_similarity |= MusicSimilarity::LENGTH;
    }

    let check_method_index = combo_box_audio_check_type.active().unwrap() as usize;
    let check_method = AUDIO_TYPE_CHECK_METHOD_COMBO_BOX[check_method_index].check_method;

    let maximum_difference = scale_similarity_same_music.value();
    let minimum_segment_duration = scale_seconds_same_music.value() as f32;

    if music_similarity != MusicSimilarity::NONE || check_method == CheckingMethod::AudioContent {
        thread::spawn(move || {
            let mut mf = SameMusic::new();

            mf.set_included_directory(loaded_common_items.included_directories);
            mf.set_excluded_directory(loaded_common_items.excluded_directories);
            mf.set_reference_directory(loaded_common_items.reference_directories);
            mf.set_excluded_items(loaded_common_items.excluded_items);
            mf.set_use_cache(loaded_common_items.use_cache);
            mf.set_minimal_file_size(loaded_common_items.minimal_file_size);
            mf.set_maximal_file_size(loaded_common_items.maximal_file_size);
            mf.set_allowed_extensions(loaded_common_items.allowed_extensions);
            mf.set_recursive_search(loaded_common_items.recursive_search);
            mf.set_music_similarity(music_similarity);
            mf.set_maximum_difference(maximum_difference);
            mf.set_minimum_segment_duration(minimum_segment_duration);
            mf.set_check_type(check_method);
            mf.set_approximate_comparison(approximate_comparison);
            mf.set_save_also_as_json(loaded_common_items.save_also_as_json);
            mf.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
            mf.find_same_music(Some(&stop_receiver), Some(&progress_data_sender));
            glib_stop_sender.send(Message::SameMusic(mf)).unwrap();
        });
    } else {
        let shared_buttons = gui_data.shared_buttons.clone();
        let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
        let buttons_names = gui_data.bottom_buttons.buttons_names;
        let entry_info = gui_data.entry_info.clone();
        let notebook_main = gui_data.main_notebook.notebook_main.clone();
        let notebook_upper = gui_data.upper_notebook.notebook_upper.clone();
        let button_settings = gui_data.header.button_settings.clone();
        let button_app_info = gui_data.header.button_app_info.clone();

        set_buttons(
            &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap(),
            &buttons_array,
            &buttons_names,
        );
        entry_info.set_text(&flg!("search_not_choosing_any_music"));
        show_dialog.store(false, Ordering::Relaxed);

        notebook_main.set_sensitive(true);
        notebook_upper.set_sensitive(true);
        button_settings.set_sensitive(true);
        button_app_info.set_sensitive(true);
    }
}

fn broken_files_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
    show_dialog: &Arc<AtomicBool>,
) {
    grid_progress_stages.show();

    let check_button_broken_files_archive: gtk4::CheckButton = gui_data.main_notebook.check_button_broken_files_archive.clone();
    let check_button_broken_files_pdf: gtk4::CheckButton = gui_data.main_notebook.check_button_broken_files_pdf.clone();
    let check_button_broken_files_audio: gtk4::CheckButton = gui_data.main_notebook.check_button_broken_files_audio.clone();
    let check_button_broken_files_image: gtk4::CheckButton = gui_data.main_notebook.check_button_broken_files_image.clone();
    let tree_view_broken_files = gui_data.main_notebook.tree_view_broken_files.clone();

    clean_tree_view(&tree_view_broken_files);

    let mut checked_types: CheckedTypes = CheckedTypes::NONE;

    if check_button_broken_files_audio.is_active() {
        checked_types |= CheckedTypes::AUDIO;
    }
    if check_button_broken_files_pdf.is_active() {
        checked_types |= CheckedTypes::PDF;
    }
    if check_button_broken_files_image.is_active() {
        checked_types |= CheckedTypes::IMAGE;
    }
    if check_button_broken_files_archive.is_active() {
        checked_types |= CheckedTypes::ARCHIVE;
    }

    if checked_types != CheckedTypes::NONE {
        thread::spawn(move || {
            let mut br = BrokenFiles::new();

            br.set_included_directory(loaded_common_items.included_directories);
            br.set_excluded_directory(loaded_common_items.excluded_directories);
            br.set_recursive_search(loaded_common_items.recursive_search);
            br.set_excluded_items(loaded_common_items.excluded_items);
            br.set_use_cache(loaded_common_items.use_cache);
            br.set_allowed_extensions(loaded_common_items.allowed_extensions);
            br.set_save_also_as_json(loaded_common_items.save_also_as_json);
            br.set_checked_types(checked_types);
            br.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
            br.find_broken_files(Some(&stop_receiver), Some(&progress_data_sender));
            glib_stop_sender.send(Message::BrokenFiles(br)).unwrap();
        });
    } else {
        let shared_buttons = gui_data.shared_buttons.clone();
        let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
        let buttons_names = gui_data.bottom_buttons.buttons_names;
        let entry_info = gui_data.entry_info.clone();
        let notebook_main = gui_data.main_notebook.notebook_main.clone();
        let notebook_upper = gui_data.upper_notebook.notebook_upper.clone();
        let button_settings = gui_data.header.button_settings.clone();
        let button_app_info = gui_data.header.button_app_info.clone();

        set_buttons(
            &mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::BrokenFiles).unwrap(),
            &buttons_array,
            &buttons_names,
        );
        entry_info.set_text(&flg!("search_not_choosing_any_broken_files"));
        show_dialog.store(false, Ordering::Relaxed);

        notebook_main.set_sensitive(true);
        notebook_upper.set_sensitive(true);
        button_settings.set_sensitive(true);
        button_app_info.set_sensitive(true);
    }
}

fn similar_image_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
) {
    grid_progress_stages.show();

    let combo_box_image_hash_size = gui_data.main_notebook.combo_box_image_hash_size.clone();
    let combo_box_image_hash_algorithm = gui_data.main_notebook.combo_box_image_hash_algorithm.clone();
    let combo_box_image_resize_algorithm = gui_data.main_notebook.combo_box_image_resize_algorithm.clone();
    let check_button_image_ignore_same_size = gui_data.main_notebook.check_button_image_ignore_same_size.clone();
    let check_button_settings_similar_images_delete_outdated_cache = gui_data.settings.check_button_settings_similar_images_delete_outdated_cache.clone();
    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let scale_similarity_similar_images = gui_data.main_notebook.scale_similarity_similar_images.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();

    clean_tree_view(&tree_view_similar_images_finder);
    image_preview_similar_images.hide();

    let hash_size_index = combo_box_image_hash_size.active().unwrap() as usize;
    let hash_size = IMAGES_HASH_SIZE_COMBO_BOX[hash_size_index] as u8;

    let image_filter_index = combo_box_image_resize_algorithm.active().unwrap() as usize;
    let image_filter = IMAGES_RESIZE_ALGORITHM_COMBO_BOX[image_filter_index].filter;

    let hash_alg_index = combo_box_image_hash_algorithm.active().unwrap() as usize;
    let hash_alg = IMAGES_HASH_TYPE_COMBO_BOX[hash_alg_index].hash_alg;

    let ignore_same_size = check_button_image_ignore_same_size.is_active();

    let similarity = scale_similarity_similar_images.value() as u32;

    let delete_outdated_cache = check_button_settings_similar_images_delete_outdated_cache.is_active();

    thread::spawn(move || {
        let mut sf = SimilarImages::new();

        sf.set_included_directory(loaded_common_items.included_directories);
        sf.set_excluded_directory(loaded_common_items.excluded_directories);
        sf.set_reference_directory(loaded_common_items.reference_directories);
        sf.set_recursive_search(loaded_common_items.recursive_search);
        sf.set_excluded_items(loaded_common_items.excluded_items);
        sf.set_minimal_file_size(loaded_common_items.minimal_file_size);
        sf.set_maximal_file_size(loaded_common_items.maximal_file_size);
        sf.set_similarity(similarity);
        sf.set_use_cache(loaded_common_items.use_cache);
        sf.set_hash_alg(hash_alg);
        sf.set_hash_size(hash_size);
        sf.set_image_filter(image_filter);
        sf.set_allowed_extensions(loaded_common_items.allowed_extensions);
        sf.set_delete_outdated_cache(delete_outdated_cache);
        sf.set_exclude_images_with_same_size(ignore_same_size);
        sf.set_save_also_as_json(loaded_common_items.save_also_as_json);
        sf.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
        sf.find_similar_images(Some(&stop_receiver), Some(&progress_data_sender));
        glib_stop_sender.send(Message::SimilarImages(sf)).unwrap();
    });
}

fn similar_video_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
) {
    grid_progress_stages.show();

    let check_button_video_ignore_same_size = gui_data.main_notebook.check_button_video_ignore_same_size.clone();
    let check_button_settings_similar_videos_delete_outdated_cache = gui_data.settings.check_button_settings_similar_videos_delete_outdated_cache.clone();
    let scale_similarity_similar_videos = gui_data.main_notebook.scale_similarity_similar_videos.clone();
    let tree_view_similar_videos_finder = gui_data.main_notebook.tree_view_similar_videos_finder.clone();
    clean_tree_view(&tree_view_similar_videos_finder);

    let tolerance = scale_similarity_similar_videos.value() as i32;

    let delete_outdated_cache = check_button_settings_similar_videos_delete_outdated_cache.is_active();

    let ignore_same_size = check_button_video_ignore_same_size.is_active();

    thread::spawn(move || {
        let mut sf = SimilarVideos::new();

        sf.set_included_directory(loaded_common_items.included_directories);
        sf.set_excluded_directory(loaded_common_items.excluded_directories);
        sf.set_reference_directory(loaded_common_items.reference_directories);
        sf.set_recursive_search(loaded_common_items.recursive_search);
        sf.set_excluded_items(loaded_common_items.excluded_items);
        sf.set_minimal_file_size(loaded_common_items.minimal_file_size);
        sf.set_maximal_file_size(loaded_common_items.maximal_file_size);
        sf.set_allowed_extensions(loaded_common_items.allowed_extensions);
        sf.set_use_cache(loaded_common_items.use_cache);
        sf.set_tolerance(tolerance);
        sf.set_delete_outdated_cache(delete_outdated_cache);
        sf.set_exclude_videos_with_same_size(ignore_same_size);
        sf.set_save_also_as_json(loaded_common_items.save_also_as_json);
        sf.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
        sf.find_similar_videos(Some(&stop_receiver), Some(&progress_data_sender));
        glib_stop_sender.send(Message::SimilarVideos(sf)).unwrap();
    });
}

fn bad_symlinks_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
) {
    grid_progress_stages.hide();

    let tree_view_invalid_symlinks = gui_data.main_notebook.tree_view_invalid_symlinks.clone();
    clean_tree_view(&tree_view_invalid_symlinks);

    thread::spawn(move || {
        let mut isf = InvalidSymlinks::new();

        isf.set_included_directory(loaded_common_items.included_directories);
        isf.set_excluded_directory(loaded_common_items.excluded_directories);
        isf.set_recursive_search(loaded_common_items.recursive_search);
        isf.set_excluded_items(loaded_common_items.excluded_items);
        isf.set_allowed_extensions(loaded_common_items.allowed_extensions);
        isf.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
        isf.find_invalid_links(Some(&stop_receiver), Some(&progress_data_sender));
        glib_stop_sender.send(Message::InvalidSymlinks(isf)).unwrap();
    });
}

fn bad_extensions_search(
    gui_data: &GuiData,
    loaded_common_items: LoadedCommonItems,
    stop_receiver: Receiver<()>,
    glib_stop_sender: Sender<Message>,
    grid_progress_stages: &Grid,
    progress_data_sender: UnboundedSender<ProgressData>,
) {
    grid_progress_stages.show();

    let tree_view_bad_extensions = gui_data.main_notebook.tree_view_bad_extensions.clone();
    clean_tree_view(&tree_view_bad_extensions);

    thread::spawn(move || {
        let mut be = BadExtensions::new();

        be.set_included_directory(loaded_common_items.included_directories);
        be.set_excluded_directory(loaded_common_items.excluded_directories);
        be.set_excluded_items(loaded_common_items.excluded_items);
        be.set_minimal_file_size(loaded_common_items.minimal_file_size);
        be.set_maximal_file_size(loaded_common_items.maximal_file_size);
        be.set_allowed_extensions(loaded_common_items.allowed_extensions);
        be.set_recursive_search(loaded_common_items.recursive_search);
        be.set_exclude_other_filesystems(loaded_common_items.ignore_other_filesystems);
        be.find_bad_extensions_files(Some(&stop_receiver), Some(&progress_data_sender));
        glib_stop_sender.send(Message::BadExtensions(be)).unwrap();
    });
}

#[fun_time(message = "clean_tree_view", level = "debug")]
fn clean_tree_view(tree_view: &gtk4::TreeView) {
    let list_store = get_list_store(tree_view);
    list_store.clear();
}
