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
use gtk4::{Scale, ScrollType, TextView, TreeView, Widget};
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, GenericImageView, RgbaImage};
use resvg::tiny_skia;
use resvg::usvg::{Options, Tree};

use crate::flg;
use crate::gtk_traits::WidgetTraits;
use crate::helpers::enums::BottomButtonsEnum;
use crate::notebook_enums::NotebookUpperEnum;
use crate::notebook_info::{NOTEBOOKS_INFO, NotebookObject};

pub const KEY_DELETE: u32 = 119;
pub const KEY_ENTER: u32 = 36;
pub const KEY_SPACE: u32 = 65;

pub type SharedState<T> = Rc<RefCell<Option<T>>>;

pub const MAIN_ROW_COLOR: &str = "#222222";
pub const HEADER_ROW_COLOR: &str = "#111111";
pub const TEXT_COLOR: &str = "#ffffff";

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
    let image = button.get_custom_image();
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

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use gtk4::prelude::*;
    use image::DynamicImage;

    use super::*;

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
