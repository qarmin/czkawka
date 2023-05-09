use gtk4::prelude::*;
use gtk4::subclass::tree_view;
use gtk4::{CellRendererText, CellRendererToggle, ListStore, TreeView, TreeViewColumn};

use crate::help_functions::*;

// When adding new column do not forget to update translations

pub fn create_tree_view_included_directories(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.set_title("Folders to check");
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", ColumnsIncludedDirectory::Path as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model.get::<bool>(&iter, ColumnsIncludedDirectory::ReferenceButton as i32);
        fixed = !fixed;
        model.set_value(&iter, ColumnsIncludedDirectory::ReferenceButton as u32, &fixed.to_value());
    });
    renderer.set_activatable(true);
    let column = TreeViewColumn::new();
    column.set_title("Reference folder");
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "active", ColumnsIncludedDirectory::ReferenceButton as i32);
    tree_view.append_column(&column);
}

pub fn create_tree_view_excluded_directories(tree_view: &TreeView) {
    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", ColumnsExcludedDirectory::Path as i32);
    tree_view.append_column(&column);

    tree_view.set_headers_visible(false);
}

pub fn create_tree_view_duplicates(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model.get::<bool>(&iter, ColumnsDuplicates::SelectionButton as i32);
        fixed = !fixed;
        model.set_value(&iter, ColumnsDuplicates::SelectionButton as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsDuplicates::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsDuplicates::SelectionButton as i32);
    column.add_attribute(&renderer, "cell-background", ColumnsDuplicates::Color as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsDuplicates::Size as i32);
    column.add_attribute(&renderer, "background", ColumnsDuplicates::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsDuplicates::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsDuplicates::Name as i32);
    column.add_attribute(&renderer, "background", ColumnsDuplicates::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsDuplicates::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsDuplicates::Path as i32);
    column.add_attribute(&renderer, "background", ColumnsDuplicates::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsDuplicates::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsDuplicates::Modification as i32);
    column.add_attribute(&renderer, "background", ColumnsDuplicates::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsDuplicates::TextColor as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_empty_folders(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model.get::<bool>(&iter, ColumnsEmptyFolders::SelectionButton as i32);
        fixed = !fixed;
        model.set_value(&iter, ColumnsEmptyFolders::SelectionButton as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsEmptyFolders::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Folder Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Name as i32);
    column.set_sort_column_id(ColumnsEmptyFolders::Name as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Path as i32);
    column.set_sort_column_id(ColumnsEmptyFolders::Path as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Modification as i32);
    column.set_sort_column_id(ColumnsEmptyFolders::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_big_files(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model.get::<bool>(&iter, ColumnsBigFiles::SelectionButton as i32);
        fixed = !fixed;
        model.set_value(&iter, ColumnsBigFiles::SelectionButton as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsBigFiles::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Size as i32);
    column.set_sort_column_id(ColumnsBigFiles::SizeAsBytes as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Name as i32);
    column.set_sort_column_id(ColumnsBigFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Path as i32);
    column.set_sort_column_id(ColumnsBigFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Modification as i32);
    column.set_sort_column_id(ColumnsBigFiles::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_temporary_files(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model.get::<bool>(&iter, ColumnsTemporaryFiles::SelectionButton as i32);
        fixed = !fixed;
        model.set_value(&iter, ColumnsTemporaryFiles::SelectionButton as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsTemporaryFiles::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Name as i32);
    column.set_sort_column_id(ColumnsTemporaryFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Path as i32);
    column.set_sort_column_id(ColumnsTemporaryFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Modification as i32);
    column.set_sort_column_id(ColumnsTemporaryFiles::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_empty_files(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model.get::<bool>(&iter, ColumnsEmptyFiles::SelectionButton as i32);
        fixed = !fixed;
        model.set_value(&iter, ColumnsEmptyFiles::SelectionButton as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsEmptyFiles::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Name as i32);
    column.set_sort_column_id(ColumnsEmptyFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Path as i32);
    column.set_sort_column_id(ColumnsEmptyFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Modification as i32);
    column.set_sort_column_id(ColumnsEmptyFiles::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_similar_images(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model.get::<bool>(&iter, ColumnsSimilarImages::SelectionButton as i32);
        fixed = !fixed;
        model.set_value(&iter, ColumnsSimilarImages::SelectionButton as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsSimilarImages::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsSimilarImages::SelectionButton as i32);
    column.add_attribute(&renderer, "cell-background", ColumnsSimilarImages::Color as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Similarity");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Similarity as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Size as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Dimensions");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Dimensions as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Name as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Path as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Modification as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_similar_videos(tree_view: &TreeView) {
    let model = get_list_store(tree_view);

    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model.get::<bool>(&iter, ColumnsSimilarVideos::SelectionButton as i32);
        fixed = !fixed;
        model.set_value(&iter, ColumnsSimilarVideos::SelectionButton as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsSimilarVideos::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsSimilarVideos::SelectionButton as i32);
    column.add_attribute(&renderer, "cell-background", ColumnsSimilarVideos::Color as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarVideos::Size as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarVideos::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarVideos::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarVideos::Name as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarVideos::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarVideos::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarVideos::Path as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarVideos::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarVideos::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarVideos::Modification as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarVideos::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarVideos::TextColor as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_same_music(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);
    let columns_colors = (ColumnsSameMusic::Color as i32, ColumnsSameMusic::TextColor as i32);
    let activatable_colors = (ColumnsSameMusic::ActivatableSelectButton as i32, ColumnsSameMusic::Color as i32);

    create_default_selection_button_column(&tree_view, ColumnsSameMusic::SelectionButton as i32, model, Some(activatable_colors));

    create_default_column(&tree_view, ColumnsSameMusic::Size as i32, None, Some(columns_colors));
    create_default_column(&tree_view, ColumnsSameMusic::Name as i32, None, Some(columns_colors));
    create_default_column(&tree_view, ColumnsSameMusic::Title as i32, None, Some(columns_colors));
    create_default_column(&tree_view, ColumnsSameMusic::Artist as i32, None, Some(columns_colors));
    create_default_column(&tree_view, ColumnsSameMusic::Year as i32, None, Some(columns_colors));
    create_default_column(&tree_view, ColumnsSameMusic::Bitrate as i32, None, Some(columns_colors));
    create_default_column(&tree_view, ColumnsSameMusic::Length as i32, None, Some(columns_colors));
    create_default_column(&tree_view, ColumnsSameMusic::Genre as i32, None, Some(columns_colors));
    create_default_column(&tree_view, ColumnsSameMusic::Path as i32, None, Some(columns_colors));
    create_default_column(&tree_view, ColumnsSameMusic::Modification as i32, None, Some(columns_colors));
}

pub fn create_tree_view_invalid_symlinks(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(&tree_view, ColumnsInvalidSymlinks::SelectionButton as i32, model, None);

    create_default_column(&tree_view, ColumnsInvalidSymlinks::Name as i32, None, None);
    create_default_column(&tree_view, ColumnsInvalidSymlinks::Path as i32, None, None);
    create_default_column(&tree_view, ColumnsInvalidSymlinks::DestinationPath as i32, None, None);
    create_default_column(&tree_view, ColumnsInvalidSymlinks::TypeOfError as i32, None, None);
    create_default_column(
        &tree_view,
        ColumnsInvalidSymlinks::Modification as i32,
        Some(ColumnsInvalidSymlinks::ModificationAsSecs as i32),
        None,
    );
}

pub fn create_tree_view_broken_files(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(&tree_view, ColumnsBrokenFiles::SelectionButton as i32, model, None);

    create_default_column(&tree_view, ColumnsBrokenFiles::Name as i32, None, None);
    create_default_column(&tree_view, ColumnsBrokenFiles::Path as i32, None, None);
    create_default_column(&tree_view, ColumnsBrokenFiles::ErrorType as i32, None, None);
    create_default_column(
        &tree_view,
        ColumnsBrokenFiles::Modification as i32,
        Some(ColumnsBrokenFiles::ModificationAsSecs as i32),
        None,
    );
}

pub fn create_tree_view_bad_extensions(tree_view: &TreeView) {
    tree_view.set_vexpand(true);

    let model = get_list_store(tree_view);

    create_default_selection_button_column(&tree_view, ColumnsBadExtensions::SelectionButton as i32, model, None);

    create_default_column(&tree_view, ColumnsBadExtensions::Name as i32, None, None);
    create_default_column(&tree_view, ColumnsBadExtensions::Path as i32, None, None);
    create_default_column(&tree_view, ColumnsBadExtensions::CurrentExtension as i32, None, None);
    create_default_column(&tree_view, ColumnsBadExtensions::ValidExtensions as i32, None, None);
}

fn create_default_selection_button_column(
    tree_view: &TreeView,
    column_id: i32,
    model: ListStore,
    activatable_color_columns: Option<(i32, i32)>,
) -> (CellRendererToggle, TreeViewColumn) {
    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
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

fn create_default_column(tree_view: &TreeView, column_id: i32, sort_column_id: Option<i32>, colors_columns_id: Option<(i32, i32)>) -> (CellRendererText, TreeViewColumn) {
    let renderer = CellRendererText::new();
    let column: TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", column_id);
    if let Some(sort_column_id) = sort_column_id {
        column.set_sort_column_id(sort_column_id);
    } else {
        column.set_sort_column_id(column_id);
    }
    if let Some(colors_columns_id) = colors_columns_id {
        column.add_attribute(&renderer, "background", colors_columns_id.0);
        column.add_attribute(&renderer, "foreground", colors_columns_id.1);
    }
    tree_view.append_column(&column);
    (renderer, column)
}
