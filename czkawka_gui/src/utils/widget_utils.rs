use std::collections::HashMap;

use gtk4::Widget;
use gtk4::prelude::*;

use crate::utils::enums::BottomButtonsEnum;

pub fn get_custom_label_from_widget<P: IsA<Widget>>(item: &P) -> gtk4::Label {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];
    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(label) = widget.clone().downcast::<gtk4::Label>() {
            return label;
        }
        widgets_to_check.extend(get_all_direct_children(&widget));
    }
    panic!("Button doesn't have proper custom label child");
}

pub fn get_custom_image_from_widget<P: IsA<Widget>>(item: &P) -> gtk4::Image {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];
    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(image) = widget.clone().downcast::<gtk4::Image>() {
            return image;
        }
        widgets_to_check.extend(get_all_direct_children(&widget));
    }
    panic!("Button doesn't have proper custom label child");
}

pub fn get_all_boxes_from_widget<P: IsA<Widget>>(item: &P) -> Vec<gtk4::Box> {
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

pub fn get_all_direct_children<P: IsA<Widget>>(wid: &P) -> Vec<Widget> {
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

#[allow(dead_code)]
pub fn debug_print_widget<P: IsA<Widget>>(item: &P) {
    use log::debug;
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

pub fn set_buttons(hashmap: &mut HashMap<BottomButtonsEnum, bool>, buttons_array: &[Widget], button_names: &[BottomButtonsEnum]) {
    for (index, button) in buttons_array.iter().enumerate() {
        if *hashmap.get_mut(&button_names[index]).expect("Invalid button name") {
            button.show();
        } else {
            button.hide();
        }
    }
}

pub fn hide_all_buttons(buttons_array: &[Widget]) {
    for button in buttons_array {
        button.hide();
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use gtk4::Button;
    use gtk4::prelude::*;

    use super::*;

    #[gtk4::test]
    fn test_set_buttons_and_hide_all_buttons() {
        let button1 = Button::with_label("Button1");
        let button2 = Button::with_label("Button2");
        let button3 = Button::with_label("Button3");
        let buttons_array = vec![button1.upcast::<Widget>(), button2.upcast::<Widget>(), button3.upcast::<Widget>()];
        let button_names = vec![BottomButtonsEnum::Search, BottomButtonsEnum::Delete, BottomButtonsEnum::Save];
        let mut hashmap = HashMap::new();
        hashmap.insert(BottomButtonsEnum::Search, true);
        hashmap.insert(BottomButtonsEnum::Delete, false);
        hashmap.insert(BottomButtonsEnum::Save, true);
        set_buttons(&mut hashmap, &buttons_array, &button_names);
        assert!(buttons_array[0].is_visible());
        assert!(!buttons_array[1].is_visible());
        assert!(buttons_array[2].is_visible());
        hide_all_buttons(&buttons_array);
        assert!(!buttons_array[0].is_visible());
        assert!(!buttons_array[1].is_visible());
        assert!(!buttons_array[2].is_visible());
    }
}
