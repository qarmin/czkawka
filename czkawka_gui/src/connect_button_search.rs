use czkawka_core::*;

extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use czkawka_core::big_file::BigFile;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::same_music::{MusicSimilarity, SameMusic};
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::temporary::Temporary;
use czkawka_core::zeroed::ZeroedFiles;
use glib::Sender;
use gtk::prelude::*;
use std::thread;

#[allow(clippy::too_many_arguments)]
pub fn connect_button_search(
    gui_data: &GuiData,
    glib_stop_sender: Sender<Message>,
    futures_sender_duplicate_files: futures::channel::mpsc::Sender<duplicate::ProgressData>,
    futures_sender_empty_files: futures::channel::mpsc::Sender<empty_files::ProgressData>,
    futures_sender_empty_folder: futures::channel::mpsc::Sender<empty_folder::ProgressData>,
    futures_sender_big_file: futures::channel::mpsc::Sender<big_file::ProgressData>,
    futures_sender_same_music: futures::channel::mpsc::Sender<same_music::ProgressData>,
    futures_sender_similar_images: futures::channel::mpsc::Sender<similar_images::ProgressData>,
    futures_sender_temporary: futures::channel::mpsc::Sender<temporary::ProgressData>,
    futures_sender_zeroed: futures::channel::mpsc::Sender<zeroed::ProgressData>,
) {
    let entry_info = gui_data.entry_info.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let scrolled_window_included_directories = gui_data.scrolled_window_included_directories.clone();
    let scrolled_window_excluded_directories = gui_data.scrolled_window_excluded_directories.clone();
    let buttons_search_clone = gui_data.buttons_search.clone();
    let buttons_array = gui_data.buttons_array.clone();
    let check_button_recursive = gui_data.check_button_recursive.clone();
    let entry_excluded_items = gui_data.entry_excluded_items.clone();
    let entry_same_music_minimal_size = gui_data.entry_same_music_minimal_size.clone();
    let entry_allowed_extensions = gui_data.entry_allowed_extensions.clone();
    let buttons_names = gui_data.buttons_names.clone();
    let radio_button_duplicates_name = gui_data.radio_button_duplicates_name.clone();
    let radio_button_duplicates_size = gui_data.radio_button_duplicates_size.clone();
    let radio_button_duplicates_hashmb = gui_data.radio_button_duplicates_hashmb.clone();
    let radio_button_duplicates_hash = gui_data.radio_button_duplicates_hash.clone();
    let radio_button_similar_images_very_small = gui_data.radio_button_similar_images_very_small.clone();
    let radio_button_similar_images_small = gui_data.radio_button_similar_images_small.clone();
    let radio_button_similar_images_medium = gui_data.radio_button_similar_images_medium.clone();
    let radio_button_similar_images_high = gui_data.radio_button_similar_images_high.clone();
    let radio_button_similar_images_very_high = gui_data.radio_button_similar_images_very_high.clone();
    let entry_duplicate_minimal_size = gui_data.entry_duplicate_minimal_size.clone();
    let stop_receiver = gui_data.stop_receiver.clone();
    let entry_big_files_number = gui_data.entry_big_files_number.clone();
    let entry_similar_images_minimal_size = gui_data.entry_similar_images_minimal_size.clone();
    let check_button_music_title: gtk::CheckButton = gui_data.check_button_music_title.clone();
    let check_button_music_artist: gtk::CheckButton = gui_data.check_button_music_artist.clone();
    let check_button_music_album_title: gtk::CheckButton = gui_data.check_button_music_album_title.clone();
    let check_button_music_album_artist: gtk::CheckButton = gui_data.check_button_music_album_artist.clone();
    let check_button_music_year: gtk::CheckButton = gui_data.check_button_music_year.clone();
    let shared_buttons = gui_data.shared_buttons.clone();
    let scrolled_window_main_empty_folder_finder = gui_data.scrolled_window_main_empty_folder_finder.clone();
    let scrolled_window_main_empty_files_finder = gui_data.scrolled_window_main_empty_files_finder.clone();
    let scrolled_window_big_files_finder = gui_data.scrolled_window_big_files_finder.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let scrolled_window_main_temporary_files_finder = gui_data.scrolled_window_main_temporary_files_finder.clone();
    let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let scrolled_window_zeroed_files_finder = gui_data.scrolled_window_zeroed_files_finder.clone();
    let text_view_errors = gui_data.text_view_errors.clone();
    let dialog_progress = gui_data.dialog_progress.clone();
    let label_stage = gui_data.label_stage.clone();
    let grid_progress_stages = gui_data.grid_progress_stages.clone();
    let progress_bar_current_stage = gui_data.progress_bar_current_stage.clone();
    let progress_bar_all_stages = gui_data.progress_bar_all_stages.clone();

    buttons_search_clone.connect_clicked(move |_| {
        let included_directories = get_string_from_list_store(&scrolled_window_included_directories);
        let excluded_directories = get_string_from_list_store(&scrolled_window_excluded_directories);
        let recursive_search = check_button_recursive.get_active();
        let excluded_items = entry_excluded_items.get_text().as_str().to_string();
        let allowed_extensions = entry_allowed_extensions.get_text().as_str().to_string();

        hide_all_buttons(&buttons_array);

        // Disable main notebook from any iteration until search will end
        notebook_main.set_sensitive(false);

        entry_info.set_text("Searching data, it may take a while, please wait...");

        // Resets progress bars
        progress_bar_all_stages.set_fraction(0 as f64);
        progress_bar_current_stage.set_fraction(0 as f64);

        match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
            "notebook_main_duplicate_finder_label" => {
                label_stage.show();
                grid_progress_stages.show_all();
                dialog_progress.resize(1, 1);

                get_list_store(&scrolled_window_duplicate_finder).clear();
                text_view_errors.get_buffer().unwrap().set_text("");

                let check_method;
                if radio_button_duplicates_name.get_active() {
                    check_method = duplicate::CheckingMethod::Name;
                } else if radio_button_duplicates_size.get_active() {
                    check_method = duplicate::CheckingMethod::Size;
                } else if radio_button_duplicates_hashmb.get_active() {
                    check_method = duplicate::CheckingMethod::HashMB;
                } else if radio_button_duplicates_hash.get_active() {
                    check_method = duplicate::CheckingMethod::Hash;
                } else {
                    panic!("No radio button is pressed");
                }
                let minimal_file_size = match entry_duplicate_minimal_size.get_text().as_str().parse::<u64>() {
                    Ok(t) => t,
                    Err(_) => 1024, // By default
                };

                let glib_stop_sender = glib_stop_sender.clone();
                let stop_receiver = stop_receiver.clone();

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
                    df.set_check_method(check_method);
                    df.find_duplicates(Some(&stop_receiver), Some(&futures_sender_duplicate_files));
                    let _ = glib_stop_sender.send(Message::Duplicates(df));
                });
            }
            "scrolled_window_main_empty_files_finder" => {
                label_stage.show();
                grid_progress_stages.hide();
                dialog_progress.resize(1, 1);

                get_list_store(&scrolled_window_main_empty_files_finder).clear();
                text_view_errors.get_buffer().unwrap().set_text("");

                let glib_stop_sender = glib_stop_sender.clone();
                let stop_receiver = stop_receiver.clone();

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
            "scrolled_window_main_empty_folder_finder" => {
                label_stage.show();
                grid_progress_stages.hide();
                dialog_progress.resize(1, 1);

                get_list_store(&scrolled_window_main_empty_folder_finder).clear();
                text_view_errors.get_buffer().unwrap().set_text("");

                let glib_stop_sender = glib_stop_sender.clone();
                let stop_receiver = stop_receiver.clone();

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
            "notebook_big_main_file_finder" => {
                label_stage.show();
                grid_progress_stages.hide();
                dialog_progress.resize(1, 1);

                get_list_store(&scrolled_window_big_files_finder).clear();
                text_view_errors.get_buffer().unwrap().set_text("");

                let numbers_of_files_to_check = match entry_big_files_number.get_text().as_str().parse::<usize>() {
                    Ok(t) => t,
                    Err(_) => 50, // By default
                };

                let glib_stop_sender = glib_stop_sender.clone();
                let stop_receiver = stop_receiver.clone();
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
            "scrolled_window_main_temporary_files_finder" => {
                label_stage.show();
                grid_progress_stages.hide();
                dialog_progress.resize(1, 1);

                get_list_store(&scrolled_window_main_temporary_files_finder).clear();
                text_view_errors.get_buffer().unwrap().set_text("");

                let glib_stop_sender = glib_stop_sender.clone();
                let stop_receiver = stop_receiver.clone();

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
            "notebook_main_similar_images_finder_label" => {
                label_stage.show();
                grid_progress_stages.show_all();
                dialog_progress.resize(1, 1);

                get_list_store(&scrolled_window_similar_images_finder).clear();
                text_view_errors.get_buffer().unwrap().set_text("");

                let glib_stop_sender = glib_stop_sender.clone();
                let stop_receiver = stop_receiver.clone();

                let minimal_file_size = match entry_similar_images_minimal_size.get_text().as_str().parse::<u64>() {
                    Ok(t) => t,
                    Err(_) => 1024 * 16, // By default
                };

                let similarity;
                if radio_button_similar_images_very_small.get_active() {
                    similarity = similar_images::Similarity::VerySmall;
                } else if radio_button_similar_images_small.get_active() {
                    similarity = similar_images::Similarity::Small;
                } else if radio_button_similar_images_medium.get_active() {
                    similarity = similar_images::Similarity::Medium;
                } else if radio_button_similar_images_high.get_active() {
                    similarity = similar_images::Similarity::High;
                } else if radio_button_similar_images_very_high.get_active() {
                    similarity = similar_images::Similarity::VeryHigh;
                } else {
                    panic!("No radio button is pressed");
                }

                let futures_sender_similar_images = futures_sender_similar_images.clone();
                // Find similar images
                thread::spawn(move || {
                    let mut sf = SimilarImages::new();

                    sf.set_included_directory(included_directories);
                    sf.set_excluded_directory(excluded_directories);
                    sf.set_recursive_search(recursive_search);
                    sf.set_excluded_items(excluded_items);
                    sf.set_minimal_file_size(minimal_file_size);
                    sf.set_similarity(similarity);
                    sf.find_similar_images(Some(&stop_receiver), Some(&futures_sender_similar_images));
                    let _ = glib_stop_sender.send(Message::SimilarImages(sf));
                });
            }
            "notebook_main_zeroed_files_finder" => {
                label_stage.show();
                grid_progress_stages.show_all();
                dialog_progress.resize(1, 1);

                get_list_store(&scrolled_window_zeroed_files_finder).clear();
                text_view_errors.get_buffer().unwrap().set_text("");

                let glib_stop_sender = glib_stop_sender.clone();
                let stop_receiver = stop_receiver.clone();

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
            "notebook_main_same_music_finder" => {
                label_stage.show();
                grid_progress_stages.show_all();
                dialog_progress.resize(1, 1);

                get_list_store(&scrolled_window_same_music_finder).clear();
                text_view_errors.get_buffer().unwrap().set_text("");

                let minimal_file_size = match entry_same_music_minimal_size.get_text().as_str().parse::<u64>() {
                    Ok(t) => t,
                    Err(_) => 1024, // By default
                };
                let mut music_similarity: MusicSimilarity = MusicSimilarity::NONE;

                if check_button_music_title.get_active() {
                    music_similarity |= MusicSimilarity::TITLE;
                }
                if check_button_music_artist.get_active() {
                    music_similarity |= MusicSimilarity::ARTIST;
                }
                if check_button_music_album_title.get_active() {
                    music_similarity |= MusicSimilarity::ALBUM_TITLE;
                }
                if check_button_music_album_artist.get_active() {
                    music_similarity |= MusicSimilarity::ALBUM_ARTIST;
                }
                if check_button_music_year.get_active() {
                    music_similarity |= MusicSimilarity::YEAR;
                }

                if music_similarity != MusicSimilarity::NONE {
                    let glib_stop_sender = glib_stop_sender.clone();
                    let stop_receiver = stop_receiver.clone();

                    let futures_sender_same_music = futures_sender_same_music.clone();
                    // Find Similar music
                    thread::spawn(move || {
                        let mut mf = SameMusic::new();

                        mf.set_included_directory(included_directories);
                        mf.set_excluded_directory(excluded_directories);
                        mf.set_excluded_items(excluded_items);
                        mf.set_minimal_file_size(minimal_file_size);
                        mf.set_recursive_search(recursive_search);
                        mf.set_music_similarity(music_similarity);
                        mf.find_same_music(Some(&stop_receiver), Some(&futures_sender_same_music));
                        let _ = glib_stop_sender.send(Message::SameMusic(mf));
                    });
                } else {
                    notebook_main.set_sensitive(true);
                    set_buttons(&mut *shared_buttons.borrow_mut().get_mut("same_music").unwrap(), &buttons_array, &buttons_names);
                    entry_info.set_text("ERROR: You must select at least one checkbox with music searching types.");
                }
            }
            e => panic!("Not existent {}", e),
        }

        // Show progress dialog
        dialog_progress.show();
    });
}
