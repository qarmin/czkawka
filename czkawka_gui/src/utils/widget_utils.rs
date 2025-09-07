use gtk4::prelude::*;
use gtk4::{Widget};

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

