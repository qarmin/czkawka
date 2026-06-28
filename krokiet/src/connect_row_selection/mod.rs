pub(crate) mod checker;
mod clipboard;
mod context_menu;
mod opener;
mod selection;

pub(crate) use selection::{initialize_selection_struct, recalculate_small_selection_if_needed, reset_selection};

use crate::MainWindow;

pub(crate) fn connect_row_selections(app: &MainWindow) {
    initialize_selection_struct();

    selection::connect_select_all_rows(app); // CTRL + A
    selection::reverse_single_unique_item(app); // LMB
    selection::reverse_checked_on_selection(app); // Space
    selection::reverse_selection_on_specific_item(app); // CTRL + LMB
    selection::select_items_with_shift(app); // SHIFT + LMB
    opener::open_provided_item(app);
    opener::open_provided_parent_item(app);
    opener::connect_on_open_item(app);
    checker::change_number_of_checked_items(app);
    context_menu::connect_context_menu_actions(app);
}
