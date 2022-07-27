use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crossbeam_channel::bounded;
use gdk4::gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::Builder;

use czkawka_core::bad_extensions::BadExtensions;
use czkawka_core::big_file::BigFile;
use czkawka_core::broken_files::BrokenFiles;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::SameMusic;
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::similar_videos::SimilarVideos;
use czkawka_core::temporary::Temporary;

use crate::flg;
use crate::gui_structs::gui_about::GuiAbout;
use crate::gui_structs::gui_bottom_buttons::GuiBottomButtons;
use crate::gui_structs::gui_compare_images::GuiCompareImages;
use crate::gui_structs::gui_header::GuiHeader;
use crate::gui_structs::gui_main_notebook::GuiMainNotebook;
use crate::gui_structs::gui_popovers::GuiPopovers;
use crate::gui_structs::gui_progress_dialog::GuiProgressDialog;
use crate::gui_structs::gui_settings::GuiSettings;
use crate::gui_structs::gui_upper_notebook::GuiUpperNotebook;
use crate::help_functions::BottomButtonsEnum;
use crate::notebook_enums::*;
use crate::taskbar_progress::TaskbarProgress;

pub const ICON_ABOUT: &[u8; 4458] = include_bytes!("../../icons/icon_about.png");
pub const CZK_ICON_ADD: &[u8; 677] = include_bytes!("../../icons/czk_add.svg");
pub const CZK_ICON_COMPARE: &[u8; 5700] = include_bytes!("../../icons/czk_compare.svg");
pub const CZK_ICON_DELETE: &[u8; 489] = include_bytes!("../../icons/czk_delete.svg");
pub const CZK_ICON_HARDLINK: &[u8; 17326] = include_bytes!("../../icons/czk_hardlink.svg");
pub const CZK_ICON_HIDE_DOWN: &[u8; 3057] = include_bytes!("../../icons/czk_hide_down.svg");
pub const CZK_ICON_HIDE_UP: &[u8; 3310] = include_bytes!("../../icons/czk_hide_up.svg");
pub const CZK_ICON_INFO: &[u8; 3325] = include_bytes!("../../icons/czk_info.svg");
pub const CZK_ICON_LEFT: &[u8; 245] = include_bytes!("../../icons/czk_left.svg");
pub const CZK_ICON_MANUAL_ADD: &[u8; 677] = include_bytes!("../../icons/czk_manual_add.svg");
pub const CZK_ICON_MOVE: &[u8; 2535] = include_bytes!("../../icons/czk_move.svg");
pub const CZK_ICON_RIGHT: &[u8; 278] = include_bytes!("../../icons/czk_right.svg");
pub const CZK_ICON_SAVE: &[u8; 462] = include_bytes!("../../icons/czk_save.svg");
pub const CZK_ICON_SEARCH: &[u8; 1517] = include_bytes!("../../icons/czk_search.svg");
pub const CZK_ICON_SELECT: &[u8; 370] = include_bytes!("../../icons/czk_select.svg");
pub const CZK_ICON_SETTINGS: &[u8; 11677] = include_bytes!("../../icons/czk_settings.svg");
pub const CZK_ICON_STOP: &[u8; 618] = include_bytes!("../../icons/czk_stop.svg");
pub const CZK_ICON_SYMLINK: &[u8; 2455] = include_bytes!("../../icons/czk_symlink.svg");
pub const CZK_ICON_TRASH: &[u8; 709] = include_bytes!("../../icons/czk_trash.svg");

#[derive(Clone)]
pub struct GuiData {
    // Glade builder
    pub glade_src: String,
    pub builder: Builder,

    // Windows
    pub window_main: gtk4::Window,

    pub main_notebook: GuiMainNotebook,
    pub upper_notebook: GuiUpperNotebook,
    pub popovers: GuiPopovers,
    pub bottom_buttons: GuiBottomButtons,
    pub progress_window: GuiProgressDialog,
    pub about: GuiAbout,
    pub settings: GuiSettings,
    pub header: GuiHeader,
    pub compare_images: GuiCompareImages,

    // Taskbar state
    pub taskbar_state: Rc<RefCell<TaskbarProgress>>,

    // Buttons state
    pub shared_buttons: Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,

    // State of search results
    pub shared_duplication_state: Rc<RefCell<DuplicateFinder>>,
    pub shared_empty_folders_state: Rc<RefCell<EmptyFolder>>,
    pub shared_empty_files_state: Rc<RefCell<EmptyFiles>>,
    pub shared_temporary_files_state: Rc<RefCell<Temporary>>,
    pub shared_big_files_state: Rc<RefCell<BigFile>>,
    pub shared_similar_images_state: Rc<RefCell<SimilarImages>>,
    pub shared_similar_videos_state: Rc<RefCell<SimilarVideos>>,
    pub shared_same_music_state: Rc<RefCell<SameMusic>>,
    pub shared_same_invalid_symlinks: Rc<RefCell<InvalidSymlinks>>,
    pub shared_broken_files_state: Rc<RefCell<BrokenFiles>>,
    pub shared_bad_extensions_state: Rc<RefCell<BadExtensions>>,

    pub preview_path: Rc<RefCell<String>>,

    //// Entry
    pub entry_info: gtk4::Entry,

    //// Bottom
    pub text_view_errors: gtk4::TextView,
    pub scrolled_window_errors: gtk4::ScrolledWindow,

    // Used for sending stop signal to thread
    pub stop_sender: crossbeam_channel::Sender<()>,
    pub stop_receiver: crossbeam_channel::Receiver<()>,
}

impl GuiData {
    pub fn new_with_application(application: &gtk4::Application) -> Self {
        //// Loading glade file content and build with it help UI
        let glade_src = include_str!("../../ui/main_window.ui").to_string();
        let builder = Builder::from_string(glade_src.as_str());

        //// Windows
        let window_main: gtk4::Window = builder.object("window_main").unwrap();
        window_main.set_title(Some(&flg!("window_main_title")));
        window_main.show();

        let pixbuf = Pixbuf::from_read(std::io::BufReader::new(&ICON_ABOUT[..])).unwrap();
        // window_main.set_icon(Some(&pixbuf)); // TODO

        window_main.set_application(Some(application));

        let main_notebook = GuiMainNotebook::create_from_builder(&builder);
        let upper_notebook = GuiUpperNotebook::create_from_builder(&builder);
        let popovers = GuiPopovers::create_from_builder();
        let bottom_buttons = GuiBottomButtons::create_from_builder(&builder, &popovers.popover_select);
        let progress_window = GuiProgressDialog::create_from_builder(&window_main);
        let about = GuiAbout::create_from_builder(&window_main, &pixbuf);
        let header = GuiHeader::create_from_builder(&builder);
        let settings = GuiSettings::create_from_builder(&window_main);
        let compare_images = GuiCompareImages::create_from_builder(&window_main);

        ////////////////////////////////////////////////////////////////////////////////////////////////

        // Taskbar state
        let taskbar_state = Rc::new(RefCell::new(TaskbarProgress::new()));

        // Buttons State - to remember existence of different buttons on pages
        let shared_buttons: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>::new()));

        // Show by default only search button
        for i in get_all_main_tabs().iter() {
            let mut temp_hashmap: HashMap<BottomButtonsEnum, bool> = Default::default();
            for button_name in bottom_buttons.buttons_names.iter() {
                if *button_name == BottomButtonsEnum::Search {
                    temp_hashmap.insert(*button_name, true);
                } else {
                    temp_hashmap.insert(*button_name, false);
                }
            }
            shared_buttons.borrow_mut().insert(i.clone(), temp_hashmap);
        }

        // State of search results

        let shared_duplication_state: Rc<RefCell<_>> = Rc::new(RefCell::new(DuplicateFinder::new()));
        let shared_empty_folders_state: Rc<RefCell<_>> = Rc::new(RefCell::new(EmptyFolder::new()));
        let shared_empty_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(EmptyFiles::new()));
        let shared_temporary_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(Temporary::new()));
        let shared_big_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(BigFile::new()));
        let shared_similar_images_state: Rc<RefCell<_>> = Rc::new(RefCell::new(SimilarImages::new()));
        let shared_similar_videos_state: Rc<RefCell<_>> = Rc::new(RefCell::new(SimilarVideos::new()));
        let shared_same_music_state: Rc<RefCell<_>> = Rc::new(RefCell::new(SameMusic::new()));
        let shared_same_invalid_symlinks: Rc<RefCell<_>> = Rc::new(RefCell::new(InvalidSymlinks::new()));
        let shared_broken_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(BrokenFiles::new()));
        let shared_bad_extensions_state: Rc<RefCell<_>> = Rc::new(RefCell::new(BadExtensions::new()));

        let preview_path: Rc<RefCell<_>> = Rc::new(RefCell::new("".to_string()));

        //// Entry
        let entry_info: gtk4::Entry = builder.object("entry_info").unwrap();

        //// Bottom
        let text_view_errors: gtk4::TextView = builder.object("text_view_errors").unwrap();
        let scrolled_window_errors: gtk4::ScrolledWindow = builder.object("scrolled_window_errors").unwrap();
        scrolled_window_errors.show(); // Not sure why needed, but without it text view errors sometimes hide itself

        // Used for sending stop signal to thread
        let (stop_sender, stop_receiver): (crossbeam_channel::Sender<()>, crossbeam_channel::Receiver<()>) = bounded(1);

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
            compare_images,
            taskbar_state,
            shared_buttons,
            shared_duplication_state,
            shared_empty_folders_state,
            shared_empty_files_state,
            shared_temporary_files_state,
            shared_big_files_state,
            shared_similar_images_state,
            shared_similar_videos_state,
            shared_same_music_state,
            shared_same_invalid_symlinks,
            shared_broken_files_state,
            shared_bad_extensions_state,
            preview_path,
            entry_info,
            text_view_errors,
            scrolled_window_errors,
            stop_sender,
            stop_receiver,
        }
    }

    pub fn update_language(&self) {
        self.window_main.set_title(Some(&flg!("window_main_title")));

        self.main_notebook.update_language();
        self.upper_notebook.update_language();
        self.popovers.update_language();
        self.bottom_buttons.update_language();
        self.progress_window.update_language();
        self.about.update_language();
        self.header.update_language();
        self.settings.update_language();
        self.compare_images.update_language();
    }
}
