use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

use chrono::DateTime;
use crossbeam_channel::{Receiver, Sender};
use czkawka_core::bad_extensions::{BadExtensions, BadExtensionsParameters, BadFileEntry};
use czkawka_core::big_file::{BigFile, BigFileParameters, SearchMode};
use czkawka_core::broken_files::{BrokenEntry, BrokenFiles, BrokenFilesParameters, CheckedTypes};
use czkawka_core::common::{split_path, split_path_compare, DEFAULT_THREAD_SIZE};
use czkawka_core::common_dir_traversal::{CheckingMethod, FileEntry};
use czkawka_core::common_tool::CommonData;
use czkawka_core::common_traits::ResultEntry;
use czkawka_core::duplicate::{DuplicateEntry, DuplicateFinder, DuplicateFinderParameters};
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::{EmptyFolder, FolderEntry};
use czkawka_core::invalid_symlinks::{InvalidSymlinks, SymlinksFileEntry};
use czkawka_core::progress_data::ProgressData;
use czkawka_core::same_music::{MusicEntry, MusicSimilarity, SameMusic, SameMusicParameters};
use czkawka_core::similar_images;
use czkawka_core::similar_images::{ImagesEntry, SimilarImages, SimilarImagesParameters};
use czkawka_core::similar_videos::{SimilarVideos, SimilarVideosParameters, VideosEntry};
use czkawka_core::temporary::{Temporary, TemporaryFileEntry};
use humansize::{format_size, BINARY};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::common::{check_if_all_included_dirs_are_referenced, check_if_there_are_any_included_folders, split_u64_into_i32s};
use crate::settings::{
    collect_settings, get_audio_check_type_idx, get_biggest_item_idx, get_duplicates_check_method_idx, get_duplicates_hash_type_idx, get_image_hash_alg_idx,
    get_resize_algorithm_idx, SettingsCustom, ALLOWED_AUDIO_CHECK_TYPE_VALUES, ALLOWED_BIG_FILE_SIZE_VALUES, ALLOWED_DUPLICATES_CHECK_METHOD_VALUES,
    ALLOWED_DUPLICATES_HASH_TYPE_VALUES, ALLOWED_IMAGE_HASH_ALG_VALUES, ALLOWED_RESIZE_ALGORITHM_VALUES,
};
use crate::shared_models::SharedModels;
use crate::{CurrentTab, GuiState, MainListModel, MainWindow, ProgressToSend};

pub fn connect_scan_button(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, shared_models: Arc<Mutex<SharedModels>>) {
    let a = app.as_weak();
    app.on_scan_starting(move |active_tab| {
        let app = a.upgrade().expect("Failed to upgrade app :(");

        if !check_if_there_are_any_included_folders(&app) {
            app.invoke_scan_ended("Cannot start scan when no included directories are set.".into());
            return;
        }

        if check_if_all_included_dirs_are_referenced(&app) {
            app.invoke_scan_ended("Cannot start scan when all included directories are set as referenced folders.".into());
            return;
        }

        let progress_sender = progress_sender.clone();
        let stop_receiver = stop_receiver.clone();

        app.set_progress_datas(ProgressToSend {
            all_progress: 0,
            current_progress: -1,
            step_name: "".into(),
        });

        let custom_settings = collect_settings(&app);

        let cloned_model = Arc::clone(&shared_models);

        let a = app.as_weak();
        match active_tab {
            CurrentTab::DuplicateFiles => {
                scan_duplicates(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::EmptyFolders => {
                scan_empty_folders(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::BigFiles => {
                scan_big_files(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::EmptyFiles => {
                scan_empty_files(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::SimilarImages => {
                scan_similar_images(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::SimilarVideos => {
                scan_similar_videos(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::SimilarMusic => {
                scan_similar_music(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::InvalidSymlinks => {
                scan_invalid_symlinks(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::BadExtensions => {
                scan_bad_extensions(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::BrokenFiles => {
                scan_broken_files(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::TemporaryFiles => {
                scan_temporary_files(a, progress_sender, stop_receiver, custom_settings, cloned_model);
            }
            CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
        }
    });
}

// Scan Duplicates

fn scan_duplicates(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let hash_type =
                ALLOWED_DUPLICATES_HASH_TYPE_VALUES[get_duplicates_hash_type_idx(&custom_settings.duplicates_sub_available_hash_type).expect("Failed to get hash type")].2;
            let check_method =
                ALLOWED_DUPLICATES_CHECK_METHOD_VALUES[get_duplicates_check_method_idx(&custom_settings.duplicates_sub_check_method).expect("Failed to get check method")].2;
            let params = DuplicateFinderParameters::new(
                check_method,
                hash_type,
                custom_settings.duplicate_hide_hard_links,
                custom_settings.duplicate_use_prehash,
                custom_settings.duplicate_minimal_hash_cache_size as u64,
                custom_settings.duplicate_minimal_prehash_cache_size as u64,
                custom_settings.duplicates_sub_name_case_sensitive,
            );
            let mut item = DuplicateFinder::new(params);

            set_common_settings(&mut item, &custom_settings);
            item.set_delete_outdated_cache(custom_settings.duplicate_delete_outdated_entries);
            item.find_duplicates(Some(&stop_receiver), Some(&progress_sender));
            let messages = item.get_text_messages().create_messages_text();

            let mut vector;
            if item.get_use_reference() {
                match item.get_params().check_method {
                    CheckingMethod::Hash => {
                        vector = item
                            .get_files_with_identical_hashes_referenced()
                            .values()
                            .flatten()
                            .cloned()
                            .map(|(original, other)| (Some(original), other))
                            .collect::<Vec<_>>();
                    }
                    CheckingMethod::Name | CheckingMethod::Size | CheckingMethod::SizeName => {
                        let values: Vec<_> = match item.get_params().check_method {
                            CheckingMethod::Name => item.get_files_with_identical_name_referenced().values().cloned().collect(),
                            CheckingMethod::Size => item.get_files_with_identical_size_referenced().values().cloned().collect(),
                            CheckingMethod::SizeName => item.get_files_with_identical_size_names_referenced().values().cloned().collect(),
                            _ => unreachable!("Invalid check method."),
                        };
                        vector = values.into_iter().map(|(original, other)| (Some(original), other)).collect::<Vec<_>>();
                    }
                    _ => unreachable!("Invalid check method."),
                }
            } else {
                match item.get_params().check_method {
                    CheckingMethod::Hash => {
                        vector = item.get_files_sorted_by_hash().values().flatten().cloned().map(|items| (None, items)).collect::<Vec<_>>();
                    }
                    CheckingMethod::Name | CheckingMethod::Size | CheckingMethod::SizeName => {
                        let values: Vec<_> = match item.get_params().check_method {
                            CheckingMethod::Name => item.get_files_sorted_by_names().values().cloned().collect(),
                            CheckingMethod::Size => item.get_files_sorted_by_size().values().cloned().collect(),
                            CheckingMethod::SizeName => item.get_files_sorted_by_size_name().values().cloned().collect(),
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

            shared_models.lock().unwrap().shared_duplication_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_duplicate_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_duplicate_results(app: &MainWindow, vector: Vec<(Option<DuplicateEntry>, Vec<DuplicateEntry>)>, messages: String) {
    let items_found = vector.len();
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
    app.invoke_scan_ended(format!("Found {items_found} similar duplicates files").into());
    app.global::<GuiState>().set_info_text(messages.into());
}
fn prepare_data_model_duplicates(fe: &DuplicateEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str = VecModel::from_slice(&[
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        DateTime::from_timestamp(fe.get_modified_date() as i64, 0)
            .expect("Cannot create DateTime")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}

////////////////////////////////////////// Empty Folders
fn scan_empty_folders(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut item = EmptyFolder::new();
            set_common_settings(&mut item, &custom_settings);
            item.find_empty_folders(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = item.get_empty_folder_list().values().cloned().collect::<Vec<_>>();
            let messages = item.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            shared_models.lock().unwrap().shared_empty_folders_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_empty_folders_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_empty_folders_results(app: &MainWindow, vector: Vec<FolderEntry>, messages: String) {
    let items_found = vector.len();
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_empty_folders(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_empty_folder_model(items.into());
    app.invoke_scan_ended(format!("Found {items_found} empty folders").into());
    app.global::<GuiState>().set_info_text(messages.into());
}

fn prepare_data_model_empty_folders(fe: &FolderEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str = VecModel::from_slice(&[
        file.into(),
        directory.into(),
        DateTime::from_timestamp(fe.modified_date as i64, 0)
            .expect("Modified date always should be in valid range")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1]);
    (data_model_str, data_model_int)
}

////////////////////////////////////////// Big files
fn scan_big_files(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let big_files_mode = ALLOWED_BIG_FILE_SIZE_VALUES[get_biggest_item_idx(&custom_settings.biggest_files_sub_method).expect("Failed to get big files mode")].2;
            let params = BigFileParameters::new(custom_settings.biggest_files_sub_number_of_files as usize, big_files_mode);
            let mut item = BigFile::new(params);

            set_common_settings(&mut item, &custom_settings);
            item.find_big_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = item.get_big_files().clone();
            let messages = item.get_text_messages().create_messages_text();

            if big_files_mode == SearchMode::BiggestFiles {
                vector.par_sort_unstable_by_key(|fe| u64::MAX - fe.size);
            } else {
                vector.par_sort_unstable_by_key(|fe| fe.size);
            }

            shared_models.lock().unwrap().shared_big_files_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_big_files_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_big_files_results(app: &MainWindow, vector: Vec<FileEntry>, messages: String) {
    let items_found = vector.len();
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_big_files(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_big_files_model(items.into());
    app.invoke_scan_ended(format!("Found {items_found} files").into());
    app.global::<GuiState>().set_info_text(messages.into());
}

fn prepare_data_model_big_files(fe: &FileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str = VecModel::from_slice(&[
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        DateTime::from_timestamp(fe.modified_date as i64, 0)
            .expect("Modified date always should be in valid range")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}

///////////////////////////////// Empty Files
fn scan_empty_files(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut item = EmptyFiles::new();
            set_common_settings(&mut item, &custom_settings);
            item.find_empty_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = item.get_empty_files().clone();
            let messages = item.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            shared_models.lock().unwrap().shared_empty_files_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_empty_files_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_empty_files_results(app: &MainWindow, vector: Vec<FileEntry>, messages: String) {
    let items_found = vector.len();
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_empty_files(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_empty_files_model(items.into());
    app.invoke_scan_ended(format!("Found {items_found} empty files").into());
    app.global::<GuiState>().set_info_text(messages.into());
}

fn prepare_data_model_empty_files(fe: &FileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str = VecModel::from_slice(&[
        file.into(),
        directory.into(),
        DateTime::from_timestamp(fe.get_modified_date() as i64, 0)
            .expect("Cannot create DateTime")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}
// Scan Similar Images

fn scan_similar_images(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let hash_alg = ALLOWED_IMAGE_HASH_ALG_VALUES[get_image_hash_alg_idx(&custom_settings.similar_images_sub_hash_alg).expect("Failed to get hash algorithm")].2;
            let resize_algorithm =
                ALLOWED_RESIZE_ALGORITHM_VALUES[get_resize_algorithm_idx(&custom_settings.similar_images_sub_resize_algorithm).expect("Failed to get resize algorithm")].2;

            let params = SimilarImagesParameters::new(
                custom_settings.similar_images_sub_similarity as u32,
                custom_settings.similar_images_sub_hash_size,
                hash_alg,
                resize_algorithm,
                custom_settings.similar_images_sub_ignore_same_size,
                custom_settings.similar_images_hide_hard_links,
            );
            let mut item = SimilarImages::new(params);

            set_common_settings(&mut item, &custom_settings);

            item.set_delete_outdated_cache(custom_settings.similar_images_delete_outdated_entries);

            item.find_similar_images(Some(&stop_receiver), Some(&progress_sender));

            let messages = item.get_text_messages().create_messages_text();
            let hash_size = custom_settings.similar_images_sub_hash_size;

            let mut vector;
            if item.get_use_reference() {
                vector = item
                    .get_similar_images_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect::<Vec<_>>();
            } else {
                vector = item.get_similar_images().iter().cloned().map(|items| (None, items)).collect::<Vec<_>>();
            }
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by_key(|e| e.similarity);
            }

            shared_models.lock().unwrap().shared_similar_images_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_similar_images_results(&app, vector, messages, hash_size);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_similar_images_results(app: &MainWindow, vector: Vec<(Option<ImagesEntry>, Vec<ImagesEntry>)>, messages: String, hash_size: u8) {
    let items_found = vector.len();
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
    app.invoke_scan_ended(format!("Found {items_found} similar image files").into());
    app.global::<GuiState>().set_info_text(messages.into());
}
fn prepare_data_model_similar_images(fe: &ImagesEntry, hash_size: u8) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str = VecModel::from_slice(&[
        similar_images::get_string_from_similarity(&fe.similarity, hash_size).into(),
        format_size(fe.size, BINARY).into(),
        format!("{}x{}", fe.width, fe.height).into(),
        file.into(),
        directory.into(),
        DateTime::from_timestamp(fe.get_modified_date() as i64, 0)
            .expect("Cannot create DateTime")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1, fe.width as i32, fe.height as i32]);
    (data_model_str, data_model_int)
}

// Scan Similar Videos

fn scan_similar_videos(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let params = SimilarVideosParameters::new(
                custom_settings.similar_videos_sub_similarity,
                custom_settings.similar_videos_sub_ignore_same_size,
                custom_settings.similar_videos_hide_hard_links,
            );
            let mut item = SimilarVideos::new(params);
            set_common_settings(&mut item, &custom_settings);

            item.set_delete_outdated_cache(custom_settings.similar_videos_delete_outdated_entries);

            item.find_similar_videos(Some(&stop_receiver), Some(&progress_sender));

            let messages = item.get_text_messages().create_messages_text();

            let mut vector;
            if item.get_use_reference() {
                vector = item
                    .get_similar_videos_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect::<Vec<_>>();
            } else {
                vector = item.get_similar_videos().iter().cloned().map(|items| (None, items)).collect::<Vec<_>>();
            }
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));
            }

            shared_models.lock().unwrap().shared_similar_videos_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_similar_videos_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_similar_videos_results(app: &MainWindow, vector: Vec<(Option<VideosEntry>, Vec<VideosEntry>)>, messages: String) {
    let items_found = vector.len();
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
    app.invoke_scan_ended(format!("Found {items_found} similar video files").into());
    app.global::<GuiState>().set_info_text(messages.into());
}
fn prepare_data_model_similar_videos(fe: &VideosEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str = VecModel::from_slice(&[
        format_size(fe.size, BINARY).into(),
        file.into(),
        directory.into(),
        DateTime::from_timestamp(fe.get_modified_date() as i64, 0)
            .expect("Cannot create DateTime")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}
// Scan Similar Music
fn scan_similar_music(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
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
                    app.invoke_scan_ended("Cannot find similar music files without any similarity method selected.".into());
                })
                .expect("Cannot upgrade in event loop :(");
                return Ok(());
            }
            let audio_check_type =
                ALLOWED_AUDIO_CHECK_TYPE_VALUES[get_audio_check_type_idx(&custom_settings.similar_music_sub_audio_check_type).expect("Failed to get audio check type")].2;

            let params = SameMusicParameters::new(
                music_similarity,
                custom_settings.similar_music_sub_approximate_comparison,
                audio_check_type,
                custom_settings.similar_music_sub_minimal_fragment_duration_value,
                custom_settings.similar_music_sub_maximum_difference_value as f64,
                custom_settings.similar_music_compare_fingerprints_only_with_similar_titles,
            );
            let mut item = SameMusic::new(params);
            set_common_settings(&mut item, &custom_settings);

            item.find_same_music(Some(&stop_receiver), Some(&progress_sender));

            let messages = item.get_text_messages().create_messages_text();

            let mut vector;
            if item.get_use_reference() {
                vector = item
                    .get_similar_music_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect::<Vec<_>>();
            } else {
                vector = item.get_duplicated_music_entries().iter().cloned().map(|items| (None, items)).collect::<Vec<_>>();
            }
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));
            }

            shared_models.lock().unwrap().shared_same_music_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_similar_music_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_similar_music_results(app: &MainWindow, vector: Vec<(Option<MusicEntry>, Vec<MusicEntry>)>, messages: String) {
    let items_found = vector.len();
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
    app.invoke_scan_ended(format!("Found {items_found} similar music files").into());
    app.global::<GuiState>().set_info_text(messages.into());
}
fn prepare_data_model_similar_music(fe: &MusicEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str = VecModel::from_slice(&[
        format_size(fe.size, BINARY).into(),
        file.into(),
        fe.track_title.clone().into(),
        fe.track_artist.clone().into(),
        fe.year.clone().into(),
        fe.bitrate.to_string().into(),
        fe.length.clone().into(),
        fe.genre.clone().into(),
        directory.into(),
        DateTime::from_timestamp(fe.get_modified_date() as i64, 0)
            .expect("Cannot create DateTime")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}
// Invalid Symlinks
fn scan_invalid_symlinks(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut item = InvalidSymlinks::new();
            set_common_settings(&mut item, &custom_settings);

            item.find_invalid_links(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = item.get_invalid_symlinks().clone();
            let messages = item.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            shared_models.lock().unwrap().shared_same_invalid_symlinks = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_invalid_symlinks_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_invalid_symlinks_results(app: &MainWindow, vector: Vec<SymlinksFileEntry>, messages: String) {
    let items_found = vector.len();
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_invalid_symlinks(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_invalid_symlinks_model(items.into());
    app.invoke_scan_ended(format!("Found {items_found} invalid symlinks").into());
    app.global::<GuiState>().set_info_text(messages.into());
}

fn prepare_data_model_invalid_symlinks(fe: &SymlinksFileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(fe.get_path());
    let data_model_str = VecModel::from_slice(&[
        file.into(),
        directory.into(),
        fe.symlink_info.destination_path.to_string_lossy().to_string().into(),
        fe.symlink_info.type_of_error.to_string().into(),
        DateTime::from_timestamp(fe.get_modified_date() as i64, 0)
            .expect("Cannot create DateTime")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1]);
    (data_model_str, data_model_int)
} ////////////////////////////////////////// Temporary Files
fn scan_temporary_files(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut item = Temporary::new();
            set_common_settings(&mut item, &custom_settings);

            item.find_temporary_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = item.get_temporary_files().clone();
            let messages = item.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            shared_models.lock().unwrap().shared_temporary_files_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_temporary_files_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_temporary_files_results(app: &MainWindow, vector: Vec<TemporaryFileEntry>, messages: String) {
    let items_found = vector.len();
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_temporary_files(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_temporary_files_model(items.into());
    app.invoke_scan_ended(format!("Found {items_found} files").into());
    app.global::<GuiState>().set_info_text(messages.into());
}

fn prepare_data_model_temporary_files(fe: &TemporaryFileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str = VecModel::from_slice(&[
        file.into(),
        directory.into(),
        DateTime::from_timestamp(fe.modified_date as i64, 0)
            .expect("Modified date always should be in valid range")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1]);
    (data_model_str, data_model_int)
}
////////////////////////////////////////// Broken Files
fn scan_broken_files(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
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
                    app.invoke_scan_ended("Cannot find broken files without any file type selected.".into());
                })
                .expect("Cannot upgrade in event loop :(");
                return Ok(());
            }

            let params = BrokenFilesParameters::new(checked_types);
            let mut item = BrokenFiles::new(params);
            set_common_settings(&mut item, &custom_settings);

            item.find_broken_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = item.get_broken_files().clone();
            let messages = item.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            shared_models.lock().unwrap().shared_broken_files_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_broken_files_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_broken_files_results(app: &MainWindow, vector: Vec<BrokenEntry>, messages: String) {
    let items_found = vector.len();
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_broken_files(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_broken_files_model(items.into());
    app.invoke_scan_ended(format!("Found {items_found} files").into());
    app.global::<GuiState>().set_info_text(messages.into());
}

fn prepare_data_model_broken_files(fe: &BrokenEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str = VecModel::from_slice(&[
        file.into(),
        directory.into(),
        fe.error_string.clone().into(),
        format_size(fe.size, BINARY).into(),
        DateTime::from_timestamp(fe.modified_date as i64, 0)
            .expect("Modified date always should be in valid range")
            .to_string()
            .into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}
////////////////////////////////////////// Bad Extensions
fn scan_bad_extensions(
    a: Weak<MainWindow>,
    progress_sender: Sender<ProgressData>,
    stop_receiver: Receiver<()>,
    custom_settings: SettingsCustom,
    shared_models: Arc<Mutex<SharedModels>>,
) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let params = BadExtensionsParameters::new();
            let mut item = BadExtensions::new(params);
            set_common_settings(&mut item, &custom_settings);
            item.find_bad_extensions_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = item.get_bad_extensions_files().clone();
            let messages = item.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            shared_models.lock().unwrap().shared_bad_extensions_state = Some(item);

            a.upgrade_in_event_loop(move |app| {
                write_bad_extensions_results(&app, vector, messages);
            })
        })
        .expect("Cannot start thread - not much we can do here");
}
fn write_bad_extensions_results(app: &MainWindow, vector: Vec<BadFileEntry>, messages: String) {
    let items_found = vector.len();
    let items = Rc::new(VecModel::default());
    for fe in vector {
        let (data_model_str, data_model_int) = prepare_data_model_bad_extensions(&fe);
        insert_data_to_model(&items, data_model_str, data_model_int, None);
    }
    app.set_bad_extensions_model(items.into());
    app.invoke_scan_ended(format!("Found {items_found} files with bad extensions").into());
    app.global::<GuiState>().set_info_text(messages.into());
}

fn prepare_data_model_bad_extensions(fe: &BadFileEntry) -> (ModelRc<SharedString>, ModelRc<i32>) {
    let (directory, file) = split_path(&fe.path);
    let data_model_str = VecModel::from_slice(&[
        file.into(),
        directory.into(),
        fe.current_extension.clone().into(),
        fe.proper_extensions_group.clone().into(),
        fe.proper_extension.clone().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}
////////////////////////////////////////// Common
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

fn set_common_settings<T>(component: &mut T, custom_settings: &SettingsCustom)
where
    T: CommonData,
{
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
