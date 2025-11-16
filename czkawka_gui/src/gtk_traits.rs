use std::vec::Vec;

use gtk4::prelude::{ComboBoxExtManual, *};
use gtk4::{Box as GtkBox, ComboBoxText, Dialog, Image, Label, Widget};
use log::{debug, info};

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

pub trait DialogTraits {
    fn get_box_child(&self) -> GtkBox;
}

impl DialogTraits for Dialog {
    fn get_box_child(&self) -> GtkBox {
        self.child().expect("Dialog has no child").downcast::<GtkBox>().expect("Dialog child is not Box")
    }
}

pub trait WidgetTraits {
    fn get_all_direct_children(&self) -> Vec<Widget>;
    fn get_custom_label(&self) -> Label;
    fn get_custom_image(&self) -> Image;
    fn get_all_boxes(&self) -> Vec<GtkBox>;
    #[expect(dead_code)]
    fn debug_print_widget(&self, print_only_direct_children: bool);
    fn get_all_widgets_of_type<T: IsA<Widget>>(&self) -> Vec<T>;
}

impl<P: IsA<Widget>> WidgetTraits for P {
    fn get_all_direct_children(&self) -> Vec<Widget> {
        let mut vector = vec![];
        if let Some(mut child) = self.first_child() {
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

    fn get_all_widgets_of_type<T: IsA<Widget>>(&self) -> Vec<T> {
        let mut widgets_to_check = vec![self.clone().upcast::<Widget>()];
        let mut found_widgets = Vec::new();

        while let Some(widget) = widgets_to_check.pop() {
            widgets_to_check.extend(widget.get_all_direct_children());
            if let Ok(specific_widget) = widget.clone().downcast::<T>() {
                found_widgets.push(specific_widget);
            }
        }
        found_widgets
    }

    fn get_custom_label(&self) -> Label {
        let mut widgets_to_check = vec![self.clone().upcast::<Widget>()];

        while let Some(widget) = widgets_to_check.pop() {
            if let Ok(label) = widget.clone().downcast::<Label>() {
                return label;
            }
            widgets_to_check.extend(widget.get_all_direct_children());
        }
        panic!("Widget doesn't have proper custom label child");
    }

    fn get_custom_image(&self) -> Image {
        let mut widgets_to_check = vec![self.clone().upcast::<Widget>()];

        while let Some(widget) = widgets_to_check.pop() {
            if let Ok(image) = widget.clone().downcast::<Image>() {
                return image;
            }
            widgets_to_check.extend(widget.get_all_direct_children());
        }
        panic!("Widget doesn't have proper custom image child");
    }

    fn get_all_boxes(&self) -> Vec<GtkBox> {
        let mut widgets_to_check = vec![self.clone().upcast::<Widget>()];
        let mut boxes = Vec::new();

        while let Some(widget) = widgets_to_check.pop() {
            widgets_to_check.extend(widget.get_all_direct_children());
            if let Ok(bbox) = widget.clone().downcast::<GtkBox>() {
                boxes.push(bbox);
            }
        }
        boxes
    }

    #[expect(clippy::print_stdout)]
    fn debug_print_widget(&self, print_only_direct_children: bool) {
        struct WidgetInfo {
            depth: usize,
            widget: Widget,
        }

        fn collect_widgets(widget: &Widget, depth: usize, print_only_direct_children: bool) -> Vec<WidgetInfo> {
            let mut result = vec![WidgetInfo {
                depth,
                widget: widget.clone(),
            }];

            // If print_only_direct_children is true, only process children at depth 0
            // Otherwise, recurse through all descendants
            if !print_only_direct_children || depth == 0 {
                for child in widget.get_all_direct_children() {
                    result.extend(collect_widgets(&child, depth + 1, print_only_direct_children));
                }
            }

            result
        }

        let widget_infos = collect_widgets(&self.clone().upcast::<Widget>(), 0, print_only_direct_children);

        println!("Widget hierarchy:");

        for widget_info in widget_infos {
            let indent = "  ".repeat(widget_info.depth);
            println!("{}{:?}", indent, widget_info.widget);
        }
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
        assert_eq!(obj.get_all_direct_children().len(), 2);
    }

    #[gtk4::test]
    fn test_get_all_boxes() {
        let obj = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj2 = gtk4::Box::new(Orientation::Horizontal, 0);
        let obj3 = gtk4::Image::new();
        let obj4 = gtk4::Image::new();
        let obj5 = gtk4::Image::new();
        obj.append(&obj2);
        obj.append(&obj3);
        obj2.append(&obj4);
        obj2.append(&obj5);
        assert_eq!(obj.get_all_boxes().len(), 2);
    }

    #[gtk4::test]
    fn test_get_custom_label() {
        let container = gtk4::Box::new(Orientation::Horizontal, 0);
        let inner_box = gtk4::Box::new(Orientation::Vertical, 0);
        let label = gtk4::Label::new(Some("Test Label"));
        let image = gtk4::Image::new();

        container.append(&inner_box);
        container.append(&image);
        inner_box.append(&label);

        let found_label = container.get_custom_label();
        assert_eq!(found_label.text(), "Test Label");
    }

    #[gtk4::test]
    fn test_get_custom_image() {
        let container = gtk4::Box::new(Orientation::Horizontal, 0);
        let inner_box = gtk4::Box::new(Orientation::Vertical, 0);
        let image = gtk4::Image::new();
        let label = gtk4::Label::new(Some("Test"));

        container.append(&inner_box);
        container.append(&label);
        inner_box.append(&image);

        let found_image = container.get_custom_image();
        assert_eq!(found_image, image);
    }

    #[gtk4::test]
    fn test_get_all_direct_children_empty() {
        let obj = gtk4::Box::new(Orientation::Horizontal, 0);
        assert_eq!(obj.get_all_direct_children().len(), 0);
    }

    #[gtk4::test]
    fn test_get_all_boxes_nested() {
        let root = gtk4::Box::new(Orientation::Horizontal, 0);
        let box1 = gtk4::Box::new(Orientation::Vertical, 0);
        let box2 = gtk4::Box::new(Orientation::Horizontal, 0);
        let box3 = gtk4::Box::new(Orientation::Vertical, 0);

        root.append(&box1);
        box1.append(&box2);
        box2.append(&box3);

        assert_eq!(root.get_all_boxes().len(), 4);
    }

    #[gtk4::test]
    fn test_get_all_boxes_with_mixed_widgets() {
        let root = gtk4::Box::new(Orientation::Horizontal, 0);
        let box1 = gtk4::Box::new(Orientation::Vertical, 0);
        let label = gtk4::Label::new(Some("Test"));
        let image = gtk4::Image::new();
        let box2 = gtk4::Box::new(Orientation::Horizontal, 0);

        root.append(&box1);
        root.append(&label);
        root.append(&image);
        box1.append(&box2);

        assert_eq!(root.get_all_boxes().len(), 3);
    }

    #[gtk4::test]
    fn test_combo_box_set_model_and_first() {
        let combo = gtk4::ComboBoxText::new();
        combo.set_model_and_first(["Option 1", "Option 2", "Option 3"]);

        assert_eq!(combo.active(), Some(0));
        assert_eq!(combo.active_text().unwrap(), "Option 1");
    }

    #[gtk4::test]
    fn test_dialog_get_box_child() {
        let dialog = gtk4::Dialog::new();

        let result = dialog.get_box_child();
        assert_eq!(result.spacing(), 0);
    }

    #[gtk4::test]
    #[should_panic(expected = "Widget doesn't have proper custom label child")]
    fn test_get_custom_label_panic() {
        let container = gtk4::Box::new(Orientation::Horizontal, 0);
        let image = gtk4::Image::new();
        container.append(&image);

        // This should panic because there's no label
        container.get_custom_label();
    }

    #[gtk4::test]
    #[should_panic(expected = "Widget doesn't have proper custom image child")]
    fn test_get_custom_image_panic() {
        let container = gtk4::Box::new(Orientation::Horizontal, 0);
        let label = gtk4::Label::new(Some("Test"));
        container.append(&label);

        // This should panic because there's no image
        container.get_custom_image();
    }
}
