use crate::help_functions::*;
use gtk::prelude::*;
use gtk::TreeViewColumn;

pub fn create_tree_view_duplicates(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsDuplicates::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsDuplicates::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsDuplicates::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsDuplicates::ActiveSelectButton as i32);
    column.add_attribute(&renderer, "cell-background", ColumnsDuplicates::Color as i32);
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

pub fn create_tree_view_empty_folders(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsEmptyFolders::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsEmptyFolders::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsEmptyFolders::ActiveSelectButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Folder Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_big_files(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsBigFiles::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsBigFiles::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsBigFiles::ActiveSelectButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Size as i32);

    tree_view.append_column(&column);
    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_temporary_files(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsTemporaryFiles::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsTemporaryFiles::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsTemporaryFiles::ActiveSelectButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_empty_files(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsEmptyFiles::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsEmptyFiles::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsEmptyFiles::ActiveSelectButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_similar_images(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsSimilarImages::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsSimilarImages::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsSimilarImages::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsSimilarImages::ActiveSelectButton as i32);
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

pub fn create_tree_view_directories(tree_view: &mut gtk::TreeView) {
    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", ColumnsDirectory::Path as i32);
    tree_view.append_column(&column);

    tree_view.set_headers_visible(false);
}

pub fn create_tree_view_zeroed_files(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsZeroedFiles::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsZeroedFiles::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsZeroedFiles::ActiveSelectButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Size");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsZeroedFiles::Size as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsZeroedFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsZeroedFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsZeroedFiles::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_same_music(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsSameMusic::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsSameMusic::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "activatable", ColumnsSameMusic::ActivatableSelectButton as i32);
    column.add_attribute(&renderer, "active", ColumnsSameMusic::ActiveSelectButton as i32);
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
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsSameMusic::Modification as i32);
    column.add_attribute(&renderer, "background", ColumnsSameMusic::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSameMusic::TextColor as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_invalid_symlinks(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsInvalidSymlinks::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsInvalidSymlinks::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsInvalidSymlinks::ActiveSelectButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Symlink File Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Symlink Folder");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Destination Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::DestinationPath as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Type of Error");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::TypeOfError as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsInvalidSymlinks::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_broken_files(tree_view: &mut gtk::TreeView) {
    let model = get_list_store(tree_view);

    let renderer = gtk::CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).unwrap();
        let mut fixed = model
            .value(&iter, ColumnsBrokenFiles::ActiveSelectButton as i32)
            .get::<bool>()
            .unwrap_or_else(|err| panic!("ListStore value missing at path {}: {}", path, err));
        fixed = !fixed;
        model.set_value(&iter, ColumnsBrokenFiles::ActiveSelectButton as u32, &fixed.to_value());
    });
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", ColumnsBrokenFiles::ActiveSelectButton as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Name");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBrokenFiles::Name as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Path");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBrokenFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("ErrorType");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBrokenFiles::ErrorType as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(50);
    column.add_attribute(&renderer, "text", ColumnsBrokenFiles::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}
