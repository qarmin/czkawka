use czkawka_core::*;

use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use czkawka_core::big_file::BigFile;
use czkawka_core::broken_files::BrokenFiles;
use czkawka_core::duplicate::{DuplicateFinder, HashType};
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::{MusicSimilarity, SameMusic};
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::temporary::Temporary;
use czkawka_core::zeroed::ZeroedFiles;
use glib::Sender;
use gtk::prelude::*;
use gtk::WindowPosition;
use img_hash::{FilterType, HashAlg};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use crate::taskbar_progress::tbp_flags::TBPF_NOPROGRESS;

#[allow(clippy::too_many_arguments)]
pub fn connect_button_search(
    gui_data: &GuiData,
    glib_stop_sender: Sender<Message>,
    futures_sender_duplicate_files: futures::channel::mpsc::UnboundedSender<duplicate::ProgressData>,
    futures_sender_empty_files: futures::channel::mpsc::UnboundedSender<empty_files::ProgressData>,
    futures_sender_empty_folder: futures::channel::mpsc::UnboundedSender<empty_folder::ProgressData>,
    futures_sender_big_file: futures::channel::mpsc::UnboundedSender<big_file::ProgressData>,
    futures_sender_same_music: futures::channel::mpsc::UnboundedSender<same_music::ProgressData>,
    futures_sender_similar_images: futures::channel::mpsc::UnboundedSender<similar_images::ProgressData>,
    futures_sender_temporary: futures::channel::mpsc::UnboundedSender<temporary::ProgressData>,
    futures_sender_zeroed: futures::channel::mpsc::UnboundedSender<zeroed::ProgressData>,
    futures_sender_invalid_symlinks: futures::channel::mpsc::UnboundedSender<invalid_symlinks::ProgressData>,
    futures_sender_broken_files: futures::channel::mpsc::UnboundedSender<broken_files::ProgressData>,
) {
    let entry_info = gui_data.entry_info.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let tree_view_included_directories = gui_data.upper_notebook.tree_view_included_directories.clone();
    let tree_view_excluded_directories = gui_data.upper_notebook.tree_view_excluded_directories.clone();
    let buttons_search_clone = gui_data.bottom_buttons.buttons_search.clone();
    let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
    let check_button_recursive = gui_data.upper_notebook.check_button_recursive.clone();
    let entry_excluded_items = gui_data.upper_notebook.entry_excluded_items.clone();
    let entry_same_music_minimal_size = gui_data.main_notebook.entry_same_music_minimal_size.clone();
    let entry_same_music_maximal_size = gui_data.main_notebook.entry_same_music_maximal_size.clone();
    let entry_allowed_extensions = gui_data.upper_notebook.entry_allowed_extensions.clone();
    let buttons_names = gui_data.bottom_buttons.buttons_names.clone();
    let radio_button_duplicates_name = gui_data.main_notebook.radio_button_duplicates_name.clone();
    let radio_button_duplicates_size = gui_data.main_notebook.radio_button_duplicates_size.clone();
    let radio_button_duplicates_hashmb = gui_data.main_notebook.radio_button_duplicates_hashmb.clone();
    let radio_button_duplicates_hash = gui_data.main_notebook.radio_button_duplicates_hash.clone();
    let scale_similarity = gui_data.main_notebook.scale_similarity.clone();
    let entry_duplicate_minimal_size = gui_data.main_notebook.entry_duplicate_minimal_size.clone();
    let entry_duplicate_maximal_size = gui_data.main_notebook.entry_duplicate_maximal_size.clone();
    let stop_receiver = gui_data.stop_receiver.clone();
    let entry_big_files_number = gui_data.main_notebook.entry_big_files_number.clone();
    let entry_similar_images_minimal_size = gui_data.main_notebook.entry_similar_images_minimal_size.clone();
    let entry_similar_images_maximal_size = gui_data.main_notebook.entry_similar_images_maximal_size.clone();
    let check_button_music_title: gtk::CheckButton = gui_data.main_notebook.check_button_music_title.clone();
    let check_button_music_artist: gtk::CheckButton = gui_data.main_notebook.check_button_music_artist.clone();
    let check_button_music_album_title: gtk::CheckButton = gui_data.main_notebook.check_button_music_album_title.clone();
    let check_button_music_album_artist: gtk::CheckButton = gui_data.main_notebook.check_button_music_album_artist.clone();
    let check_button_music_year: gtk::CheckButton = gui_data.main_notebook.check_button_music_year.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
    let tree_view_empty_folder_finder = gui_data.main_notebook.tree_view_empty_folder_finder.clone();
    let tree_view_empty_files_finder = gui_data.main_notebook.tree_view_empty_files_finder.clone();
    let tree_view_big_files_finder = gui_data.main_notebook.tree_view_big_files_finder.clone();
    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_temporary_files_finder = gui_data.main_notebook.tree_view_temporary_files_finder.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_zeroed_files_finder = gui_data.main_notebook.tree_view_zeroed_files_finder.clone();
    let tree_view_invalid_symlinks = gui_data.main_notebook.tree_view_invalid_symlinks.clone();
    let tree_view_broken_files = gui_data.main_notebook.tree_view_broken_files.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let window_progress = gui_data.progress_window.window_progress.clone();
    let label_stage = gui_data.progress_window.label_stage.clone();
    let grid_progress_stages = gui_data.progress_window.grid_progress_stages.clone();
    let progress_bar_current_stage = gui_data.progress_window.progress_bar_current_stage.clone();
    let progress_bar_all_stages = gui_data.progress_window.progress_bar_all_stages.clone();
    let taskbar_state = gui_data.taskbar_state.clone();
    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let radio_button_hash_type_blake3 = gui_data.main_notebook.radio_button_hash_type_blake3.clone();
    let radio_button_hash_type_crc32 = gui_data.main_notebook.radio_button_hash_type_crc32.clone();
    let radio_button_hash_type_xxh3 = gui_data.main_notebook.radio_button_hash_type_xxh3.clone();
    let check_button_settings_hide_hard_links = gui_data.settings.check_button_settings_hide_hard_links.clone();
    let check_button_settings_use_cache = gui_data.settings.check_button_settings_use_cache.clone();
    let entry_settings_cache_file_minimal_size = gui_data.settings.entry_settings_cache_file_minimal_size.clone();
    let radio_button_similar_hash_size_4 = gui_data.main_notebook.radio_button_similar_hash_size_4.clone();
    let radio_button_similar_hash_size_8 = gui_data.main_notebook.radio_button_similar_hash_size_8.clone();
    let radio_button_similar_hash_size_16 = gui_data.main_notebook.radio_button_similar_hash_size_16.clone();
    let radio_button_resize_algorithm_catmullrom = gui_data.main_notebook.radio_button_resize_algorithm_catmullrom.clone();
    let radio_button_resize_algorithm_lanczos3 = gui_data.main_notebook.radio_button_resize_algorithm_lanczos3.clone();
    let radio_button_resize_algorithm_nearest = gui_data.main_notebook.radio_button_resize_algorithm_nearest.clone();
    let radio_button_resize_algorithm_triangle = gui_data.main_notebook.radio_button_resize_algorithm_triangle.clone();
    let radio_button_resize_algorithm_gaussian = gui_data.main_notebook.radio_button_resize_algorithm_gaussian.clone();
    let radio_button_similar_hash_algorithm_gradient = gui_data.main_notebook.radio_button_similar_hash_algorithm_gradient.clone();
    let radio_button_similar_hash_algorithm_blockhash = gui_data.main_notebook.radio_button_similar_hash_algorithm_blockhash.clone();
    let radio_button_similar_hash_algorithm_mean = gui_data.main_notebook.radio_button_similar_hash_algorithm_mean.clone();
    let radio_button_similar_hash_algorithm_vertgradient = gui_data.main_notebook.radio_button_similar_hash_algorithm_vertgradient.clone();
    let radio_button_similar_hash_algorithm_doublegradient = gui_data.main_notebook.radio_button_similar_hash_algorithm_doublegradient.clone();

    buttons_search_clone.connect_clicked(move |_| {
        let included_directories = get_path_buf_from_vector_of_strings(get_string_from_list_store(&tree_view_included_directories));
        let excluded_directories = get_path_buf_from_vector_of_strings(get_string_from_list_store(&tree_view_excluded_directories));
        let recursive_search = check_button_recursive.is_active();
        let excluded_items = entry_excluded_items.text().as_str().to_string().split(',').map(|e| e.to_string()).collect::<Vec<String>>();
        let allowed_extensions = entry_allowed_extensions.text().as_str().to_string();
        let hide_hard_links = check_button_settings_hide_hard_links.is_active();
        let use_cache = check_button_settings_use_cache.is_active();
        let minimal_cache_file_size = entry_settings_cache_file_minimal_size.text().as_str().parse::<u64>().unwrap_or(2 * 1024 * 1024);

        let show_dialog = Arc::new(AtomicBool::new(true));

        hide_all_buttons(&buttons_array);

        // Disable main notebook from any iteration until search will end
        notebook_main.set_sensitive(false);

        entry_info.set_text("Searching data, it may take a while, please wait...");

        // Set dialog to center to current screen(it is impossible to center it to main window)
        window_progress.set_position(WindowPosition::Center);

        // Resets progress bars
        progress_bar_all_stages.set_fraction(0 as f64);
        progress_bar_current_stage.set_fraction(0 as f64);

        reset_text_view(&text_view_errors);

        let glib_stop_sender = glib_stop_sender.clone();
        let stop_receiver = stop_receiver.clone();

        match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
            NotebookMainEnum::Duplicate => {
                label_stage.show();
                grid_progress_stages.show_all();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_duplicate_finder).clear();

                let check_method;
                if radio_button_duplicates_name.is_active() {
                    check_method = duplicate::CheckingMethod::Name;
                } else if radio_button_duplicates_size.is_active() {
                    check_method = duplicate::CheckingMethod::Size;
                } else if radio_button_duplicates_hashmb.is_active() {
                    check_method = duplicate::CheckingMethod::HashMb;
                } else if radio_button_duplicates_hash.is_active() {
                    check_method = duplicate::CheckingMethod::Hash;
                } else {
                    panic!("No radio button is pressed");
                }
                let minimal_file_size = entry_duplicate_minimal_size.text().as_str().parse::<u64>().unwrap_or(1024 * 8);
                let maximal_file_size = entry_duplicate_maximal_size.text().as_str().parse::<u64>().unwrap_or(1024 * 1024 * 1024 * 1024);

                let hash_type: HashType;
                if radio_button_hash_type_blake3.is_active() {
                    hash_type = duplicate::HashType::Blake3;
                } else if radio_button_hash_type_crc32.is_active() {
                    hash_type = duplicate::HashType::Crc32;
                } else if radio_button_hash_type_xxh3.is_active() {
                    hash_type = duplicate::HashType::Xxh3;
                } else {
                    panic!("No radio button is pressed");
                }

                let futures_sender_duplicate_files = futures_sender_duplicate_files.clone();
                // Find duplicates
                thread::spawn(move || {
                    let mut df = DuplicateFinder::new();
                    df.set_included_directory(included_directories);
                    df.set_excluded_directory(excluded_directories);
                    df.set_recursive_search(recursive_search);
                    df.set_excluded_items(excluded_items);
                    df.set_allowed_extensions(allowed_extensions);
                    df.set_minimal_file_size(minimal_file_size);
                    df.set_maximal_file_size(maximal_file_size);
                    df.set_minimal_cache_file_size(minimal_cache_file_size);
                    df.set_check_method(check_method);
                    df.set_hash_type(hash_type);
                    df.set_ignore_hard_links(hide_hard_links);
                    df.set_use_cache(use_cache);
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

                let hash_size;
                if radio_button_similar_hash_size_4.is_active() {
                    hash_size = 4;
                } else if radio_button_similar_hash_size_8.is_active() {
                    hash_size = 8;
                } else if radio_button_similar_hash_size_16.is_active() {
                    hash_size = 16;
                } else {
                    panic!("No radio button is pressed");
                }

                let image_filter;
                if radio_button_resize_algorithm_catmullrom.is_active() {
                    image_filter = FilterType::CatmullRom;
                } else if radio_button_resize_algorithm_lanczos3.is_active() {
                    image_filter = FilterType::Lanczos3;
                } else if radio_button_resize_algorithm_nearest.is_active() {
                    image_filter = FilterType::Nearest;
                } else if radio_button_resize_algorithm_triangle.is_active() {
                    image_filter = FilterType::Triangle;
                } else if radio_button_resize_algorithm_gaussian.is_active() {
                    image_filter = FilterType::Gaussian;
                } else {
                    panic!("No radio button is pressed");
                }

                let hash_alg;
                if radio_button_similar_hash_algorithm_blockhash.is_active() {
                    hash_alg = HashAlg::Blockhash;
                } else if radio_button_similar_hash_algorithm_gradient.is_active() {
                    hash_alg = HashAlg::Gradient;
                } else if radio_button_similar_hash_algorithm_mean.is_active() {
                    hash_alg = HashAlg::Mean;
                } else if radio_button_similar_hash_algorithm_vertgradient.is_active() {
                    hash_alg = HashAlg::VertGradient;
                } else if radio_button_similar_hash_algorithm_doublegradient.is_active() {
                    hash_alg = HashAlg::DoubleGradient;
                } else {
                    panic!("No radio button is pressed");
                }

                let minimal_file_size = entry_similar_images_minimal_size.text().as_str().parse::<u64>().unwrap_or(1024 * 16);
                let maximal_file_size = entry_similar_images_maximal_size.text().as_str().parse::<u64>().unwrap_or(1024 * 1024 * 1024 * 1024);

                let similarity = similar_images::Similarity::Similar(scale_similarity.value() as u32);

                let futures_sender_similar_images = futures_sender_similar_images.clone();
                // Find similar images
                thread::spawn(move || {
                    let mut sf = SimilarImages::new();

                    sf.set_included_directory(included_directories);
                    sf.set_excluded_directory(excluded_directories);
                    sf.set_recursive_search(recursive_search);
                    sf.set_excluded_items(excluded_items);
                    sf.set_minimal_file_size(minimal_file_size);
                    sf.set_maximal_file_size(maximal_file_size);
                    sf.set_similarity(similarity);
                    sf.set_use_cache(use_cache);
                    sf.set_hash_alg(hash_alg);
                    sf.set_hash_size(hash_size);
                    sf.set_image_filter(image_filter);
                    sf.find_similar_images(Some(&stop_receiver), Some(&futures_sender_similar_images));
                    let _ = glib_stop_sender.send(Message::SimilarImages(sf));
                });
            }
            NotebookMainEnum::Zeroed => {
                label_stage.show();
                grid_progress_stages.show_all();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_zeroed_files_finder).clear();

                let futures_sender_zeroed = futures_sender_zeroed.clone();
                // Find zeroed files
                thread::spawn(move || {
                    let mut zf = ZeroedFiles::new();

                    zf.set_included_directory(included_directories);
                    zf.set_excluded_directory(excluded_directories);
                    zf.set_recursive_search(recursive_search);
                    zf.set_excluded_items(excluded_items);
                    zf.set_allowed_extensions(allowed_extensions);
                    zf.find_zeroed_files(Some(&stop_receiver), Some(&futures_sender_zeroed));
                    let _ = glib_stop_sender.send(Message::ZeroedFiles(zf));
                });
            }
            NotebookMainEnum::SameMusic => {
                label_stage.show();
                grid_progress_stages.show_all();
                window_progress.resize(1, 1);

                get_list_store(&tree_view_same_music_finder).clear();

                let minimal_file_size = entry_same_music_minimal_size.text().as_str().parse::<u64>().unwrap_or(1024 * 8);
                let maximal_file_size = entry_same_music_maximal_size.text().as_str().parse::<u64>().unwrap_or(1024 * 1024 * 1024 * 1024);

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
                        mf.set_excluded_items(excluded_items);
                        mf.set_minimal_file_size(minimal_file_size);
                        mf.set_maximal_file_size(maximal_file_size);
                        mf.set_recursive_search(recursive_search);
                        mf.set_music_similarity(music_similarity);
                        mf.find_same_music(Some(&stop_receiver), Some(&futures_sender_same_music));
                        let _ = glib_stop_sender.send(Message::SameMusic(mf));
                    });
                } else {
                    notebook_main.set_sensitive(true);
                    set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&NotebookMainEnum::SameMusic).unwrap(), &buttons_array, &buttons_names);
                    entry_info.set_text("ERROR: You must select at least one checkbox with music searching types.");
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
