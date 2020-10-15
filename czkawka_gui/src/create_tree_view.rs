use crate::help_functions::*;
use gtk::prelude::*;
use gtk::TreeViewColumn;

pub fn create_tree_view_duplicates(tree_view: &mut gtk::TreeView) {
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
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsDuplicates::Path as i32);
    column.add_attribute(&renderer, "background", ColumnsDuplicates::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsDuplicates::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsDuplicates::Modification as i32);
    column.add_attribute(&renderer, "background", ColumnsDuplicates::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsDuplicates::TextColor as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_empty_folders(tree_view: &mut gtk::TreeView) {
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
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsEmptyFolders::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_big_files(tree_view: &mut gtk::TreeView) {
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
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsBigFiles::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_temporary_files(tree_view: &mut gtk::TreeView) {
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
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsTemporaryFiles::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_empty_files(tree_view: &mut gtk::TreeView) {
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
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Path as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsEmptyFiles::Modification as i32);
    tree_view.append_column(&column);

    tree_view.set_vexpand(true);
}

pub fn create_tree_view_similar_images(tree_view: &mut gtk::TreeView) {
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
    column.set_min_width(100);
    column.add_attribute(&renderer, "text", ColumnsSimilarImages::Path as i32);
    column.add_attribute(&renderer, "background", ColumnsSimilarImages::Color as i32);
    column.add_attribute(&renderer, "foreground", ColumnsSimilarImages::TextColor as i32);
    tree_view.append_column(&column);

    let renderer = gtk::CellRendererText::new();
    let column: gtk::TreeViewColumn = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_title("Modification Date");
    column.set_resizable(true);
    column.set_min_width(100);
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
