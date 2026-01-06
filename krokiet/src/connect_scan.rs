use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use chrono::DateTime;
use crossbeam_channel::Sender;
use czkawka_core::common::consts::DEFAULT_THREAD_SIZE;
use czkawka_core::common::model::{CheckingMethod, FileEntry};
use czkawka_core::common::progress_data::ProgressData;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::{ResultEntry, Search};
use czkawka_core::common::{format_time, split_path, split_path_compare};
use czkawka_core::helpers::messages::MessageLimit;
use czkawka_core::tools::bad_extensions::{BadExtensions, BadExtensionsParameters, BadFileEntry};
use czkawka_core::tools::big_file::{BigFile, BigFileParameters, SearchMode};
use czkawka_core::tools::broken_files::{BrokenEntry, BrokenFiles, BrokenFilesParameters, CheckedTypes};
use czkawka_core::tools::duplicate::{DuplicateEntry, DuplicateFinder, DuplicateFinderParameters};
use czkawka_core::tools::empty_files::EmptyFiles;
use czkawka_core::tools::empty_folder::{EmptyFolder, FolderEntry};
use czkawka_core::tools::exif_remover::{ExifEntry, ExifRemover, ExifRemoverParameters};
use czkawka_core::tools::invalid_symlinks::{InvalidSymlinks, SymlinksFileEntry};
use czkawka_core::tools::same_music::core::format_audio_duration;
use czkawka_core::tools::same_music::{MusicEntry, MusicSimilarity, SameMusic, SameMusicParameters};
use czkawka_core::tools::similar_images::core::get_string_from_similarity;
use czkawka_core::tools::similar_images::{ImagesEntry, SimilarImages, SimilarImagesParameters};
use czkawka_core::tools::similar_videos::core::{format_bitrate_opt, format_duration_opt};
use czkawka_core::tools::similar_videos::{SimilarVideos, SimilarVideosParameters, VideosEntry, crop_detect_from_str};
use czkawka_core::tools::temporary::{Temporary, TemporaryFileEntry};
use czkawka_core::tools::video_optimizer::{OptimizerMode, VideoOptimizer, VideoOptimizerParameters, VideoTranscodeEntry};
use humansize::{BINARY, format_size};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::common::{
    MAX_INT_DATA_BAD_EXTENSIONS, MAX_INT_DATA_BIG_FILES, MAX_INT_DATA_BROKEN_FILES, MAX_INT_DATA_DUPLICATE_FILES, MAX_INT_DATA_EMPTY_FILES, MAX_INT_DATA_EMPTY_FOLDERS,
    MAX_INT_DATA_EXIF_REMOVER, MAX_INT_DATA_INVALID_SYMLINKS, MAX_INT_DATA_SIMILAR_IMAGES, MAX_INT_DATA_SIMILAR_MUSIC, MAX_INT_DATA_SIMILAR_VIDEOS, MAX_INT_DATA_TEMPORARY_FILES,
    MAX_INT_DATA_VIDEO_OPTIMIZER, MAX_STR_DATA_BAD_EXTENSIONS, MAX_STR_DATA_BIG_FILES, MAX_STR_DATA_BROKEN_FILES, MAX_STR_DATA_DUPLICATE_FILES, MAX_STR_DATA_EMPTY_FILES,
    MAX_STR_DATA_EMPTY_FOLDERS, MAX_STR_DATA_EXIF_REMOVER, MAX_STR_DATA_INVALID_SYMLINKS, MAX_STR_DATA_SIMILAR_IMAGES, MAX_STR_DATA_SIMILAR_MUSIC, MAX_STR_DATA_SIMILAR_VIDEOS,
    MAX_STR_DATA_TEMPORARY_FILES, MAX_STR_DATA_VIDEO_OPTIMIZER, check_if_all_included_dirs_are_referenced, check_if_there_are_any_included_folders, split_u64_into_i32s,
};
use crate::connect_row_selection::checker::set_number_of_enabled_items;
use crate::connect_row_selection::reset_selection;
use crate::settings::combo_box::StringComboBoxItems;
use crate::settings::model::{BasicSettings, SettingsCustom};
use crate::settings::{collect_base_settings, collect_settings};
use crate::shared_models::SharedModels;
use crate::{ActiveTab, GuiState, MainListModel, MainWindow, ProgressToSend, flk};

pub(crate) fn connect_scan_button(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_flag: Arc<AtomicBool>, shared_models: Arc<Mutex<SharedModels>>) {
    let a = app.as_weak();
    app.on_scan_starting(move |active_tab| {
        let app = a.upgrade().expect("Failed to upgrade app :(");

        if !check_if_there_are_any_included_folders(&app) {
            app.invoke_scan_ended(flk!("rust_no_included_directories").into());
            return;
        }

        if check_if_all_included_dirs_are_referenced(&app) {
            app.invoke_scan_ended(flk!("rust_all_dirs_referenced").into());
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

        let cloned_model = Arc::clone(&shared_models);

        app.global::<GuiState>().set_info_text("".into());

        let a = app.as_weak();
        match active_tab {
            ActiveTab::DuplicateFiles => {
                scan_duplicates(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::EmptyFolders => {
                scan_empty_folders(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::BigFiles => {
                scan_big_files(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::EmptyFiles => {
                scan_empty_files(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::SimilarImages => {
                scan_similar_images(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::SimilarVideos => {
                scan_similar_videos(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::SimilarMusic => {
                scan_similar_music(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::InvalidSymlinks => {
                scan_invalid_symlinks(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::BadExtensions => {
                scan_bad_extensions(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::BrokenFiles => {
                scan_broken_files(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::TemporaryFiles => {
                scan_temporary_files(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::ExifRemover => {
                scan_exif_remover(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::VideoOptimizer => {
                scan_video_optimizer(a, progress_sender, stop_flag, custom_settings, basic_settings, cloned_model);
            }
            ActiveTab::Settings | ActiveTab::About => panic!("Button should be disabled"),
        }
    });
}

// Scan Duplicates

fn scan_duplicates(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let collected_items = StringComboBoxItems::get_items();

            let hash_type = StringComboBoxItems::get_value_from_config_name(&custom_settings.duplicates_sub_available_hash_type, &collected_items.duplicates_hash_type);
            let check_method = StringComboBoxItems::get_value_from_config_name(&custom_settings.duplicates_sub_check_method, &collected_items.duplicates_check_method);

            let params = DuplicateFinderParameters::new(
                check_method,
                hash_type,
                custom_settings.duplicate_hide_hard_links,
                custom_settings.duplicate_use_prehash,
                custom_settings.duplicate_minimal_hash_cache_size as u64,
                custom_settings.duplicate_minimal_prehash_cache_size as u64,
                custom_settings.duplicates_sub_name_case_sensitive,
            );
            let mut tool = DuplicateFinder::new(params);

            set_common_settings(&mut tool, &custom_settings, &stop_flag);
            tool.set_delete_outdated_cache(custom_settings.duplicate_delete_outdated_entries);
            tool.search(&stop_flag, Some(&progress_sender));
            let messages = get_text_messages(&tool, &basic_settings);

            let mut vector;
            if tool.get_use_reference() {
                match tool.get_params().check_method {
                    CheckingMethod::Hash => {
                        vector = tool
                            .get_files_with_identical_hashes_referenced()
                            .values()
                            .flatten()
                            .cloned()
                            .map(|(original, other)| (Some(original), other))
                            .collect::<Vec<_>>();
                    }
                    CheckingMethod::Name | CheckingMethod::Size | CheckingMethod::SizeName => {
                        let values: Vec<_> = match tool.get_params().check_method {
                            CheckingMethod::Name => tool.get_files_with_identical_name_referenced().values().cloned().collect(),
                            CheckingMethod::Size => tool.get_files_with_identical_size_referenced().values().cloned().collect(),
                            CheckingMethod::SizeName => tool.get_files_with_identical_size_names_referenced().values().cloned().collect(),
                            _ => unreachable!("Invalid check method."),
                        };
                        vector = values.into_iter().map(|(original, other)| (Some(original), other)).collect::<Vec<_>>();
                    }
                    _ => unreachable!("Invalid check method."),
                }
            } else {
                match tool.get_params().check_method {
                    CheckingMethod::Hash => {
                        vector = tool.get_files_sorted_by_hash().values().flatten().cloned().map(|items| (None, items)).collect::<Vec<_>>();
                    }
                    CheckingMethod::Name | CheckingMethod::Size | CheckingMethod::SizeName => {
                        let values: Vec<_> = match tool.get_params().check_method {
                            CheckingMethod::Name => tool.get_files_sorted_by_names().values().cloned().collect(),
                            CheckingMethod::Size => tool.get_files_sorted_by_size().values().cloned().collect(),
                            CheckingMethod::SizeName => tool.get_files_sorted_by_size_name().values().cloned().collect(),
                            _ => unreachable!("Invalid check method."),
                        };
                        vector = values.into_iter().map(|items| (None, items)).collect::<Vec<_>>();
                    }
                    _ => unreachable!("Invalid check method."),
                }
            }

            for (_first, vec) in &mut vector {
                vec.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));
            }

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let (duplicates_number, groups_number, lost_space) = match tool.get_check_method() {
                CheckingMethod::Hash => (info.number_of_duplicated_files_by_hash, info.number_of_groups_by_hash, info.lost_space_by_hash),
                CheckingMethod::Name => (info.number_of_duplicated_files_by_name, info.number_of_groups_by_name, 0),
                CheckingMethod::Size => (info.number_of_duplicated_files_by_size, info.number_of_groups_by_size, info.lost_space_by_size),
                CheckingMethod::SizeName => (info.number_of_duplicated_files_by_size_name, info.number_of_groups_by_size_name, info.lost_space_by_size),
                _ => unreachable!("invalid check method {:?}", tool.get_check_method()),
            };
            shared_models.lock().unwrap().shared_duplication_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_duplicate_results(&app, vector, messages, &scanning_time_str, duplicates_number, groups_number, lost_space);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_duplicate_results(
    app: &MainWindow,
    vector: Vec<(Option<DuplicateEntry>, Vec<DuplicateEntry>)>,
    messages: String,
    scanning_time_str: &str,
    items_found: usize,
    groups: usize,
    lost_space: u64,
) {
    let items = Rc::new(VecModel::default());
    for (ref_fe, vec_fe) in vector.into_iter().rev() {
        if let Some(ref_fe) = ref_fe {
            let (data_model_str, data_model_int) = prepare_data_model_duplicates(&ref_fe);
            insert_data_to_model(&items, data_model_str, data_model_int, Some(true));
        } else {
            insert_data_to_model(&items, ModelRc::new(VecModel::default()), ModelRc::new(VecModel::default()), Some(false));
        }

        for fe in vec_fe {
            let (data_model_str, data_model_int) = prepare_data_model_duplicates(&fe);
            insert_data_to_model(&items, data_model_str, data_model_int, None);
        }
    }
    app.set_duplicate_files_model(items.into());
    if lost_space > 0 {
        app.invoke_scan_ended(
            flk!(
                "rust_found_duplicate_files",
                items_found = items_found,
                groups = groups,
                size = format_size(lost_space, BINARY),
                time = scanning_time_str
            )
            .into(),
        );
    } else {
        app.invoke_scan_ended(
            flk!(
                "rust_found_duplicate_files_no_lost_space",
                items_found = items_found,
                groups = groups,
                time = scanning_time_str
            )
            .into(),
        );
    }
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::DuplicateFiles);
}
fn prepare_data_model_duplicates(fe: &DuplicateEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str_arr: [SharedString; MAX_STR_DATA_DUPLICATE_FILES] = [
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_DUPLICATE_FILES] = [modification_split.0, modification_split.1, size_split.0, size_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}

////////////////////////////////////////// Empty Folders
fn scan_empty_folders(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut tool = EmptyFolder::new();
            set_common_settings(&mut tool, &custom_settings, &stop_flag);
            tool.search(&stop_flag, Some(&progress_sender));

            let mut vector = tool.get_empty_folder_list().values().cloned().collect::<Vec<_>>();
            let messages = get_text_messages(&tool, &basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_empty_folders;
            shared_models.lock().unwrap().shared_empty_folders_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_empty_folders_results(&app, vector, messages, &scanning_time_str, items_found);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_empty_folders_results(app: &MainWindow, vector: Vec<FolderEntry>, messages: String, scanning_time_str: &str, items_found: usize) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_empty_folders(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_empty_folder_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_empty_folders", items_found = items_found, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::EmptyFolders);
}

fn prepare_data_model_empty_folders(fe: &FolderEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_EMPTY_FOLDERS] = [file.into(), directory.into(), get_dt_timestamp_string(fe.modified_date).into()];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int_arr: [i32; MAX_INT_DATA_EMPTY_FOLDERS] = [modification_split.0, modification_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}

////////////////////////////////////////// Big files
fn scan_big_files(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let collected_items = StringComboBoxItems::get_items();
            let big_files_mode = StringComboBoxItems::get_value_from_config_name(&custom_settings.biggest_files_sub_method, &collected_items.biggest_files_method);

            let params = BigFileParameters::new(custom_settings.biggest_files_sub_number_of_files as usize, big_files_mode);
            let mut tool = BigFile::new(params);

            set_common_settings(&mut tool, &custom_settings, &stop_flag);
            tool.search(&stop_flag, Some(&progress_sender));

            let mut vector = tool.get_big_files().clone();
            let messages = get_text_messages(&tool, &basic_settings);

            if big_files_mode == SearchMode::BiggestFiles {
                vector.par_sort_unstable_by_key(|fe| u64::MAX - fe.size);
            } else {
                vector.par_sort_unstable_by_key(|fe| fe.size);
            }

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_real_files;
            let files_size = tool.get_big_files().iter().map(|f| f.size).sum::<u64>();
            shared_models.lock().unwrap().shared_big_files_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_big_files_results(&app, vector, messages, &scanning_time_str, items_found, files_size);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_big_files_results(app: &MainWindow, vector: Vec<FileEntry>, messages: String, scanning_time_str: &str, items_found: usize, files_size: u64) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_big_files(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_big_files_model(items.into());
    app.invoke_scan_ended(
        flk!(
            "rust_found_big_files",
            items_found = items_found,
            time = scanning_time_str,
            size = format_size(files_size, BINARY)
        )
        .into(),
    );
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::BigFiles);
}

fn prepare_data_model_big_files(fe: &FileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_BIG_FILES] = [
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        get_dt_timestamp_string(fe.modified_date).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_BIG_FILES] = [modification_split.0, modification_split.1, size_split.0, size_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}

///////////////////////////////// Empty Files
fn scan_empty_files(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut tool = EmptyFiles::new();
            set_common_settings(&mut tool, &custom_settings, &stop_flag);
            tool.search(&stop_flag, Some(&progress_sender));

            let mut vector = tool.get_empty_files().clone();
            let messages = get_text_messages(&tool, &basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_empty_files;
            shared_models.lock().unwrap().shared_empty_files_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_empty_files_results(&app, vector, messages, &scanning_time_str, items_found);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_empty_files_results(app: &MainWindow, vector: Vec<FileEntry>, messages: String, scanning_time_str: &str, items_found: usize) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_empty_files(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_empty_files_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_empty_files", items_found = items_found, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::EmptyFiles);
}

fn prepare_data_model_empty_files(fe: &FileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str_arr: [SharedString; MAX_STR_DATA_EMPTY_FILES] = [file.into(), directory.into(), get_dt_timestamp_string(fe.get_modified_date()).into()];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_EMPTY_FILES] = [modification_split.0, modification_split.1, size_split.0, size_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
// Scan Similar Images

fn scan_similar_images(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let collected_items = StringComboBoxItems::get_items();
            let hash_alg = StringComboBoxItems::get_value_from_config_name(&custom_settings.similar_images_sub_hash_alg, &collected_items.image_hash_alg);
            let resize_algorithm = StringComboBoxItems::get_value_from_config_name(&custom_settings.similar_images_sub_resize_algorithm, &collected_items.resize_algorithm);
            let hash_size = custom_settings
                .similar_images_sub_hash_size
                .parse()
                .unwrap_or_else(|_| panic!("Cannot parse hash size {}", custom_settings.similar_images_sub_hash_size));

            let params = SimilarImagesParameters::new(
                custom_settings.similar_images_sub_similarity as u32,
                hash_size,
                hash_alg,
                resize_algorithm,
                custom_settings.similar_images_sub_ignore_same_size,
                custom_settings.similar_images_hide_hard_links,
            );
            let mut tool = SimilarImages::new(params);

            set_common_settings(&mut tool, &custom_settings, &stop_flag);

            tool.set_delete_outdated_cache(custom_settings.similar_images_delete_outdated_entries);

            tool.search(&stop_flag, Some(&progress_sender));

            let messages = get_text_messages(&tool, &basic_settings);

            let mut vector: Vec<_> = if tool.get_use_reference() {
                tool.get_similar_images_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect()
            } else {
                tool.get_similar_images().iter().cloned().map(|items| (None, items)).collect()
            };

            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by_key(|e| (e.similarity, u64::MAX - e.size));
            }
            vector.sort_by_key(|(_header, vc)| u64::MAX - vc.iter().map(|e| e.size).sum::<u64>()); // Also sorts by size, to show the biggest groups first

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_duplicates;
            let groups = info.number_of_groups;
            shared_models.lock().unwrap().shared_similar_images_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_similar_images_results(&app, vector, messages, hash_size, &scanning_time_str, items_found, groups);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_similar_images_results(
    app: &MainWindow,
    vector: Vec<(Option<ImagesEntry>, Vec<ImagesEntry>)>,
    messages: String,
    hash_size: u8,
    scanning_time_str: &str,
    items_found: usize,
    groups: usize,
) {
    let items = Rc::new(VecModel::default());
    for (ref_fe, vec_fe) in vector {
        if let Some(ref_fe) = ref_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_images(&ref_fe, hash_size);
            insert_data_to_model(&items, data_model_str, data_model_int, Some(true));
        } else {
            insert_data_to_model(&items, ModelRc::new(VecModel::default()), ModelRc::new(VecModel::default()), Some(false));
        }

        for fe in vec_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_images(&fe, hash_size);
            insert_data_to_model(&items, data_model_str, data_model_int, None);
        }
    }
    app.set_similar_images_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_similar_images", items_found = items_found, groups = groups, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::SimilarImages);
}
fn prepare_data_model_similar_images(fe: &ImagesEntry, hash_size: u8) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str_arr: [SharedString; MAX_STR_DATA_SIMILAR_IMAGES] = [
        get_string_from_similarity(&fe.similarity, hash_size).into(),
        format_size(fe.size, BINARY).into(),
        format!("{}x{}", fe.width, fe.height).into(),
        file.into(),
        directory.into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let pixels = split_u64_into_i32s((fe.width as u64) * (fe.height as u64));
    let data_model_int_arr: [i32; MAX_INT_DATA_SIMILAR_IMAGES] = [
        modification_split.0,
        modification_split.1,
        size_split.0,
        size_split.1,
        fe.width as i32,
        fe.height as i32,
        pixels.0,
        pixels.1,
    ];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}

// Scan Similar Videos

fn scan_similar_videos(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let params = SimilarVideosParameters::new(
                custom_settings.similar_videos_sub_similarity,
                custom_settings.similar_videos_sub_ignore_same_size,
                custom_settings.similar_videos_hide_hard_links,
                custom_settings.similar_videos_skip_forward_amount,
                custom_settings.similar_videos_vid_hash_duration,
                crop_detect_from_str(&custom_settings.similar_videos_crop_detect),
                custom_settings.similar_videos_image_preview,
                custom_settings.similar_videos_thumbnail_percentage,
                custom_settings.similar_videos_generate_thumbnail_grid_instead_of_single_image,
            );
            let mut tool = SimilarVideos::new(params);
            set_common_settings(&mut tool, &custom_settings, &stop_flag);

            tool.set_delete_outdated_cache(custom_settings.similar_videos_delete_outdated_entries);

            tool.search(&stop_flag, Some(&progress_sender));

            let messages = get_text_messages(&tool, &basic_settings);

            let mut vector: Vec<_> = if tool.get_use_reference() {
                tool.get_similar_videos_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect()
            } else {
                tool.get_similar_videos().iter().cloned().map(|items| (None, items)).collect()
            };
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by(|a, b| match a.size.cmp(&b.size) {
                    std::cmp::Ordering::Equal => split_path_compare(a.path.as_path(), b.path.as_path()),
                    std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                });
            }
            vector.sort_by_key(|(_header, vc)| u64::MAX - vc.iter().map(|e| e.size).sum::<u64>()); // Also sorts by size, to show the biggest groups first

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_duplicates;
            let groups = info.number_of_groups;
            shared_models.lock().unwrap().shared_similar_videos_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_similar_videos_results(&app, vector, messages, &scanning_time_str, items_found, groups);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_similar_videos_results(
    app: &MainWindow,
    vector: Vec<(Option<VideosEntry>, Vec<VideosEntry>)>,
    messages: String,
    scanning_time_str: &str,
    items_found: usize,
    groups: usize,
) {
    let items = Rc::new(VecModel::default());
    for (ref_fe, vec_fe) in vector {
        if let Some(ref_fe) = ref_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_videos(&ref_fe);
            insert_data_to_model(&items, data_model_str, data_model_int, Some(true));
        } else {
            insert_data_to_model(&items, ModelRc::new(VecModel::default()), ModelRc::new(VecModel::default()), Some(false));
        }

        for fe in vec_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_videos(&fe);
            insert_data_to_model(&items, data_model_str, data_model_int, None);
        }
    }
    app.set_similar_videos_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_similar_videos", items_found = items_found, groups = groups, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::SimilarVideos);
}

fn prepare_data_model_similar_videos(fe: &VideosEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let bitrate = format_bitrate_opt(fe.bitrate);
    let fps = fe.fps.map(|e| format!("{e:.2}")).unwrap_or_default();
    let codec = fe.codec.clone().unwrap_or_default();
    let dimensions = if let (Some(w), Some(h)) = (fe.width, fe.height) {
        format!("{w}x{h}")
    } else {
        "".to_string()
    };
    let preview_path = fe.thumbnail_path.as_ref().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
    let duration = format_duration_opt(fe.duration);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_SIMILAR_VIDEOS] = [
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        dimensions.into(),
        duration.into(),
        bitrate.into(),
        fps.into(),
        codec.into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
        preview_path.into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let bitrate_split = split_u64_into_i32s(fe.bitrate.unwrap_or(0));
    let duration_i32 = fe.duration.map_or(0, |d| (d * 100.0) as i32);
    let fps_i32 = fe.fps.map_or(0, |f| (f * 100.0) as i32);
    let pixels_int = fe.width.and_then(|w| fe.height.map(|h| w as u64 * h as u64)).unwrap_or_default();
    let pixels_split = split_u64_into_i32s(pixels_int);
    let data_model_int_arr: [i32; MAX_INT_DATA_SIMILAR_VIDEOS] = [
        modification_split.0,
        modification_split.1,
        size_split.0,
        size_split.1,
        bitrate_split.0,
        bitrate_split.1,
        duration_i32,
        fps_i32,
        pixels_split.0,
        pixels_split.1,
    ];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
// Scan Similar Music
fn scan_similar_music(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut music_similarity: MusicSimilarity = MusicSimilarity::NONE;
            if custom_settings.similar_music_sub_title {
                music_similarity |= MusicSimilarity::TRACK_TITLE;
            }
            if custom_settings.similar_music_sub_artist {
                music_similarity |= MusicSimilarity::TRACK_ARTIST;
            }
            if custom_settings.similar_music_sub_bitrate {
                music_similarity |= MusicSimilarity::BITRATE;
            }
            if custom_settings.similar_music_sub_length {
                music_similarity |= MusicSimilarity::LENGTH;
            }
            if custom_settings.similar_music_sub_year {
                music_similarity |= MusicSimilarity::YEAR;
            }
            if custom_settings.similar_music_sub_genre {
                music_similarity |= MusicSimilarity::GENRE;
            }

            if music_similarity == MusicSimilarity::NONE {
                a.upgrade_in_event_loop(move |app| {
                    app.invoke_scan_ended(flk!("rust_no_similarity_method_selected").into());
                })
                .expect("Cannot upgrade in event loop :(");
                return Ok(());
            }

            let collected_items = StringComboBoxItems::get_items();
            let audio_check_type = StringComboBoxItems::get_value_from_config_name(&custom_settings.similar_music_sub_audio_check_type, &collected_items.audio_check_type);

            let params = SameMusicParameters::new(
                music_similarity,
                custom_settings.similar_music_sub_approximate_comparison,
                audio_check_type,
                custom_settings.similar_music_sub_minimal_fragment_duration_value,
                custom_settings.similar_music_sub_maximum_difference_value as f64,
                custom_settings.similar_music_compare_fingerprints_only_with_similar_titles,
            );
            let mut tool = SameMusic::new(params);
            set_common_settings(&mut tool, &custom_settings, &stop_flag);

            tool.search(&stop_flag, Some(&progress_sender));

            let messages = get_text_messages(&tool, &basic_settings);

            let mut vector: Vec<_> = if tool.get_use_reference() {
                tool.get_similar_music_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect()
            } else {
                tool.get_duplicated_music_entries().iter().cloned().map(|items| (None, items)).collect()
            };

            vector.sort_by_cached_key(|(_, a)| u64::MAX - a.iter().map(|e| e.size).sum::<u64>());
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.sort_unstable_by_key(|a| u64::MAX - a.size);
            }

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_duplicates;
            let groups = info.number_of_groups;
            shared_models.lock().unwrap().shared_same_music_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_similar_music_results(&app, vector, messages, &scanning_time_str, items_found, groups);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_similar_music_results(app: &MainWindow, vector: Vec<(Option<MusicEntry>, Vec<MusicEntry>)>, messages: String, scanning_time_str: &str, items_found: usize, groups: usize) {
    let items = Rc::new(VecModel::default());
    for (ref_fe, vec_fe) in vector {
        if let Some(ref_fe) = ref_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_music(&ref_fe);
            insert_data_to_model(&items, data_model_str, data_model_int, Some(true));
        } else {
            insert_data_to_model(&items, ModelRc::new(VecModel::default()), ModelRc::new(VecModel::default()), Some(false));
        }

        for fe in vec_fe {
            let (data_model_str, data_model_int) = prepare_data_model_similar_music(&fe);
            insert_data_to_model(&items, data_model_str, data_model_int, None);
        }
    }
    app.set_similar_music_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_similar_music_files", items_found = items_found, groups = groups, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::SimilarMusic);
}
fn prepare_data_model_similar_music(fe: &MusicEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str_arr: [SharedString; MAX_STR_DATA_SIMILAR_MUSIC] = [
        format_size(fe.size, BINARY).into(),
        file.into(),
        fe.track_title.clone().into(),
        fe.track_artist.clone().into(),
        fe.year.clone().into(),
        fe.bitrate.to_string().into(),
        format_audio_duration(fe.length).into(),
        fe.genre.clone().into(),
        directory.into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_SIMILAR_MUSIC] = [modification_split.0, modification_split.1, size_split.0, size_split.1, fe.bitrate as i32, fe.length as i32];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
// Invalid Symlinks
fn scan_invalid_symlinks(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut tool = InvalidSymlinks::new();
            set_common_settings(&mut tool, &custom_settings, &stop_flag);

            tool.search(&stop_flag, Some(&progress_sender));

            let mut vector = tool.get_invalid_symlinks().clone();
            let messages = get_text_messages(&tool, &basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_invalid_symlinks;
            shared_models.lock().unwrap().shared_same_invalid_symlinks = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_invalid_symlinks_results(&app, vector, messages, &scanning_time_str, items_found);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_invalid_symlinks_results(app: &MainWindow, vector: Vec<SymlinksFileEntry>, messages: String, scanning_time_str: &str, items_found: usize) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_invalid_symlinks(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_invalid_symlinks_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_invalid_symlinks", items_found = items_found, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::InvalidSymlinks);
}

fn prepare_data_model_invalid_symlinks(fe: &SymlinksFileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str_arr: [SharedString; MAX_STR_DATA_INVALID_SYMLINKS] = [
        file.into(),
        directory.into(),
        fe.symlink_info.destination_path.to_string_lossy().to_string().into(),
        fe.symlink_info.type_of_error.to_string().into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int_arr: [i32; MAX_INT_DATA_INVALID_SYMLINKS] = [modification_split.0, modification_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
////////////////////////////////////////// Temporary Files
fn scan_temporary_files(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut tool = Temporary::new();
            set_common_settings(&mut tool, &custom_settings, &stop_flag);

            tool.search(&stop_flag, Some(&progress_sender));

            let mut vector = tool.get_temporary_files().clone();
            let messages = get_text_messages(&tool, &basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_temporary_files;
            shared_models.lock().unwrap().shared_temporary_files_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_temporary_files_results(&app, vector, messages, &scanning_time_str, items_found);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_temporary_files_results(app: &MainWindow, vector: Vec<TemporaryFileEntry>, messages: String, scanning_time_str: &str, items_found: usize) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_temporary_files(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_temporary_files_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_temporary_files", items_found = items_found, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::TemporaryFiles);
}

fn prepare_data_model_temporary_files(fe: &TemporaryFileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_TEMPORARY_FILES] = [file.into(), directory.into(), get_dt_timestamp_string(fe.modified_date).into()];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_TEMPORARY_FILES] = [modification_split.0, modification_split.1, size_split.0, size_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
////////////////////////////////////////// Broken Files
fn scan_broken_files(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut checked_types: CheckedTypes = CheckedTypes::NONE;
            if custom_settings.broken_files_sub_audio {
                checked_types |= CheckedTypes::AUDIO;
            }
            if custom_settings.broken_files_sub_pdf {
                checked_types |= CheckedTypes::PDF;
            }
            if custom_settings.broken_files_sub_image {
                checked_types |= CheckedTypes::IMAGE;
            }
            if custom_settings.broken_files_sub_archive {
                checked_types |= CheckedTypes::ARCHIVE;
            }

            if checked_types == CheckedTypes::NONE {
                a.upgrade_in_event_loop(move |app| {
                    app.invoke_scan_ended(flk!("rust_no_file_type_selected").into());
                })
                .expect("Cannot upgrade in event loop :(");
                return Ok(());
            }

            let params = BrokenFilesParameters::new(checked_types);
            let mut tool = BrokenFiles::new(params);
            set_common_settings(&mut tool, &custom_settings, &stop_flag);

            tool.search(&stop_flag, Some(&progress_sender));

            let mut vector = tool.get_broken_files().clone();
            let messages = get_text_messages(&tool, &basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_broken_files;
            let size = tool.get_broken_files().iter().map(|e| e.size).sum::<u64>();
            shared_models.lock().unwrap().shared_broken_files_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_broken_files_results(&app, vector, messages, &scanning_time_str, items_found, size);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_broken_files_results(app: &MainWindow, vector: Vec<BrokenEntry>, messages: String, scanning_time_str: &str, items_found: usize, size: u64) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_broken_files(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_broken_files_model(items.into());
    app.invoke_scan_ended(
        flk!(
            "rust_found_broken_files",
            items_found = items_found,
            time = scanning_time_str,
            size = format_size(size, BINARY)
        )
        .into(),
    );
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::BrokenFiles);
}

fn prepare_data_model_broken_files(fe: &BrokenEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_BROKEN_FILES] = [
        file.into(),
        directory.into(),
        fe.error_string.clone().into(),
        format_size(fe.size, BINARY).into(),
        get_dt_timestamp_string(fe.modified_date).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_BROKEN_FILES] = [modification_split.0, modification_split.1, size_split.0, size_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
////////////////////////////////////////// Bad Extensions
fn scan_bad_extensions(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let params = BadExtensionsParameters::new();
            let mut tool = BadExtensions::new(params);
            set_common_settings(&mut tool, &custom_settings, &stop_flag);
            tool.search(&stop_flag, Some(&progress_sender));

            let mut vector = tool.get_bad_extensions_files().clone();
            let messages = get_text_messages(&tool, &basic_settings);

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_files_with_bad_extension;
            shared_models.lock().unwrap().shared_bad_extensions_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_bad_extensions_results(&app, vector, messages, &scanning_time_str, items_found);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_bad_extensions_results(app: &MainWindow, vector: Vec<BadFileEntry>, messages: String, scanning_time_str: &str, items_found: usize) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_bad_extensions(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_bad_extensions_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_bad_extensions", items_found = items_found, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::BadExtensions);
}

fn prepare_data_model_bad_extensions(fe: &BadFileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_BAD_EXTENSIONS] = [
        file.into(),
        directory.into(),
        fe.current_extension.clone().into(),
        fe.proper_extensions_group.clone().into(),
        fe.proper_extension.clone().into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_BAD_EXTENSIONS] = [modification_split.0, modification_split.1, size_split.0, size_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}
////////////////////////////////////////// Exif Remover
fn scan_exif_remover(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            // Parse ignored tags from comma-separated string, trimming whitespace
            let ignored_tags: Vec<String> = custom_settings
                .ignored_exif_tags
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let params = ExifRemoverParameters::new(ignored_tags);
            let mut tool = ExifRemover::new(params);
            set_common_settings(&mut tool, &custom_settings, &stop_flag);
            tool.search(&stop_flag, Some(&progress_sender));

            let mut vector = tool.get_exif_files().clone();
            let messages = get_text_messages(&tool, &basic_settings);

            vector.par_sort_unstable_by(|a, b| b.exif_tags.len().cmp(&a.exif_tags.len()));

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_files_with_exif;
            shared_models.lock().unwrap().shared_exif_remover_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_exif_remover_results(&app, vector, messages, &scanning_time_str, items_found);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_exif_remover_results(app: &MainWindow, vector: Vec<ExifEntry>, messages: String, scanning_time_str: &str, items_found: usize) {
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_exif_remover(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_exif_remover_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_exif_files", items_found = items_found, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::ExifRemover);
}

fn prepare_data_model_exif_remover(fe: &ExifEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let size_str = format_size(fe.size, BINARY);
    let exif_tags = format!(
        "{} ({})",
        fe.exif_tags.len(),
        fe.exif_tags.iter().map(|item_tag| item_tag.name.clone()).collect::<Vec<String>>().join(", ")
    );
    let exif_groups_name = fe.exif_tags.iter().map(|item_tag| item_tag.group.clone()).collect::<Vec<String>>().join(",");
    let exif_tag_u16 = fe.exif_tags.iter().map(|item_tag| item_tag.code.to_string()).collect::<Vec<String>>().join(",");
    let data_model_str_arr: [SharedString; MAX_STR_DATA_EXIF_REMOVER] = [
        size_str.into(),
        file.into(),
        directory.into(),
        exif_tags.into(),
        get_dt_timestamp_string(fe.get_modified_date()).into(),
        exif_groups_name.into(),
        exif_tag_u16.into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int_arr: [i32; MAX_INT_DATA_EXIF_REMOVER] = [modification_split.0, modification_split.1, size_split.0, size_split.1, fe.exif_tags.len() as i32];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}

////////////////////////////////////////// Video Optimizer
fn scan_video_optimizer(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_flag: Arc<AtomicBool>,
    custom_settings: SettingsCustom,
    basic_settings: BasicSettings,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let excluded_codecs: Vec<String> = custom_settings
                .video_optimizer_excluded_codecs
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect();

            let params = VideoOptimizerParameters {
                mode: OptimizerMode::VideoTranscode,
                excluded_codecs,
            };

            let mut tool = VideoOptimizer::new(params);
            set_common_settings(&mut tool, &custom_settings, &stop_flag);

            tool.search(&stop_flag, Some(&progress_sender));

            let video_transcode_entries = tool.get_video_transcode_entries().clone();
            let messages = get_text_messages(&tool, &basic_settings);

            let info = tool.get_information();
            let scanning_time_str = format_time(info.scanning_time);
            let items_found = info.number_of_processed_files;

            shared_models.lock().unwrap().shared_video_optimizer_state = Some(tool);

            a.upgrade_in_event_loop(move |app| {
                write_video_optimizer_results(&app, video_transcode_entries, messages, &scanning_time_str, items_found);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}

fn write_video_optimizer_results(app: &MainWindow, video_transcode_entries: Vec<VideoTranscodeEntry>, messages: String, scanning_time_str: &str, items_found: usize) {
    let items = Rc::new(VecModel::default());

    for fe in video_transcode_entries {
        let (data_model_str, data_model_int) = prepare_data_model_video_optimizer_video(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }

    app.set_video_optimizer_model(items.into());
    app.invoke_scan_ended(flk!("rust_found_video_optimizer", items_found = items_found, time = scanning_time_str).into());
    app.global::<GuiState>().set_info_text(messages.into());
    reset_selection_at_end(app, ActiveTab::VideoOptimizer);
}

fn prepare_data_model_video_optimizer_video(fe: &VideoTranscodeEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str_arr: [SharedString; MAX_STR_DATA_VIDEO_OPTIMIZER] = [
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        fe.codec.clone().into(),
        format!("{}x{}", fe.width, fe.height).into(),
        get_dt_timestamp_string(fe.modified_date).into(),
    ];
    let data_model_str = VecModel::from_slice(&data_model_str_arr);
    let modification_split = split_u64_into_i32s(fe.modified_date);
    let size_split = split_u64_into_i32s(fe.size);
    let dimension_split = split_u64_into_i32s(fe.width as u64 * fe.height as u64);
    let data_model_int_arr: [i32; MAX_INT_DATA_VIDEO_OPTIMIZER] = [modification_split.0, modification_split.1, size_split.0, size_split.1, dimension_split.0, dimension_split.1];
    let data_model_int = VecModel::from_slice(&data_model_int_arr);
    (data_model_str, data_model_int)
}

fn get_dt_timestamp_string(timestamp: u64) -> String {
    DateTime::from_timestamp(timestamp as i64, 0)
        .expect("Modified date always should be in valid range")
        .to_string()
}

////////////////////////////////////////// Common

fn reset_selection_at_end(app: &MainWindow, active_tab: ActiveTab) {
    reset_selection(app, active_tab, true);
    set_number_of_enabled_items(app, active_tab, 0);
}

fn insert_data_to_model(items: &Rc<VecModel<MainListModel>>, data_model_str: ModelRc<SharedString>, data_model_int: ModelRc<i32>, filled_header_row: Option<bool>) {
    let main = MainListModel {
        checked: false,
        header_row: filled_header_row.is_some(),
        filled_header_row: filled_header_row.unwrap_or(false),
        selected_row: false,
        val_str: ModelRc::new(data_model_str),
        val_int: ModelRc::new(data_model_int),
    };
    items.push(main);
}

fn get_text_messages<T>(component: &T, basic_settings: &BasicSettings) -> String
where
    T: CommonData,
{
    let limit = if basic_settings.settings_limit_lines_of_messages {
        MessageLimit::Lines(500)
    } else {
        MessageLimit::NoLimit
    };
    component.get_text_messages().create_messages_text(limit)
}

fn set_common_settings<T>(component: &mut T, custom_settings: &SettingsCustom, stop_flag: &Arc<AtomicBool>)
where
    T: CommonData,
{
    stop_flag.store(false, Ordering::Relaxed);

    component.set_included_directory(custom_settings.included_directories.clone());
    component.set_reference_directory(custom_settings.included_directories_referenced.clone());
    component.set_excluded_directory(custom_settings.excluded_directories.clone());
    component.set_recursive_search(custom_settings.recursive_search);
    component.set_minimal_file_size(custom_settings.minimum_file_size as u64 * 1024);
    component.set_maximal_file_size(custom_settings.maximum_file_size as u64 * 1024);
    component.set_allowed_extensions(custom_settings.allowed_extensions.clone());
    component.set_excluded_extensions(custom_settings.excluded_extensions.clone());
    component.set_excluded_items(custom_settings.excluded_items.split(',').map(str::to_string).collect());
    component.set_exclude_other_filesystems(custom_settings.ignore_other_file_systems);
    component.set_use_cache(custom_settings.use_cache);
    component.set_save_also_as_json(custom_settings.save_also_as_json);
}
