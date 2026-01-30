#![allow(clippy::needless_pass_by_value)]

mod bad_extensions;
mod bad_names;
mod big_files;
mod broken_files;
mod duplicate;
mod empty_files;
mod empty_folders;
mod exif_remover;
mod invalid_symlinks;
mod same_music;
mod similar_images;
mod similar_videos;
mod temporary_files;
mod video_optimizer;

use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use chrono::{Local, TimeZone, Utc};
use crossbeam_channel::Sender;
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::helpers::messages::MessageLimit;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::audio_player::AudioPlayer;
use crate::common::{check_if_all_included_dirs_are_referenced, check_if_there_are_any_included_folders};
use crate::connect_row_selection::checker::set_number_of_enabled_items;
use crate::connect_row_selection::reset_selection;
use crate::connect_scan::bad_extensions::scan_bad_extensions;
use crate::connect_scan::bad_names::scan_bad_names;
use crate::connect_scan::big_files::scan_big_files;
use crate::connect_scan::broken_files::scan_broken_files;
use crate::connect_scan::duplicate::scan_duplicates;
use crate::connect_scan::empty_files::scan_empty_files;
use crate::connect_scan::empty_folders::scan_empty_folders;
use crate::connect_scan::exif_remover::scan_exif_remover;
use crate::connect_scan::invalid_symlinks::scan_invalid_symlinks;
use crate::connect_scan::same_music::scan_similar_music;
use crate::connect_scan::similar_images::scan_similar_images;
use crate::connect_scan::similar_videos::scan_similar_videos;
use crate::connect_scan::temporary_files::scan_temporary_files;
use crate::connect_scan::video_optimizer::scan_video_optimizer;
use crate::settings::model::{BasicSettings, ComboBoxItems, SettingsCustom};
use crate::settings::{collect_base_settings, collect_combo_box_settings, collect_settings};
use crate::shared_models::SharedModels;
use crate::{ActiveTab, GuiState, MainWindow, ProgressToSend, SingleMainListModel, flk};

pub struct ScanData {
    pub progress_sender: Sender<ProgressData>,
    pub stop_flag: Arc<AtomicBool>,
    pub custom_settings: SettingsCustom,
    pub basic_settings: BasicSettings,
    pub combo_box_items: ComboBoxItems,
    pub shared_models: Arc<Mutex<SharedModels>>,
    pub audio_player: Arc<AudioPlayer>,
}

pub struct MessagesData {
    pub critical: Option<String>,
    pub messages: String,
}

pub(crate) fn connect_scan_button(
    app: &MainWindow,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    shared_models: Arc<Mutex<SharedModels>>,
    audio_player: Arc<AudioPlayer>,
) {
    let a = app.as_weak();
    app.on_scan_starting(move |active_tab| {
        let app = a.upgrade().expect("Failed to upgrade app :(");

        if !check_if_there_are_any_included_folders(&app) {
            app.invoke_scan_ended(flk!("rust_no_included_paths").into());
            return;
        }

        if check_if_all_included_dirs_are_referenced(&app) {
            app.invoke_scan_ended(flk!("rust_all_paths_referenced").into());
            return;
        }

        let progress_sender = progress_sender.clone();
        let stop_flag = stop_flag.clone();

        app.set_progress_datas(ProgressToSend {
            all_progress: 0,
            current_progress: -1,
            current_progress_size: -1,
            step_name: "".into(),
        });

        let custom_settings = collect_settings(&app);
        let basic_settings = collect_base_settings(&app);
        let combo_box_items = collect_combo_box_settings(&app);

        let cloned_model = Arc::clone(&shared_models);

        app.global::<GuiState>().set_info_text("".into());

        let a = app.as_weak();
        let audio_player_clone = Arc::clone(&audio_player);

        let scan_data = ScanData {
            progress_sender,
            stop_flag,
            custom_settings,
            basic_settings,
            combo_box_items,
            shared_models: cloned_model,
            audio_player: audio_player_clone,
        };

        match active_tab {
            ActiveTab::DuplicateFiles => scan_duplicates(a, scan_data),
            ActiveTab::EmptyFolders => scan_empty_folders(a, scan_data),
            ActiveTab::BigFiles => scan_big_files(a, scan_data),
            ActiveTab::EmptyFiles => scan_empty_files(a, scan_data),
            ActiveTab::SimilarImages => scan_similar_images(a, scan_data),
            ActiveTab::SimilarVideos => scan_similar_videos(a, scan_data),
            ActiveTab::SimilarMusic => scan_similar_music(a, scan_data),
            ActiveTab::InvalidSymlinks => scan_invalid_symlinks(a, scan_data),
            ActiveTab::BadExtensions => scan_bad_extensions(a, scan_data),
            ActiveTab::BadNames => scan_bad_names(a, scan_data),
            ActiveTab::BrokenFiles => scan_broken_files(a, scan_data),
            ActiveTab::TemporaryFiles => scan_temporary_files(a, scan_data),
            ActiveTab::ExifRemover => scan_exif_remover(a, scan_data),
            ActiveTab::VideoOptimizer => scan_video_optimizer(a, scan_data),
            ActiveTab::Settings | ActiveTab::About => panic!("Button should be disabled"),
        }
    });
}

fn get_dt_timestamp_string(timestamp: u64) -> String {
    let dt_local = Utc.timestamp_opt(timestamp as i64, 0).single().unwrap_or_default().with_timezone(&Local);
    dt_local.format("%Y-%m-%d %H:%M:%S").to_string()
}

////////////////////////////////////////// Common

fn reset_selection_at_end(app: &MainWindow, active_tab: ActiveTab) {
    reset_selection(app, active_tab, true);
    set_number_of_enabled_items(app, active_tab, 0);
}

fn insert_data_to_model(items: &Rc<VecModel<SingleMainListModel>>, data_model_str: ModelRc<SharedString>, data_model_int: ModelRc<i32>, filled_header_row: Option<bool>) {
    let main = SingleMainListModel {
        checked: false,
        header_row: filled_header_row.is_some(),
        filled_header_row: filled_header_row.unwrap_or(false),
        selected_row: false,
        val_str: ModelRc::new(data_model_str),
        val_int: ModelRc::new(data_model_int),
    };
    items.push(main);
}

fn get_text_messages<T>(component: &T, basic_settings: &BasicSettings) -> (Option<String>, String)
where
    T: CommonData,
{
    let limit = if basic_settings.settings_limit_lines_of_messages {
        MessageLimit::Lines(500)
    } else {
        MessageLimit::NoLimit
    };

    let text_messages = component.get_text_messages();
    (text_messages.critical.clone(), text_messages.create_messages_text(limit))
}

fn set_common_settings<T>(component: &mut T, custom_settings: &SettingsCustom, stop_flag: &Arc<AtomicBool>)
where
    T: CommonData,
{
    stop_flag.store(false, Ordering::Relaxed);

    component.set_included_paths(custom_settings.included_paths.clone());
    component.set_reference_paths(custom_settings.included_paths_referenced.clone());
    component.set_excluded_paths(custom_settings.excluded_paths.clone());
    component.set_recursive_search(custom_settings.recursive_search);
    component.set_minimal_file_size(custom_settings.minimum_file_size as u64 * 1024);
    component.set_maximal_file_size(custom_settings.maximum_file_size as u64 * 1024);
    component.set_allowed_extensions(custom_settings.allowed_extensions.split(',').map(str::to_string).collect());
    component.set_excluded_extensions(custom_settings.excluded_extensions.split(',').map(str::to_string).collect());
    component.set_excluded_items(custom_settings.excluded_items.split(',').map(str::to_string).collect());
    component.set_exclude_other_filesystems(custom_settings.ignore_other_file_systems);
    component.set_use_cache(custom_settings.use_cache);
    component.set_save_also_as_json(custom_settings.save_also_as_json);
}
