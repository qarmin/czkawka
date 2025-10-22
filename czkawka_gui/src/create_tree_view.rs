use gtk4::TreeView;
use gtk4::prelude::*;

use crate::gui_structs::common_tree_view::{ColumnSort, create_default_columns, create_default_selection_button_column};
use crate::help_functions::{ColumnsExcludedDirectory, ColumnsIncludedDirectory, get_list_store};

// When adding new column do not forget to update translations

pub(crate) fn create_tree_view_included_directories(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    create_default_columns(tree_view, &[(ColumnsIncludedDirectory::Path as i32, ColumnSort::Default)], None);

    create_default_selection_button_column(tree_view, ColumnsIncludedDirectory::ReferenceButton as i32, model, None);
}

pub(crate) fn create_tree_view_excluded_directories(tree_view: &TreeView) {
    tree_view.set_headers_visible(false);
    create_default_columns(tree_view, &[(ColumnsExcludedDirectory::Path as i32, ColumnSort::Default)], None);
}
