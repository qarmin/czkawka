use std::vec::Vec;

use gtk4::prelude::{ComboBoxExtManual, *};
use gtk4::{Box as GtkBox, ComboBoxText, Dialog, Image, Label, Widget};
use log::debug;

pub trait ComboBoxTraits {
    fn set_model_and_first<I, S>(&self, models: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>;
}

impl ComboBoxTraits for ComboBoxText {
    fn set_model_and_first<I, S>(&self, models: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for item in models {
            self.append_text(item.as_ref());
        }
        self.set_active(Some(0));
    }
}

pub fn get_dialog_box_child(dialog: &Dialog) -> GtkBox {
    dialog.child().expect("Dialog has no child").downcast::<GtkBox>().expect("Dialog child is not Box")
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

pub fn get_custom_label_from_widget<P: IsA<Widget>>(item: &P) -> Label {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];

    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(label) = widget.clone().downcast::<Label>() {
            return label;
        }
        widgets_to_check.extend(get_all_direct_children(&widget));
    }
    panic!("Button doesn't have proper custom label child");
}

pub fn get_custom_image_from_widget<P: IsA<Widget>>(item: &P) -> Image {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];

    while let Some(widget) = widgets_to_check.pop() {
        if let Ok(image) = widget.clone().downcast::<Image>() {
            return image;
        }
        widgets_to_check.extend(get_all_direct_children(&widget));
    }
    panic!("Button doesn't have proper custom image child");
}

pub fn get_all_boxes_from_widget<P: IsA<Widget>>(item: &P) -> Vec<GtkBox> {
    let mut widgets_to_check = vec![item.clone().upcast::<Widget>()];
    let mut boxes = Vec::new();

    while let Some(widget) = widgets_to_check.pop() {
        widgets_to_check.extend(get_all_direct_children(&widget));
        if let Ok(bbox) = widget.clone().downcast::<GtkBox>() {
            boxes.push(bbox);
        }
    }
    boxes
}

#[expect(dead_code)]
pub fn debug_print_widget<P: IsA<Widget>>(item: &P) {
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

#[cfg(test)]
mod test {
    use gtk4::Orientation;
    use gtk4::prelude::BoxExt;

    use super::*;

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
}
