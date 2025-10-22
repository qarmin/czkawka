use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufReader;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use gdk4::gdk_pixbuf::Pixbuf;
use gtk4::prelude::*;
use gtk4::{Builder, FileChooserNative};

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
use crate::help_functions::BottomButtonsEnum;
use crate::notebook_enums::{NotebookMainEnum, get_all_main_tabs};
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
pub const CZK_ICON_REPLACE: &[u8] = include_bytes!("../../icons/czk_replace.svg");

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

    //// Entry
    pub entry_info: gtk4::Entry,

    //// Bottom
    pub text_view_errors: gtk4::TextView,
    pub scrolled_window_errors: gtk4::ScrolledWindow,

    // Used for sending stop signal to thread
    pub stop_flag: Arc<AtomicBool>,
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

        let pixbuf = Pixbuf::from_read(BufReader::new(ICON_ABOUT))
            .unwrap_or(Pixbuf::new(gdk4::gdk_pixbuf::Colorspace::Rgb, false, 8, 1, 1).expect("Crash is a lot of less likely than loading png file"));

        window_main.set_application(Some(application));

        let upper_notebook = GuiUpperNotebook::create_from_builder(&builder);
        let popovers_select = GuiSelectPopovers::create_from_builder();
        let popovers_sort = GuiSortPopovers::create_from_builder();
        let bottom_buttons = GuiBottomButtons::create_from_builder(&builder, &popovers_select.popover_select, &popovers_sort.popover_sort);
        let progress_window = GuiProgressDialog::create_from_builder(&window_main);
        let about = GuiAbout::create_from_builder(&window_main, &pixbuf);
        let header = GuiHeader::create_from_builder(&builder);
        let settings = GuiSettings::create_from_builder(&window_main);
        let compare_images = GuiCompareImages::create_from_builder(&window_main);
        let main_notebook = GuiMainNotebook::create_from_builder(&builder, &settings);

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

        // File Dialogs - Native file dialogs must exist all the time in opposite to normal dialog

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

        //// Entry
        let entry_info: gtk4::Entry = builder.object("entry_info").expect("Cambalache");

        //// Bottom
        let text_view_errors: gtk4::TextView = builder.object("text_view_errors").expect("Cambalache");
        let scrolled_window_errors: gtk4::ScrolledWindow = builder.object("scrolled_window_errors").expect("Cambalache");
        scrolled_window_errors.show(); // Not sure why needed, but without it text view errors sometimes hide itself

        // Used for sending stop signal to thread
        let stop_flag = Arc::default();

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
            entry_info,
            text_view_errors,
            scrolled_window_errors,
            stop_flag,
        }
    }

    pub(crate) fn setup(&self) {
        self.main_notebook.setup(self);
    }

    pub(crate) fn update_language(&self) {
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
