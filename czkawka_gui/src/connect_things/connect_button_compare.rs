use std::cell::RefCell;
use std::rc::Rc;

use gdk4::gdk_pixbuf::{InterpType, Pixbuf};
use gtk4::prelude::*;
use gtk4::{Align, CheckButton, Image, ListStore, Orientation, ScrolledWindow, TreeIter, TreeModel, TreePath, TreeSelection, Widget};
use image::DynamicImage;

#[cfg(feature = "heif")]
use czkawka_core::common::get_dynamic_image_from_heic;
use czkawka_core::common::HEIC_EXTENSIONS;

use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::{
    count_number_of_groups, get_all_direct_children, get_full_name_from_path_name, get_max_file_name, get_pixbuf_from_dynamic_image, resize_pixbuf_dimension,
};
use crate::notebook_info::{NotebookObject, NOTEBOOKS_INFO};

const BIG_PREVIEW_SIZE: i32 = 600;
const SMALL_PREVIEW_SIZE: i32 = 130;

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

    window_compare.set_default_size(700, 700);

    button_compare.connect_clicked(move |_| {
        let nb_number = notebook_main.current_page().expect("Current page not set");
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];
        let model = tree_view.model().expect("Missing tree_view model");

        let group_number = count_number_of_groups(tree_view, nb_object.column_header.expect("Missing column_header"));

        if group_number == 0 {
            return;
        }

        // Check selected items
        let (current_group, tree_path) = get_current_group_and_iter_from_selection(&model, &tree_view.selection(), nb_object.column_header.expect("Missing column_header"));

        *shared_current_of_groups.borrow_mut() = current_group;
        *shared_numbers_of_groups.borrow_mut() = group_number;

        populate_groups_at_start(
            nb_object,
            &model,
            &shared_current_path,
            tree_path,
            &image_compare_left,
            &image_compare_right,
            current_group,
            group_number,
            &check_button_left_preview_text,
            &check_button_right_preview_text,
            &scrolled_window_compare_choose_images,
            &label_group_info,
            &shared_image_cache,
            &shared_using_for_preview,
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
    window_compare.connect_close_request(move |window_compare| {
        window_compare.hide();
        *shared_image_cache.borrow_mut() = Vec::new();
        *shared_current_path.borrow_mut() = None;
        *shared_current_of_groups.borrow_mut() = 0;
        *shared_numbers_of_groups.borrow_mut() = 0;
        *shared_using_for_preview.borrow_mut() = (None, None);
        image_compare_left.set_from_pixbuf(None);
        image_compare_right.set_from_pixbuf(None);
        glib::Propagation::Stop
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
        let nb_number = notebook_main.current_page().expect("Current page not set");
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];
        let model = tree_view.model().expect("Missing tree_view model");

        *shared_current_of_groups.borrow_mut() -= 1;

        let current_group = *shared_current_of_groups.borrow();
        let group_number = *shared_numbers_of_groups.borrow();

        let tree_path = move_iter(
            &model,
            shared_current_path.borrow().as_ref().expect("Missing current path"),
            nb_object.column_header.expect("Missing column_header"),
            false,
        );

        populate_groups_at_start(
            nb_object,
            &model,
            &shared_current_path,
            tree_path,
            &image_compare_left,
            &image_compare_right,
            current_group,
            group_number,
            &check_button_left_preview_text,
            &check_button_right_preview_text,
            &scrolled_window_compare_choose_images,
            &label_group_info,
            &shared_image_cache,
            &shared_using_for_preview,
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
        let nb_number = notebook_main.current_page().expect("Current page not set");
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];
        let model = tree_view.model().expect("Missing tree_view model");

        *shared_current_of_groups.borrow_mut() += 1;

        let current_group = *shared_current_of_groups.borrow();
        let group_number = *shared_numbers_of_groups.borrow();

        let tree_path = move_iter(
            &model,
            shared_current_path.borrow().as_ref().expect("Missing current path"),
            nb_object.column_header.expect("Missing column_header"),
            true,
        );

        populate_groups_at_start(
            nb_object,
            &model,
            &shared_current_path,
            tree_path,
            &image_compare_left,
            &image_compare_right,
            current_group,
            group_number,
            &check_button_left_preview_text,
            &check_button_right_preview_text,
            &scrolled_window_compare_choose_images,
            &label_group_info,
            &shared_image_cache,
            &shared_using_for_preview,
            &button_go_previous_compare_group,
            button_go_next_compare_group,
        );
    });

    let check_button_left_preview_text = gui_data.compare_images.check_button_left_preview_text.clone();
    let shared_using_for_preview = gui_data.compare_images.shared_using_for_preview.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let shared_current_path = gui_data.compare_images.shared_current_path.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();
    check_button_left_preview_text.connect_toggled(move |check_button_left_preview_text| {
        let nb_number = notebook_main.current_page().expect("Current page not set");
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];
        let model = tree_view
            .model()
            .expect("Missing tree_view model")
            .downcast::<ListStore>()
            .expect("Failed to downcast to ListStore");

        let main_tree_path = shared_current_path.borrow().as_ref().expect("Missing current path").clone();
        let this_tree_path = shared_using_for_preview.borrow().0.clone().expect("Missing left preview path");
        if main_tree_path == this_tree_path {
            return; // Selected header, so we don't need to select result in treeview
                    // TODO this should be handled by disabling entirely check box
        }

        let is_active = check_button_left_preview_text.is_active();
        model.set_value(
            &model.iter(&this_tree_path).expect("Using invalid tree_path"),
            nb_object.column_selection as u32,
            &is_active.to_value(),
        );
    });

    let check_button_right_preview_text = gui_data.compare_images.check_button_right_preview_text.clone();
    let shared_using_for_preview = gui_data.compare_images.shared_using_for_preview.clone();
    let shared_current_path = gui_data.compare_images.shared_current_path.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    let main_tree_views = gui_data.main_notebook.get_main_tree_views();

    check_button_right_preview_text.connect_toggled(move |check_button_right_preview_text| {
        let nb_number = notebook_main.current_page().expect("Current page not set");
        let tree_view = &main_tree_views[nb_number as usize];
        let nb_object = &NOTEBOOKS_INFO[nb_number as usize];
        let model = tree_view
            .model()
            .expect("Missing tree_view model")
            .downcast::<ListStore>()
            .expect("Failed to downcast to ListStore");

        let main_tree_path = shared_current_path.borrow().as_ref().expect("Missing current path").clone();
        let this_tree_path = shared_using_for_preview.borrow().1.clone().expect("Missing right preview path");
        if main_tree_path == this_tree_path {
            return; // Selected header, so we don't need to select result in treeview
                    // TODO this should be handled by disabling entirely check box
        }

        let is_active = check_button_right_preview_text.is_active();
        model.set_value(
            &model.iter(&this_tree_path).expect("Using invalid tree_path"),
            nb_object.column_selection as u32,
            &is_active.to_value(),
        );
    });
}

fn populate_groups_at_start(
    nb_object: &NotebookObject,
    model: &TreeModel,
    shared_current_path: &Rc<RefCell<Option<TreePath>>>,
    tree_path: TreePath,
    image_compare_left: &Image,
    image_compare_right: &Image,
    current_group: u32,
    group_number: u32,
    check_button_left_preview_text: &CheckButton,
    check_button_right_preview_text: &CheckButton,
    scrolled_window_compare_choose_images: &ScrolledWindow,
    label_group_info: &gtk4::Label,
    shared_image_cache: &Rc<RefCell<Vec<(String, String, Image, Image, TreePath)>>>,
    shared_using_for_preview: &Rc<RefCell<(Option<TreePath>, Option<TreePath>)>>,
    button_go_previous_compare_group: &gtk4::Button,
    button_go_next_compare_group: &gtk4::Button,
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

    let all_vec = get_all_path(
        model,
        &tree_path,
        nb_object.column_header.expect("Missing column_header"),
        nb_object.column_path,
        nb_object.column_name,
    );
    *shared_current_path.borrow_mut() = Some(tree_path);

    let cache_all_images = generate_cache_for_results(all_vec);

    // This is safe, because cache have at least 2 results
    image_compare_left.set_paintable(cache_all_images[0].2.paintable().as_ref());
    image_compare_right.set_paintable(cache_all_images[1].2.paintable().as_ref());

    *shared_using_for_preview.borrow_mut() = (Some(cache_all_images[0].4.clone()), Some(cache_all_images[1].4.clone()));

    check_button_left_preview_text.set_label(Some(&format!("1. {}", get_max_file_name(&cache_all_images[0].0, 60))));
    check_button_right_preview_text.set_label(Some(&format!("2. {}", get_max_file_name(&cache_all_images[1].0, 60))));

    label_group_info.set_text(
        flg!(
            "compare_groups_number",
            current_group = current_group,
            all_groups = group_number,
            images_in_group = cache_all_images.len()
        )
        .as_str(),
    );

    populate_similar_scrolled_view(
        scrolled_window_compare_choose_images,
        &cache_all_images,
        image_compare_left,
        image_compare_right,
        shared_using_for_preview,
        shared_image_cache,
        check_button_left_preview_text,
        check_button_right_preview_text,
        model,
        nb_object.column_selection,
    );

    *shared_image_cache.borrow_mut() = cache_all_images.clone();

    let mut found = false;
    for i in get_all_direct_children(
        &scrolled_window_compare_choose_images
            .child()
            .expect("Failed to get child of scrolled_window_compare_choose_images")
            .downcast::<gtk4::Viewport>()
            .expect("Failed to downcast to Viewport"),
    ) {
        if i.widget_name() == "all_box" {
            let gtk_box = i.downcast::<gtk4::Box>().expect("Failed to downcast to Box");
            update_bottom_buttons(&gtk_box, shared_using_for_preview, shared_image_cache);
            found = true;
            break;
        }
    }
    assert!(found);

    let is_active = model.get::<bool>(&model.iter(&cache_all_images[0].4).expect("Using invalid tree_path"), nb_object.column_selection);
    check_button_left_preview_text.set_active(is_active);
    let is_active = model.get::<bool>(&model.iter(&cache_all_images[1].4).expect("Using invalid tree_path"), nb_object.column_selection);
    check_button_right_preview_text.set_active(is_active);
}

fn generate_cache_for_results(vector_with_path: Vec<(String, String, TreePath)>) -> Vec<(String, String, Image, Image, TreePath)> {
    // TODO use here threads,
    // For now threads cannot be used because Image and TreeIter cannot be used in threads
    let mut cache_all_images = Vec::new();
    for (full_path, name, tree_path) in vector_with_path {
        let small_img = Image::new();
        let big_img = Image::new();

        let mut pixbuf = get_pixbuf_from_dynamic_image(&DynamicImage::new_rgb8(1, 1)).expect("Failed to create pixbuf");
        let extension_lowercase = full_path.split('.').last().map(str::to_lowercase);
        let is_heic = match extension_lowercase {
            Some(extension) => HEIC_EXTENSIONS.iter().any(|e| e == &extension),
            None => false,
        };

        if is_heic {
            #[allow(clippy::never_loop)]
            'czystka: loop {
                #[cfg(feature = "heif")]
                if is_heic {
                    match get_dynamic_image_from_heic(&full_path) {
                        Ok(t) => {
                            match get_pixbuf_from_dynamic_image(&t) {
                                Ok(t) => {
                                    pixbuf = t;
                                }
                                Err(e) => {
                                    println!("Failed to open image {full_path}, reason {e}");
                                }
                            };
                        }
                        Err(e) => {
                            println!("Failed to open image {full_path}, reason {e}");
                        }
                    };
                    break 'czystka;
                }
                break 'czystka;
            }
        } else {
            match Pixbuf::from_file(&full_path) {
                Ok(t) => {
                    pixbuf = t;
                }
                Err(e) => {
                    println!("Failed to open image {full_path}, reason {e}");
                }
            };
        }

        #[allow(clippy::never_loop)]
        loop {
            let Some(pixbuf_big) = resize_pixbuf_dimension(&pixbuf, (BIG_PREVIEW_SIZE, BIG_PREVIEW_SIZE), InterpType::Bilinear) else {
                println!("Failed to resize image {full_path}.");
                break;
            };
            let Some(pixbuf_small) = resize_pixbuf_dimension(&pixbuf_big, (SMALL_PREVIEW_SIZE, SMALL_PREVIEW_SIZE), InterpType::Bilinear) else {
                println!("Failed to resize image {full_path}.");
                break;
            };

            big_img.set_from_pixbuf(Some(&pixbuf_big));
            small_img.set_from_pixbuf(Some(&pixbuf_small));
            break;
        }

        cache_all_images.push((full_path, name, big_img, small_img, tree_path));
    }
    cache_all_images
}

fn get_all_path(model: &TreeModel, current_path: &TreePath, column_header: i32, column_path: i32, column_name: i32) -> Vec<(String, String, TreePath)> {
    let used_iter = model.iter(current_path).expect("Using invalid tree_path");

    assert!(model.get::<bool>(&used_iter, column_header));
    let using_reference = !model.get::<String>(&used_iter, column_path).is_empty();

    let mut returned_vector = Vec::new();

    if using_reference {
        let name = model.get::<String>(&used_iter, column_name);
        let path = model.get::<String>(&used_iter, column_path);

        let full_name = get_full_name_from_path_name(&path, &name);

        returned_vector.push((full_name, name, model.path(&used_iter)));
    }

    assert!(model.iter_next(&used_iter), "Found only header!");

    loop {
        let name = model.get::<String>(&used_iter, column_name);
        let path = model.get::<String>(&used_iter, column_path);

        let full_name = get_full_name_from_path_name(&path, &name);

        returned_vector.push((full_name, name, model.path(&used_iter)));

        if !model.iter_next(&used_iter) {
            break;
        }

        if model.get::<bool>(&used_iter, column_header) {
            break;
        }
    }

    assert!(returned_vector.len() > 1);

    returned_vector
}

fn move_iter(model: &TreeModel, tree_path: &TreePath, column_header: i32, go_next: bool) -> TreePath {
    let tree_iter = model.iter(tree_path).expect("Using invalid tree_path");

    assert!(model.get::<bool>(&tree_iter, column_header));

    if go_next {
        assert!(model.iter_next(&tree_iter), "Found only header!");
    } else {
        assert!(model.iter_previous(&tree_iter), "Found only header!");
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

        if model.get::<bool>(&tree_iter, column_header) {
            break;
        }
    }
    model.path(&tree_iter)
}

fn populate_similar_scrolled_view(
    scrolled_window: &ScrolledWindow,
    image_cache: &[(String, String, Image, Image, TreePath)],
    image_compare_left: &Image,
    image_compare_right: &Image,
    shared_using_for_preview: &Rc<RefCell<(Option<TreePath>, Option<TreePath>)>>,
    shared_image_cache: &Rc<RefCell<Vec<(String, String, Image, Image, TreePath)>>>,
    check_button_left_preview_text: &CheckButton,
    check_button_right_preview_text: &CheckButton,
    model: &TreeModel,
    column_selection: i32,
) {
    scrolled_window.set_child(None::<&Widget>);

    let all_gtk_box = gtk4::Box::new(Orientation::Horizontal, 5);
    all_gtk_box.set_widget_name("all_box");
    all_gtk_box.set_halign(Align::Fill);
    all_gtk_box.set_valign(Align::Fill);

    for (number, (path, _name, big_thumbnail, small_thumbnail, tree_path)) in image_cache.iter().enumerate() {
        let small_box = gtk4::Box::new(Orientation::Vertical, 3);

        let smaller_box = gtk4::Box::new(Orientation::Horizontal, 2);

        let button_left = gtk4::Button::builder().label(&flg!("compare_move_left_button")).build();
        let label = gtk4::Label::builder().label((number + 1).to_string()).build();
        let button_right = gtk4::Button::builder().label(&flg!("compare_move_right_button")).build();

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
            update_bottom_buttons(&all_gtk_box_clone, &shared_using_for_preview_clone, &shared_image_cache_clone);
            image_compare_left.set_paintable(big_thumbnail_clone.paintable().as_ref());

            let is_active = model_clone.get::<bool>(&model_clone.iter(&tree_path_clone).expect("Invalid tree_path"), column_selection);
            check_button_left_preview_text_clone.set_active(is_active);
            check_button_left_preview_text_clone.set_label(Some(&format!("{}. {}", number + 1, get_max_file_name(&path_clone, 60))));
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
            update_bottom_buttons(&all_gtk_box_clone, &shared_using_for_preview_clone, &shared_image_cache_clone);
            image_compare_right.set_paintable(big_thumbnail_clone.paintable().as_ref());

            let is_active = model_clone.get::<bool>(&model_clone.iter(&tree_path_clone).expect("Invalid tree_path"), column_selection);
            check_button_right_preview_text_clone.set_active(is_active);
            check_button_right_preview_text_clone.set_label(Some(&format!("{}. {}", number + 1, get_max_file_name(&path_clone, 60))));
        });

        smaller_box.append(&button_left);
        smaller_box.append(&label);
        smaller_box.append(&button_right);

        small_box.append(&smaller_box);
        small_box.set_halign(Align::Fill);
        small_box.set_valign(Align::Fill);
        small_box.set_hexpand_set(true);
        small_box.set_vexpand_set(true);
        small_thumbnail.set_halign(Align::Fill);
        small_thumbnail.set_valign(Align::Fill);
        small_thumbnail.set_hexpand(true);
        small_thumbnail.set_hexpand_set(true);
        small_thumbnail.set_vexpand(true);
        small_thumbnail.set_vexpand_set(true);

        small_box.append(small_thumbnail);

        all_gtk_box.append(&small_box);
    }

    all_gtk_box.show();
    scrolled_window.set_child(Some(&all_gtk_box));
}

fn update_bottom_buttons(
    all_gtk_box: &gtk4::Box,
    shared_using_for_preview: &Rc<RefCell<(Option<TreePath>, Option<TreePath>)>>,
    image_cache: &Rc<RefCell<Vec<(String, String, Image, Image, TreePath)>>>,
) {
    let left_tree_view = shared_using_for_preview.borrow().0.clone().expect("Left tree_view not set");
    let right_tree_view = shared_using_for_preview.borrow().1.clone().expect("Right tree_view not set");

    for (number, i) in get_all_direct_children(all_gtk_box).into_iter().enumerate() {
        let cache_tree_path = (*image_cache.borrow())[number].4.clone();
        let is_chosen = cache_tree_path != right_tree_view && cache_tree_path != left_tree_view;

        let bx = i.downcast::<gtk4::Box>().expect("Not Box");
        let smaller_bx = bx.first_child().expect("No first child").downcast::<gtk4::Box>().expect("First child is not Box");
        for items in get_all_direct_children(&smaller_bx) {
            if let Ok(btn) = items.downcast::<gtk4::Button>() {
                btn.set_sensitive(is_chosen);
            }
        }
    }
}

fn get_current_group_and_iter_from_selection(model: &TreeModel, selection: &TreeSelection, column_header: i32) -> (u32, TreePath) {
    let mut current_group = 1;
    let mut possible_group = 1;
    let mut header_clone: TreeIter;
    let mut possible_header: TreeIter;

    let selected_records = selection.selected_rows().0;

    let iter = model.iter_first().expect("Model is no empty, so should have first item"); // Checking that treeview is not empty should be done before
    header_clone = iter; // if nothing selected, use first group
    possible_header = iter; // if nothing selected, use first group
    assert!(model.get::<bool>(&iter, column_header)); // First element should be header

    if !selected_records.is_empty() {
        let first_selected_record = selected_records[0].clone();
        loop {
            if !model.iter_next(&iter) {
                break;
            }

            if model.get::<bool>(&iter, column_header) {
                possible_group += 1;
                possible_header = iter;
            }

            if model.path(&iter) == first_selected_record {
                header_clone = possible_header;
                current_group = possible_group;
            }
        }
    }

    (current_group, model.path(&header_clone))
}
