use crate::flg;
use czkawka_core::common::get_dynamic_image_from_raw_image;
use czkawka_core::similar_images::RAW_IMAGE_EXTENSIONS;
use gtk::prelude::*;
use gtk::{CheckButton, Image, ListStore, Orientation, ScrolledWindow, TreeIter, TreeModel, TreePath, TreeSelection};
use image::imageops::FilterType;
use image::DynamicImage;
use std::cell::RefCell;
use std::rc::Rc;

use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::{
    count_number_of_groups, get_full_name_from_path_name, get_image_path_temporary, get_max_file_name, resize_dynamic_image_dimension, NotebookObject, HEADER_ROW_COLOR,
    NOTEBOOKS_INFOS,
};
use crate::localizer_core::generate_translation_hashmap;

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

    let check_button_left_preview_text = gui_data.compare_images.check_button_left_preview_text.clone();
    let check_button_right_preview_text = gui_data.compare_images.check_button_right_preview_text.clone();

    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_current_path = gui_data.compare_images.shared_current_path.clone();
    let shared_image_cache = gui_data.compare_images.shared_image_cache.clone();
    let shared_using_for_preview = gui_data.compare_images.shared_using_for_preview.clone();

    let image_compare_left = gui_data.compare_images.image_compare_left.clone();
    let image_compare_right = gui_data.compare_images.image_compare_right.clone();

    button_compare.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];
        let model = tree_view.model().unwrap();

        let group_number = count_number_of_groups(tree_view, nb_object.column_color.unwrap());

        if group_number == 0 {
            return;
        }

        // Check selected items
        let (current_group, tree_path) = get_current_group_and_iter_from_selection(&model, tree_view.selection(), nb_object.column_color.unwrap());

        *shared_current_of_groups.borrow_mut() = current_group;
        *shared_numbers_of_groups.borrow_mut() = group_number;

        populate_groups_at_start(
            nb_object,
            &model,
            shared_current_path.clone(),
            tree_path,
            &image_compare_left,
            &image_compare_right,
            current_group,
            group_number,
            &check_button_left_preview_text,
            &check_button_right_preview_text,
            &scrolled_window_compare_choose_images,
            &label_group_info,
            shared_image_cache.clone(),
            shared_using_for_preview.clone(),
            &button_go_previous_compare_group,
            &button_go_next_compare_group,
        );

        window_compare.show();
    });

    let shared_image_cache = gui_data.compare_images.shared_image_cache.clone();
    let shared_current_path = gui_data.compare_images.shared_current_path.clone();
    let shared_using_for_preview = gui_data.compare_images.shared_using_for_preview.clone();
    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let window_compare = gui_data.compare_images.window_compare.clone();
    let image_compare_left = gui_data.compare_images.image_compare_left.clone();
    let image_compare_right = gui_data.compare_images.image_compare_right.clone();
    window_compare.connect_delete_event(move |window_compare, _| {
        window_compare.hide();
        *shared_image_cache.borrow_mut() = Vec::new();
        *shared_current_path.borrow_mut() = None;
        *shared_current_of_groups.borrow_mut() = 0;
        *shared_numbers_of_groups.borrow_mut() = 0;
        *shared_using_for_preview.borrow_mut() = (None, None);
        image_compare_left.set_from_pixbuf(None);
        image_compare_right.set_from_pixbuf(None);
        gtk::Inhibit(true)
    });

    let button_go_previous_compare_group = gui_data.compare_images.button_go_previous_compare_group.clone();
    let button_go_next_compare_group = gui_data.compare_images.button_go_next_compare_group.clone();
    let label_group_info = gui_data.compare_images.label_group_info.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    let scrolled_window_compare_choose_images = gui_data.compare_images.scrolled_window_compare_choose_images.clone();

    let check_button_left_preview_text = gui_data.compare_images.check_button_left_preview_text.clone();
    let check_button_right_preview_text = gui_data.compare_images.check_button_right_preview_text.clone();

    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let shared_current_path = gui_data.compare_images.shared_current_path.clone();
    let shared_image_cache = gui_data.compare_images.shared_image_cache.clone();
    let shared_using_for_preview = gui_data.compare_images.shared_using_for_preview.clone();

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

        let tree_iter = move_iter(&model, shared_current_path.borrow().as_ref().unwrap(), nb_object.column_color.unwrap(), false);

        populate_groups_at_start(
            nb_object,
            &model,
            shared_current_path.clone(),
            tree_iter,
            &image_compare_left,
            &image_compare_right,
            current_group,
            group_number,
            &check_button_left_preview_text,
            &check_button_right_preview_text,
            &scrolled_window_compare_choose_images,
            &label_group_info,
            shared_image_cache.clone(),
            shared_using_for_preview.clone(),
            button_go_previous_compare_group,
            &button_go_next_compare_group,
        );
    });

    let button_go_previous_compare_group = gui_data.compare_images.button_go_previous_compare_group.clone();
    let button_go_next_compare_group = gui_data.compare_images.button_go_next_compare_group.clone();
    let label_group_info = gui_data.compare_images.label_group_info.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    let scrolled_window_compare_choose_images = gui_data.compare_images.scrolled_window_compare_choose_images.clone();

    let check_button_left_preview_text = gui_data.compare_images.check_button_left_preview_text.clone();
    let check_button_right_preview_text = gui_data.compare_images.check_button_right_preview_text.clone();

    let shared_current_of_groups = gui_data.compare_images.shared_current_of_groups.clone();
    let shared_numbers_of_groups = gui_data.compare_images.shared_numbers_of_groups.clone();
    let shared_current_path = gui_data.compare_images.shared_current_path.clone();
    let shared_image_cache = gui_data.compare_images.shared_image_cache.clone();
    let shared_using_for_preview = gui_data.compare_images.shared_using_for_preview.clone();

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

        let tree_path = move_iter(&model, shared_current_path.borrow().as_ref().unwrap(), nb_object.column_color.unwrap(), true);

        populate_groups_at_start(
            nb_object,
            &model,
            shared_current_path.clone(),
            tree_path,
            &image_compare_left,
            &image_compare_right,
            current_group,
            group_number,
            &check_button_left_preview_text,
            &check_button_right_preview_text,
            &scrolled_window_compare_choose_images,
            &label_group_info,
            shared_image_cache.clone(),
            shared_using_for_preview.clone(),
            &button_go_previous_compare_group,
            button_go_next_compare_group,
        );
    });

    let check_button_left_preview_text = gui_data.compare_images.check_button_left_preview_text.clone();
    let shared_using_for_preview = gui_data.compare_images.shared_using_for_preview.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let shared_current_path = gui_data.compare_images.shared_current_path.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    check_button_left_preview_text.connect_clicked(move |check_button_left_preview_text| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];
        let model = tree_view.model().unwrap().downcast::<ListStore>().unwrap();

        let main_tree_path = shared_current_path.borrow().as_ref().unwrap().clone();
        let this_tree_path = shared_using_for_preview.borrow().0.clone().unwrap();
        if main_tree_path == this_tree_path {
            return; // Selected header, so we don't need to select result in treeview
                    // TODO this should be handled by disabling entirely check box
        }

        let is_active = check_button_left_preview_text.is_active();
        model.set_value(&model.iter(&this_tree_path).unwrap(), nb_object.column_selection as u32, &is_active.to_value());
    });

    let check_button_right_preview_text = gui_data.compare_images.check_button_right_preview_text.clone();
    let shared_using_for_preview = gui_data.compare_images.shared_using_for_preview.clone();
    let shared_current_path = gui_data.compare_images.shared_current_path.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    check_button_right_preview_text.connect_clicked(move |check_button_right_preview_text| {
        let nb_number = notebook_main.current_page().unwrap();
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFOS[nb_number as usize];
        let model = tree_view.model().unwrap().downcast::<ListStore>().unwrap();

        let main_tree_path = shared_current_path.borrow().as_ref().unwrap().clone();
        let this_tree_path = shared_using_for_preview.borrow().1.clone().unwrap();
        if main_tree_path == this_tree_path {
            return; // Selected header, so we don't need to select result in treeview
                    // TODO this should be handled by disabling entirely check box
        }

        let is_active = check_button_right_preview_text.is_active();
        model.set_value(&model.iter(&this_tree_path).unwrap(), nb_object.column_selection as u32, &is_active.to_value());
    });
}

/// Populate all parameters for current group, it is used at start and when changing groups
fn populate_groups_at_start(
    nb_object: &NotebookObject,
    model: &TreeModel,
    shared_current_path: Rc<RefCell<Option<TreePath>>>,
    tree_path: TreePath,
    image_compare_left: &gtk::Image,
    image_compare_right: &gtk::Image,
    current_group: u32,
    group_number: u32,
    check_button_left_preview_text: &gtk::CheckButton,
    check_button_right_preview_text: &gtk::CheckButton,
    scrolled_window_compare_choose_images: &gtk::ScrolledWindow,
    label_group_info: &gtk::Label,
    shared_image_cache: Rc<RefCell<Vec<(String, String, gtk::Image, gtk::Image, gtk::TreePath)>>>,
    shared_using_for_preview: Rc<RefCell<(Option<TreePath>, Option<TreePath>)>>,
    button_go_previous_compare_group: &gtk::Button,
    button_go_next_compare_group: &gtk::Button,
) {
    if current_group == 1 {
        button_go_previous_compare_group.set_sensitive(false);
    } else {
        button_go_previous_compare_group.set_sensitive(true);
    }
    if current_group == group_number {
        button_go_next_compare_group.set_sensitive(false);
    } else {
        button_go_next_compare_group.set_sensitive(true);
    }

    let all_vec = get_all_path(model, &tree_path, nb_object.column_color.unwrap(), nb_object.column_path, nb_object.column_name);
    *shared_current_path.borrow_mut() = Some(tree_path);

    let cache_all_images = generate_cache_for_results(all_vec);

    // This is safe, because cache have at least 2 results
    image_compare_left.set_from_pixbuf(cache_all_images[0].2.pixbuf().as_ref());
    image_compare_right.set_from_pixbuf(cache_all_images[1].2.pixbuf().as_ref());

    *shared_using_for_preview.borrow_mut() = (Some(cache_all_images[0].4.clone()), Some(cache_all_images[1].4.clone()));

    check_button_left_preview_text.set_label(&format!("1. {}", get_max_file_name(&cache_all_images[0].0, 70)));
    check_button_right_preview_text.set_label(&format!("2. {}", get_max_file_name(&cache_all_images[1].0, 70)));

    label_group_info.set_text(
        flg!(
            "compare_groups_number",
            generate_translation_hashmap(vec![
                ("current_group", current_group.to_string()),
                ("all_groups", group_number.to_string()),
                ("images_in_group", cache_all_images.len().to_string())
            ])
        )
        .as_str(),
    );

    populate_similar_scrolled_view(
        scrolled_window_compare_choose_images,
        &cache_all_images,
        image_compare_left,
        image_compare_right,
        shared_using_for_preview.clone(),
        shared_image_cache.clone(),
        check_button_left_preview_text,
        check_button_right_preview_text,
        model,
        nb_object.column_selection,
    );

    *shared_image_cache.borrow_mut() = cache_all_images.clone();

    let mut found = false;
    for i in scrolled_window_compare_choose_images.child().unwrap().downcast::<gtk::Viewport>().unwrap().children() {
        if i.widget_name() == "all_box" {
            let gtk_box = i.downcast::<gtk::Box>().unwrap();
            update_bottom_buttons(&gtk_box, shared_using_for_preview, shared_image_cache);
            found = true;
            break;
        }
    }
    assert!(found);

    let is_active = model.value(&model.iter(&cache_all_images[0].4).unwrap(), nb_object.column_selection).get::<bool>().unwrap();
    check_button_left_preview_text.set_active(is_active);
    let is_active = model.value(&model.iter(&cache_all_images[1].4).unwrap(), nb_object.column_selection).get::<bool>().unwrap();
    check_button_right_preview_text.set_active(is_active);
}

/// Generate images which will be used later as preview images without needing to open them again and again
fn generate_cache_for_results(vector_with_path: Vec<(String, String, gtk::TreePath)>) -> Vec<(String, String, gtk::Image, gtk::Image, gtk::TreePath)> {
    // TODO use here threads,
    // For now threads cannot be used because Image and TreeIter cannot be used in threads
    let mut cache_all_images = Vec::new();
    for (full_path, name, tree_path) in vector_with_path {
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

        cache_all_images.push((full_path, name, big_img, small_img, tree_path));
    }
    cache_all_images
}

/// Takes info about current items in groups like path
fn get_all_path(model: &TreeModel, current_path: &TreePath, column_color: i32, column_path: i32, column_name: i32) -> Vec<(String, String, gtk::TreePath)> {
    let used_iter = model.iter(current_path).unwrap();

    assert_eq!(model.value(&used_iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR);
    let using_reference = !model.value(&used_iter, column_path).get::<String>().unwrap().is_empty();

    let mut returned_vector = Vec::new();

    if using_reference {
        let name = model.value(&used_iter, column_name).get::<String>().unwrap();
        let path = model.value(&used_iter, column_path).get::<String>().unwrap();

        let full_name = get_full_name_from_path_name(&path, &name);

        returned_vector.push((full_name, name, model.path(&used_iter).unwrap()));
    }

    if !model.iter_next(&used_iter) {
        panic!("Found only header!");
    }

    loop {
        let name = model.value(&used_iter, column_name).get::<String>().unwrap();
        let path = model.value(&used_iter, column_path).get::<String>().unwrap();

        let full_name = get_full_name_from_path_name(&path, &name);

        returned_vector.push((full_name, name, model.path(&used_iter).unwrap()));

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
fn move_iter(model: &gtk::TreeModel, tree_path: &TreePath, column_color: i32, go_next: bool) -> TreePath {
    let tree_iter = model.iter(tree_path).unwrap();
    assert_eq!(model.value(&tree_iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR);

    if go_next {
        if !model.iter_next(&tree_iter) {
            panic!("Found only header!");
        }
    } else {
        if !model.iter_previous(&tree_iter) {
            panic!("Found only header!");
        }
    }

    loop {
        if go_next {
            if !model.iter_next(&tree_iter) {
                break;
            }
        } else {
            if !model.iter_previous(&tree_iter) {
                break;
            }
        }

        let color = model.value(&tree_iter, column_color).get::<String>().unwrap();

        if color == HEADER_ROW_COLOR {
            break;
        }
    }
    model.path(&tree_iter).unwrap()
}

/// Populate bottom Scrolled View with small thumbnails
fn populate_similar_scrolled_view(
    scrolled_window: &ScrolledWindow,
    image_cache: &[(String, String, Image, Image, TreePath)],
    image_compare_left: &Image,
    image_compare_right: &Image,
    shared_using_for_preview: Rc<RefCell<(Option<TreePath>, Option<TreePath>)>>,
    shared_image_cache: Rc<RefCell<Vec<(String, String, gtk::Image, gtk::Image, gtk::TreePath)>>>,
    check_button_left_preview_text: &CheckButton,
    check_button_right_preview_text: &CheckButton,
    model: &TreeModel,
    column_selection: i32,
) {
    if let Some(child) = scrolled_window.child() {
        scrolled_window.remove(&child);
    };
    scrolled_window.set_propagate_natural_height(true);

    let all_gtk_box = gtk::Box::new(Orientation::Horizontal, 5);
    all_gtk_box.set_widget_name("all_box");

    for (number, (path, _name, big_thumbnail, small_thumbnail, tree_path)) in image_cache.iter().enumerate() {
        let small_box = gtk::Box::new(Orientation::Vertical, 3);

        let smaller_box = gtk::Box::new(Orientation::Horizontal, 2);

        let button_left = gtk::Button::builder().label(&flg!("compare_move_left_button")).build();
        let label = gtk::Label::builder().label(&(number + 1).to_string()).build();
        let button_right = gtk::Button::builder().label(&flg!("compare_move_right_button")).build();

        let image_compare_left = image_compare_left.clone();
        let image_compare_right = image_compare_right.clone();

        let big_thumbnail_clone = big_thumbnail.clone();
        let tree_path_clone = tree_path.clone();
        let all_gtk_box_clone = all_gtk_box.clone();
        let shared_using_for_preview_clone = shared_using_for_preview.clone();
        let shared_image_cache_clone = shared_image_cache.clone();
        let check_button_left_preview_text_clone = check_button_left_preview_text.clone();
        let model_clone = model.clone();
        let path_clone = path.clone();

        button_left.connect_clicked(move |_button_left| {
            shared_using_for_preview_clone.borrow_mut().0 = Some(tree_path_clone.clone());
            update_bottom_buttons(&all_gtk_box_clone, shared_using_for_preview_clone.clone(), shared_image_cache_clone.clone());
            image_compare_left.set_from_pixbuf(big_thumbnail_clone.pixbuf().as_ref());

            let is_active = model_clone.value(&model_clone.iter(&tree_path_clone).unwrap(), column_selection).get::<bool>().unwrap();
            check_button_left_preview_text_clone.set_active(is_active);
            check_button_left_preview_text_clone.set_label(&format!("{}. {}", number + 1, get_max_file_name(&path_clone, 70)));
        });

        let big_thumbnail_clone = big_thumbnail.clone();
        let tree_path_clone = tree_path.clone();
        let all_gtk_box_clone = all_gtk_box.clone();
        let shared_using_for_preview_clone = shared_using_for_preview.clone();
        let shared_image_cache_clone = shared_image_cache.clone();
        let check_button_right_preview_text_clone = check_button_right_preview_text.clone();
        let model_clone = model.clone();
        let path_clone = path.clone();

        button_right.connect_clicked(move |_button_right| {
            shared_using_for_preview_clone.borrow_mut().1 = Some(tree_path_clone.clone());
            update_bottom_buttons(&all_gtk_box_clone, shared_using_for_preview_clone.clone(), shared_image_cache_clone.clone());
            image_compare_right.set_from_pixbuf(big_thumbnail_clone.pixbuf().as_ref());

            let is_active = model_clone.value(&model_clone.iter(&tree_path_clone).unwrap(), column_selection).get::<bool>().unwrap();
            check_button_right_preview_text_clone.set_active(is_active);
            check_button_right_preview_text_clone.set_label(&format!("{}. {}", number + 1, get_max_file_name(&path_clone, 70)));
        });

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

/// Disables/Enables L/R buttons at the bottom scrolled view
fn update_bottom_buttons(
    all_gtk_box: &gtk::Box,
    shared_using_for_preview: Rc<RefCell<(Option<TreePath>, Option<TreePath>)>>,
    image_cache: Rc<RefCell<Vec<(String, String, Image, Image, TreePath)>>>,
) {
    let left_tree_view = (*shared_using_for_preview.borrow()).0.clone().unwrap();
    let right_tree_view = (*shared_using_for_preview.borrow()).1.clone().unwrap();

    for (number, i) in all_gtk_box.children().into_iter().enumerate() {
        let cache_tree_path = (*image_cache.borrow())[number].4.clone();
        let is_chosen = cache_tree_path != right_tree_view && cache_tree_path != left_tree_view;

        let bx = i.downcast::<gtk::Box>().unwrap();
        let smaller_bx = bx.children()[0].clone().downcast::<gtk::Box>().unwrap();
        for items in smaller_bx.children() {
            if let Ok(btn) = items.downcast::<gtk::Button>() {
                btn.set_sensitive(is_chosen);
            }
        }
    }
}

fn get_current_group_and_iter_from_selection(model: &TreeModel, selection: TreeSelection, column_color: i32) -> (u32, TreePath) {
    let mut current_group = 1;
    let mut possible_group = 1;
    let mut header_clone: TreeIter;
    let mut possible_header: TreeIter;

    let selected_records = selection.selected_rows().0;

    let iter = model.iter_first().unwrap(); // Checking that treeview is not empty should be done before
    header_clone = iter.clone(); // if nothing selected, use first group
    possible_header = iter.clone(); // if nothing selected, use first group
    assert_eq!(model.value(&iter, column_color).get::<String>().unwrap(), HEADER_ROW_COLOR); // First element should be header

    if !selected_records.is_empty() {
        let first_selected_record = selected_records[0].clone();
        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if model.value(&iter, column_color).get::<String>().unwrap() == HEADER_ROW_COLOR {
                possible_group += 1;
                possible_header = iter.clone();
            }

            if model.path(&iter).unwrap() == first_selected_record {
                header_clone = possible_header.clone();
                current_group = possible_group;
            }
        }
    }

    (current_group, model.path(&header_clone).unwrap())
}
