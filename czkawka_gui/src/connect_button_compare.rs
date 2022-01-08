use gtk::prelude::*;
use gtk::{TreeIter, TreeModel};

use crate::gui_data::GuiData;
use crate::help_functions::{count_number_of_groups, get_full_name_from_path_name, HEADER_ROW_COLOR, NOTEBOOKS_INFOS};

pub fn connect_button_compare(gui_data: &GuiData) {
    let button_compare = gui_data.bottom_buttons.buttons_compare.clone();
    let window_compare = gui_data.compare_images.window_compare.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let label_group_info = gui_data.compare_images.label_group_info.clone();

    let button_go_previous_compare_group = gui_data.compare_images.button_go_previous_compare_group.clone();
    let button_go_next_compare_group = gui_data.compare_images.button_go_next_compare_group.clone();

    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_current_iter = gui_data.compare_images.shared_current_iter.clone();

    button_compare.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];

        let model = tree_view.model().unwrap();
        *shared_current_iter.borrow_mut() = Some(model.iter_first().unwrap());

        let group_number = count_number_of_groups(&tree_view, nb_object.column_color.unwrap());

        if group_number == 0 {
            return;
        }

        window_compare.show();

        *shared_current_of_groups.borrow_mut() = 1;
        *shared_numbers_of_groups.borrow_mut() = group_number;

        button_go_previous_compare_group.set_sensitive(false);
        if group_number == 1 {
            button_go_next_compare_group.set_sensitive(false);
        } else {
            button_go_next_compare_group.set_sensitive(true);
        }

        let tree_iter = shared_current_iter.borrow().clone().unwrap();

        println!(
            "{:?}",
            get_all_path(&model, &tree_iter, nb_object.column_color.unwrap(), nb_object.column_path, nb_object.column_name)
        );

        label_group_info.set_text(format!("Group 1/{}", group_number).as_str());
    });

    let window_compare = gui_data.compare_images.window_compare.clone();
    window_compare.connect_delete_event(move |window_compare, _| {
        window_compare.hide();
        gtk::Inhibit(true)
    });

    let button_go_previous_compare_group = gui_data.compare_images.button_go_previous_compare_group.clone();
    let button_go_next_compare_group = gui_data.compare_images.button_go_next_compare_group.clone();
    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let label_group_info = gui_data.compare_images.label_group_info.clone();

    button_go_previous_compare_group.connect_clicked(move |button_go_previous_compare_group| {
        *shared_current_of_groups.borrow_mut() -= 1;

        let current_group = *shared_current_of_groups.borrow();
        let number_of_groups = *shared_numbers_of_groups.borrow();

        if current_group == 1 {
            button_go_previous_compare_group.set_sensitive(false);
        }
        button_go_next_compare_group.set_sensitive(true);

        label_group_info.set_text(format!("Group {}/{}", current_group, number_of_groups).as_str());
    });

    let button_go_previous_compare_group = gui_data.compare_images.button_go_previous_compare_group.clone();
    let button_go_next_compare_group = gui_data.compare_images.button_go_next_compare_group.clone();
    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let label_group_info = gui_data.compare_images.label_group_info.clone();

    button_go_next_compare_group.connect_clicked(move |button_go_next_compare_group| {
        *shared_current_of_groups.borrow_mut() += 1;

        let current_group = *shared_current_of_groups.borrow();
        let number_of_groups = *shared_numbers_of_groups.borrow();

        if number_of_groups == current_group {
            button_go_next_compare_group.set_sensitive(false);
        }
        button_go_previous_compare_group.set_sensitive(true);

        label_group_info.set_text(format!("Group {}/{}", current_group, number_of_groups).as_str());
    });
}

fn get_all_path(model: &TreeModel, current_iter: &TreeIter, column_color: i32, column_path: i32, column_name: i32) -> Vec<(String, TreeIter)> {
    let used_iter = current_iter.clone();

    assert_eq!(model.value(&used_iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR);
    let using_reference = !model.value(&used_iter, column_path).get::<String>().unwrap().is_empty();

    let mut returned_vector = Vec::new();

    if using_reference {
        let name = model.value(&used_iter, column_name).get::<String>().unwrap();
        let path = model.value(&used_iter, column_path).get::<String>().unwrap();

        let full_name = get_full_name_from_path_name(&path, &name);

        returned_vector.push((full_name, used_iter.clone()));
    }

    loop {
        let name = model.value(&used_iter, column_name).get::<String>().unwrap();
        let path = model.value(&used_iter, column_path).get::<String>().unwrap();
        let color = model.value(&used_iter, column_color).get::<String>().unwrap();

        let full_name = get_full_name_from_path_name(&path, &name);

        returned_vector.push((full_name, used_iter.clone()));

        if !model.iter_next(&used_iter) {
            break;
        }

        if color == HEADER_ROW_COLOR {
            break;
        }
    }

    // assert!(returned_vector.len() > 1);

    returned_vector
}
// fn move_iter(model: &ListStore, tree_iter: &TreeIter, go_next: bool) {}
