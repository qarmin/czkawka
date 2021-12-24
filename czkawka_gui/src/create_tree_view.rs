use gtk::prelude::*;
use gtk::TreeViewColumn;

use crate::help_functions::*;

// When adding new column do not forget to update translations

pub fn create_tree_view_included_directories(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.set_title("Folders to check");
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", ColumnsIncludedDirectory::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsIncludedDirectory::ReferenceButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsIncludedDirectory::ReferenceButton as u32, &fixed.to_value());
    });
    renderer.set_activatable(true);
    let column = gtk::TreeViewColumn::new();
    column.set_title("Reference folder");
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "active", ColumnsIncludedDirectory::ReferenceButton as i32);
    tree_view.append_column(&column);
}

pub fn create_tree_view_excluded_directories(tree_view: &gtk::TreeView) {
    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", ColumnsExcludedDirectory::Path as i32);
    tree_view.append_column(&column);

    tree_view.set_headers_visible(false);
}

pub fn create_tree_view_duplicates(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsDuplicates::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsDuplicates::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsDuplicates::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsDuplicates::SelectionButton as i32);
    column.add_attribute(&renderer, "cell-background", ColumnsDuplicates::Color as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsDuplicates::Size as i32);
    column.add_attribute(&renderer, "background", ColumnsDuplicates::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsDuplicates::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsDuplicates::Name as i32);
    column.add_attribute(&renderer, "background", ColumnsDuplicates::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsDuplicates::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsDuplicates::Path as i32);
    column.add_attribute(&renderer, "background", ColumnsDuplicates::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsDuplicates::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
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

pub fn create_tree_view_empty_folders(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsEmptyFolders::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsEmptyFolders::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsEmptyFolders::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Folder Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Name as i32);
    column.set_sort_column_id(ColumnsEmptyFolders::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Path as i32);
    column.set_sort_column_id(ColumnsEmptyFolders::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Modification as i32);
    column.set_sort_column_id(ColumnsEmptyFolders::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_big_files(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsBigFiles::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsBigFiles::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsBigFiles::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Size as i32);
    column.set_sort_column_id(ColumnsBigFiles::SizeAsBytes as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Name as i32);
    column.set_sort_column_id(ColumnsBigFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Path as i32);
    column.set_sort_column_id(ColumnsBigFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Modification as i32);
    column.set_sort_column_id(ColumnsBigFiles::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_temporary_files(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsTemporaryFiles::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsTemporaryFiles::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsTemporaryFiles::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Name as i32);
    column.set_sort_column_id(ColumnsTemporaryFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Path as i32);
    column.set_sort_column_id(ColumnsTemporaryFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Modification as i32);
    column.set_sort_column_id(ColumnsTemporaryFiles::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_empty_files(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsEmptyFiles::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsEmptyFiles::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsEmptyFiles::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Name as i32);
    column.set_sort_column_id(ColumnsEmptyFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Path as i32);
    column.set_sort_column_id(ColumnsEmptyFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Modification as i32);
    column.set_sort_column_id(ColumnsEmptyFiles::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_similar_images(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsSimilarImages::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsSimilarImages::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsSimilarImages::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsSimilarImages::SelectionButton as i32);
    column.add_attribute(&renderer, "cell-background", ColumnsSimilarImages::Color as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Similarity");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Similarity as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Size as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Dimensions");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Dimensions as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Name as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Path as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
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

pub fn create_tree_view_similar_videos(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsSimilarVideos::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsSimilarVideos::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsSimilarVideos::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsSimilarVideos::SelectionButton as i32);
    column.add_attribute(&renderer, "cell-background", ColumnsSimilarVideos::Color as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarVideos::Size as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarVideos::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarVideos::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarVideos::Name as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarVideos::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarVideos::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSimilarVideos::Path as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarVideos::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarVideos::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
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

pub fn create_tree_view_same_music(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsSameMusic::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsSameMusic::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsSameMusic::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsSameMusic::SelectionButton as i32);
    column.add_attribute(&renderer, "cell-background", ColumnsSameMusic::Color as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::Size as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::Name as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::Path as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Title");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::Title as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Artist");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::Artist as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Year");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::Year as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Album Title");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::AlbumTitle as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Album Artist");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::AlbumArtist as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::Modification as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_invalid_symlinks(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsInvalidSymlinks::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsInvalidSymlinks::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsInvalidSymlinks::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Symlink File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::Name as i32);
    column.set_sort_column_id(ColumnsInvalidSymlinks::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Symlink Folder");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::Path as i32);
    column.set_sort_column_id(ColumnsInvalidSymlinks::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Destination Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::DestinationPath as i32);
    column.set_sort_column_id(ColumnsInvalidSymlinks::DestinationPath as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Type of Error");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::TypeOfError as i32);
    column.set_sort_column_id(ColumnsInvalidSymlinks::TypeOfError as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::Modification as i32);
    column.set_sort_column_id(ColumnsInvalidSymlinks::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_broken_files(tree_view: &gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsBrokenFiles::SelectionButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {:?}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsBrokenFiles::SelectionButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsBrokenFiles::SelectionButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBrokenFiles::Name as i32);
    column.set_sort_column_id(ColumnsBrokenFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBrokenFiles::Path as i32);
    column.set_sort_column_id(ColumnsBrokenFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("ErrorType");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBrokenFiles::ErrorType as i32);
    column.set_sort_column_id(ColumnsBrokenFiles::ErrorType as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBrokenFiles::Modification as i32);
    column.set_sort_column_id(ColumnsBrokenFiles::ModificationAsSecs as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}
