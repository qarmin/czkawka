use crate::gui_about::GuiAbout;
use crate::gui_bottom_buttons::GuiBottomButtons;
use crate::gui_header::GuiHeader;
use crate::gui_main_notebook::GuiMainNotebook;
use crate::gui_popovers::GuiPopovers;
use crate::gui_progress_dialog::GuiProgressDialog;
use crate::gui_settings::GuiSettings;
use crate::gui_upper_notepad::GuiUpperNotebook;
use crate::notebook_enums::*;
use crate::taskbar_progress::TaskbarProgress;
use crossbeam_channel::unbounded;
use czkawka_core::big_file::BigFile;
use czkawka_core::broken_files::BrokenFiles;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::SameMusic;
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::temporary::Temporary;
use czkawka_core::zeroed::ZeroedFiles;
use gtk::prelude::*;
use gtk::{Builder, WindowPosition};
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

    pub main_notebook: GuiMainNotebook,
    pub upper_notebook: GuiUpperNotebook,
    pub popovers: GuiPopovers,
    pub bottom_buttons: GuiBottomButtons,
    pub progress_window: GuiProgressDialog,
    pub about: GuiAbout,
    pub settings: GuiSettings,
    pub header: GuiHeader,

    // Taskbar state
    pub taskbar_state: Rc<RefCell<TaskbarProgress>>,

    // Buttons state
    pub shared_buttons: Rc<RefCell<HashMap<NotebookMainEnum, HashMap<String, bool>>>>,

    // Upper Notebook state
    pub shared_upper_notebooks: Rc<RefCell<HashMap<NotebookMainEnum, HashMap<NotebookUpperEnum, bool>>>>,

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
    pub shared_broken_files_state: Rc<RefCell<BrokenFiles>>,

    //// Entry
    pub entry_info: gtk::Entry,

    //// Bottom
    pub text_view_errors: gtk::TextView,
    pub scrolled_window_errors: gtk::ScrolledWindow,

    // Used for sending stop signal to thread
    pub stop_sender: crossbeam_channel::Sender<()>,
    pub stop_receiver: crossbeam_channel::Receiver<()>,
}

impl GuiData {
    pub fn new() -> Self {
        //// Loading glade file content and build with it help UI
        let glade_src = include_str!("../ui/main_window.glade").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        //// Windows
        let window_main: gtk::Window = builder.object("window_main").unwrap();
        window_main.show_all();
        window_main.set_title("Czkawka");
        window_main.set_position(WindowPosition::Center);

        let main_notebook = GuiMainNotebook::create_from_builder(&builder);
        let upper_notebook = GuiUpperNotebook::create_from_builder(&builder);
        let popovers = GuiPopovers::create_from_builder();
        let bottom_buttons = GuiBottomButtons::create_from_builder(&builder);
        let progress_window = GuiProgressDialog::create_from_builder();
        let about = GuiAbout::create_from_builder();
        let header = GuiHeader::create_from_builder(&builder);
        let settings = GuiSettings::create_from_builder();

        ////////////////////////////////////////////////////////////////////////////////////////////////

        // Taskbar state
        let taskbar_state = Rc::new(RefCell::new(TaskbarProgress::new()));

        // Buttons State - to remember existence of different buttons on pages
        let shared_buttons: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::<NotebookMainEnum, HashMap<String, bool>>::new()));

        // Show by default only search button
        for i in get_all_main_tabs().iter() {
            let mut temp_hashmap: HashMap<String, bool> = Default::default();
            for button_name in bottom_buttons.buttons_names.iter() {
                if *button_name == "search" {
                    temp_hashmap.insert(button_name.to_string(), true);
                } else {
                    temp_hashmap.insert(button_name.to_string(), false);
                }
            }
            shared_buttons.borrow_mut().insert(i.clone(), temp_hashmap);
        }

        // Upper Notebook state
        let shared_upper_notebooks: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::<NotebookMainEnum, HashMap<NotebookUpperEnum, bool>>::new()));

        for i in get_all_main_tabs().iter() {
            let mut temp_hashmap: HashMap<NotebookUpperEnum, bool> = Default::default();
            for j in get_all_upper_tabs().iter() {
                temp_hashmap.insert(j.clone(), true);
            }
            shared_upper_notebooks.borrow_mut().insert(i.clone(), temp_hashmap);
        }
        // Some upper notebook tabs are disabled
        *shared_upper_notebooks.borrow_mut().get_mut(&NotebookMainEnum::Temporary).unwrap().get_mut(&NotebookUpperEnum::AllowedExtensions).unwrap() = false;

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
        let shared_broken_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(BrokenFiles::new()));

        //// Entry
        let entry_info: gtk::Entry = builder.object("entry_info").unwrap();

        //// Bottom
        let text_view_errors: gtk::TextView = builder.object("text_view_errors").unwrap();
        let scrolled_window_errors: gtk::ScrolledWindow = builder.object("scrolled_window_errors").unwrap();
        scrolled_window_errors.show_all(); // Not sure why needed, but without it text view errors sometimes hide itself

        // Used for sending stop signal to thread
        let (stop_sender, stop_receiver): (crossbeam_channel::Sender<()>, crossbeam_channel::Receiver<()>) = unbounded();

        Self {
            glade_src,
            builder,
            window_main,
            main_notebook,
            upper_notebook,
            popovers,
            bottom_buttons,
            progress_window,
            about,
            settings,
            header,
            taskbar_state,
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
            shared_broken_files_state,
            entry_info,
            text_view_errors,
            scrolled_window_errors,
            stop_sender,
            stop_receiver,
        }
    }
}
