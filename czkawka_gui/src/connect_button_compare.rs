use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use gdk::ffi::gdk_window_edge_get_type;

use gtk::prelude::*;
use gtk::{Button, Entry};

use czkawka_core::common_traits::SaveResults;
use czkawka_core::fl;

use crate::gui_data::GuiData;
use crate::help_functions::{count_number_of_groups, NOTEBOOKS_INFOS};
use crate::localizer::generate_translation_hashmap;
use crate::notebook_enums::*;

pub fn connect_button_compare(gui_data: &GuiData) {

    let button_compare = gui_data.bottom_buttons.buttons_compare.clone();
    let window_compare = gui_data.compare_images.window_compare.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    button_compare.connect_clicked(move |_|{
        window_compare.show();

        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];


        count_number_of_groups(&tree_view);

    });

    let window_compare = gui_data.compare_images.window_compare.clone();
    window_compare.connect_delete_event(move |window_compare, _| {
        window_compare.hide();
        gtk::Inhibit(true)
    });

}