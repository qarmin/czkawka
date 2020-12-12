extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use gtk::prelude::*;
use gtk::TreeIter;

// File length variable allows users to choose duplicates which have shorter file name
// e.g. 'tar.gz' will be selected instead 'tar.gz (copy)' etc.

fn popover_select_all(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();

    selection.select_all();
    popover.popdown();
}
fn popover_unselect_all(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();

    selection.unselect_all();
    popover.popdown();
}
fn popover_reverse(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();

    let (vector_tree_path, tree_model) = selection.get_selected_rows();

    if vector_tree_path.is_empty() {
        selection.select_all();
    } else {
        let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

        let mut current_path_index = 0;
        let mut tree_iter_selected: TreeIter;
        loop {
            if current_path_index >= vector_tree_path.len() {
                selection.select_iter(&tree_iter_all);
            } else {
                tree_iter_selected = tree_model.get_iter(vector_tree_path.get(current_path_index).unwrap()).unwrap();
                if tree_model.get_path(&tree_iter_all).unwrap() == tree_model.get_path(&tree_iter_selected).unwrap() {
                    selection.unselect_iter(&tree_iter_selected);
                    current_path_index += 1;
                } else {
                    selection.select_iter(&tree_iter_all);
                }
            }
            if !tree_model.iter_next(&tree_iter_all) {
                break;
            }
        }
    }

    popover.popdown();
}

fn popover_all_except_oldest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_modification_as_secs: i32, column_file_name: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut oldest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut oldest_modification_time: u64 = u64::max_value();

        let mut file_length: usize = 0;

        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let modification = tree_model.get_value(&tree_iter_all, column_modification_as_secs).get::<u64>().unwrap().unwrap();
            let current_file_length = tree_model.get_value(&tree_iter_all, column_file_name).get::<String>().unwrap().unwrap().len();
            if modification < oldest_modification_time || (modification == oldest_modification_time && current_file_length < file_length) {
                file_length = current_file_length;
                oldest_modification_time = modification;
                oldest_index = Some(current_index);
            }

            current_index += 1;

            if !tree_model.iter_next(&tree_iter_all) {
                end = true;
                break;
            }
        }
        if oldest_index == None {
            continue;
        }
        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
            if index != oldest_index.unwrap() {
                selection.select_iter(tree_iter);
            } else {
                selection.unselect_iter(tree_iter);
            }
        }

        if end {
            break;
        }
    }

    popover.popdown();
}
fn popover_all_except_newest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_modification_as_secs: i32, column_file_name: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut newest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut newest_modification_time: u64 = 0;

        let mut file_length: usize = 0;

        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let modification = tree_model.get_value(&tree_iter_all, column_modification_as_secs).get::<u64>().unwrap().unwrap();
            let current_file_length = tree_model.get_value(&tree_iter_all, column_file_name).get::<String>().unwrap().unwrap().len();
            if modification > newest_modification_time || (modification == newest_modification_time && current_file_length < file_length) {
                file_length = current_file_length;
                newest_modification_time = modification;
                newest_index = Some(current_index);
            }

            current_index += 1;

            if !tree_model.iter_next(&tree_iter_all) {
                end = true;
                break;
            }
        }
        if newest_index == None {
            continue;
        }
        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
            if index != newest_index.unwrap() {
                selection.select_iter(tree_iter);
            } else {
                selection.unselect_iter(tree_iter);
            }
        }

        if end {
            break;
        }
    }

    popover.popdown();
}
fn popover_one_oldest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_modification_as_secs: i32, column_file_name: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut oldest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut oldest_modification_time: u64 = u64::max_value();

        let mut file_length: usize = 0;

        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let modification = tree_model.get_value(&tree_iter_all, column_modification_as_secs).get::<u64>().unwrap().unwrap();
            let current_file_length = tree_model.get_value(&tree_iter_all, column_file_name).get::<String>().unwrap().unwrap().len();
            if modification < oldest_modification_time || (modification == oldest_modification_time && current_file_length > file_length) {
                file_length = current_file_length;
                oldest_modification_time = modification;
                oldest_index = Some(current_index);
            }

            current_index += 1;

            if !tree_model.iter_next(&tree_iter_all) {
                end = true;
                break;
            }
        }
        if oldest_index == None {
            continue;
        }
        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
            if index == oldest_index.unwrap() {
                selection.select_iter(tree_iter);
            } else {
                selection.unselect_iter(tree_iter);
            }
        }

        if end {
            break;
        }
    }

    popover.popdown();
}
fn popover_one_newest(popover: &gtk::Popover, scrolled_window: &gtk::ScrolledWindow, column_color: i32, column_modification_as_secs: i32, column_file_name: i32) {
    let tree_view = get_tree_view(&scrolled_window);
    let selection = tree_view.get_selection();
    let tree_model = tree_view.get_model().unwrap();

    let tree_iter_all = tree_model.get_iter_first().unwrap(); // Never should be available button where there is no available records

    let mut end: bool = false;

    loop {
        let mut tree_iter_array: Vec<TreeIter> = Vec::new();
        let mut newest_index: Option<usize> = None;
        let mut current_index: usize = 0;
        let mut newest_modification_time: u64 = 0;

        let mut file_length: usize = 0;
        loop {
            let color = tree_model.get_value(&tree_iter_all, column_color).get::<String>().unwrap().unwrap();
            if color == HEADER_ROW_COLOR {
                if !tree_model.iter_next(&tree_iter_all) {
                    end = true;
                }
                break;
            }
            tree_iter_array.push(tree_iter_all.clone());
            let modification = tree_model.get_value(&tree_iter_all, column_modification_as_secs).get::<u64>().unwrap().unwrap();
            let current_file_length = tree_model.get_value(&tree_iter_all, column_file_name).get::<String>().unwrap().unwrap().len();
            if modification > newest_modification_time || (modification == newest_modification_time && current_file_length > file_length) {
                file_length = current_file_length;
                newest_modification_time = modification;
                newest_index = Some(current_index);
            }

            current_index += 1;

            if !tree_model.iter_next(&tree_iter_all) {
                end = true;
                break;
            }
        }
        if newest_index == None {
            continue;
        }
        for (index, tree_iter) in tree_iter_array.iter().enumerate() {
            if index == newest_index.unwrap() {
                selection.select_iter(tree_iter);
            } else {
                selection.unselect_iter(tree_iter);
            }
        }

        if end {
            break;
        }
    }

    popover.popdown();
}

pub fn connect_popovers(gui_data: &GuiData) {
    connect_select_all(&gui_data);
    connect_unselect_all(&gui_data);
    connect_reverse(&gui_data);

    connect_all_except_oldest(&gui_data);
    connect_all_except_newest(&gui_data);
    connect_one_oldest(&gui_data);
    connect_one_newest(&gui_data);
}
pub fn connect_select_all(gui_data: &GuiData) {
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();

    let scrolled_window_main_empty_folder_finder = gui_data.scrolled_window_main_empty_folder_finder.clone();
    let scrolled_window_big_files_finder = gui_data.scrolled_window_big_files_finder.clone();
    let scrolled_window_main_empty_files_finder = gui_data.scrolled_window_main_empty_files_finder.clone();
    let scrolled_window_main_temporary_files_finder = gui_data.scrolled_window_main_temporary_files_finder.clone();
    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let scrolled_window_zeroed_files_finder = gui_data.scrolled_window_zeroed_files_finder.clone();
    let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let popover_select_duplicate = gui_data.popover_select_duplicate.clone();
    let popover_select_simple_list = gui_data.popover_select_simple_list.clone();
    let buttons_popover_simple_list_select_all = gui_data.buttons_popover_simple_list_select_all.clone();
    let buttons_popover_duplicate_select_all = gui_data.buttons_popover_duplicate_select_all.clone();
    buttons_popover_duplicate_select_all.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            popover_select_all(&popover_select_duplicate, &scrolled_window_duplicate_finder);
        }
        "notebook_main_same_music_finder" => {
            popover_select_all(&popover_select_duplicate, &scrolled_window_same_music_finder);
        }
        "notebook_main_similar_images_finder_label" => {
            popover_select_all(&popover_select_duplicate, &scrolled_window_similar_images_finder);
        }
        e => panic!("Not existent {}", e),
    });

    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    buttons_popover_simple_list_select_all.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "scrolled_window_main_empty_folder_finder" => {
            popover_select_all(&popover_select_simple_list, &scrolled_window_main_empty_folder_finder);
        }
        "scrolled_window_main_empty_files_finder" => {
            popover_select_all(&popover_select_simple_list, &scrolled_window_main_empty_files_finder);
        }
        "scrolled_window_main_temporary_files_finder" => {
            popover_select_all(&popover_select_simple_list, &scrolled_window_main_temporary_files_finder);
        }
        "notebook_main_zeroed_files_finder" => {
            popover_select_all(&popover_select_simple_list, &scrolled_window_zeroed_files_finder);
        }
        "notebook_big_main_file_finder" => {
            popover_select_all(&popover_select_simple_list, &scrolled_window_big_files_finder);
        }
        e => panic!("Not existent {}", e),
    });
}
pub fn connect_unselect_all(gui_data: &GuiData) {
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();

    let scrolled_window_main_empty_folder_finder = gui_data.scrolled_window_main_empty_folder_finder.clone();
    let scrolled_window_big_files_finder = gui_data.scrolled_window_big_files_finder.clone();
    let scrolled_window_main_empty_files_finder = gui_data.scrolled_window_main_empty_files_finder.clone();
    let scrolled_window_main_temporary_files_finder = gui_data.scrolled_window_main_temporary_files_finder.clone();
    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let scrolled_window_zeroed_files_finder = gui_data.scrolled_window_zeroed_files_finder.clone();
    let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let popover_select_duplicate = gui_data.popover_select_duplicate.clone();
    let popover_select_simple_list = gui_data.popover_select_simple_list.clone();
    let buttons_popover_simple_list_unselect_all = gui_data.buttons_popover_simple_list_unselect_all.clone();
    let buttons_popover_duplicate_unselect_all = gui_data.buttons_popover_duplicate_unselect_all.clone();
    buttons_popover_duplicate_unselect_all.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            popover_unselect_all(&popover_select_duplicate, &scrolled_window_duplicate_finder);
        }
        "notebook_main_same_music_finder" => {
            popover_unselect_all(&popover_select_duplicate, &scrolled_window_same_music_finder);
        }
        "notebook_main_similar_images_finder_label" => {
            popover_unselect_all(&popover_select_duplicate, &scrolled_window_similar_images_finder);
        }
        e => panic!("Not existent {}", e),
    });

    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    buttons_popover_simple_list_unselect_all.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "scrolled_window_main_empty_folder_finder" => {
            popover_unselect_all(&popover_select_simple_list, &scrolled_window_main_empty_folder_finder);
        }
        "scrolled_window_main_empty_files_finder" => {
            popover_unselect_all(&popover_select_simple_list, &scrolled_window_main_empty_files_finder);
        }
        "scrolled_window_main_temporary_files_finder" => {
            popover_unselect_all(&popover_select_simple_list, &scrolled_window_main_temporary_files_finder);
        }
        "notebook_main_zeroed_files_finder" => {
            popover_unselect_all(&popover_select_simple_list, &scrolled_window_zeroed_files_finder);
        }
        "notebook_big_main_file_finder" => {
            popover_unselect_all(&popover_select_simple_list, &scrolled_window_big_files_finder);
        }
        e => panic!("Not existent {}", e),
    });
}
pub fn connect_reverse(gui_data: &GuiData) {
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();

    let scrolled_window_main_empty_folder_finder = gui_data.scrolled_window_main_empty_folder_finder.clone();
    let scrolled_window_big_files_finder = gui_data.scrolled_window_big_files_finder.clone();
    let scrolled_window_main_empty_files_finder = gui_data.scrolled_window_main_empty_files_finder.clone();
    let scrolled_window_main_temporary_files_finder = gui_data.scrolled_window_main_temporary_files_finder.clone();
    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let scrolled_window_zeroed_files_finder = gui_data.scrolled_window_zeroed_files_finder.clone();
    let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let popover_select_duplicate = gui_data.popover_select_duplicate.clone();
    let popover_select_simple_list = gui_data.popover_select_simple_list.clone();
    let buttons_popover_simple_list_reverse = gui_data.buttons_popover_simple_list_reverse.clone();
    let buttons_popover_duplicate_reverse = gui_data.buttons_popover_duplicate_reverse.clone();
    buttons_popover_duplicate_reverse.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            popover_reverse(&popover_select_duplicate, &scrolled_window_duplicate_finder);
        }
        "notebook_main_same_music_finder" => {
            popover_reverse(&popover_select_duplicate, &scrolled_window_same_music_finder);
        }
        "notebook_main_similar_images_finder_label" => {
            popover_reverse(&popover_select_duplicate, &scrolled_window_similar_images_finder);
        }
        e => panic!("Not existent {}", e),
    });

    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    buttons_popover_simple_list_reverse.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "scrolled_window_main_empty_folder_finder" => {
            popover_reverse(&popover_select_simple_list, &scrolled_window_main_empty_folder_finder);
        }
        "scrolled_window_main_empty_files_finder" => {
            popover_reverse(&popover_select_simple_list, &scrolled_window_main_empty_files_finder);
        }
        "scrolled_window_main_temporary_files_finder" => {
            popover_reverse(&popover_select_simple_list, &scrolled_window_main_temporary_files_finder);
        }
        "notebook_main_zeroed_files_finder" => {
            popover_reverse(&popover_select_simple_list, &scrolled_window_zeroed_files_finder);
        }
        "notebook_big_main_file_finder" => {
            popover_reverse(&popover_select_simple_list, &scrolled_window_big_files_finder);
        }
        e => panic!("Not existent {}", e),
    });
}

pub fn connect_all_except_oldest(gui_data: &GuiData) {
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();

    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let popover_select_duplicate = gui_data.popover_select_duplicate.clone();
    let buttons_popover_duplicate_select_all_except_oldest = gui_data.buttons_popover_duplicate_select_all_except_oldest.clone();
    buttons_popover_duplicate_select_all_except_oldest.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            popover_all_except_oldest(
                &popover_select_duplicate,
                &scrolled_window_duplicate_finder,
                ColumnsDuplicates::Color as i32,
                ColumnsDuplicates::ModificationAsSecs as i32,
                ColumnsDuplicates::Name as i32,
            );
        }
        "notebook_main_same_music_finder" => {
            popover_all_except_oldest(
                &popover_select_duplicate,
                &scrolled_window_same_music_finder,
                ColumnsSameMusic::Color as i32,
                ColumnsSameMusic::ModificationAsSecs as i32,
                ColumnsSameMusic::Name as i32,
            );
        }
        "notebook_main_similar_images_finder_label" => {
            popover_all_except_oldest(
                &popover_select_duplicate,
                &scrolled_window_similar_images_finder,
                ColumnsSimilarImages::Color as i32,
                ColumnsSimilarImages::ModificationAsSecs as i32,
                ColumnsSimilarImages::Name as i32,
            );
        }
        e => panic!("Not existent {}", e),
    });
}
pub fn connect_all_except_newest(gui_data: &GuiData) {
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();

    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let popover_select_duplicate = gui_data.popover_select_duplicate.clone();
    let buttons_popover_duplicate_select_all_except_newest = gui_data.buttons_popover_duplicate_select_all_except_newest.clone();
    buttons_popover_duplicate_select_all_except_newest.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            popover_all_except_newest(
                &popover_select_duplicate,
                &scrolled_window_duplicate_finder,
                ColumnsDuplicates::Color as i32,
                ColumnsDuplicates::ModificationAsSecs as i32,
                ColumnsDuplicates::Name as i32,
            );
        }
        "notebook_main_same_music_finder" => {
            popover_all_except_newest(
                &popover_select_duplicate,
                &scrolled_window_same_music_finder,
                ColumnsSameMusic::Color as i32,
                ColumnsSameMusic::ModificationAsSecs as i32,
                ColumnsSameMusic::Name as i32,
            );
        }
        "notebook_main_similar_images_finder_label" => {
            popover_all_except_newest(
                &popover_select_duplicate,
                &scrolled_window_similar_images_finder,
                ColumnsSimilarImages::Color as i32,
                ColumnsSimilarImages::ModificationAsSecs as i32,
                ColumnsSimilarImages::Name as i32,
            );
        }
        e => panic!("Not existent {}", e),
    });
}
pub fn connect_one_newest(gui_data: &GuiData) {
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();

    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let popover_select_duplicate = gui_data.popover_select_duplicate.clone();
    let buttons_popover_duplicate_select_one_newest = gui_data.buttons_popover_duplicate_select_one_newest.clone();
    buttons_popover_duplicate_select_one_newest.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            popover_one_newest(
                &popover_select_duplicate,
                &scrolled_window_duplicate_finder,
                ColumnsDuplicates::Color as i32,
                ColumnsDuplicates::ModificationAsSecs as i32,
                ColumnsDuplicates::Name as i32,
            );
        }
        "notebook_main_same_music_finder" => {
            popover_one_newest(
                &popover_select_duplicate,
                &scrolled_window_same_music_finder,
                ColumnsSameMusic::Color as i32,
                ColumnsSameMusic::ModificationAsSecs as i32,
                ColumnsSameMusic::Name as i32,
            );
        }
        "notebook_main_similar_images_finder_label" => {
            popover_one_newest(
                &popover_select_duplicate,
                &scrolled_window_similar_images_finder,
                ColumnsSimilarImages::Color as i32,
                ColumnsSimilarImages::ModificationAsSecs as i32,
                ColumnsSimilarImages::Name as i32,
            );
        }
        e => panic!("Not existent {}", e),
    });
}
pub fn connect_one_oldest(gui_data: &GuiData) {
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();

    let scrolled_window_similar_images_finder = gui_data.scrolled_window_similar_images_finder.clone();
    let scrolled_window_same_music_finder = gui_data.scrolled_window_same_music_finder.clone();
    let scrolled_window_duplicate_finder = gui_data.scrolled_window_duplicate_finder.clone();
    let popover_select_duplicate = gui_data.popover_select_duplicate.clone();
    let buttons_popover_duplicate_select_one_oldest = gui_data.buttons_popover_duplicate_select_one_oldest.clone();
    buttons_popover_duplicate_select_one_oldest.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            popover_one_oldest(
                &popover_select_duplicate,
                &scrolled_window_duplicate_finder,
                ColumnsDuplicates::Color as i32,
                ColumnsDuplicates::ModificationAsSecs as i32,
                ColumnsDuplicates::Name as i32,
            );
        }
        "notebook_main_same_music_finder" => {
            popover_one_oldest(
                &popover_select_duplicate,
                &scrolled_window_same_music_finder,
                ColumnsSameMusic::Color as i32,
                ColumnsSameMusic::ModificationAsSecs as i32,
                ColumnsSameMusic::Name as i32,
            );
        }
        "notebook_main_similar_images_finder_label" => {
            popover_one_oldest(
                &popover_select_duplicate,
                &scrolled_window_similar_images_finder,
                ColumnsSimilarImages::Color as i32,
                ColumnsSimilarImages::ModificationAsSecs as i32,
                ColumnsSimilarImages::Name as i32,
            );
        }
        e => panic!("Not existent {}", e),
    });
}
