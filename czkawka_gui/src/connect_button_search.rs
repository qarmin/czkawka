use czkawka_core::*;

extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use czkawka_core::big_file::BigFile;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::similar_files::SimilarImages;
use czkawka_core::temporary::Temporary;
use czkawka_core::zeroed::ZeroedFiles;
use glib::Sender;
use gtk::prelude::*;
use std::thread;

pub fn connect_button_search(gui_data: &GuiData, sender: Sender<Message>) {
    let entry_info = gui_data.entry_info.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    let scrolled_window_included_directories = gui_data.scrolled_window_included_directories.clone();
    let scrolled_window_excluded_directories = gui_data.scrolled_window_excluded_directories.clone();
    let buttons_search_clone = gui_data.buttons_search.clone();
    let buttons_array = gui_data.buttons_array.clone();
    let check_button_recursive = gui_data.check_button_recursive.clone();
    let entry_excluded_items = gui_data.entry_excluded_items.clone();
    let entry_allowed_extensions = gui_data.entry_allowed_extensions.clone();
    let buttons_names = gui_data.buttons_names.clone();
    let radio_button_name = gui_data.radio_button_name.clone();
    let radio_button_size = gui_data.radio_button_size.clone();
    let radio_button_hashmb = gui_data.radio_button_hashmb.clone();
    let radio_button_hash = gui_data.radio_button_hash.clone();
    let entry_duplicate_minimal_size = gui_data.entry_duplicate_minimal_size.clone();
    // let sender = gui_data.sender.clone();
    let rx = gui_data.rx.clone();
    let entry_big_files_number = gui_data.entry_big_files_number.clone();
    let entry_similar_images_minimal_size = gui_data.entry_similar_images_minimal_size.clone();
    buttons_search_clone.connect_clicked(move |_| {
        let included_directories = get_string_from_list_store(&scrolled_window_included_directories);
        let excluded_directories = get_string_from_list_store(&scrolled_window_excluded_directories);
        let recursive_search = check_button_recursive.get_active();
        let excluded_items = entry_excluded_items.get_text().as_str().to_string();
        let allowed_extensions = entry_allowed_extensions.get_text().as_str().to_string();

        hide_all_buttons_except("stop", &buttons_array, &buttons_names);

        // Disable main notebook from any iteraction until search will end
        notebook_main.set_sensitive(false);

        entry_info.set_text("Searching data, it may take a while, please wait...");

        match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
            "notebook_main_duplicate_finder_label" => {
                let check_method;
                if radio_button_name.get_active() {
                    check_method = duplicate::CheckingMethod::Name;
                } else if radio_button_size.get_active() {
                    check_method = duplicate::CheckingMethod::Size;
                } else if radio_button_hashmb.get_active() {
                    check_method = duplicate::CheckingMethod::HashMB;
                } else if radio_button_hash.get_active() {
                    check_method = duplicate::CheckingMethod::Hash;
                } else {
                    panic!("No radio button is pressed");
                }
                let minimal_file_size = match entry_duplicate_minimal_size.get_text().as_str().parse::<u64>() {
                    Ok(t) => t,
                    Err(_) => 1024, // By default
                };
                let delete_method = duplicate::DeleteMethod::None;

                let sender = sender.clone();
                let receiver_stop = rx.clone();
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
                    df.set_delete_method(delete_method);
                    df.find_duplicates(Option::from(&receiver_stop)); //&rc_stop_signal.borrow().1);
                    let _ = sender.send(Message::Duplicates(df));
                });
            }
            "scrolled_window_main_empty_folder_finder" => {
                let sender = sender.clone();
                let receiver_stop = rx.clone();

                // Find empty folders
                thread::spawn(move || {
                    let mut ef = EmptyFolder::new();
                    ef.set_included_directory(included_directories);
                    ef.set_delete_folder(false);
                    ef.find_empty_folders(Option::from(&receiver_stop));
                    let _ = sender.send(Message::EmptyFolders(ef));
                });
            }
            "scrolled_window_main_empty_files_finder" => {
                let sender = sender.clone();
                let receiver_stop = rx.clone();

                // Find empty files
                thread::spawn(move || {
                    let mut vf = EmptyFiles::new();

                    vf.set_included_directory(included_directories);
                    vf.set_excluded_directory(excluded_directories);
                    vf.set_recursive_search(recursive_search);
                    vf.set_excluded_items(excluded_items);
                    vf.set_allowed_extensions(allowed_extensions);
                    vf.find_empty_files(Option::from(&receiver_stop));
                    let _ = sender.send(Message::EmptyFiles(vf));
                });
            }
            "scrolled_window_main_temporary_files_finder" => {
                let sender = sender.clone();
                let receiver_stop = rx.clone();

                // Find temporary files
                thread::spawn(move || {
                    let mut tf = Temporary::new();

                    tf.set_included_directory(included_directories);
                    tf.set_excluded_directory(excluded_directories);
                    tf.set_recursive_search(recursive_search);
                    tf.set_excluded_items(excluded_items);
                    tf.find_temporary_files(Option::from(&receiver_stop));
                    let _ = sender.send(Message::Temporary(tf));
                });
            }
            "notebook_big_main_file_finder" => {
                let numbers_of_files_to_check = match entry_big_files_number.get_text().as_str().parse::<usize>() {
                    Ok(t) => t,
                    Err(_) => 50, // By default
                };

                let sender = sender.clone();
                let receiver_stop = rx.clone();

                // Find big files
                thread::spawn(move || {
                    let mut bf = BigFile::new();

                    bf.set_included_directory(included_directories);
                    bf.set_excluded_directory(excluded_directories);
                    bf.set_recursive_search(recursive_search);
                    bf.set_excluded_items(excluded_items);
                    bf.set_number_of_files_to_check(numbers_of_files_to_check);
                    bf.find_big_files(Option::from(&receiver_stop));
                    let _ = sender.send(Message::BigFiles(bf));
                });
            }

            "notebook_main_similar_images_finder_label" => {
                let sender = sender.clone();
                let receiver_stop = rx.clone();

                let minimal_file_size = match entry_similar_images_minimal_size.get_text().as_str().parse::<u64>() {
                    Ok(t) => t,
                    Err(_) => 1024 * 16, // By default
                };

                // Find similar images
                thread::spawn(move || {
                    let mut sf = SimilarImages::new();

                    sf.set_included_directory(included_directories);
                    sf.set_excluded_directory(excluded_directories);
                    sf.set_recursive_search(recursive_search);
                    sf.set_excluded_items(excluded_items);
                    sf.set_minimal_file_size(minimal_file_size);
                    sf.find_similar_images(Option::from(&receiver_stop));
                    let _ = sender.send(Message::SimilarImages(sf));
                });
            }
            "notebook_main_zeroed_files_finder" => {
                let sender = sender.clone();
                let receiver_stop = rx.clone();

                // Find temporary files
                thread::spawn(move || {
                    let mut zf = ZeroedFiles::new();

                    zf.set_included_directory(included_directories);
                    zf.set_excluded_directory(excluded_directories);
                    zf.set_recursive_search(recursive_search);
                    zf.set_excluded_items(excluded_items);
                    zf.find_zeroed_files(Option::from(&receiver_stop));
                    let _ = sender.send(Message::ZeroedFiles(zf));
                });
            }
            e => panic!("Not existent {}", e),
        }
    });
}
