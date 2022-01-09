use czkawka_core::common::get_dynamic_image_from_raw_image;
use czkawka_core::similar_images::RAW_IMAGE_EXTENSIONS;
use gtk::prelude::*;
use gtk::{Image, Orientation, ScrolledWindow, TreeIter, TreeModel};
use image::imageops::FilterType;
use image::DynamicImage;
use std::cell::RefCell;
use std::rc::Rc;

use crate::gui_data::GuiData;
use crate::help_functions::{
    count_number_of_groups, get_full_name_from_path_name, get_image_path_temporary, get_max_file_name, resize_dynamic_image_dimension, NotebookObject, HEADER_ROW_COLOR,
    NOTEBOOKS_INFOS,
};

const BIG_PREVIEW_SIZE: u32 = 600;
const SMALL_PREVIEW_SIZE: u32 = 100;

pub fn connect_button_compare(gui_data: &GuiData) {
    let button_compare = gui_data.bottom_buttons.buttons_compare.clone();
    let window_compare = gui_data.compare_images.window_compare.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    let scrolled_window_compare_choose_images = gui_data.compare_images.scrolled_window_compare_choose_images.clone();

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

        let tree_iter = model.iter_first().unwrap();

        populate_groups_at_start(
            nb_object,
            &model,
            shared_current_iter.clone(),
            tree_iter,
            &image_compare_left,
            &image_compare_right,
            current_group,
            group_number,
            &check_button_first_text,
            &check_button_second_text,
            &scrolled_window_compare_choose_images,
            &label_group_info,
            shared_image_cache.clone(),
        );

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
    let scrolled_window_compare_choose_images = gui_data.compare_images.scrolled_window_compare_choose_images.clone();

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

        let tree_iter = move_iter(&model, shared_current_iter.borrow().as_ref().unwrap(), nb_object.column_color.unwrap(), false);

        populate_groups_at_start(
            nb_object,
            &model,
            shared_current_iter.clone(),
            tree_iter,
            &image_compare_left,
            &image_compare_right,
            current_group,
            group_number,
            &check_button_first_text,
            &check_button_second_text,
            &scrolled_window_compare_choose_images,
            &label_group_info,
            shared_image_cache.clone(),
        );
    });

    let button_go_previous_compare_group = gui_data.compare_images.button_go_previous_compare_group.clone();
    let button_go_next_compare_group = gui_data.compare_images.button_go_next_compare_group.clone();
    let label_group_info = gui_data.compare_images.label_group_info.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    let scrolled_window_compare_choose_images = gui_data.compare_images.scrolled_window_compare_choose_images.clone();

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

        let tree_iter = move_iter(&model, shared_current_iter.borrow().as_ref().unwrap(), nb_object.column_color.unwrap(), true);

        populate_groups_at_start(
            nb_object,
            &model,
            shared_current_iter.clone(),
            tree_iter,
            &image_compare_left,
            &image_compare_right,
            current_group,
            group_number,
            &check_button_first_text,
            &check_button_second_text,
            &scrolled_window_compare_choose_images,
            &label_group_info,
            shared_image_cache.clone(),
        );
    });
}

/// Populate all parameters for current group, it is used at start and when changing groups
fn populate_groups_at_start(
    nb_object: &NotebookObject,
    model: &TreeModel,
    shared_current_iter: Rc<RefCell<Option<TreeIter>>>,
    tree_iter: TreeIter,
    image_compare_left: &gtk::Image,
    image_compare_right: &gtk::Image,
    current_group: u32,
    group_number: u32,
    check_button_first_text: &gtk::CheckButton,
    check_button_second_text: &gtk::CheckButton,
    scrolled_window_compare_choose_images: &gtk::ScrolledWindow,
    label_group_info: &gtk::Label,
    shared_image_cache: Rc<RefCell<Vec<(String, String, gtk::Image, gtk::Image)>>>,
) {
    let all_vec = get_all_path(model, &tree_iter, nb_object.column_color.unwrap(), nb_object.column_path, nb_object.column_name);
    *shared_current_iter.borrow_mut() = Some(tree_iter);

    let cache_all_images = generate_cache_for_results(all_vec);

    // This is safe, because cache have at least 2 results
    image_compare_left.set_from_pixbuf(cache_all_images[0].2.pixbuf().as_ref());
    image_compare_right.set_from_pixbuf(cache_all_images[1].2.pixbuf().as_ref());

    check_button_first_text.set_label(&format!("1. {}", get_max_file_name(&cache_all_images[0].0, 70)));
    check_button_second_text.set_label(&format!("2. {}", get_max_file_name(&cache_all_images[1].0, 70)));

    label_group_info.set_text(format!("Group {}/{} ({} images)", current_group, group_number, cache_all_images.len()).as_str());

    populate_similar_scrolled_view(scrolled_window_compare_choose_images, &cache_all_images);

    *shared_image_cache.borrow_mut() = cache_all_images;
}

/// Generate images which will be used later as preview images without needing to open them again and again
fn generate_cache_for_results(vector_with_path: Vec<(String, String, gtk::TreeIter)>) -> Vec<(String, String, gtk::Image, gtk::Image)> {
    // TODO use here threads,
    // For now threads cannot be used because Image and TreeIter cannot be used in threads
    let mut cache_all_images = Vec::new();
    for (full_path, name, _tree_iter) in vector_with_path {
        let name_lowercase = name.to_lowercase();
        let dynamic_image = if RAW_IMAGE_EXTENSIONS.iter().any(|f| name_lowercase.ends_with(f)) {
            match get_dynamic_image_from_raw_image(&full_path) {
                Some(t) => t,
                None => {
                    println!("Failed to convert rawimage {}", full_path);
                    DynamicImage::new_rgb8(1, 1)
                }
            }
        } else {
            match image::open(&full_path) {
                Ok(t) => t,
                Err(_) => {
                    println!("Failed to open image {}", full_path);
                    DynamicImage::new_rgb8(1, 1)
                }
            }
        };

        let big_thumbnail = resize_dynamic_image_dimension(dynamic_image, (BIG_PREVIEW_SIZE, BIG_PREVIEW_SIZE), &FilterType::Triangle);
        let big_path = get_image_path_temporary("roman", 1, "jpg");
        let _ = big_thumbnail.save(&big_path);
        let big_img = gtk::Image::new();
        big_img.set_from_file(big_path);

        let small_thumbnail = resize_dynamic_image_dimension(big_thumbnail, (SMALL_PREVIEW_SIZE, SMALL_PREVIEW_SIZE), &FilterType::Triangle);
        let small_path = get_image_path_temporary("roman", 1, "jpg");
        let _ = small_thumbnail.save(&small_path);
        let small_img = gtk::Image::new();
        small_img.set_from_file(small_path);

        cache_all_images.push((full_path, name, big_img, small_img));
    }
    cache_all_images
}

/// Takes info about current items in groups like path
fn get_all_path(model: &TreeModel, current_iter: &TreeIter, column_color: i32, column_path: i32, column_name: i32) -> Vec<(String, String, TreeIter)> {
    let used_iter = current_iter.clone();

    assert_eq!(model.value(&used_iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR);
    let using_reference = !model.value(&used_iter, column_path).get::<String>().unwrap().is_empty();

    let mut returned_vector = Vec::new();

    if using_reference {
        let name = model.value(&used_iter, column_name).get::<String>().unwrap();
        let path = model.value(&used_iter, column_path).get::<String>().unwrap();

        let full_name = get_full_name_from_path_name(&path, &name);

        returned_vector.push((full_name, name, used_iter.clone()));
    }

    if !model.iter_next(&used_iter) {
        panic!("Found only header!");
    }

    loop {
        let name = model.value(&used_iter, column_name).get::<String>().unwrap();
        let path = model.value(&used_iter, column_path).get::<String>().unwrap();

        let full_name = get_full_name_from_path_name(&path, &name);

        returned_vector.push((full_name, name, used_iter.clone()));

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

/// Moves iterator to previous/next header
fn move_iter(model: &gtk::TreeModel, tree_iter: &TreeIter, column_color: i32, go_next: bool) -> TreeIter {
    assert_eq!(model.value(tree_iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR);

    if go_next {
        if !model.iter_next(tree_iter) {
            panic!("Found only header!");
        }
    } else {
        if !model.iter_previous(tree_iter) {
            panic!("Found only header!");
        }
    }

    loop {
        if go_next {
            if !model.iter_next(tree_iter) {
                break;
            }
        } else {
            if !model.iter_previous(tree_iter) {
                break;
            }
        }

        let color = model.value(tree_iter, column_color).get::<String>().unwrap();

        if color == HEADER_ROW_COLOR {
            break;
        }
    }
    tree_iter.clone()
}

/// Populate bottom Scrolled View with small thumbnails
fn populate_similar_scrolled_view(scrolled_window: &ScrolledWindow, image_cache: &[(String, String, Image, Image)]) {
    if let Some(child) = scrolled_window.child() {
        scrolled_window.remove(&child);
    };
    scrolled_window.set_propagate_natural_height(true);

    let all_gtk_box = gtk::Box::new(Orientation::Horizontal, 5);
    for (number, (_path, _name, _big_thumbnail, small_thumbnail)) in image_cache.iter().enumerate() {
        let small_box = gtk::Box::new(Orientation::Vertical, 5);

        let smaller_box = gtk::Box::new(Orientation::Horizontal, 5);

        let button_left = gtk::Button::builder().label("L").build();
        let label = gtk::Label::builder().label(&(number + 1).to_string()).build();
        let button_right = gtk::Button::builder().label("R").build();

        if number == 0 || number == 1 {
            button_left.set_sensitive(false);
            button_right.set_sensitive(false);
        }

        smaller_box.add(&button_left);
        smaller_box.add(&label);
        smaller_box.add(&button_right);

        small_box.add(&smaller_box);
        small_box.add(small_thumbnail);

        all_gtk_box.add(&small_box);
    }

    all_gtk_box.show_all();
    scrolled_window.add(&all_gtk_box);
}
