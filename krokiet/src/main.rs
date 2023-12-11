// Remove console window in Windows OS
#![windows_subsystem = "windows"]
#![allow(unknown_lints)] // May be disabled, but locally I use nightly clippy
#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::should_panic_without_expect)]
#![allow(clippy::struct_field_names)] // Generated code
#![allow(clippy::overly_complex_bool_expr)] // Generated code
#![allow(clippy::semicolon_if_nothing_returned)] // Generated code
#![allow(clippy::used_underscore_binding)] // Generated code
#![allow(clippy::unreadable_literal)] // Generated code
#![allow(clippy::float_cmp)] // Generated code
#![allow(clippy::no_effect_underscore_binding)] // Generated code
#![allow(clippy::uninlined_format_args)] // Generated code
#![allow(clippy::needless_pass_by_value)] // Generated code
#![allow(clippy::redundant_closure_for_method_calls)] // Generated code
#![allow(clippy::items_after_statements)] // Generated code
#![allow(clippy::match_same_arms)] // Generated code

mod common;
mod connect_delete;
mod connect_directories_changes;
mod connect_open;
mod connect_progress_receiver;
mod connect_scan;
mod connect_show_preview;
mod connect_stop;
mod connect_translation;
mod localizer_krokiet;
mod set_initial_gui_info;
mod settings;

use crossbeam_channel::{unbounded, Receiver, Sender};
use slint::VecModel;
use std::rc::Rc;
// use std::rc::Rc;

use crate::connect_delete::connect_delete_button;
use crate::connect_open::connect_open_items;
use crate::connect_scan::connect_scan_button;

use crate::connect_directories_changes::connect_add_remove_directories;
use crate::connect_progress_receiver::connect_progress_gathering;
use crate::connect_show_preview::connect_show_preview;
use crate::connect_stop::connect_stop_button;
use crate::connect_translation::connect_translations;
use crate::set_initial_gui_info::set_initial_gui_infos;
use crate::settings::{connect_changing_settings_preset, create_default_settings_files, load_settings_from_file, save_all_settings_to_file};
use czkawka_core::common::{print_version_mode, setup_logger};
use czkawka_core::common_dir_traversal::ProgressData;
// use slint::{ModelRc, VecModel};

slint::include_modules!();
fn main() {
    setup_logger(false);
    print_version_mode();

    let app = MainWindow::new().unwrap();

    let (progress_sender, progress_receiver): (Sender<ProgressData>, Receiver<ProgressData>) = unbounded();
    let (stop_sender, stop_receiver): (Sender<()>, Receiver<()>) = unbounded();

    // to_remove_debug(&app);

    // Slint files may already contains data in models, so clear them before starting - todo,
    // check if non zeroed models are useful
    zeroing_all_models(&app);

    set_initial_gui_infos(&app);

    create_default_settings_files();
    load_settings_from_file(&app);

    connect_delete_button(&app);
    connect_scan_button(&app, progress_sender, stop_receiver);
    connect_stop_button(&app, stop_sender);
    connect_open_items(&app);
    connect_progress_gathering(&app, progress_receiver);
    connect_add_remove_directories(&app);
    connect_show_preview(&app);
    connect_translations(&app);
    connect_changing_settings_preset(&app);

    app.run().unwrap();

    save_all_settings_to_file(&app);
}

pub fn zeroing_all_models(app: &MainWindow) {
    app.set_empty_folder_model(Rc::new(VecModel::default()).into());
    app.set_empty_files_model(Rc::new(VecModel::default()).into());
    app.set_similar_images_model(Rc::new(VecModel::default()).into());
}

// // TODO remove this after debugging - or leave commented
// pub fn to_remove_debug(app: &MainWindow) {
//     app.set_empty_folder_model(to_remove_create_without_header("@@").into());
//     app.set_empty_files_model(to_remove_create_without_header("%%").into());
//     app.set_similar_images_model(to_remove_create_with_header().into());
// }

// fn to_remove_create_with_header() -> Rc<VecModel<MainListModel>> {
//     let header_row_data: Rc<VecModel<MainListModel>> = Rc::new(VecModel::default());
//     for r in 0..10_000 {
//         let items = VecModel::default();
//
//         for c in 0..3 {
//             items.push(slint::format!("Item {r}.{c}"));
//         }
//
//         let is_header = r % 3 == 0;
//         let is_checked = (r % 2 == 0) && !is_header;
//
//         let item = MainListModel {
//             checked: is_checked,
//             header_row: is_header,
//             selected_row: false,
//             val: ModelRc::new(items),
//         };
//
//         header_row_data.push(item);
//     }
//     header_row_data
// }
// fn to_remove_create_without_header(s: &str) -> Rc<VecModel<MainListModel>> {
//     let non_header_row_data: Rc<VecModel<MainListModel>> = Rc::new(VecModel::default());
//     for r in 0..100_000 {
//         let items = VecModel::default();
//
//         for c in 0..3 {
//             items.push(slint::format!("Item {r}.{c}.{s}"));
//         }
//
//         let is_checked = r % 2 == 0;
//
//         let item = MainListModel {
//             checked: is_checked,
//             header_row: false,
//             selected_row: false,
//             val: ModelRc::new(items),
//         };
//
//         non_header_row_data.push(item);
//     }
//     non_header_row_data
// }
