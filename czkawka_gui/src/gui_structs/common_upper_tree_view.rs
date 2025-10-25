use glib::Type;
use gtk4::prelude::*;
use gtk4::{Builder, EventControllerKey, GestureClick, ScrolledWindow, SelectionMode, TreeView};

use crate::gui_structs::common_tree_view::{ColumnSort, TreeViewListStoreTrait, create_default_columns, create_default_selection_button_column};
use crate::help_functions::{ColumnsExcludedDirectory, ColumnsIncludedDirectory, KEY_DELETE};
use crate::notebook_enums::NotebookUpperEnum;
use crate::opening_selecting_records::{opening_double_click_function_directories, opening_enter_function_ported_upper_directories};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UpperTreeViewEnum {
    IncludedDirectories,
    ExcludedDirectories,
}

#[derive(Clone)]
pub struct CommonUpperTreeViews {
    pub subviews: Vec<UpperSubView>,
}

#[derive(Clone)]
pub struct UpperSubView {
    pub scrolled_window: ScrolledWindow,
    pub tree_view: TreeView,
    pub gesture_click: GestureClick,
    pub event_controller_key: EventControllerKey,
    pub enum_value: NotebookUpperEnum,
    pub upper_tree_view_enum: UpperTreeViewEnum,
    pub tree_view_name: &'static str,
}

impl CommonUpperTreeViews {
    pub fn get_subview(&self, item: UpperTreeViewEnum) -> &UpperSubView {
        self.subviews.iter().find(|s| s.upper_tree_view_enum == item).expect("Cannot find subview")
    }
    pub fn get_tree_view(&self, item: UpperTreeViewEnum) -> &TreeView {
        &self.get_subview(item).tree_view
    }
    // pub fn get_current_page(&self) -> Option<NotebookMainEnum {
    //     let current_page = self.notebook_main.current_page().expect("Cannot get current page from notebook");
    //     NOTEBOOKS_INFO[current_page as usize].notebook_type
    // }
    pub fn setup(&self) {
        for subview in &self.subviews {
            subview.setup();
        }
    }
}

impl UpperSubView {
    // pub fn get_model(&self) -> ListStore {
    //     self.tree_view.get_model()
    // }
    // pub fn get_tree_model(&self) -> TreeModel {
    //     self.tree_view.model().expect("TreeView has no model")
    // }
    // pub fn get_tree_selection(&self) -> TreeSelection {
    //     self.tree_view.selection()
    // }
    pub fn new(builder: &Builder, scrolled_name: &str, enum_value: NotebookUpperEnum, upper_tree_view_enum: UpperTreeViewEnum, tree_view_name: &'static str) -> Self {
        let tree_view: TreeView = TreeView::new();
        let event_controller_key: EventControllerKey = EventControllerKey::new();
        tree_view.add_controller(event_controller_key.clone());
        let gesture_click: GestureClick = GestureClick::new();
        tree_view.add_controller(gesture_click.clone());

        Self {
            scrolled_window: builder.object(scrolled_name).unwrap_or_else(|| panic!("Cannot find scrolled window {scrolled_name}")),
            tree_view,
            gesture_click,
            event_controller_key,
            enum_value,
            tree_view_name,
            upper_tree_view_enum,
        }
    }

    fn _setup_tree_view(&self) {
        self._setup_tree_view_config();
        self.tree_view.selection().set_mode(SelectionMode::Multiple);

        self.tree_view.set_vexpand(true);

        self.tree_view.set_widget_name(self.tree_view_name);
        self.scrolled_window.set_child(Some(&self.tree_view));
        self.scrolled_window.set_visible(true);
    }
    fn _setup_gesture_click(&self) {
        self.gesture_click.connect_pressed(opening_double_click_function_directories);
    }

    fn _setup_evk(&self) {
        let tree_view = self.tree_view.clone();
        self.event_controller_key.connect_key_pressed(opening_enter_function_ported_upper_directories);
        self.event_controller_key
            .connect_key_released(move |_event_controller_key, _key_value, key_code, _modifier_type| {
                if key_code == KEY_DELETE {
                    let list_store = tree_view.get_model();
                    let selection = tree_view.selection();

                    let (vec_tree_path, _tree_model) = selection.selected_rows();

                    for tree_path in vec_tree_path.iter().rev() {
                        list_store.remove(&list_store.iter(tree_path).expect("Using invalid tree_path"));
                    }
                }
            });
    }

    fn setup(&self) {
        self._setup_tree_view();
        self._setup_gesture_click();
        self._setup_evk();
    }
    fn _setup_tree_view_config(&self) {
        let tree_view = &self.tree_view;
        match self.upper_tree_view_enum {
            UpperTreeViewEnum::IncludedDirectories => {
                let col_types: [Type; 2] = [
                    Type::STRING, // Path
                    Type::BOOL,   // ReferenceButton
                ];
                let model: gtk4::ListStore = gtk4::ListStore::new(&col_types);
                tree_view.set_model(Some(&model));

                create_default_columns(tree_view, &[(ColumnsIncludedDirectory::Path as i32, ColumnSort::Default)], None);
                create_default_selection_button_column(tree_view, ColumnsIncludedDirectory::ReferenceButton as i32, model, None);
            }
            UpperTreeViewEnum::ExcludedDirectories => {
                let col_types: [Type; 1] = [Type::STRING];
                let list_store: gtk4::ListStore = gtk4::ListStore::new(&col_types);
                tree_view.set_model(Some(&list_store));

                tree_view.set_headers_visible(false);
                create_default_columns(tree_view, &[(ColumnsExcludedDirectory::Path as i32, ColumnSort::Default)], None);
            }
        }
    }
}
