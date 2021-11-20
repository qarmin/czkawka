use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use gtk::prelude::*;

pub fn connect_notebook_tabs(gui_data: &GuiData) {
    let shared_buttons = gui_data.shared_buttons.clone();
    let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
    let notebook_main_clone = gui_data.main_notebook.notebook_main.clone();
    let buttons_names = gui_data.bottom_buttons.buttons_names.clone();
    let shared_upper_notebooks = gui_data.shared_upper_notebooks.clone();
    let notebook_upper = gui_data.upper_notebook.notebook_upper.clone();

    notebook_main_clone.connect_switch_page(move |_, _, number| {
        let current_tab_in_main_notebook = to_notebook_main_enum(number);

        // Buttons
        set_buttons(&mut *shared_buttons.borrow_mut().get_mut(&current_tab_in_main_notebook).unwrap(), &buttons_array, &buttons_names);

        // Upper notebook
        {
            for (index, upper_tab) in get_all_upper_tabs().iter().enumerate() {
                if *shared_upper_notebooks.borrow_mut().get_mut(&current_tab_in_main_notebook).unwrap().get_mut(upper_tab).unwrap() {
                    notebook_upper.children().get(index).unwrap().show();
                } else {
                    notebook_upper.children().get(index).unwrap().hide();
                }
            }
        }
    });
}
