use std::cell::RefCell;
use std::rc::Rc;

use czkawka_core::common::image::get_dynamic_image_from_path;
use czkawka_core::localizer_core::generate_translation_hashmap;
use gdk4::gdk_pixbuf::{InterpType, Pixbuf};
use gtk4::prelude::*;
use gtk4::{
    Builder, CellRendererText, CellRendererToggle, CheckButton, EventControllerKey, GestureClick, ListStore, Notebook, Picture, ScrolledWindow, SelectionMode, TextView, TreeView,
    TreeViewColumn,
};

use crate::connect_things::connect_button_delete::delete_things;
use crate::flg;
use crate::gui_structs::gui_data::GuiData;
use crate::help_functions::{
    ColumnsBadExtensions, ColumnsBigFiles, ColumnsBrokenFiles, ColumnsDuplicates, ColumnsEmptyFiles, ColumnsEmptyFolders, ColumnsInvalidSymlinks, ColumnsSameMusic,
    ColumnsSimilarImages, ColumnsSimilarVideos, ColumnsTemporaryFiles, KEY_DELETE, add_text_to_text_view, get_full_name_from_path_name, get_pixbuf_from_dynamic_image,
    resize_pixbuf_dimension,
};
use crate::notebook_enums::NotebookMainEnum;
use crate::notebook_info::{NOTEBOOKS_INFO, NotebookObject};
use crate::opening_selecting_records::{opening_double_click_function, opening_enter_function_ported, opening_middle_mouse_function, select_function_header};

#[derive(Clone)]
pub struct CommonTreeViews {
    pub subviews: Vec<SubView>,
    pub notebook_main: Notebook,
    pub preview_path: Rc<RefCell<String>>,
}
impl CommonTreeViews {
    pub fn get_subview(&self, item: NotebookMainEnum) -> &SubView {
        self.subviews.iter().find(|s| s.enum_value == item).expect("Cannot find subview")
    }
    pub fn get_current_page(&self) -> NotebookMainEnum {
        let current_page = self.notebook_main.current_page().expect("Cannot get current page from notebook");
        NOTEBOOKS_INFO[current_page as usize].notebook_type
    }
    pub fn get_current_subview(&self) -> &SubView {
        let current_page = self.notebook_main.current_page().expect("Cannot get current page from notebook");
        let enum_value = NOTEBOOKS_INFO[current_page as usize].notebook_type;
        self.get_subview(enum_value)
    }
    // pub fn get_tree_view_from_its_name(&self, name: &str) -> TreeView {
    //     for subview in &self.subviews {
    //         if subview.tree_view_name == name {
    //             return subview.tree_view.clone();
    //         }
    //     }
    //     panic!("Cannot find tree view with name {name}");
    // }
    pub fn setup(&self, gui_data: &GuiData) {
        for subview in &self.subviews {
            subview.setup(&self.preview_path, gui_data);
        }
    }
}

pub trait TreeViewListStoreTrait {
    fn get_model(&self) -> ListStore;
}
impl TreeViewListStoreTrait for TreeView {
    fn get_model(&self) -> ListStore {
        self.model()
            .expect("TreeView has no model")
            .downcast_ref::<ListStore>()
            .expect("TreeView model is not ListStore")
            .clone()
    }
}
pub trait GetTreeViewTrait {
    fn get_tree_view(&self) -> TreeView;
}
impl GetTreeViewTrait for &EventControllerKey {
    fn get_tree_view(&self) -> TreeView {
        self.widget()
            .expect("EventControllerKey has no widget")
            .downcast_ref::<TreeView>()
            .expect("EventControllerKey widget is not TreeView")
            .clone()
    }
}
impl GetTreeViewTrait for &GestureClick {
    fn get_tree_view(&self) -> TreeView {
        self.widget()
            .expect("GestureClick has no widget")
            .downcast_ref::<TreeView>()
            .expect("GestureClick widget is not TreeView")
            .clone()
    }
}

#[derive(Clone)]
pub struct SubView {
    pub scrolled_window: ScrolledWindow,
    pub tree_view: TreeView,
    pub gesture_click: GestureClick,
    pub event_controller_key: EventControllerKey,
    pub nb_object: NotebookObject,
    pub enum_value: NotebookMainEnum,
    pub tree_view_name: String,
    pub preview_struct: Option<PreviewStruct>,
}

#[derive(Clone)]
pub struct PreviewStruct {
    pub image_preview: Picture,
    pub settings_show_preview: CheckButton,
}

impl SubView {
    pub fn new(
        builder: &Builder,
        scrolled_name: &str,
        enum_value: NotebookMainEnum,
        preview_str: Option<&str>,
        tree_view_name: &str,
        settings_show_preview: Option<CheckButton>,
    ) -> Self {
        let tree_view: TreeView = TreeView::new();
        let event_controller_key: EventControllerKey = EventControllerKey::new();
        tree_view.add_controller(event_controller_key.clone());
        let gesture_click: GestureClick = GestureClick::new();
        tree_view.add_controller(gesture_click.clone());

        let image_preview = preview_str.map(|name| builder.object(name).unwrap_or_else(|| panic!("Cannot find preview image {name}")));

        let nb_object = NOTEBOOKS_INFO[enum_value as usize].clone();
        assert_eq!(nb_object.notebook_type, enum_value);

        let preview_struct = if let (Some(image_preview), Some(settings_show_preview)) = (image_preview, settings_show_preview) {
            Some(PreviewStruct {
                image_preview,
                settings_show_preview,
            })
        } else {
            None
        };

        Self {
            scrolled_window: builder.object(scrolled_name).unwrap_or_else(|| panic!("Cannot find scrolled window {scrolled_name}")),
            tree_view,
            gesture_click,
            event_controller_key,
            nb_object,
            enum_value,
            preview_struct,
            tree_view_name: tree_view_name.to_string(),
        }
    }

    fn _setup_tree_view(&self) {
        self.tree_view.set_model(Some(&ListStore::new(self.nb_object.columns_types)));
        self.tree_view.selection().set_mode(SelectionMode::Multiple);

        if let Some(column_header) = self.nb_object.column_header {
            self.tree_view.selection().set_select_function(select_function_header(column_header));
        }

        self.tree_view.set_vexpand(true);

        self._setup_tree_view_config();

        self.tree_view.set_widget_name(&self.tree_view_name);
        self.scrolled_window.set_child(Some(&self.tree_view));
        self.scrolled_window.show();
    }
    fn _setup_gesture_click(&self) {
        self.gesture_click.set_button(0);
        self.gesture_click.connect_pressed(opening_double_click_function);
        self.gesture_click.connect_released(opening_middle_mouse_function); // TODO GTK 4 - https://github.com/gtk-rs/gtk4-rs/issues/1043
    }

    fn _setup_evk(&self, gui_data: &GuiData) {
        let gui_data_clone = gui_data.clone();
        self.event_controller_key.connect_key_pressed(opening_enter_function_ported);

        self.event_controller_key
            .connect_key_released(move |_event_controller_key, _key_value, key_code, _modifier_type| {
                if key_code == KEY_DELETE {
                    glib::MainContext::default().spawn_local(delete_things(gui_data_clone.clone()));
                }
            });
    }

    fn _connect_show_mouse_preview(&self, gui_data: &GuiData, preview_path: &Rc<RefCell<String>>) {
        // TODO GTK 4, currently not works, connect_pressed shows previous thing - https://gitlab.gnome.org/GNOME/gtk/-/issues/4939
        // Use connect_released when it will be fixed, currently using connect_row_activated workaround
        let use_rust_preview = gui_data.settings.check_button_settings_use_rust_preview.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        if let Some(preview_struct) = self.preview_struct.clone() {
            self.tree_view.set_property("activate-on-single-click", true);
            let preview_path = preview_path.clone();
            let nb_object = self.nb_object.clone();

            self.tree_view.clone().connect_row_activated(move |tree_view, _b, _c| {
                show_preview(
                    tree_view,
                    &text_view_errors,
                    &preview_struct.settings_show_preview,
                    &preview_struct.image_preview,
                    &preview_path,
                    nb_object.column_path,
                    nb_object.column_name,
                    use_rust_preview.is_active(),
                );
            });
        }
    }
    fn _connect_show_keyboard_preview(&self, gui_data: &GuiData, preview_path: &Rc<RefCell<String>>, preview_struct: &PreviewStruct) {
        let use_rust_preview = gui_data.settings.check_button_settings_use_rust_preview.clone();
        let text_view_errors = gui_data.text_view_errors.clone();
        let check_button_settings_show_preview = preview_struct.settings_show_preview.clone();
        let image_preview = preview_struct.image_preview.clone();
        let gui_data_clone = gui_data.clone();

        self.event_controller_key.connect_key_pressed(opening_enter_function_ported);
        let preview_path = preview_path.clone();
        let nb_object = self.nb_object.clone();

        self.event_controller_key
            .clone()
            .connect_key_released(move |event_controller_key, _key_value, key_code, _modifier_type| {
                if key_code == KEY_DELETE {
                    glib::MainContext::default().spawn_local(delete_things(gui_data_clone.clone()));
                }
                show_preview(
                    &event_controller_key.get_tree_view(),
                    &text_view_errors,
                    &check_button_settings_show_preview,
                    &image_preview,
                    &preview_path,
                    nb_object.column_path,
                    nb_object.column_name,
                    use_rust_preview.is_active(),
                );
            });
    }

    fn setup(&self, preview_path: &Rc<RefCell<String>>, gui_data: &GuiData) {
        if let Some(preview_struct) = &self.preview_struct {
            preview_struct.image_preview.hide();
        }
        self._setup_tree_view();
        self._setup_gesture_click();
        self._connect_show_mouse_preview(gui_data, preview_path);

        // Items with image preview, are differently handled
        if let Some(preview_struct) = &self.preview_struct {
            self._connect_show_keyboard_preview(gui_data, preview_path, preview_struct);
        } else {
            self._setup_evk(gui_data);
        }
    }
    fn _setup_tree_view_config(&self) {
        let tree_view = &self.tree_view;
        let model = tree_view.get_model();
        match self.enum_value {
            NotebookMainEnum::Duplicate => {
                let columns_colors = (ColumnsDuplicates::Color as i32, ColumnsDuplicates::TextColor as i32);
                let activatable_colors = (ColumnsDuplicates::ActivatableSelectButton as i32, ColumnsDuplicates::Color as i32);
                create_default_selection_button_column(tree_view, ColumnsDuplicates::SelectionButton as i32, model, Some(activatable_colors));
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsDuplicates::Size as i32, ColumnSort::None),
                        (ColumnsDuplicates::Name as i32, ColumnSort::None),
                        (ColumnsDuplicates::Path as i32, ColumnSort::None),
                        (ColumnsDuplicates::Modification as i32, ColumnSort::None),
                    ],
                    Some(columns_colors),
                );
                assert_eq!(tree_view.columns().len(), 5);
            }
            NotebookMainEnum::EmptyDirectories => {
                create_default_selection_button_column(tree_view, ColumnsEmptyFolders::SelectionButton as i32, model, None);
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsEmptyFolders::Name as i32, ColumnSort::Default),
                        (ColumnsEmptyFolders::Path as i32, ColumnSort::Default),
                        (ColumnsEmptyFolders::Modification as i32, ColumnSort::Custom(ColumnsEmptyFolders::ModificationAsSecs as i32)),
                    ],
                    None,
                );
                assert_eq!(tree_view.columns().len(), 4);
            }
            NotebookMainEnum::BigFiles => {
                create_default_selection_button_column(tree_view, ColumnsBigFiles::SelectionButton as i32, model, None);
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsBigFiles::Size as i32, ColumnSort::Default),
                        (ColumnsBigFiles::Name as i32, ColumnSort::Default),
                        (ColumnsBigFiles::Path as i32, ColumnSort::Default),
                        (ColumnsBigFiles::Modification as i32, ColumnSort::Custom(ColumnsBigFiles::ModificationAsSecs as i32)),
                    ],
                    None,
                );
                assert_eq!(tree_view.columns().len(), 5);
            }
            NotebookMainEnum::EmptyFiles => {
                create_default_selection_button_column(tree_view, ColumnsEmptyFiles::SelectionButton as i32, model, None);
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsEmptyFiles::Name as i32, ColumnSort::Default),
                        (ColumnsEmptyFiles::Path as i32, ColumnSort::Default),
                        (ColumnsEmptyFiles::Modification as i32, ColumnSort::Custom(ColumnsEmptyFiles::ModificationAsSecs as i32)),
                    ],
                    None,
                );
                assert_eq!(tree_view.columns().len(), 4);
            }
            NotebookMainEnum::Temporary => {
                create_default_selection_button_column(tree_view, ColumnsTemporaryFiles::SelectionButton as i32, model, None);
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsTemporaryFiles::Name as i32, ColumnSort::Default),
                        (ColumnsTemporaryFiles::Path as i32, ColumnSort::Default),
                        (
                            ColumnsTemporaryFiles::Modification as i32,
                            ColumnSort::Custom(ColumnsTemporaryFiles::ModificationAsSecs as i32),
                        ),
                    ],
                    None,
                );
                assert_eq!(tree_view.columns().len(), 4);
            }
            NotebookMainEnum::SimilarImages => {
                let columns_colors = (ColumnsSimilarImages::Color as i32, ColumnsSimilarImages::TextColor as i32);
                let activatable_colors = (ColumnsSimilarImages::ActivatableSelectButton as i32, ColumnsSimilarImages::Color as i32);
                create_default_selection_button_column(tree_view, ColumnsSimilarImages::SelectionButton as i32, model, Some(activatable_colors));
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsSimilarImages::Similarity as i32, ColumnSort::None),
                        (ColumnsSimilarImages::Size as i32, ColumnSort::None),
                        (ColumnsSimilarImages::Dimensions as i32, ColumnSort::None),
                        (ColumnsSimilarImages::Name as i32, ColumnSort::None),
                        (ColumnsSimilarImages::Path as i32, ColumnSort::None),
                        (ColumnsSimilarImages::Modification as i32, ColumnSort::None),
                    ],
                    Some(columns_colors),
                );
                assert_eq!(tree_view.columns().len(), 7);
            }
            NotebookMainEnum::SimilarVideos => {
                let columns_colors = (ColumnsSimilarVideos::Color as i32, ColumnsSimilarVideos::TextColor as i32);
                let activatable_colors = (ColumnsSimilarVideos::ActivatableSelectButton as i32, ColumnsSimilarVideos::Color as i32);
                create_default_selection_button_column(tree_view, ColumnsSimilarVideos::SelectionButton as i32, model, Some(activatable_colors));
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsSimilarVideos::Size as i32, ColumnSort::None),
                        (ColumnsSimilarVideos::Name as i32, ColumnSort::None),
                        (ColumnsSimilarVideos::Path as i32, ColumnSort::None),
                        (ColumnsSimilarVideos::Modification as i32, ColumnSort::None),
                    ],
                    Some(columns_colors),
                );
                assert_eq!(tree_view.columns().len(), 5);
            }
            NotebookMainEnum::SameMusic => {
                let columns_colors = (ColumnsSameMusic::Color as i32, ColumnsSameMusic::TextColor as i32);
                let activatable_colors = (ColumnsSameMusic::ActivatableSelectButton as i32, ColumnsSameMusic::Color as i32);
                create_default_selection_button_column(tree_view, ColumnsSameMusic::SelectionButton as i32, model, Some(activatable_colors));
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsSameMusic::Size as i32, ColumnSort::None),
                        (ColumnsSameMusic::Name as i32, ColumnSort::None),
                        (ColumnsSameMusic::Title as i32, ColumnSort::None),
                        (ColumnsSameMusic::Artist as i32, ColumnSort::None),
                        (ColumnsSameMusic::Year as i32, ColumnSort::None),
                        (ColumnsSameMusic::Bitrate as i32, ColumnSort::None),
                        (ColumnsSameMusic::Length as i32, ColumnSort::None),
                        (ColumnsSameMusic::Genre as i32, ColumnSort::None),
                        (ColumnsSameMusic::Path as i32, ColumnSort::None),
                        (ColumnsSameMusic::Modification as i32, ColumnSort::None),
                    ],
                    Some(columns_colors),
                );
                assert_eq!(tree_view.columns().len(), 11);
            }
            NotebookMainEnum::Symlinks => {
                create_default_selection_button_column(tree_view, ColumnsInvalidSymlinks::SelectionButton as i32, model, None);
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsInvalidSymlinks::Name as i32, ColumnSort::Default),
                        (ColumnsInvalidSymlinks::Path as i32, ColumnSort::Default),
                        (ColumnsInvalidSymlinks::DestinationPath as i32, ColumnSort::Default),
                        (ColumnsInvalidSymlinks::TypeOfError as i32, ColumnSort::Default),
                        (
                            ColumnsInvalidSymlinks::Modification as i32,
                            ColumnSort::Custom(ColumnsInvalidSymlinks::ModificationAsSecs as i32),
                        ),
                    ],
                    None,
                );
                assert_eq!(tree_view.columns().len(), 6);
            }
            NotebookMainEnum::BrokenFiles => {
                create_default_selection_button_column(tree_view, ColumnsBrokenFiles::SelectionButton as i32, model, None);
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsBrokenFiles::Name as i32, ColumnSort::Default),
                        (ColumnsBrokenFiles::Path as i32, ColumnSort::Default),
                        (ColumnsBrokenFiles::ErrorType as i32, ColumnSort::Default),
                        (ColumnsBrokenFiles::Modification as i32, ColumnSort::Custom(ColumnsBrokenFiles::ModificationAsSecs as i32)),
                    ],
                    None,
                );
                assert_eq!(tree_view.columns().len(), 5);
            }
            NotebookMainEnum::BadExtensions => {
                create_default_selection_button_column(tree_view, ColumnsBadExtensions::SelectionButton as i32, model, None);
                create_default_columns(
                    tree_view,
                    &[
                        (ColumnsBadExtensions::Name as i32, ColumnSort::Default),
                        (ColumnsBadExtensions::Path as i32, ColumnSort::Default),
                        (ColumnsBadExtensions::CurrentExtension as i32, ColumnSort::Default),
                        (ColumnsBadExtensions::ValidExtensions as i32, ColumnSort::Default),
                    ],
                    None,
                );
                assert_eq!(tree_view.columns().len(), 5);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum ColumnSort {
    None,
    Default,
    Custom(i32),
}

pub(crate) fn create_default_selection_button_column(
    tree_view: &TreeView,
    column_id: i32,
    model: ListStore,
    activatable_color_columns: Option<(i32, i32)>,
) -> (CellRendererToggle, TreeViewColumn) {
    let renderer = CellRendererToggle::new();
    renderer.connect_toggled(move |_r, path| {
        let iter = model.iter(&path).expect("Failed to get iter from tree_path");
        let mut fixed = model.get::<bool>(&iter, column_id);
        fixed = !fixed;
        model.set_value(&iter, column_id as u32, &fixed.to_value());
    });
    let column = TreeViewColumn::new();
    column.pack_start(&renderer, true);
    column.set_resizable(false);
    column.set_fixed_width(30);
    column.add_attribute(&renderer, "active", column_id);
    if let Some(activatable_color_columns) = activatable_color_columns {
        column.add_attribute(&renderer, "activatable", activatable_color_columns.0);
        column.add_attribute(&renderer, "cell-background", activatable_color_columns.1);
    }
    tree_view.append_column(&column);
    (renderer, column)
}

pub(crate) fn create_default_columns(tree_view: &TreeView, columns: &[(i32, ColumnSort)], colors_columns_id: Option<(i32, i32)>) {
    for (col_id, sort_method) in columns {
        let renderer = CellRendererText::new();
        let column: TreeViewColumn = TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_resizable(true);
        column.set_min_width(50);
        column.add_attribute(&renderer, "text", *col_id);
        match sort_method {
            ColumnSort::None => {}
            ColumnSort::Default => column.set_sort_column_id(*col_id),
            ColumnSort::Custom(val) => column.set_sort_column_id(*val),
        }
        if let Some(colors_columns_id) = colors_columns_id {
            column.add_attribute(&renderer, "background", colors_columns_id.0);
            column.add_attribute(&renderer, "foreground", colors_columns_id.1);
        }
        tree_view.append_column(&column);
    }
}

pub(crate) fn show_preview(
    tree_view: &TreeView,
    text_view_errors: &TextView,
    check_button_settings_show_preview: &CheckButton,
    image_preview: &Picture,
    preview_path: &Rc<RefCell<String>>,
    column_path: i32,
    column_name: i32,
    use_rust_preview: bool,
) {
    let (selected_rows, tree_model) = tree_view.selection().selected_rows();

    let mut created_image = false;

    // Only show preview when selected is only one item, because there is no method to recognize current clicked item in multiselection
    if selected_rows.len() == 1 && check_button_settings_show_preview.is_active() {
        let tree_path = selected_rows[0].clone();
        // TODO labels on {} are in testing stage, so we just ignore for now this warning until found better idea how to fix this
        #[expect(clippy::never_loop)]
        'dir: loop {
            let path = tree_model.get::<String>(&tree_model.iter(&tree_path).expect("Invalid tree_path"), column_path);
            let name = tree_model.get::<String>(&tree_model.iter(&tree_path).expect("Invalid tree_path"), column_name);

            let file_name = get_full_name_from_path_name(&path, &name);

            if file_name == preview_path.borrow().as_str() {
                return; // Preview is already created, no need to recreate it
            }

            let mut pixbuf = if use_rust_preview {
                let image = match get_dynamic_image_from_path(&file_name) {
                    Ok(t) => t,
                    Err(e) => {
                        add_text_to_text_view(text_view_errors, flg!("preview_image_opening_failure", name = file_name, reason = e).as_str());
                        break 'dir;
                    }
                };

                match get_pixbuf_from_dynamic_image(&image) {
                    Ok(t) => t,
                    Err(e) => {
                        add_text_to_text_view(text_view_errors, flg!("preview_image_opening_failure", name = file_name, reason = e).as_str());
                        break 'dir;
                    }
                }
            } else {
                match Pixbuf::from_file(&file_name) {
                    Ok(pixbuf) => pixbuf,
                    Err(e) => {
                        add_text_to_text_view(
                            text_view_errors,
                            flg!(
                                "preview_image_opening_failure",
                                generate_translation_hashmap(vec![("name", file_name), ("reason", e.to_string())])
                            )
                            .as_str(),
                        );
                        break 'dir;
                    }
                }
            };
            pixbuf = match resize_pixbuf_dimension(&pixbuf, (800, 800), InterpType::Bilinear) {
                None => {
                    add_text_to_text_view(text_view_errors, flg!("preview_image_resize_failure", name = file_name).as_str());
                    break 'dir;
                }
                Some(pixbuf) => pixbuf,
            };

            image_preview.set_pixbuf(Some(&pixbuf));
            {
                let mut preview_path = preview_path.borrow_mut();
                *preview_path = file_name;
            }

            created_image = true;

            break 'dir;
        }
    }
    if created_image {
        image_preview.show();
    } else {
        image_preview.hide();
        {
            let mut preview_path = preview_path.borrow_mut();
            *preview_path = String::new();
        }
    }
}
