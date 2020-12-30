extern crate gtk;
use crossbeam_channel::unbounded;
use czkawka_core::big_file::BigFile;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::SameMusic;
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::temporary::Temporary;
use czkawka_core::zeroed::ZeroedFiles;
use gtk::prelude::*;
use gtk::{Builder, Button};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct GuiData {
    // Glade builder
    pub glade_src: String,
    pub builder: Builder,

    // Windows
    pub window_main: gtk::Window,

    // States
    pub main_notebooks_labels: [String; 9],
    pub upper_notebooks_labels: [String; 5],
    pub buttons_labels: [String; 5],

    // Buttons state
    pub shared_buttons: Rc<RefCell<HashMap<String, HashMap<String, bool>>>>,

    // Upper Notebook state
    pub shared_upper_notebooks: Rc<RefCell<HashMap<String, HashMap<String, bool>>>>,

    // State of search results
    pub shared_duplication_state: Rc<RefCell<DuplicateFinder>>,
    pub shared_empty_folders_state: Rc<RefCell<EmptyFolder>>,
    pub shared_empty_files_state: Rc<RefCell<EmptyFiles>>,
    pub shared_temporary_files_state: Rc<RefCell<Temporary>>,
    pub shared_big_files_state: Rc<RefCell<BigFile>>,
    pub shared_similar_images_state: Rc<RefCell<SimilarImages>>,
    pub shared_zeroed_files_state: Rc<RefCell<ZeroedFiles>>,
    pub shared_same_music_state: Rc<RefCell<SameMusic>>,
    pub shared_same_invalid_symlinks: Rc<RefCell<InvalidSymlinks>>,

    //// GUI Entry
    pub entry_similar_images_minimal_size: gtk::Entry,
    pub entry_duplicate_minimal_size: gtk::Entry,
    pub entry_allowed_extensions: gtk::Entry,
    pub entry_excluded_items: gtk::Entry,
    pub entry_big_files_number: gtk::Entry,
    pub entry_same_music_minimal_size: gtk::Entry,

    //// GUI Buttons
    pub buttons_search: gtk::Button,
    pub buttons_select: gtk::Button,
    pub buttons_delete: gtk::Button,
    pub buttons_save: gtk::Button,
    pub buttons_symlink: gtk::Button,
    pub buttons_show_errors: gtk::Button,
    pub buttons_names: [String; 5],
    pub buttons_array: [Button; 5],

    pub buttons_manual_add_directory: gtk::Button,
    pub buttons_add_included_directory: gtk::Button,
    pub buttons_remove_included_directory: gtk::Button,
    pub buttons_add_excluded_directory: gtk::Button,
    pub buttons_remove_excluded_directory: gtk::Button,

    // Buttons search popover buttons
    pub buttons_popover_select_all: gtk::Button,
    pub buttons_popover_unselect_all: gtk::Button,
    pub buttons_popover_reverse: gtk::Button,
    pub buttons_popover_select_all_except_oldest: gtk::Button,
    pub buttons_popover_select_all_except_newest: gtk::Button,
    pub buttons_popover_select_one_oldest: gtk::Button,
    pub buttons_popover_select_one_newest: gtk::Button,
    pub buttons_popover_select_custom: gtk::Button,
    pub buttons_popover_unselect_custom: gtk::Button,
    pub buttons_popover_select_all_images_except_biggest: gtk::Button,
    pub buttons_popover_select_all_images_except_smallest: gtk::Button,

    pub separator_select_image_size: gtk::Separator,
    pub separator_select_reverse: gtk::Separator,
    pub separator_select_date: gtk::Separator,
    pub separator_select_custom: gtk::Separator,

    pub buttons_popover_right_click_open_file: gtk::Button,
    pub buttons_popover_right_click_open_folder: gtk::Button,

    //// Popovers
    pub popover_select: gtk::Popover,
    pub popover_right_click: gtk::Popover,

    //// Check Buttons
    pub check_button_recursive: gtk::CheckButton,

    pub check_button_music_title: gtk::CheckButton,
    pub check_button_music_artist: gtk::CheckButton,
    pub check_button_music_album_title: gtk::CheckButton,
    pub check_button_music_album_artist: gtk::CheckButton,
    pub check_button_music_year: gtk::CheckButton,

    //// Radio Buttons
    // Duplicates
    pub radio_button_duplicates_name: gtk::RadioButton,
    pub radio_button_duplicates_size: gtk::RadioButton,
    pub radio_button_duplicates_hashmb: gtk::RadioButton,
    pub radio_button_duplicates_hash: gtk::RadioButton,

    pub radio_button_similar_images_minimal: gtk::RadioButton,
    pub radio_button_similar_images_very_small: gtk::RadioButton,
    pub radio_button_similar_images_small: gtk::RadioButton,
    pub radio_button_similar_images_medium: gtk::RadioButton,
    pub radio_button_similar_images_high: gtk::RadioButton,
    pub radio_button_similar_images_very_high: gtk::RadioButton,

    //// Notebooks
    pub notebook_main: gtk::Notebook,
    pub notebook_upper: gtk::Notebook,

    pub notebook_main_children_names: Vec<String>,
    pub notebook_upper_children_names: Vec<String>,

    //// Entry
    pub entry_info: gtk::Entry, // To show default

    //// Bottom
    pub text_view_errors: gtk::TextView,
    pub scrolled_window_errors: gtk::ScrolledWindow,

    //// Scrolled windows
    // Main notebook
    pub scrolled_window_duplicate_finder: gtk::ScrolledWindow,
    pub scrolled_window_main_empty_folder_finder: gtk::ScrolledWindow,
    pub scrolled_window_main_empty_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_main_temporary_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_big_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_similar_images_finder: gtk::ScrolledWindow,
    pub scrolled_window_zeroed_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_same_music_finder: gtk::ScrolledWindow,
    pub scrolled_window_invalid_symlinks: gtk::ScrolledWindow,

    // Upper notebook
    pub scrolled_window_included_directories: gtk::ScrolledWindow,
    pub scrolled_window_excluded_directories: gtk::ScrolledWindow,

    //// Dialog State - dialog with progress state, which allows to stop task
    pub dialog_progress: gtk::Dialog,

    pub progress_bar_current_stage: gtk::ProgressBar,
    pub progress_bar_all_stages: gtk::ProgressBar,

    pub label_stage: gtk::Label,

    pub grid_progress_stages: gtk::Grid,

    pub button_stop_in_dialog: gtk::Button,

    //// Similar Images
    pub image_preview_similar_images: gtk::Image,

    //// Settings
    pub check_button_settings_save_at_exit: gtk::CheckButton,
    pub check_button_settings_load_at_start: gtk::CheckButton,
    pub check_button_settings_confirm_deletion: gtk::CheckButton,
    pub check_button_settings_show_preview_similar_images: gtk::CheckButton,
    pub check_button_settings_show_text_view: gtk::CheckButton,

    pub button_settings_save_configuration: gtk::Button,
    pub button_settings_load_configuration: gtk::Button,
    pub button_settings_reset_configuration: gtk::Button,
    //// Threads

    // Used for sending stop signal to thread
    pub stop_sender: crossbeam_channel::Sender<()>,
    pub stop_receiver: crossbeam_channel::Receiver<()>,
}

impl GuiData {
    pub fn new() -> Self {
        //// Loading glade file content and build with it help UI
        let glade_src = include_str!("../czkawka.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        //// Windows
        let window_main: gtk::Window = builder.get_object("window_main").unwrap();
        window_main.show_all();
        window_main.set_title("Czkawka");

        ////////////////////////////////////////////////////////////////////////////////////////////////
        //// States
        let main_notebooks_labels = [
            "duplicate".to_string(),
            "empty_folder".to_string(),
            "empty_file".to_string(),
            "temporary_file".to_string(),
            "big_file".to_string(),
            "similar_images".to_string(),
            "zeroed_files".to_string(),
            "same_music".to_string(),
            "invalid_symlinks".to_string(),
        ];
        let upper_notebooks_labels = [
            "included_directories".to_string(),
            "excluded_directories".to_string(),
            "excluded_items".to_string(),
            "allowed_extensions".to_string(),
            "settings".to_string(),
        ];
        let buttons_labels = ["search".to_string(), "select".to_string(), "delete".to_string(), "save".to_string(), "symlink".to_string()];

        // Buttons State - to remember existence of different buttons on pages

        let shared_buttons: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::<String, HashMap<String, bool>>::new()));
        shared_buttons.borrow_mut().clear();

        // Show by default only search button
        for i in main_notebooks_labels.iter() {
            let mut temp_hashmap: HashMap<String, bool> = Default::default();
            for j in buttons_labels.iter() {
                if *j == "search" {
                    temp_hashmap.insert(j.to_string(), true);
                } else {
                    temp_hashmap.insert(j.to_string(), false);
                }
            }
            shared_buttons.borrow_mut().insert(i.to_string(), temp_hashmap);
        }

        // Upper Notebook state
        let shared_upper_notebooks: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::<String, HashMap<String, bool>>::new()));

        for i in main_notebooks_labels.iter() {
            let mut temp_hashmap: HashMap<String, bool> = Default::default();
            for j in upper_notebooks_labels.iter() {
                temp_hashmap.insert(j.to_string(), true);
            }
            shared_upper_notebooks.borrow_mut().insert(i.to_string(), temp_hashmap);
        }
        // Some upper notebook tabs are disabled
        *shared_upper_notebooks.borrow_mut().get_mut("temporary_file").unwrap().get_mut("allowed_extensions").unwrap() = false;

        // State of search results

        let shared_duplication_state: Rc<RefCell<_>> = Rc::new(RefCell::new(DuplicateFinder::new()));
        let shared_empty_folders_state: Rc<RefCell<_>> = Rc::new(RefCell::new(EmptyFolder::new()));
        let shared_empty_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(EmptyFiles::new()));
        let shared_temporary_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(Temporary::new()));
        let shared_big_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(BigFile::new()));
        let shared_similar_images_state: Rc<RefCell<_>> = Rc::new(RefCell::new(SimilarImages::new()));
        let shared_zeroed_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(ZeroedFiles::new()));
        let shared_same_music_state: Rc<RefCell<_>> = Rc::new(RefCell::new(SameMusic::new()));
        let shared_same_invalid_symlinks: Rc<RefCell<_>> = Rc::new(RefCell::new(InvalidSymlinks::new()));

        ////////////////////////////////////////////////////////////////////////////////////////////////

        //// GUI Entry
        let entry_similar_images_minimal_size: gtk::Entry = builder.get_object("entry_similar_images_minimal_size").unwrap();
        let entry_duplicate_minimal_size: gtk::Entry = builder.get_object("entry_duplicate_minimal_size").unwrap();
        let entry_allowed_extensions: gtk::Entry = builder.get_object("entry_allowed_extensions").unwrap();
        let entry_excluded_items: gtk::Entry = builder.get_object("entry_excluded_items").unwrap();
        let entry_big_files_number: gtk::Entry = builder.get_object("entry_big_files_number").unwrap();
        let entry_same_music_minimal_size: gtk::Entry = builder.get_object("entry_same_music_minimal_size").unwrap();

        //// GUI Buttons
        let buttons_search: gtk::Button = builder.get_object("buttons_search").unwrap();
        let buttons_select: gtk::Button = builder.get_object("buttons_select").unwrap();
        let buttons_delete: gtk::Button = builder.get_object("buttons_delete").unwrap();
        let buttons_save: gtk::Button = builder.get_object("buttons_save").unwrap();
        let buttons_symlink: gtk::Button = builder.get_object("buttons_symlink").unwrap();

        let buttons_show_errors: gtk::Button = builder.get_object("buttons_show_errors").unwrap();

        let buttons_names = ["search".to_string(), "select".to_string(), "delete".to_string(), "save".to_string(), "symlink".to_string()];
        let buttons_array = [buttons_search.clone(), buttons_select.clone(), buttons_delete.clone(), buttons_save.clone(), buttons_symlink.clone()];

        let buttons_manual_add_directory: gtk::Button = builder.get_object("buttons_manual_add_directory").unwrap();
        let buttons_add_included_directory: gtk::Button = builder.get_object("buttons_add_included_directory").unwrap();
        let buttons_remove_included_directory: gtk::Button = builder.get_object("buttons_remove_included_directory").unwrap();
        let buttons_add_excluded_directory: gtk::Button = builder.get_object("buttons_add_excluded_directory").unwrap();
        let buttons_remove_excluded_directory: gtk::Button = builder.get_object("buttons_remove_excluded_directory").unwrap();

        // Buttons search popover buttons
        let buttons_popover_select_all: gtk::Button = builder.get_object("buttons_popover_select_all").unwrap();
        let buttons_popover_unselect_all: gtk::Button = builder.get_object("buttons_popover_unselect_all").unwrap();
        let buttons_popover_reverse: gtk::Button = builder.get_object("buttons_popover_reverse").unwrap();
        let buttons_popover_select_all_except_oldest: gtk::Button = builder.get_object("buttons_popover_select_all_except_oldest").unwrap();
        let buttons_popover_select_all_except_newest: gtk::Button = builder.get_object("buttons_popover_select_all_except_newest").unwrap();
        let buttons_popover_select_one_oldest: gtk::Button = builder.get_object("buttons_popover_select_one_oldest").unwrap();
        let buttons_popover_select_one_newest: gtk::Button = builder.get_object("buttons_popover_select_one_newest").unwrap();
        let buttons_popover_select_custom: gtk::Button = builder.get_object("buttons_popover_select_custom").unwrap();
        let buttons_popover_unselect_custom: gtk::Button = builder.get_object("buttons_popover_unselect_custom").unwrap();
        let buttons_popover_select_all_images_except_biggest: gtk::Button = builder.get_object("buttons_popover_select_all_images_except_biggest").unwrap();
        let buttons_popover_select_all_images_except_smallest: gtk::Button = builder.get_object("buttons_popover_select_all_images_except_smallest").unwrap();

        let separator_select_image_size: gtk::Separator = builder.get_object("separator_select_image_size").unwrap();
        let separator_select_reverse: gtk::Separator = builder.get_object("separator_select_reverse").unwrap();
        let separator_select_date: gtk::Separator = builder.get_object("separator_select_date").unwrap();
        let separator_select_custom: gtk::Separator = builder.get_object("separator_select_custom").unwrap();

        let buttons_popover_right_click_open_file: gtk::Button = builder.get_object("buttons_popover_right_click_open_file").unwrap();
        let buttons_popover_right_click_open_folder: gtk::Button = builder.get_object("buttons_popover_right_click_open_folder").unwrap();

        //// Popovers
        let popover_select: gtk::Popover = builder.get_object("popover_select").unwrap();
        let popover_right_click: gtk::Popover = builder.get_object("popover_right_click").unwrap();

        //// Check Buttons
        let check_button_recursive: gtk::CheckButton = builder.get_object("check_button_recursive").unwrap();
        let check_button_music_title: gtk::CheckButton = builder.get_object("check_button_music_title").unwrap();
        let check_button_music_artist: gtk::CheckButton = builder.get_object("check_button_music_artist").unwrap();
        let check_button_music_album_title: gtk::CheckButton = builder.get_object("check_button_music_album_title").unwrap();
        let check_button_music_album_artist: gtk::CheckButton = builder.get_object("check_button_music_album_artist").unwrap();
        let check_button_music_year: gtk::CheckButton = builder.get_object("check_button_music_year").unwrap();

        //// Radio Buttons
        let radio_button_duplicates_name: gtk::RadioButton = builder.get_object("radio_button_duplicates_name").unwrap();
        let radio_button_duplicates_size: gtk::RadioButton = builder.get_object("radio_button_duplicates_size").unwrap();
        let radio_button_duplicates_hashmb: gtk::RadioButton = builder.get_object("radio_button_duplicates_hashmb").unwrap();
        let radio_button_duplicates_hash: gtk::RadioButton = builder.get_object("radio_button_duplicates_hash").unwrap();

        let radio_button_similar_images_minimal: gtk::RadioButton = builder.get_object("radio_button_similar_images_minimal").unwrap();
        let radio_button_similar_images_very_small: gtk::RadioButton = builder.get_object("radio_button_similar_images_very_small").unwrap();
        let radio_button_similar_images_small: gtk::RadioButton = builder.get_object("radio_button_similar_images_small").unwrap();
        let radio_button_similar_images_medium: gtk::RadioButton = builder.get_object("radio_button_similar_images_medium").unwrap();
        let radio_button_similar_images_high: gtk::RadioButton = builder.get_object("radio_button_similar_images_high").unwrap();
        let radio_button_similar_images_very_high: gtk::RadioButton = builder.get_object("radio_button_similar_images_very_high").unwrap();

        //// Notebooks
        let notebook_main: gtk::Notebook = builder.get_object("notebook_main").unwrap();
        let notebook_upper: gtk::Notebook = builder.get_object("notebook_upper").unwrap();

        let mut notebook_main_children_names: Vec<String> = Vec::new();
        let mut notebook_upper_children_names: Vec<String> = Vec::new();

        for i in notebook_main.get_children() {
            notebook_main_children_names.push(i.get_buildable_name().unwrap().to_string());
        }
        for i in notebook_upper.get_children() {
            notebook_upper_children_names.push(i.get_buildable_name().unwrap().to_string());
        }

        //// Entry
        let entry_info: gtk::Entry = builder.get_object("entry_info").unwrap();

        //// Bottom
        let text_view_errors: gtk::TextView = builder.get_object("text_view_errors").unwrap();
        let scrolled_window_errors: gtk::ScrolledWindow = builder.get_object("scrolled_window_errors").unwrap();

        //// Scrolled windows
        // Main notebook
        let scrolled_window_duplicate_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_duplicate_finder").unwrap();
        let scrolled_window_main_empty_folder_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_main_empty_folder_finder").unwrap();
        let scrolled_window_main_empty_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_main_empty_files_finder").unwrap();
        let scrolled_window_main_temporary_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_main_temporary_files_finder").unwrap();
        let scrolled_window_big_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_big_files_finder").unwrap();
        let scrolled_window_similar_images_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_similar_images_finder").unwrap();
        let scrolled_window_zeroed_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_zeroed_files_finder").unwrap();
        let scrolled_window_same_music_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_same_music_finder").unwrap();
        let scrolled_window_invalid_symlinks: gtk::ScrolledWindow = builder.get_object("scrolled_window_invalid_symlinks").unwrap();

        // Upper notebook
        let scrolled_window_included_directories: gtk::ScrolledWindow = builder.get_object("scrolled_window_included_directories").unwrap();
        let scrolled_window_excluded_directories: gtk::ScrolledWindow = builder.get_object("scrolled_window_excluded_directories").unwrap();

        //// Dialog State - dialog with progress state, which allows to stop task
        let dialog_progress: gtk::Dialog = builder.get_object("dialog_progress").unwrap();

        let progress_bar_current_stage: gtk::ProgressBar = builder.get_object("progress_bar_current_stage").unwrap();
        let progress_bar_all_stages: gtk::ProgressBar = builder.get_object("progress_bar_all_stages").unwrap();

        let label_stage: gtk::Label = builder.get_object("label_stage").unwrap();

        let grid_progress_stages: gtk::Grid = builder.get_object("grid_progress_stages").unwrap();

        let button_stop_in_dialog: gtk::Button = builder.get_object("button_stop_in_dialog").unwrap();

        //// Similar Images
        let image_preview_similar_images: gtk::Image = builder.get_object("image_preview_similar_images").unwrap();

        //// Settings
        let check_button_settings_save_at_exit: gtk::CheckButton = builder.get_object("check_button_settings_save_at_exit").unwrap();
        let check_button_settings_load_at_start: gtk::CheckButton = builder.get_object("check_button_settings_load_at_start").unwrap();
        let check_button_settings_confirm_deletion: gtk::CheckButton = builder.get_object("check_button_settings_confirm_deletion").unwrap();
        let check_button_settings_show_preview_similar_images: gtk::CheckButton = builder.get_object("check_button_settings_show_preview_similar_images").unwrap();
        let check_button_settings_show_text_view: gtk::CheckButton = builder.get_object("check_button_settings_show_text_view").unwrap();

        let button_settings_save_configuration: gtk::Button = builder.get_object("button_settings_save_configuration").unwrap();
        let button_settings_load_configuration: gtk::Button = builder.get_object("button_settings_load_configuration").unwrap();
        let button_settings_reset_configuration: gtk::Button = builder.get_object("button_settings_reset_configuration").unwrap();

        //// Threads
        // Types of messages to send to main thread where gui can be draw.

        // Used for sending stop signal to thread
        let (stop_sender, stop_receiver): (crossbeam_channel::Sender<()>, crossbeam_channel::Receiver<()>) = unbounded();

        Self {
            glade_src,
            builder,
            window_main,
            main_notebooks_labels,
            upper_notebooks_labels,
            buttons_labels,
            shared_buttons,
            shared_upper_notebooks,
            shared_duplication_state,
            shared_empty_folders_state,
            shared_empty_files_state,
            shared_temporary_files_state,
            shared_big_files_state,
            shared_similar_images_state,
            shared_zeroed_files_state,
            shared_same_music_state,
            shared_same_invalid_symlinks,
            entry_similar_images_minimal_size,
            entry_duplicate_minimal_size,
            entry_allowed_extensions,
            entry_excluded_items,
            entry_big_files_number,
            entry_same_music_minimal_size,
            buttons_search,
            buttons_select,
            buttons_delete,
            buttons_save,
            buttons_symlink,
            buttons_show_errors,
            buttons_names,
            buttons_array,
            buttons_manual_add_directory,
            buttons_add_included_directory,
            buttons_remove_included_directory,
            buttons_add_excluded_directory,
            buttons_remove_excluded_directory,
            buttons_popover_select_all,
            buttons_popover_unselect_all,
            buttons_popover_reverse,
            buttons_popover_select_all_except_oldest,
            buttons_popover_select_all_except_newest,
            buttons_popover_select_one_oldest,
            buttons_popover_select_one_newest,
            buttons_popover_select_custom,
            buttons_popover_unselect_custom,
            buttons_popover_select_all_images_except_biggest,
            buttons_popover_select_all_images_except_smallest,
            separator_select_image_size,
            separator_select_reverse,
            separator_select_date,
            separator_select_custom,
            buttons_popover_right_click_open_file,
            buttons_popover_right_click_open_folder,
            popover_select,
            popover_right_click,
            check_button_recursive,
            check_button_music_title,
            check_button_music_artist,
            check_button_music_album_title,
            check_button_music_album_artist,
            check_button_music_year,
            radio_button_duplicates_name,
            radio_button_duplicates_size,
            radio_button_duplicates_hashmb,
            radio_button_duplicates_hash,
            radio_button_similar_images_minimal,
            radio_button_similar_images_very_small,
            radio_button_similar_images_small,
            radio_button_similar_images_medium,
            radio_button_similar_images_high,
            radio_button_similar_images_very_high,
            notebook_main,
            notebook_upper,
            notebook_main_children_names,
            notebook_upper_children_names,
            entry_info,
            text_view_errors,
            scrolled_window_errors,
            scrolled_window_duplicate_finder,
            scrolled_window_main_empty_folder_finder,
            scrolled_window_main_empty_files_finder,
            scrolled_window_main_temporary_files_finder,
            scrolled_window_big_files_finder,
            scrolled_window_similar_images_finder,
            scrolled_window_zeroed_files_finder,
            scrolled_window_same_music_finder,
            scrolled_window_invalid_symlinks,
            scrolled_window_included_directories,
            scrolled_window_excluded_directories,
            dialog_progress,
            progress_bar_current_stage,
            progress_bar_all_stages,
            label_stage,
            grid_progress_stages,
            button_stop_in_dialog,
            image_preview_similar_images,
            check_button_settings_save_at_exit,
            check_button_settings_load_at_start,
            check_button_settings_confirm_deletion,
            check_button_settings_show_preview_similar_images,
            check_button_settings_show_text_view,
            button_settings_save_configuration,
            button_settings_load_configuration,
            button_settings_reset_configuration,
            stop_sender,
            stop_receiver,
        }
    }
}
