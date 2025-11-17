use std::collections::VecDeque;
use std::vec::Vec;

use gtk4::prelude::{ComboBoxExtManual, *};
use gtk4::{Box as GtkBox, ComboBoxText, Dialog, Widget};

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

#[allow(clippy::allow_attributes)]
#[allow(dead_code)]
pub trait WidgetTraits {
    fn get_all_direct_children(&self) -> Vec<Widget>;
    fn get_all_widgets_of_type<T: IsA<Widget>>(&self, recursive: bool) -> Vec<T>;
    fn get_widget_of_type<T: IsA<Widget>>(&self, recursive: bool) -> T;
    fn get_all_boxes(&self) -> Vec<GtkBox>;
    fn debug_print_widget(&self, print_only_direct_children: bool);
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

    fn get_all_widgets_of_type<T: IsA<Widget>>(&self, recursive: bool) -> Vec<T> {
        let mut widgets_to_check = VecDeque::from([self.clone().upcast::<Widget>()]);
        let mut found_widgets = Vec::new();
        let mut is_root = true;

        while let Some(widget) = widgets_to_check.pop_front() {
            if (recursive || !is_root)
                && let Ok(specific_widget) = widget.clone().downcast::<T>()
            {
                found_widgets.push(specific_widget);
            }

            if recursive || is_root {
                widgets_to_check.extend(widget.get_all_direct_children());
            }

            is_root = false;
        }
        found_widgets
    }

    fn get_widget_of_type<T: IsA<Widget>>(&self, recursive: bool) -> T {
        let mut widgets_to_check = VecDeque::from([self.clone().upcast::<Widget>()]);
        let mut is_root = true;

        while let Some(widget) = widgets_to_check.pop_front() {
            if (recursive || !is_root)
                && let Ok(specific_widget) = widget.clone().downcast::<T>()
            {
                return specific_widget;
            }

            if recursive || is_root {
                widgets_to_check.extend(widget.get_all_direct_children());
            }

            is_root = false;
        }
        panic!("Widget doesn't have proper child of specified type");
    }

    fn get_all_boxes(&self) -> Vec<GtkBox> {
        let mut widgets_to_check = VecDeque::from([self.clone().upcast::<Widget>()]);
        let mut boxes = Vec::new();

        while let Some(widget) = widgets_to_check.pop_front() {
            if let Ok(bbox) = widget.clone().downcast::<GtkBox>() {
                boxes.push(bbox);
            }
            widgets_to_check.extend(widget.get_all_direct_children());
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
            let mut result = vec![WidgetInfo { depth, widget: widget.clone() }];

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
    use gtk4::prelude::BoxExt;
    use gtk4::{Image, Label, Orientation};

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
    #[should_panic(expected = "Widget doesn't have proper child of specified type")]
    fn test_get_custom_label_panic() {
        let container = gtk4::Box::new(Orientation::Horizontal, 0);
        let image = gtk4::Image::new();
        container.append(&image);

        container.get_widget_of_type::<Label>(true);
    }

    #[gtk4::test]
    #[should_panic(expected = "Widget doesn't have proper child of specified type")]
    fn test_get_custom_image_panic() {
        let container = gtk4::Box::new(Orientation::Horizontal, 0);
        let label = gtk4::Label::new(Some("Test"));
        container.append(&label);

        container.get_widget_of_type::<Image>(true);
    }

    #[gtk4::test]
    fn test_get_all_widgets_of_type() {
        // Test finding labels recursively
        let root = gtk4::Box::new(Orientation::Horizontal, 0);
        let box1 = gtk4::Box::new(Orientation::Vertical, 0);
        let label1 = gtk4::Label::new(Some("Label 1"));
        let label2 = gtk4::Label::new(Some("Label 2"));
        let image = gtk4::Image::new();
        let label3 = gtk4::Label::new(Some("Label 3"));

        root.append(&box1);
        root.append(&label1);
        box1.append(&label2);
        box1.append(&image);
        box1.append(&label3);

        // Recursive search - finds all labels
        let labels = root.get_all_widgets_of_type::<Label>(true);
        assert_eq!(labels.len(), 3);
        assert_eq!(labels[0].text(), "Label 1");
        assert_eq!(labels[1].text(), "Label 2");
        assert_eq!(labels[2].text(), "Label 3");

        // Non-recursive search - finds only direct children (not root itself)
        let labels_direct = root.get_all_widgets_of_type::<Label>(false);
        assert_eq!(labels_direct.len(), 1);
        assert_eq!(labels_direct[0].text(), "Label 1");

        // Test finding images recursively
        let images = root.get_all_widgets_of_type::<Image>(true);
        assert_eq!(images.len(), 1);

        // Test finding boxes recursively (includes root)
        let boxes = root.get_all_widgets_of_type::<GtkBox>(true);
        assert_eq!(boxes.len(), 2); // root + box1

        // Test finding boxes non-recursively (only direct children, not root)
        let boxes_direct = root.get_all_widgets_of_type::<GtkBox>(false);
        assert_eq!(boxes_direct.len(), 1); // box1 only (direct child)

        // Test empty result
        let root2 = gtk4::Box::new(Orientation::Horizontal, 0);
        root2.append(&gtk4::Image::new());
        let labels2 = root2.get_all_widgets_of_type::<Label>(true);
        assert_eq!(labels2.len(), 0);
    }

    #[gtk4::test]
    fn test_get_widget_of_type() {
        // Test finding first label (breadth-first search) - recursive
        let root = gtk4::Box::new(Orientation::Horizontal, 0);
        let box1 = gtk4::Box::new(Orientation::Vertical, 0);
        let label1 = gtk4::Label::new(Some("First Label"));
        let label2 = gtk4::Label::new(Some("Second Label"));

        root.append(&box1);
        root.append(&label1);
        box1.append(&label2);

        let found_label = root.get_widget_of_type::<Label>(true);
        assert_eq!(found_label.text(), "First Label");

        // Test non-recursive - finds only in direct children
        let found_label_direct = root.get_widget_of_type::<Label>(false);
        assert_eq!(found_label_direct.text(), "First Label");

        // Test finding image recursively
        let root2 = gtk4::Box::new(Orientation::Horizontal, 0);
        let box2 = gtk4::Box::new(Orientation::Vertical, 0);
        let label = gtk4::Label::new(Some("Test"));
        let image = gtk4::Image::new();
        image.set_icon_name(Some("test-icon"));

        root2.append(&box2);
        box2.append(&label);
        box2.append(&image);

        let found_image = root2.get_widget_of_type::<Image>(true);
        assert_eq!(found_image.icon_name(), Some("test-icon".into()));

        // Test finding nested widget recursively
        let root3 = gtk4::Box::new(Orientation::Horizontal, 0);
        let box3 = gtk4::Box::new(Orientation::Vertical, 0);
        let box4 = gtk4::Box::new(Orientation::Horizontal, 0);
        let label3 = gtk4::Label::new(Some("Nested Label"));

        root3.append(&box3);
        box3.append(&box4);
        box4.append(&label3);

        let found_label3 = root3.get_widget_of_type::<Label>(true);
        assert_eq!(found_label3.text(), "Nested Label");

        // Test non-recursive on nested - should find box3 (direct child of root3)
        let found_box = root3.get_widget_of_type::<GtkBox>(false);
        assert_eq!(found_box.orientation(), Orientation::Vertical);
    }

    #[gtk4::test]
    #[should_panic(expected = "Widget doesn't have proper child of specified type")]
    fn test_get_widget_of_type_panic() {
        let root = gtk4::Box::new(Orientation::Horizontal, 0);
        let image = gtk4::Image::new();
        root.append(&image);

        root.get_widget_of_type::<Label>(true);
    }

    #[gtk4::test]
    #[should_panic(expected = "Widget doesn't have proper child of specified type")]
    fn test_get_widget_of_type_panic_non_recursive() {
        let root = gtk4::Box::new(Orientation::Horizontal, 0);
        let box1 = gtk4::Box::new(Orientation::Vertical, 0);
        let label = gtk4::Label::new(Some("Nested"));

        root.append(&box1);
        box1.append(&label);

        root.get_widget_of_type::<Label>(false);
    }
}
