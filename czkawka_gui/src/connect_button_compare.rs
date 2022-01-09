use gtk::prelude::*;
use gtk::{TreeIter, TreeModel};
use image::DynamicImage;
use image::imageops::FilterType;

use crate::gui_data::GuiData;
use crate::help_functions::{get_full_name_from_path_name, get_image_path_temporary, resize_dynamic_image_dimension, HEADER_ROW_COLOR, NOTEBOOKS_INFOS, count_number_of_groups, get_max_file_name};

const BIG_PREVIEW_SIZE: u32 = 600;
const SMALL_PREVIEW_SIZE: u32 = 100;

pub fn connect_button_compare(gui_data: &GuiData) {
    let button_compare = gui_data.bottom_buttons.buttons_compare.clone();
    let window_compare = gui_data.compare_images.window_compare.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let label_group_info = gui_data.compare_images.label_group_info.clone();

    let button_go_previous_compare_group = gui_data.compare_images.button_go_previous_compare_group.clone();
    let button_go_next_compare_group = gui_data.compare_images.button_go_next_compare_group.clone();

    let check_button_first_text = gui_data.compare_images.check_button_first_text.clone();
    let check_button_second_text = gui_data.compare_images.check_button_second_text.clone();

    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_current_iter = gui_data.compare_images.shared_current_iter.clone();
    let shared_image_cache = gui_data.compare_images.shared_image_cache.clone();

    let image_compare_left = gui_data.compare_images.image_compare_left.clone();
    let image_compare_right = gui_data.compare_images.image_compare_right.clone();

    button_compare.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];
        let model = tree_view.model().unwrap();

        *shared_current_iter.borrow_mut() = Some(model.iter_first().unwrap());

        let current_group = 1;
        let group_number = count_number_of_groups(tree_view, nb_object.column_color.unwrap());

        if group_number == 0 {
            return;
        }

        *shared_current_of_groups.borrow_mut() = 1;
        *shared_numbers_of_groups.borrow_mut() = group_number;

        button_go_previous_compare_group.set_sensitive(false);
        if group_number == 1 {
            button_go_next_compare_group.set_sensitive(false);
        } else {
            button_go_next_compare_group.set_sensitive(true);
        }

        let tree_iter = shared_current_iter.borrow().clone().unwrap();

        let all_vec = get_all_path(&model, &tree_iter, nb_object.column_color.unwrap(), nb_object.column_path, nb_object.column_name);
        let cache_all_images = generate_cache_for_results(all_vec);

        // This is safe, because cache have at least 2 results
        image_compare_left.set_from_pixbuf(cache_all_images[0].1.pixbuf().as_ref());
        image_compare_right.set_from_pixbuf(cache_all_images[1].1.pixbuf().as_ref());

        check_button_first_text.set_label(&format!("0. {}",get_max_file_name(&cache_all_images[0].0,70)));
        check_button_second_text.set_label(&format!("1. {}",get_max_file_name(&cache_all_images[1].0,70)));


        label_group_info.set_text(format!("Group {}/{} ({} images)", current_group, group_number,cache_all_images.len()).as_str());

        *shared_image_cache.borrow_mut() = cache_all_images;

        window_compare.show();
    });

    let shared_image_cache = gui_data.compare_images.shared_image_cache.clone();
    let window_compare = gui_data.compare_images.window_compare.clone();
    window_compare.connect_delete_event(move |window_compare, _| {
        window_compare.hide();
        // TODO clear cached data here
        *shared_image_cache.borrow_mut() = Vec::new();
        gtk::Inhibit(true)
    });

    let button_go_previous_compare_group = gui_data.compare_images.button_go_previous_compare_group.clone();
    let button_go_next_compare_group = gui_data.compare_images.button_go_next_compare_group.clone();
    let label_group_info = gui_data.compare_images.label_group_info.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let check_button_first_text = gui_data.compare_images.check_button_first_text.clone();
    let check_button_second_text = gui_data.compare_images.check_button_second_text.clone();

    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let shared_current_iter = gui_data.compare_images.shared_current_iter.clone();
    let shared_image_cache = gui_data.compare_images.shared_image_cache.clone();

    let image_compare_left = gui_data.compare_images.image_compare_left.clone();
    let image_compare_right = gui_data.compare_images.image_compare_right.clone();

    button_go_previous_compare_group.connect_clicked(move |button_go_previous_compare_group| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];
        let model = tree_view.model().unwrap();

        *shared_current_of_groups.borrow_mut() -= 1;

        let current_group = *shared_current_of_groups.borrow();
        let group_number = *shared_numbers_of_groups.borrow();

        if current_group == 1 {
            button_go_previous_compare_group.set_sensitive(false);
        }
        button_go_next_compare_group.set_sensitive(true);


        let tree_iter = move_iter(&model, shared_current_iter.borrow().as_ref().unwrap(), nb_object.column_color.unwrap(),false);

        let all_vec = get_all_path(&model, shared_current_iter.borrow().as_ref().unwrap(), nb_object.column_color.unwrap(), nb_object.column_path, nb_object.column_name);
        let cache_all_images = generate_cache_for_results(all_vec);

        *shared_current_iter.borrow_mut() = Some(tree_iter);

        // This is safe, because cache have at least 2 results
        image_compare_left.set_from_pixbuf(cache_all_images[0].1.pixbuf().as_ref());
        image_compare_right.set_from_pixbuf(cache_all_images[1].1.pixbuf().as_ref());

        check_button_first_text.set_label(&format!("0. {}",get_max_file_name(&cache_all_images[0].0,70)));
        check_button_second_text.set_label(&format!("1. {}",get_max_file_name(&cache_all_images[1].0,70)));

        label_group_info.set_text(format!("Group {}/{} ({} images)", current_group, group_number,cache_all_images.len()).as_str());

        *shared_image_cache.borrow_mut() = cache_all_images;
    });

    let button_go_previous_compare_group = gui_data.compare_images.button_go_previous_compare_group.clone();
    let button_go_next_compare_group = gui_data.compare_images.button_go_next_compare_group.clone();
    let label_group_info = gui_data.compare_images.label_group_info.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    let check_button_first_text = gui_data.compare_images.check_button_first_text.clone();
    let check_button_second_text = gui_data.compare_images.check_button_second_text.clone();

    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let shared_current_iter = gui_data.compare_images.shared_current_iter.clone();
    let shared_image_cache = gui_data.compare_images.shared_image_cache.clone();

    let image_compare_left = gui_data.compare_images.image_compare_left.clone();
    let image_compare_right = gui_data.compare_images.image_compare_right.clone();

    button_go_next_compare_group.connect_clicked(move |button_go_next_compare_group| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];
        let model = tree_view.model().unwrap();

        *shared_current_of_groups.borrow_mut() += 1;

        let current_group = *shared_current_of_groups.borrow();
        let group_number = *shared_numbers_of_groups.borrow();

        if group_number == current_group {
            button_go_next_compare_group.set_sensitive(false);
        }
        button_go_previous_compare_group.set_sensitive(true);

        let tree_iter = move_iter(&model, shared_current_iter.borrow().as_ref().unwrap(), nb_object.column_color.unwrap(),true);

        let all_vec = get_all_path(&model, shared_current_iter.borrow().as_ref().unwrap(), nb_object.column_color.unwrap(), nb_object.column_path, nb_object.column_name);
        let cache_all_images = generate_cache_for_results(all_vec);

        *shared_current_iter.borrow_mut() = Some(tree_iter);

        // This is safe, because cache have at least 2 results
        image_compare_left.set_from_pixbuf(cache_all_images[0].1.pixbuf().as_ref());
        image_compare_right.set_from_pixbuf(cache_all_images[1].1.pixbuf().as_ref());

        check_button_first_text.set_label(&format!("0. {}",get_max_file_name(&cache_all_images[0].0,70)));
        check_button_second_text.set_label(&format!("1. {}",get_max_file_name(&cache_all_images[1].0,70)));

        label_group_info.set_text(format!("Group {}/{} ({} images)", current_group, group_number,cache_all_images.len()).as_str());

        *shared_image_cache.borrow_mut() = cache_all_images;
    });
}

fn generate_cache_for_results(vector_with_path: Vec<(String, gtk::TreeIter)>) -> Vec<(String, gtk::Image, gtk::Image)> {
    let mut cache_all_images = Vec::new();
    for (path, _tree_iter) in vector_with_path {
        let dynamic_image = match image::open(&path) {
            Ok(t) => t,
            Err(_) => DynamicImage::new_bgr8(1, 1),
        };

        let big_thumbnail = resize_dynamic_image_dimension(dynamic_image, (BIG_PREVIEW_SIZE, BIG_PREVIEW_SIZE),&FilterType::Triangle);
        let big_path = get_image_path_temporary("roman", 1, "jpg");
        let _ = big_thumbnail.save(&big_path);
        let big_img = gtk::Image::new();
        big_img.set_from_file(big_path);

        let small_thumbnail = resize_dynamic_image_dimension(big_thumbnail, (SMALL_PREVIEW_SIZE, SMALL_PREVIEW_SIZE),&FilterType::Triangle);
        let small_path = get_image_path_temporary("roman", 1, "jpg");
        let _ = small_thumbnail.save(&small_path);
        let small_img = gtk::Image::new();
        small_img.set_from_file(small_path);

        cache_all_images.push((path, big_img, small_img));
    }
    cache_all_images
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

    if !model.iter_next(&used_iter) {
        panic!("Found only header!");
    }

    loop {
        let name = model.value(&used_iter, column_name).get::<String>().unwrap();
        let path = model.value(&used_iter, column_path).get::<String>().unwrap();

        let full_name = get_full_name_from_path_name(&path, &name);

        returned_vector.push((full_name, used_iter.clone()));

        if !model.iter_next(&used_iter) {
            break;
        }

        let color = model.value(&used_iter, column_color).get::<String>().unwrap();

        if color == HEADER_ROW_COLOR {
            break;
        }
    }

    assert!(returned_vector.len() > 1);

    returned_vector
}
fn move_iter(model: &gtk::TreeModel, tree_iter: &TreeIter, column_color: i32, go_next: bool) -> TreeIter {
    assert_eq!(model.value(&tree_iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR);

    if go_next {
        if !model.iter_next(&tree_iter) {
            panic!("Found only header!");
        }
    }
    else{
        if !model.iter_previous(&tree_iter) {
            panic!("Found only header!");
        }
    }

    loop {
        if go_next {
            if !model.iter_next(&tree_iter) {
                break;
            }
        }
        else{
            if !model.iter_previous(&tree_iter) {
                break;
            }
        }

        let color = model.value(&tree_iter, column_color).get::<String>().unwrap();

        if color == HEADER_ROW_COLOR {
            break;
        }
    }
    tree_iter.clone()
}
