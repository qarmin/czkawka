use gtk4::prelude::*;
use gtk4::{CellRendererText, CellRendererToggle, ListStore, TreeView, TreeViewColumn};

use crate::help_functions::*;

// When adding new column do not forget to update translations

pub fn create_tree_view_included_directories(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    create_default_column(tree_view, ColumnsIncludedDirectory::Path as i32, Some(None), None);
    create_default_selection_button_column(tree_view, ColumnsIncludedDirectory::ReferenceButton as i32, model, None);
}

pub fn create_tree_view_excluded_directories(tree_view: &TreeView) {
    tree_view.set_headers_visible(false);
    create_default_column(tree_view, ColumnsExcludedDirectory::Path as i32, Some(None), None);
}

pub fn create_tree_view_duplicates(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);
    let columns_colors = (ColumnsDuplicates::Color as i32, ColumnsDuplicates::TextColor as i32);
    let activatable_colors = (ColumnsDuplicates::ActivatableSelectButton as i32, ColumnsDuplicates::Color as i32);

    create_default_selection_button_column(tree_view, ColumnsDuplicates::SelectionButton as i32, model, Some(activatable_colors));

    create_default_column(tree_view, ColumnsDuplicates::Size as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsDuplicates::Name as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsDuplicates::Path as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsDuplicates::Modification as i32, None, Some(columns_colors));
}

pub fn create_tree_view_empty_folders(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(tree_view, ColumnsEmptyFolders::SelectionButton as i32, model, None);

    create_default_column(tree_view, ColumnsEmptyFolders::Name as i32, Some(None), None);
    create_default_column(tree_view, ColumnsEmptyFolders::Path as i32, Some(None), None);
    create_default_column(
        tree_view,
        ColumnsEmptyFolders::Modification as i32,
        Some(Some(ColumnsEmptyFolders::ModificationAsSecs as i32)),
        None,
    );
}

pub fn create_tree_view_big_files(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(tree_view, ColumnsBigFiles::SelectionButton as i32, model, None);

    create_default_column(tree_view, ColumnsBigFiles::Size as i32, Some(None), None);
    create_default_column(tree_view, ColumnsBigFiles::Name as i32, Some(None), None);
    create_default_column(tree_view, ColumnsBigFiles::Path as i32, Some(None), None);
    create_default_column(
        tree_view,
        ColumnsBigFiles::Modification as i32,
        Some(Some(ColumnsBigFiles::ModificationAsSecs as i32)),
        None,
    );
}

pub fn create_tree_view_temporary_files(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(tree_view, ColumnsTemporaryFiles::SelectionButton as i32, model, None);

    create_default_column(tree_view, ColumnsTemporaryFiles::Name as i32, Some(None), None);
    create_default_column(tree_view, ColumnsTemporaryFiles::Path as i32, Some(None), None);
    create_default_column(
        tree_view,
        ColumnsTemporaryFiles::Modification as i32,
        Some(Some(ColumnsTemporaryFiles::ModificationAsSecs as i32)),
        None,
    );
}

pub fn create_tree_view_empty_files(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(tree_view, ColumnsEmptyFiles::SelectionButton as i32, model, None);

    create_default_column(tree_view, ColumnsEmptyFiles::Name as i32, Some(None), None);
    create_default_column(tree_view, ColumnsEmptyFiles::Path as i32, Some(None), None);
    create_default_column(
        tree_view,
        ColumnsEmptyFiles::Modification as i32,
        Some(Some(ColumnsEmptyFiles::ModificationAsSecs as i32)),
        None,
    );
}

pub fn create_tree_view_similar_images(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);
    let columns_colors = (ColumnsSimilarImages::Color as i32, ColumnsSimilarImages::TextColor as i32);
    let activatable_colors = (ColumnsSimilarImages::ActivatableSelectButton as i32, ColumnsSimilarImages::Color as i32);

    create_default_selection_button_column(tree_view, ColumnsSimilarImages::SelectionButton as i32, model, Some(activatable_colors));

    create_default_column(tree_view, ColumnsSimilarImages::Similarity as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSimilarImages::Size as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSimilarImages::Dimensions as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSimilarImages::Name as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSimilarImages::Path as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSimilarImages::Modification as i32, None, Some(columns_colors));
}

pub fn create_tree_view_similar_videos(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);
    let columns_colors = (ColumnsSimilarVideos::Color as i32, ColumnsSimilarVideos::TextColor as i32);
    let activatable_colors = (ColumnsSimilarVideos::ActivatableSelectButton as i32, ColumnsSimilarVideos::Color as i32);

    create_default_selection_button_column(tree_view, ColumnsSimilarVideos::SelectionButton as i32, model, Some(activatable_colors));

    create_default_column(tree_view, ColumnsSimilarVideos::Size as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSimilarVideos::Name as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSimilarVideos::Path as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSimilarVideos::Modification as i32, None, Some(columns_colors));
}

pub fn create_tree_view_same_music(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);
    let columns_colors = (ColumnsSameMusic::Color as i32, ColumnsSameMusic::TextColor as i32);
    let activatable_colors = (ColumnsSameMusic::ActivatableSelectButton as i32, ColumnsSameMusic::Color as i32);

    create_default_selection_button_column(tree_view, ColumnsSameMusic::SelectionButton as i32, model, Some(activatable_colors));

    create_default_column(tree_view, ColumnsSameMusic::Size as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSameMusic::Name as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSameMusic::Title as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSameMusic::Artist as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSameMusic::Year as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSameMusic::Bitrate as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSameMusic::Length as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSameMusic::Genre as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSameMusic::Path as i32, None, Some(columns_colors));
    create_default_column(tree_view, ColumnsSameMusic::Modification as i32, None, Some(columns_colors));
}

pub fn create_tree_view_invalid_symlinks(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(tree_view, ColumnsInvalidSymlinks::SelectionButton as i32, model, None);

    create_default_column(tree_view, ColumnsInvalidSymlinks::Name as i32, Some(None), None);
    create_default_column(tree_view, ColumnsInvalidSymlinks::Path as i32, Some(None), None);
    create_default_column(tree_view, ColumnsInvalidSymlinks::DestinationPath as i32, Some(None), None);
    create_default_column(tree_view, ColumnsInvalidSymlinks::TypeOfError as i32, Some(None), None);
    create_default_column(
        tree_view,
        ColumnsInvalidSymlinks::Modification as i32,
        Some(Some(ColumnsInvalidSymlinks::ModificationAsSecs as i32)),
        None,
    );
}

pub fn create_tree_view_broken_files(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(tree_view, ColumnsBrokenFiles::SelectionButton as i32, model, None);

    create_default_column(tree_view, ColumnsBrokenFiles::Name as i32, Some(None), None);
    create_default_column(tree_view, ColumnsBrokenFiles::Path as i32, Some(None), None);
    create_default_column(tree_view, ColumnsBrokenFiles::ErrorType as i32, Some(None), None);
    create_default_column(
        tree_view,
        ColumnsBrokenFiles::Modification as i32,
        Some(Some(ColumnsBrokenFiles::ModificationAsSecs as i32)),
        None,
    );
}

pub fn create_tree_view_bad_extensions(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(tree_view, ColumnsBadExtensions::SelectionButton as i32, model, None);

    create_default_column(tree_view, ColumnsBadExtensions::Name as i32, Some(None), None);
    create_default_column(tree_view, ColumnsBadExtensions::Path as i32, Some(None), None);
    create_default_column(tree_view, ColumnsBadExtensions::CurrentExtension as i32, Some(None), None);
    create_default_column(tree_view, ColumnsBadExtensions::ValidExtensions as i32, Some(None), None);
}

fn create_default_selection_button_column(
    tree_view: &TreeView,
    column_id: i32,
    model: ListStore,
    activatable_color_columns: Option<(i32, i32)>,
) -> (CellRendererToggle, TreeViewColumn) {
    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).expect("Failed to get iter from tree_path");
        let mut fixed = model.get::<bool>(&iter, column_id);
        fixed = !fixed;
        model.set_value(&iter, column_id as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", column_id);
    if let Some(activatable_color_columns) = activatable_color_columns {
        column.add_attribute(&renderer, "activatable", activatable_color_columns.0);
        column.add_attribute(&renderer, "cell-background", activatable_color_columns.1);
    }
    tree_view.append_column(&column);
    (renderer, column)
}

#[allow(clippy::option_option)]
fn create_default_column(tree_view: &TreeView, column_id: i32, sort_column_id: Option<Option<i32>>, colors_columns_id: Option<(i32, i32)>) -> (CellRendererText, TreeViewColumn) {
    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", column_id);
    if let Some(sort_column_id) = sort_column_id {
        if let Some(sort_column_id) = sort_column_id {
            column.set_sort_column_id(sort_column_id);
        } else {
            column.set_sort_column_id(column_id);
        }
    }
    if let Some(colors_columns_id) = colors_columns_id {
        column.add_attribute(&renderer, "background", colors_columns_id.0);
        column.add_attribute(&renderer, "foreground", colors_columns_id.1);
    }
    tree_view.append_column(&column);
    (renderer, column)
}
