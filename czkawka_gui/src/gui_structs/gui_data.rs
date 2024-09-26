use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufReader;
use std::rc::Rc;

use crossbeam_channel::bounded;
use gdk4::gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::{Builder, FileChooserNative};

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
use crate::gui_structs::gui_popovers_select::GuiSelectPopovers;
use crate::gui_structs::gui_popovers_sort::GuiSortPopovers;
use crate::gui_structs::gui_progress_dialog::GuiProgressDialog;
use crate::gui_structs::gui_settings::GuiSettings;
use crate::gui_structs::gui_upper_notebook::GuiUpperNotebook;
use crate::help_functions::{BottomButtonsEnum, SharedState};
use crate::notebook_enums::*;
use crate::taskbar_progress::TaskbarProgress;

pub const ICON_ABOUT: &[u8] = include_bytes!("../../icons/icon_about.png");
pub const CZK_ICON_ADD: &[u8] = include_bytes!("../../icons/czk_add.svg");
pub const CZK_ICON_COMPARE: &[u8] = include_bytes!("../../icons/czk_compare.svg");
pub const CZK_ICON_DELETE: &[u8] = include_bytes!("../../icons/czk_delete.svg");
pub const CZK_ICON_HARDLINK: &[u8] = include_bytes!("../../icons/czk_hardlink.svg");
pub const CZK_ICON_HIDE_DOWN: &[u8] = include_bytes!("../../icons/czk_hide_down.svg");
pub const CZK_ICON_HIDE_UP: &[u8] = include_bytes!("../../icons/czk_hide_up.svg");
pub const CZK_ICON_INFO: &[u8] = include_bytes!("../../icons/czk_info.svg");
pub const CZK_ICON_LEFT: &[u8] = include_bytes!("../../icons/czk_left.svg");
pub const CZK_ICON_MANUAL_ADD: &[u8] = include_bytes!("../../icons/czk_manual_add.svg");
pub const CZK_ICON_MOVE: &[u8] = include_bytes!("../../icons/czk_move.svg");
pub const CZK_ICON_RIGHT: &[u8] = include_bytes!("../../icons/czk_right.svg");
pub const CZK_ICON_SAVE: &[u8] = include_bytes!("../../icons/czk_save.svg");
pub const CZK_ICON_SEARCH: &[u8] = include_bytes!("../../icons/czk_search.svg");
pub const CZK_ICON_SELECT: &[u8] = include_bytes!("../../icons/czk_select.svg");
pub const CZK_ICON_SETTINGS: &[u8] = include_bytes!("../../icons/czk_settings.svg");
pub const CZK_ICON_SORT: &[u8] = include_bytes!("../../icons/czk_sort.svg");
pub const CZK_ICON_STOP: &[u8] = include_bytes!("../../icons/czk_stop.svg");
pub const CZK_ICON_SYMLINK: &[u8] = include_bytes!("../../icons/czk_symlink.svg");
pub const CZK_ICON_TRASH: &[u8] = include_bytes!("../../icons/czk_trash.svg");

#[derive(Clone)]
pub struct GuiData {
    // Windows
    pub window_main: gtk4::Window,

    pub main_notebook: GuiMainNotebook,
    pub upper_notebook: GuiUpperNotebook,
    pub popovers_select: GuiSelectPopovers,
    pub popovers_sort: GuiSortPopovers,
    pub bottom_buttons: GuiBottomButtons,
    pub progress_window: GuiProgressDialog,
    pub about: GuiAbout,
    pub settings: GuiSettings,
    pub header: GuiHeader,
    pub compare_images: GuiCompareImages,

    pub file_dialog_include_exclude_folder_selection: FileChooserNative,
    pub file_dialog_move_to_folder: FileChooserNative,

    // Taskbar state
    pub taskbar_state: Rc<RefCell<TaskbarProgress>>,

    // Buttons state
    pub shared_buttons: Rc<RefCell<HashMap<NotebookMainEnum, HashMap<BottomButtonsEnum, bool>>>>,

    // State of search results
    pub shared_duplication_state: SharedState<DuplicateFinder>,
    pub shared_empty_folders_state: SharedState<EmptyFolder>,
    pub shared_empty_files_state: SharedState<EmptyFiles>,
    pub shared_temporary_files_state: SharedState<Temporary>,
    pub shared_big_files_state: SharedState<BigFile>,
    pub shared_similar_images_state: SharedState<SimilarImages>,
    pub shared_similar_videos_state: SharedState<SimilarVideos>,
    pub shared_same_music_state: SharedState<SameMusic>,
    pub shared_same_invalid_symlinks: SharedState<InvalidSymlinks>,
    pub shared_broken_files_state: SharedState<BrokenFiles>,
    pub shared_bad_extensions_state: SharedState<BadExtensions>,

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
        let window_main: gtk4::Window = builder.object("window_main").expect("Cambalache");
        window_main.set_title(Some(&flg!("window_main_title")));
        window_main.show();

        let pixbuf = Pixbuf::from_read(BufReader::new(ICON_ABOUT)).expect("Couldn't load icon");

        window_main.set_application(Some(application));

        let main_notebook = GuiMainNotebook::create_from_builder(&builder);
        let upper_notebook = GuiUpperNotebook::create_from_builder(&builder);
        let popovers_select = GuiSelectPopovers::create_from_builder();
        let popovers_sort = GuiSortPopovers::create_from_builder();
        let bottom_buttons = GuiBottomButtons::create_from_builder(&builder, &popovers_select.popover_select, &popovers_sort.popover_sort);
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
        for i in &get_all_main_tabs() {
            let mut temp_hashmap: HashMap<BottomButtonsEnum, bool> = Default::default();
            for button_name in &bottom_buttons.buttons_names {
                if *button_name == BottomButtonsEnum::Search {
                    temp_hashmap.insert(*button_name, true);
                } else {
                    temp_hashmap.insert(*button_name, false);
                }
            }
            shared_buttons.borrow_mut().insert(*i, temp_hashmap);
        }

        // File Dialogs - Native file dialogs must exists all the time in opposite to normal dialog

        let file_dialog_include_exclude_folder_selection = FileChooserNative::builder()
            .action(gtk4::FileChooserAction::SelectFolder)
            .transient_for(&window_main)
            .select_multiple(true)
            .modal(true)
            .build();
        let file_dialog_move_to_folder = FileChooserNative::builder()
            .title(flg!("move_files_title_dialog"))
            .action(gtk4::FileChooserAction::SelectFolder)
            .transient_for(&window_main)
            .select_multiple(false)
            .modal(true)
            .build();

        // State of search results

        let shared_duplication_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_empty_folders_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_empty_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_temporary_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_big_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_similar_images_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_similar_videos_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_same_music_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_same_invalid_symlinks: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_broken_files_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));
        let shared_bad_extensions_state: Rc<RefCell<_>> = Rc::new(RefCell::new(None));

        let preview_path: Rc<RefCell<_>> = Rc::new(RefCell::new(String::new()));

        //// Entry
        let entry_info: gtk4::Entry = builder.object("entry_info").expect("Cambalache");

        //// Bottom
        let text_view_errors: gtk4::TextView = builder.object("text_view_errors").expect("Cambalache");
        let scrolled_window_errors: gtk4::ScrolledWindow = builder.object("scrolled_window_errors").expect("Cambalache");
        scrolled_window_errors.show(); // Not sure why needed, but without it text view errors sometimes hide itself

        // Used for sending stop signal to thread
        let (stop_sender, stop_receiver): (crossbeam_channel::Sender<()>, crossbeam_channel::Receiver<()>) = bounded(1);

        Self {
            window_main,
            main_notebook,
            upper_notebook,
            popovers_select,
            popovers_sort,
            bottom_buttons,
            progress_window,
            about,
            settings,
            header,
            compare_images,
            file_dialog_include_exclude_folder_selection,
            file_dialog_move_to_folder,
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
        self.popovers_select.update_language();
        self.popovers_sort.update_language();
        self.bottom_buttons.update_language();
        self.progress_window.update_language();
        self.about.update_language();
        self.header.update_language();
        self.settings.update_language();
        self.compare_images.update_language();
    }
}
