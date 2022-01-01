use gtk::prelude::*;

use crate::gui_data::GuiData;
use crate::gui_popovers::GuiPopovers;
// use crate::help_functions::{PopoverTypes, NOTEBOOKS_INFOS};
use crate::notebook_enums::*;

pub fn connect_button_sort(gui_data: &GuiData) {
    let popovers = gui_data.popovers.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let popover_sort = gui_data.popovers.popover_sort.clone();
    let buttons_sort = gui_data.bottom_buttons.buttons_sort.clone();

    buttons_sort.connect_clicked(move |_| {
        show_required_popovers(&popovers, &to_notebook_main_enum(notebook_main.current_page().unwrap()));
        popover_sort.popup();
    });
}

fn show_required_popovers(popovers: &GuiPopovers, _current_mode: &NotebookMainEnum) {
    let buttons_popover_sort_portion_ascending = popovers.buttons_popover_sort_portion_ascending.clone();
    let buttons_popover_sort_portion_descending = popovers.buttons_popover_sort_portion_descending.clone();

    buttons_popover_sort_portion_ascending.show();
    buttons_popover_sort_portion_descending.show();
}
