use gtk4::prelude::*;

use crate::gui_structs::gui_data::GuiData;

pub(crate) fn connect_button_select(gui_data: &GuiData) {
    let select_dialog = gui_data.select_dialog.clone();
    let common_tree_views = gui_data.main_notebook.common_tree_views.clone();

    gui_data.bottom_buttons.buttons_select.connect_clicked(move |_| {
        let sv = common_tree_views.get_current_subview();
        select_dialog.set_available_modes(sv.nb_object.available_modes);
        select_dialog.dialog.set_visible(true);
    });
}
