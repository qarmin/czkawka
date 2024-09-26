use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;

pub fn connect_notebook_tabs(gui_data: &GuiData) {
    let shared_buttons = gui_data.shared_buttons.clone();
    let buttons_array = gui_data.bottom_buttons.buttons_array.clone();
    let notebook_main_clone = gui_data.main_notebook.notebook_main.clone();
    let buttons_names = gui_data.bottom_buttons.buttons_names;

    notebook_main_clone.connect_switch_page(move |_, _, number| {
        let current_tab_in_main_notebook = to_notebook_main_enum(number);

        // Buttons
        set_buttons(
            &mut *shared_buttons.borrow_mut().get_mut(&current_tab_in_main_notebook).expect("Failed to get current tab"),
            &buttons_array,
            &buttons_names,
        );
    });
}
