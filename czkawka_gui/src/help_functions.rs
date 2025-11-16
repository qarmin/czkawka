use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::path::{MAIN_SEPARATOR, PathBuf};
use std::rc::Rc;

use czkawka_core::helpers::messages::Messages;
use gdk4::gdk_pixbuf::{InterpType, Pixbuf};
use glib::Bytes;
use gtk4::gdk_pixbuf::Colorspace;
use gtk4::prelude::*;
use gtk4::{ListStore, Scale, ScrollType, TextView, TreeView, Widget};
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, GenericImageView, RgbaImage};
use log::debug;
use resvg::tiny_skia;
use resvg::usvg::{Options, Tree};

use crate::flg;
use crate::gui_structs::common_tree_view::SubView;
use crate::helpers::enums::BottomButtonsEnum;
use crate::model_iter::{iter_list, iter_list_with_break, iter_list_with_break_init};
use crate::notebook_enums::NotebookUpperEnum;
use crate::notebook_info::{NOTEBOOKS_INFO, NotebookObject};

pub const KEY_DELETE: u32 = 119;
pub const KEY_ENTER: u32 = 36;
pub const KEY_SPACE: u32 = 65;

pub type SharedState<T> = Rc<RefCell<Option<T>>>;

pub const MAIN_ROW_COLOR: &str = "#222222";
pub const HEADER_ROW_COLOR: &str = "#111111";
pub const TEXT_COLOR: &str = "#ffffff";

pub(crate) fn get_string_from_list_store(tree_view: &TreeView, column_full_path: i32, column_selection: Option<i32>) -> Vec<String> {
    let list_store: ListStore = get_list_store(tree_view);

    let mut string_vector: Vec<String> = Vec::new();

    match column_selection {
        Some(column_selection) => {
            iter_list(&list_store, |m, i| {
                if m.get::<bool>(i, column_selection) {
                    string_vector.push(m.get::<String>(i, column_full_path));
                }
            });
        }
        None => {
            iter_list(&list_store, |m, i| {
                string_vector.push(m.get::<String>(i, column_full_path));
            });
        }
    }

    string_vector
}

pub(crate) fn get_from_list_store_fnc<T>(tree_view: &TreeView, fnc: &dyn Fn(&ListStore, &gtk4::TreeIter, &mut Vec<T>)) -> Vec<T> {
    let list_store: ListStore = get_list_store(tree_view);

    let mut result_vector: Vec<T> = Vec::new();

    iter_list(&list_store, |m, i| {
        fnc(m, i, &mut result_vector);
    });

    result_vector
}

pub(crate) fn get_path_buf_from_vector_of_strings(vec_string: &[String]) -> Vec<PathBuf> {
    vec_string.iter().map(PathBuf::from).collect()
}

pub(crate) fn print_text_messages_to_text_view(text_messages: &Messages, text_view: &TextView) {
    let mut messages: String = String::new();
    if !text_messages.messages.is_empty() {
        messages += format!("############### {}({}) ###############\n", flg!("text_view_messages"), text_messages.messages.len()).as_str();
    }
    for text in &text_messages.messages {
        messages += text.as_str();
        messages += "\n";
    }
    // if !text_messages.messages.is_empty() {
    //     messages += "\n";
    // }
    if !text_messages.warnings.is_empty() {
        messages += format!("############### {}({}) ###############\n", flg!("text_view_warnings"), text_messages.warnings.len()).as_str();
    }
    for text in &text_messages.warnings {
        messages += text.as_str();
        messages += "\n";
    }
    // if !text_messages.warnings.is_empty() {
    //     messages += "\n";
    // }
    if !text_messages.errors.is_empty() {
        messages += format!("############### {}({}) ###############\n", flg!("text_view_errors"), text_messages.errors.len()).as_str();
    }
    for text in &text_messages.errors {
        messages += text.as_str();
        messages += "\n";
    }
    // if !text_messages.errors.is_empty() {
    //     messages += "\n";
    // }

    text_view.buffer().set_text(messages.as_str());
}

pub(crate) fn reset_text_view(text_view: &TextView) {
    text_view.buffer().set_text("");
}

pub(crate) fn add_text_to_text_view(text_view: &TextView, string_to_append: &str) {
    let buffer = text_view.buffer();
    let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();
    if current_text.is_empty() {
        buffer.set_text(string_to_append);
    } else {
        buffer.set_text(format!("{current_text}\n{string_to_append}").as_str());
    }
}

pub(crate) fn set_buttons(hashmap: &mut HashMap<BottomButtonsEnum, bool>, buttons_array: &[Widget], button_names: &[BottomButtonsEnum]) {
    for (index, button) in buttons_array.iter().enumerate() {
        if *hashmap.get_mut(&button_names[index]).expect("Invalid button name") {
            button.set_visible(true);
        } else {
            button.set_visible(false);
        }
    }
}

pub(crate) fn hide_all_buttons(buttons_array: &[Widget]) {
    for button in buttons_array {
        button.set_visible(false);
    }
}

pub(crate) fn get_list_store(tree_view: &TreeView) -> ListStore {
    tree_view.model().expect("Tree view have no model").downcast::<ListStore>().expect("Model is not ListStore")
}

pub(crate) fn get_dialog_box_child(dialog: &gtk4::Dialog) -> gtk4::Box {
    dialog.child().expect("Dialog has no child").downcast::<gtk4::Box>().expect("Dialog child is not Box")
}

pub(crate) fn change_dimension_to_krotka(dimensions: &str) -> (u64, u64) {
    #[expect(clippy::single_char_pattern)]
    let vec = dimensions.split::<&str>("x").collect::<Vec<_>>();
    assert_eq!(vec.len(), 2); // 400x400 - should only have two elements, if have more, then something is not good
    let number1 = vec[0].parse::<u64>().expect("Invalid data in image dimension in position 0");
    let number2 = vec[1].parse::<u64>().expect("Invalid data in image dimension in position 1");
    (number1, number2)
}

pub(crate) fn get_notebook_upper_enum_from_tree_view(tree_view: &TreeView) -> NotebookUpperEnum {
    match (*tree_view).widget_name().to_string().as_str() {
        "tree_view_upper_included_directories" => NotebookUpperEnum::IncludedDirectories,
        "tree_view_upper_excluded_directories" => NotebookUpperEnum::ExcludedDirectories,
        e => panic!("{}", e),
    }
}

pub(crate) fn get_notebook_object_from_tree_view(tree_view: &TreeView) -> &NotebookObject {
    let tree_view_name = (*tree_view).widget_name().to_string();

    NOTEBOOKS_INFO
        .iter()
        .find(|nb_object| nb_object.tree_view_name == tree_view_name)
        .map_or_else(|| panic!("Tree view name '{tree_view_name}' not found in NOTEBOOKS_INFO"), |nb_object| nb_object)
}

pub(crate) fn get_full_name_from_path_name(path: &str, name: &str) -> String {
    let mut string = String::with_capacity(path.len() + name.len() + 1);
    string.push_str(path);
    string.push(MAIN_SEPARATOR);
    string.push_str(name);
    string
}

// After e.g. deleting files, header may become orphan or have one child, so should be deleted in this case
pub(crate) fn clean_invalid_headers(model: &ListStore, column_header: i32, column_path: i32) {
    // Remove only child from header
    if let Some(first_iter) = model.iter_first() {
        let mut vec_tree_path_to_delete: Vec<gtk4::TreePath> = Vec::new();
        let mut current_iter = first_iter;
        // First element should be header
        assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");

        let mut next_iter;
        let mut next_next_iter;

        // Empty means default check type
        if model.get::<String>(&current_iter, column_path).is_empty() {
            'main: loop {
                // First element should be header
                assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");

                next_iter = current_iter;
                if !model.iter_next(&next_iter) {
                    // There is only single header left (H1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    break 'main;
                }

                if model.get::<bool>(&next_iter, column_header) {
                    // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    current_iter = next_iter;
                    continue 'main;
                }

                next_next_iter = next_iter;
                if !model.iter_next(&next_next_iter) {
                    // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    vec_tree_path_to_delete.push(model.path(&next_iter));
                    break 'main;
                }

                if model.get::<bool>(&next_next_iter, column_header) {
                    // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    vec_tree_path_to_delete.push(model.path(&next_iter));
                    current_iter = next_next_iter;
                    continue 'main;
                }

                loop {
                    // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                    if !model.iter_next(&next_next_iter) {
                        break 'main;
                    }
                    // Move to next header
                    if model.get::<bool>(&next_next_iter, column_header) {
                        current_iter = next_next_iter;
                        continue 'main;
                    }
                }
            }
        }
        // Non empty means that header points at reference folder
        else {
            'reference: loop {
                // First element should be header
                assert!(model.get::<bool>(&current_iter, column_header), "First deleted element, should be a header");

                next_iter = current_iter;
                if !model.iter_next(&next_iter) {
                    // There is only single header left (H1 -> END) -> (NOTHING)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    break 'reference;
                }

                if model.get::<bool>(&next_iter, column_header) {
                    // There are two headers each others(we remove just first) -> (H1 -> H2) -> (H2)
                    vec_tree_path_to_delete.push(model.path(&current_iter));
                    current_iter = next_iter;
                    continue 'reference;
                }

                next_next_iter = next_iter;
                if !model.iter_next(&next_next_iter) {
                    // There is only one child of header left, so we remove it with header (H1 -> C1 -> END) -> (NOTHING)
                    break 'reference;
                }

                if model.get::<bool>(&next_next_iter, column_header) {
                    // One child between two headers, we can remove them  (H1 -> C1 -> H2) -> (H2)
                    current_iter = next_next_iter;
                    continue 'reference;
                }

                loop {
                    // (H1 -> C1 -> C2 -> Cn -> END) -> (NO CHANGE, BECAUSE IS GOOD)
                    if !model.iter_next(&next_next_iter) {
                        break 'reference;
                    }
                    // Move to next header
                    if model.get::<bool>(&next_next_iter, column_header) {
                        current_iter = next_next_iter;
                        continue 'reference;
                    }
                }
            }
        }
        for tree_path in vec_tree_path_to_delete.iter().rev() {
            model.remove(&model.iter(tree_path).expect("Using invalid tree_path"));
        }
    }

    // Last step, remove orphan header if exists
    if let Some(iter) = model.iter_first()
        && !model.iter_next(&iter)
    {
        model.clear();
    }
}

pub(crate) fn check_how_much_elements_is_selected(sv: &SubView) -> (u64, u64) {
    let mut number_of_selected_items: u64 = 0;
    let mut number_of_selected_groups: u64 = 0;

    let model = sv.get_model();

    let mut is_item_currently_selected_in_group: bool = false;

    if let Some(column_header) = sv.nb_object.column_header {
        iter_list_with_break_init(
            &model,
            |m, i| {
                assert!(m.get::<bool>(i, column_header)); // First element should be header
                m.iter_next(i)
            },
            |m, i| {
                if m.get::<bool>(i, column_header) {
                    is_item_currently_selected_in_group = false;
                } else if m.get::<bool>(i, sv.nb_object.column_selection) {
                    number_of_selected_items += 1;

                    if !is_item_currently_selected_in_group {
                        number_of_selected_groups += 1;
                    }
                    is_item_currently_selected_in_group = true;
                }
            },
        );
    } else {
        iter_list(&model, |m, i| {
            if m.get::<bool>(i, sv.nb_object.column_selection) {
                number_of_selected_items += 1;
            }
        });
    }

    (number_of_selected_items, number_of_selected_groups)
}

pub(crate) fn count_number_of_groups(sv: &SubView) -> u32 {
    let mut number_of_selected_groups = 0;
    let column_header = sv.nb_object.column_header.expect("Column header should be present to count number of groups");

    let model = sv.get_model();

    iter_list_with_break_init(
        &model,
        |_m, i| {
            assert!(model.get::<bool>(i, column_header)); // First element should be header
            true
        },
        |m, i| {
            if m.get::<bool>(i, column_header) {
                number_of_selected_groups += 1;
            }
        },
    );
    number_of_selected_groups
}

pub(crate) fn resize_pixbuf_dimension(pixbuf: &Pixbuf, requested_size: (i32, i32), interp_type: InterpType) -> Option<Pixbuf> {
    let current_ratio = pixbuf.width() as f32 / pixbuf.height() as f32;
    let mut new_size;
    match current_ratio.total_cmp(&(requested_size.0 as f32 / requested_size.1 as f32)) {
        Ordering::Greater => {
            new_size = (requested_size.0, (pixbuf.height() * requested_size.0) / pixbuf.width());
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
        Ordering::Less => {
            new_size = ((pixbuf.width() * requested_size.1) / pixbuf.height(), requested_size.1);
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
        Ordering::Equal => {
            new_size = requested_size;
            new_size = (std::cmp::max(new_size.0, 1), std::cmp::max(new_size.1, 1));
        }
    }
    pixbuf.scale_simple(new_size.0, new_size.1, interp_type)
}

pub(crate) fn get_max_file_name(file_name: &str, max_length: usize) -> String {
    assert!(max_length > 10); // Maybe in future will be supported lower values
    let characters_in_filename = file_name.chars().count();
    if characters_in_filename > max_length {
        let start_characters = 10;
        let difference = characters_in_filename - max_length;
        let second_part_start = start_characters + difference;
        let mut string_pre = String::new();
        let mut string_after = String::new();

        for (index, character) in file_name.chars().enumerate() {
            if index < start_characters {
                string_pre.push(character);
            } else if index >= second_part_start {
                string_after.push(character);
            }
        }

        format!("{string_pre} ... {string_after}")
    } else {
        file_name.to_string()
    }
}

pub(crate) fn get_custom_label_from_widget<P: IsA<Widget>>(item: &P) -> gtk4::Label {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];

    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(label) = widget.clone().downcast::<gtk4::Label>() {
            return label;
        }
        widgets_to_check.extend(get_all_direct_children(&widget));
    }
    panic!("Button doesn't have proper custom label child");
}

pub(crate) fn get_custom_image_from_widget<P: IsA<Widget>>(item: &P) -> gtk4::Image {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];

    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(image) = widget.clone().downcast::<gtk4::Image>() {
            return image;
        }
        widgets_to_check.extend(get_all_direct_children(&widget));
    }
    panic!("Button doesn't have proper custom label child");
}

#[expect(dead_code)]
pub(crate) fn debug_print_widget<P: IsA<Widget>>(item: &P) {
    let mut widgets_to_check = vec![(0, 0, item.clone().upcast::<Widget>())];

    let mut next_free_number = 1;
    debug!("{}, {}, {:?} ", widgets_to_check[0].0, widgets_to_check[0].1, widgets_to_check[0].2);

    while let Some((current_number, parent_number, widget)) = widgets_to_check.pop() {
        for widget in get_all_direct_children(&widget) {
            widgets_to_check.push((next_free_number, current_number, widget));
            next_free_number += 1;
        }
        debug!("{current_number}, {parent_number}, {widget:?} ");
    }
}

pub(crate) fn get_all_boxes_from_widget<P: IsA<Widget>>(item: &P) -> Vec<gtk4::Box> {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];
    let mut boxes = Vec::new();

    while let Some(widget) = widgets_to_check.pop() {
        widgets_to_check.extend(get_all_direct_children(&widget));
        if let Ok(bbox) = widget.clone().downcast::<gtk4::Box>() {
            boxes.push(bbox);
        }
    }
    boxes
}

pub(crate) fn get_all_direct_children<P: IsA<Widget>>(wid: &P) -> Vec<Widget> {
    let mut vector = vec![];
    if let Some(mut child) = wid.first_child() {
        vector.push(child.clone());
        loop {
            child = match child.next_sibling() {
                Some(t) => t,
                None => break,
            };
            vector.push(child.clone());
        }
    }

    vector
}

const SIZE_OF_ICON: i32 = 18;
const TYPE_OF_INTERPOLATION: InterpType = InterpType::Tiles;

fn svg_to_dynamic_image(svg_data: &[u8]) -> Option<DynamicImage> {
    let opt = Options::default();
    let tree = Tree::from_data(svg_data, &opt).ok()?;

    let mut pixmap = tiny_skia::Pixmap::new(tree.size().width() as u32, tree.size().height() as u32)?;
    resvg::render(&tree, tiny_skia::Transform::default(), &mut (pixmap.as_mut()));

    let rgba = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())?;

    Some(DynamicImage::ImageRgba8(rgba))
}

fn dynamic_image_to_pixbuf(img: DynamicImage) -> Pixbuf {
    let (width, height) = img.dimensions();
    let rgba = img.into_rgba8();
    let bytes = Bytes::from(&rgba.into_raw());

    let pixbuf = Pixbuf::from_bytes(&bytes, Colorspace::Rgb, true, 8, width as i32, height as i32, (4 * width) as i32);
    pixbuf.scale_simple(SIZE_OF_ICON, SIZE_OF_ICON, TYPE_OF_INTERPOLATION).expect("Failed to scale pixbuf")
}

pub(crate) fn set_icon_of_button<P: IsA<Widget>>(button: &P, data: &'static [u8]) {
    let image = get_custom_image_from_widget(button);
    let dynamic_image = svg_to_dynamic_image(data).expect("Failed to convert SVG data to DynamicImage");
    let pixbuf = dynamic_image_to_pixbuf(dynamic_image);
    image.set_from_pixbuf(Some(&pixbuf));
}

pub(crate) fn get_pixbuf_from_dynamic_image(dynamic_image: &DynamicImage) -> Result<Pixbuf, String> {
    let mut output = Vec::new();
    JpegEncoder::new(&mut output)
        .encode_image(dynamic_image)
        .map_err(|e| format!("Failed to encode image: {e}"))?;
    Pixbuf::from_read(BufReader::new(Cursor::new(output))).map_err(|e| format!("Failed to create Pixbuf from DynamicImage: {e}"))
}

pub(crate) fn check_if_value_is_in_list_store(model: &ListStore, column: i32, value: &str) -> bool {
    let mut is_in_store = false;
    iter_list_with_break(model, |m, i| {
        if m.get::<String>(i, column) == value {
            is_in_store = true;
            return false;
        }
        true
    });

    is_in_store
}

pub(crate) fn check_if_list_store_column_have_all_same_values(model: &ListStore, column: i32, value: bool) -> bool {
    let mut all_are_same = false;
    iter_list_with_break(model, |m, i| {
        all_are_same = true;
        if m.get::<bool>(i, column) != value {
            all_are_same = false;
            return false;
        }

        true
    });

    all_are_same
}

pub(crate) fn scale_set_min_max_values(scale: &Scale, minimum: f64, maximum: f64, current_value: f64, step: Option<f64>) {
    scale.set_range(minimum, maximum);
    scale.set_fill_level(maximum);
    scale.set_value(current_value);
    if let Some(step) = step {
        scale.adjustment().set_step_increment(step);
    }
}

pub(crate) fn scale_step_function(scale: &Scale, _scroll_type: ScrollType, value: f64) -> glib::Propagation {
    scale.set_increments(1_f64, 1_f64);
    scale.set_round_digits(0);
    scale.set_fill_level(value.round());
    glib::Propagation::Proceed
}

pub(crate) fn append_row_to_list_store(list_store: &ListStore, values: &[(u32, &dyn ToValue)]) {
    list_store.set(&list_store.append(), values);
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use glib::Value;
    use glib::types::Type;
    use gtk4::prelude::*;
    use gtk4::{Orientation, TreeView};
    use image::DynamicImage;

    use super::*;
    use crate::notebook_enums::NotebookMainEnum;

    // Helper to create a minimal SubView for Duplicate notebook along with its ListStore
    fn get_test_sv_duplicate() -> (crate::gui_structs::common_tree_view::SubView, gtk4::ListStore) {
        use std::cell::RefCell;
        use std::rc::Rc;

        use czkawka_core::tools::duplicate::DuplicateFinder;

        use crate::gui_structs::common_tree_view::SharedModelEnum;

        let nb_object = NOTEBOOKS_INFO[NotebookMainEnum::Duplicate as usize].clone();

        let list_store = gtk4::ListStore::new(nb_object.columns_types);
        let tree_view = gtk4::TreeView::new();
        tree_view.set_model(Some(&list_store));

        let scrolled_window = gtk4::ScrolledWindow::new();
        let gesture_click = gtk4::GestureClick::new();
        let event_controller_key = gtk4::EventControllerKey::new();
        tree_view.add_controller(event_controller_key.clone());
        tree_view.add_controller(gesture_click.clone());

        let sv = crate::gui_structs::common_tree_view::SubView {
            scrolled_window,
            tree_view,
            gesture_click,
            event_controller_key,
            nb_object,
            enum_value: NotebookMainEnum::Duplicate,
            preview_struct: None,
            shared_model_enum: SharedModelEnum::Duplicates(Rc::new(RefCell::new(None::<DuplicateFinder>))),
        };

        (sv, list_store)
    }

    #[gtk4::test]
    fn test_get_string_from_list_store() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &"test"), (0, &"test2"), (0, &"test3")];
        for i in values_to_add {
            append_row_to_list_store(&list_store, &[*i]);
        }
        assert_eq!(
            get_string_from_list_store(&tree_view, 0, None),
            vec!["test".to_string(), "test2".to_string(), "test3".to_string()]
        );

        let columns_types: &[Type] = &[Type::BOOL, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[
            &[(0, &Into::<Value>::into(true)), (1, &Into::<Value>::into("test"))],
            &[(0, &Into::<Value>::into(true)), (1, &Into::<Value>::into("test2"))],
            &[(0, &Into::<Value>::into(false)), (1, &Into::<Value>::into("test3"))],
        ];
        for i in values_to_add {
            append_row_to_list_store(&list_store, i);
        }
        assert_eq!(get_string_from_list_store(&tree_view, 1, Some(0)), vec!["test".to_string(), "test2".to_string()]);
    }

    #[gtk4::test]
    fn test_check_if_list_store_column_have_all_same_values() {
        let columns_types: &[Type] = &[Type::BOOL];
        let list_store = gtk4::ListStore::new(columns_types);

        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &true), (0, &true), (0, &false)];
        for i in values_to_add {
            append_row_to_list_store(&list_store, &[*i]);
        }
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));

        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &true), (0, &true), (0, &true)];
        for i in values_to_add {
            append_row_to_list_store(&list_store, &[*i]);
        }
        assert!(check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));

        list_store.clear();
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &false)];
        for i in values_to_add {
            append_row_to_list_store(&list_store, &[*i]);
        }
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(check_if_list_store_column_have_all_same_values(&list_store, 0, false));

        list_store.clear();
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, true));
        assert!(!check_if_list_store_column_have_all_same_values(&list_store, 0, false));
    }

    #[gtk4::test]
    fn test_check_if_value_is_in_list_store() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let values_to_add: &[(u32, &dyn ToValue)] = &[(0, &"Koczkodan"), (0, &"Kachir")];
        for i in values_to_add {
            append_row_to_list_store(&list_store, &[*i]);
        }
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Koczkodan"));
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Kachir"));
        assert!(!check_if_value_is_in_list_store(&list_store, 0, "Koczkodan2"));

        let columns_types: &[Type] = &[Type::STRING, Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let values_to_add: &[&[(u32, &dyn ToValue)]] = &[&[(0, &"Koczkodan"), (1, &"Krakus")], &[(0, &"Kachir"), (1, &"Wodnica")]];
        for i in values_to_add {
            append_row_to_list_store(&list_store, i);
        }
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Koczkodan"));
        assert!(check_if_value_is_in_list_store(&list_store, 1, "Krakus"));
        assert!(check_if_value_is_in_list_store(&list_store, 0, "Kachir"));
        assert!(check_if_value_is_in_list_store(&list_store, 1, "Wodnica"));
        assert!(!check_if_value_is_in_list_store(&list_store, 0, "Krakus"));
        assert!(!check_if_value_is_in_list_store(&list_store, 1, "Kachir"));
    }

    #[test]
    fn test_file_name_shortener() {
        let name_to_check = "/home/rafal/czkawek/romek/atomek.txt";
        assert_eq!(get_max_file_name(name_to_check, 20), "/home/rafa ... atomek.txt");
        assert_eq!(get_max_file_name(name_to_check, 21), "/home/rafa ... /atomek.txt");
        let name_to_check = "/home/rafal/czkawek/romek/czekistan/atomek.txt";
        assert_eq!(get_max_file_name(name_to_check, 21), "/home/rafa ... /atomek.txt");
        assert_eq!(get_max_file_name(name_to_check, 80), name_to_check);
        let name_to_check = "/home/rafal/‍🌈🌈🌈🌈🌈🌈🌈🌈🌈🌈🌈🌈🌈.txt";
        assert_eq!(get_max_file_name(name_to_check, 21), "/home/rafa ... 🌈🌈🌈🌈🌈🌈🌈.txt");
        assert_eq!(get_max_file_name(name_to_check, 20), "/home/rafa ... 🌈🌈🌈🌈🌈🌈.txt");
        assert_eq!(get_max_file_name(name_to_check, 19), "/home/rafa ... 🌈🌈🌈🌈🌈.txt");
        let name_to_check = "/home/rafal/‍🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️🏳️‍🌈️.txt";
        assert_eq!(get_max_file_name(name_to_check, 21), "/home/rafa ... 🌈\u{fe0f}🏳\u{fe0f}\u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 20), "/home/rafa ... \u{fe0f}🏳\u{fe0f}\u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 19), "/home/rafa ... 🏳\u{fe0f}\u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 18), "/home/rafa ... \u{fe0f}\u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 17), "/home/rafa ... \u{200d}🌈\u{fe0f}.txt");
        assert_eq!(get_max_file_name(name_to_check, 16), "/home/rafa ... 🌈\u{fe0f}.txt");
    }

    #[test]
    fn test_pixbuf_from_dynamic_image() {
        let dynamic_image = DynamicImage::new_rgb8(1, 1);
        get_pixbuf_from_dynamic_image(&dynamic_image).expect("Failed to get pixbuf from dynamic image");
        get_pixbuf_from_dynamic_image(&dynamic_image).expect("Failed to get pixbuf from dynamic image");
        get_pixbuf_from_dynamic_image(&dynamic_image).expect("Failed to get pixbuf from dynamic image");
        get_pixbuf_from_dynamic_image(&dynamic_image).expect("Failed to get pixbuf from dynamic image");
    }

    #[gtk4::test]
    fn test_get_all_direct_children() {
        let obj = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj2 = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj3 = gtk4::Image::new();
        let obj4 = gtk4::Image::new();
        let obj5 = gtk4::Image::new();
        obj.append(&obj2);
        obj.append(&obj3);
        obj2.append(&obj4);
        obj2.append(&obj5);
        assert_eq!(get_all_direct_children(&obj).len(), 2);
    }

    #[gtk4::test]
    fn test_get_all_boxes_from_widget() {
        let obj = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj2 = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj3 = gtk4::Image::new();
        let obj4 = gtk4::Image::new();
        let obj5 = gtk4::Image::new();
        obj.append(&obj2);
        obj.append(&obj3);
        obj2.append(&obj4);
        obj2.append(&obj5);
        assert_eq!(get_all_boxes_from_widget(&obj).len(), 2);
    }

    #[test]
    fn test_get_path_buf_from_vector_of_strings() {
        let input = vec!["/tmp/test1".to_string(), "relative/path".to_string()];
        let result = get_path_buf_from_vector_of_strings(&input);
        assert_eq!(result, vec![PathBuf::from("/tmp/test1"), PathBuf::from("relative/path")]);
    }

    #[test]
    fn test_get_full_name_from_path_name() {
        let path = "/home/user";
        let name = "file.txt";
        let expected = format!("{}{}{}", path, std::path::MAIN_SEPARATOR, name);
        assert_eq!(get_full_name_from_path_name(path, name), expected);
    }

    #[gtk4::test]
    fn test_count_number_of_groups() {
        // Use helper that builds SubView + ListStore
        let (sv, list_store) = get_test_sv_duplicate();

        let column_header = sv.nb_object.column_header.expect("Duplicate NB must have header column");

        // Build rows: H, C, H, C -> expected 2 groups
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(true))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(false))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(true))]);
        append_row_to_list_store(&list_store, &[(column_header as u32, &Into::<Value>::into(false))]);

        assert_eq!(crate::help_functions::count_number_of_groups(&sv), 2);
    }

    #[gtk4::test]
    fn test_check_how_much_elements_is_selected() {
        let (sv, list_store) = get_test_sv_duplicate();

        let column_header = sv.nb_object.column_header.expect("Duplicate NB must have header column");
        let column_selection = sv.nb_object.column_selection;

        // Build rows: H, C(selected), C(not selected), H, C(selected) => 2 selected items in 2 groups
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(true)), (column_selection as u32, &Into::<Value>::into(false))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(true))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(false))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(true)), (column_selection as u32, &Into::<Value>::into(false))],
        );
        append_row_to_list_store(
            &list_store,
            &[(column_header as u32, &Into::<Value>::into(false)), (column_selection as u32, &Into::<Value>::into(true))],
        );

        let res = check_how_much_elements_is_selected(&sv);
        assert_eq!(res, (2, 2));
    }

    #[gtk4::test]
    fn test_get_from_list_store_fnc() {
        let columns_types: &[Type] = &[Type::STRING];
        let list_store = gtk4::ListStore::new(columns_types);
        let tree_view = TreeView::with_model(&list_store);

        // Append literals directly to avoid lifetime/coercion issues
        append_row_to_list_store(&list_store, &[(0, &"a")]);
        append_row_to_list_store(&list_store, &[(0, &"b")]);
        append_row_to_list_store(&list_store, &[(0, &"c")]);

        let collected: Vec<String> = get_from_list_store_fnc(&tree_view, &|m, i, vec: &mut Vec<String>| {
            vec.push(m.get::<String>(i, 0));
        });

        assert_eq!(collected, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[gtk4::test]
    fn test_set_and_hide_buttons() {
        use std::collections::HashMap;
        let btn1 = gtk4::Button::new();
        let btn2 = gtk4::Button::new();
        let w1: Widget = btn1.upcast();
        let w2: Widget = btn2.upcast();
        let buttons = vec![w1, w2];

        let mut map: HashMap<BottomButtonsEnum, bool> = HashMap::new();
        map.insert(BottomButtonsEnum::Save, true);
        map.insert(BottomButtonsEnum::Delete, false);
        let names = [BottomButtonsEnum::Save, BottomButtonsEnum::Delete];

        set_buttons(&mut map, &buttons, &names);
        assert!(buttons[0].is_visible());
        assert!(!buttons[1].is_visible());

        hide_all_buttons(&buttons);
        assert!(!buttons[0].is_visible());
        assert!(!buttons[1].is_visible());
    }
}

#[cfg(test)]
mod tests {
    use std::path::MAIN_SEPARATOR;

    use super::*;

    #[test]
    fn test_get_full_name_from_path_name() {
        let path = "some_dir";
        let name = "file.txt";
        let expected = format!("{path}{MAIN_SEPARATOR}{name}");
        assert_eq!(get_full_name_from_path_name(path, name), expected);
    }

    #[test]
    fn test_change_dimension_to_krotka() {
        let dim = "1024x768";
        let (w, h) = change_dimension_to_krotka(dim);
        assert_eq!((w, h), (1024, 768));
    }

    #[test]
    fn test_get_max_file_name_truncation() {
        let name = "very_long_filename_example.txt";
        // use max_length smaller than name length to trigger truncation
        let out = get_max_file_name(name, 20);
        // Should contain ellipsis and keep the first 10 chars
        assert!(out.contains(" ... "));
        assert!(out.starts_with(&name.chars().take(10).collect::<String>()));
    }

    #[test]
    fn test_get_path_buf_from_vector_of_strings() {
        let v = vec!["a".to_string(), "b".to_string()];
        let res = get_path_buf_from_vector_of_strings(&v);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], PathBuf::from("a"));
        assert_eq!(res[1], PathBuf::from("b"));
    }
}
