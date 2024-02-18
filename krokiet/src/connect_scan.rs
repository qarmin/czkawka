use std::rc::Rc;
use std::thread;

use chrono::NaiveDateTime;
use crossbeam_channel::{Receiver, Sender};
use czkawka_core::bad_extensions::{BadExtensions, BadFileEntry};
use czkawka_core::big_file::{BigFile, SearchMode};
use czkawka_core::broken_files::{BrokenEntry, BrokenFiles, CheckedTypes};
use humansize::{format_size, BINARY};
use rayon::prelude::*;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use czkawka_core::common::{split_path, split_path_compare, DEFAULT_THREAD_SIZE};
use czkawka_core::common_dir_traversal::{CheckingMethod, FileEntry, ProgressData};
use czkawka_core::common_tool::CommonData;
use czkawka_core::common_traits::ResultEntry;
use czkawka_core::duplicate::{DuplicateEntry, DuplicateFinder};
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::{EmptyFolder, FolderEntry};
use czkawka_core::invalid_symlinks::{InvalidSymlinks, SymlinksFileEntry};
use czkawka_core::same_music::{MusicEntry, MusicSimilarity, SameMusic};
use czkawka_core::similar_images;
use czkawka_core::similar_images::{ImagesEntry, SimilarImages};
use czkawka_core::similar_videos::{SimilarVideos, VideosEntry};
use czkawka_core::temporary::{Temporary, TemporaryFileEntry};

use crate::common::{check_if_all_included_dirs_are_referenced, split_u64_into_i32s};
use crate::settings::{
    collect_settings, get_audio_check_type_idx, get_biggest_item_idx, get_duplicates_check_method_idx, get_duplicates_hash_type_idx, get_image_hash_alg_idx,
    get_resize_algorithm_idx, SettingsCustom, ALLOWED_AUDIO_CHECK_TYPE_VALUES, ALLOWED_BIG_FILE_SIZE_VALUES, ALLOWED_DUPLICATES_CHECK_METHOD_VALUES,
    ALLOWED_DUPLICATES_HASH_TYPE_VALUES, ALLOWED_IMAGE_HASH_ALG_VALUES, ALLOWED_RESIZE_ALGORITHM_VALUES,
};
use crate::{CurrentTab, GuiState, MainListModel, MainWindow, ProgressToSend};

pub fn connect_scan_button(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>) {
    let a = app.as_weak();
    app.on_scan_starting(move |active_tab| {
        let app = a.upgrade().unwrap();

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

        let a = app.as_weak();
        match active_tab {
            CurrentTab::DuplicateFiles => {
                scan_duplicates(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::EmptyFolders => {
                scan_empty_folders(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::BigFiles => {
                scan_big_files(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::EmptyFiles => {
                scan_empty_files(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::SimilarImages => {
                scan_similar_images(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::SimilarVideos => {
                scan_similar_videos(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::SimilarMusic => {
                scan_similar_music(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::InvalidSymlinks => {
                scan_invalid_symlinks(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::BadExtensions => {
                scan_bad_extensions(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::BrokenFiles => {
                scan_broken_files(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::TemporaryFiles => {
                scan_temporary_files(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::Settings | CurrentTab::About => panic!("Button should be disabled"),
        }
    });
}

// Scan Duplicates

fn scan_duplicates(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = DuplicateFinder::new();
            set_common_settings(&mut finder, &custom_settings);
            finder.set_check_method(CheckingMethod::Hash);
            finder.set_minimal_cache_file_size(custom_settings.duplicate_minimal_hash_cache_size as u64);
            finder.set_minimal_prehash_cache_file_size(custom_settings.duplicate_minimal_prehash_cache_size as u64);
            let check_method = ALLOWED_DUPLICATES_CHECK_METHOD_VALUES[get_duplicates_check_method_idx(&custom_settings.duplicates_sub_check_method).unwrap()].2;
            finder.set_check_method(check_method);
            let hash_type = ALLOWED_DUPLICATES_HASH_TYPE_VALUES[get_duplicates_hash_type_idx(&custom_settings.duplicates_sub_available_hash_type).unwrap()].2;
            finder.set_hash_type(hash_type);
            // finder.set_ignore_hard_links(custom_settings.ignore); // TODO
            finder.set_use_prehash_cache(custom_settings.duplicate_use_prehash);
            finder.set_delete_outdated_cache(custom_settings.duplicate_delete_outdated_entries);
            finder.set_case_sensitive_name_comparison(custom_settings.duplicates_sub_name_case_sensitive);
            finder.find_duplicates(Some(&stop_receiver), Some(&progress_sender));
            let messages = finder.get_text_messages().create_messages_text();

            let mut vector;
            if finder.get_use_reference() {
                match finder.get_check_method() {
                    CheckingMethod::Hash => {
                        vector = finder
                            .get_files_with_identical_hashes_referenced()
                            .values()
                            .flatten()
                            .cloned()
                            .map(|(original, other)| (Some(original), other))
                            .collect::<Vec<_>>();
                    }
                    CheckingMethod::Name | CheckingMethod::Size | CheckingMethod::SizeName => {
                        let values: Vec<_> = match finder.get_check_method() {
                            CheckingMethod::Name => finder.get_files_with_identical_name_referenced().values().cloned().collect(),
                            CheckingMethod::Size => finder.get_files_with_identical_size_referenced().values().cloned().collect(),
                            CheckingMethod::SizeName => finder.get_files_with_identical_size_names_referenced().values().cloned().collect(),
                            _ => unreachable!("Invalid check method."),
                        };
                        vector = values.into_iter().map(|(original, other)| (Some(original), other)).collect::<Vec<_>>();
                    }
                    _ => unreachable!("Invalid check method."),
                }
            } else {
                match finder.get_check_method() {
                    CheckingMethod::Hash => {
                        vector = finder.get_files_sorted_by_hash().values().flatten().cloned().map(|items| (None, items)).collect::<Vec<_>>();
                    }
                    CheckingMethod::Name | CheckingMethod::Size | CheckingMethod::SizeName => {
                        let values: Vec<_> = match finder.get_check_method() {
                            CheckingMethod::Name => finder.get_files_sorted_by_names().values().cloned().collect(),
                            CheckingMethod::Size => finder.get_files_sorted_by_size().values().cloned().collect(),
                            CheckingMethod::SizeName => finder.get_files_sorted_by_size_name().values().cloned().collect(),
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

            a.upgrade_in_event_loop(move |app| {
                write_duplicate_results(&app, vector, messages);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}

////////////////////////////////////////// Empty Folders
fn scan_empty_folders(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = EmptyFolder::new();
            set_common_settings(&mut finder, &custom_settings);
            finder.find_empty_folders(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = finder.get_empty_folder_list().values().cloned().collect::<Vec<_>>();
            let messages = finder.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            a.upgrade_in_event_loop(move |app| {
                write_empty_folders_results(&app, vector, messages);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.modified_date as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1]);
    (data_model_str, data_model_int)
}

////////////////////////////////////////// Big files
fn scan_big_files(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = BigFile::new();
            set_common_settings(&mut finder, &custom_settings);
            finder.set_number_of_files_to_check(custom_settings.biggest_files_sub_number_of_files as usize);
            let big_files_mode = ALLOWED_BIG_FILE_SIZE_VALUES[get_biggest_item_idx(&custom_settings.biggest_files_sub_method).unwrap()].2;
            finder.set_search_mode(big_files_mode);
            finder.find_big_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = finder.get_big_files().clone();
            let messages = finder.get_text_messages().create_messages_text();

            if big_files_mode == SearchMode::BiggestFiles {
                vector.par_sort_unstable_by_key(|fe| u64::MAX - fe.size);
            } else {
                vector.par_sort_unstable_by_key(|fe| fe.size);
            }

            a.upgrade_in_event_loop(move |app| {
                write_big_files_results(&app, vector, messages);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.modified_date as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}

///////////////////////////////// Empty Files
fn scan_empty_files(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = EmptyFiles::new();
            set_common_settings(&mut finder, &custom_settings);
            finder.find_empty_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = finder.get_empty_files().clone();
            let messages = finder.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            a.upgrade_in_event_loop(move |app| {
                write_empty_files_results(&app, vector, messages);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}
// Scan Similar Images

fn scan_similar_images(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = SimilarImages::new();
            set_common_settings(&mut finder, &custom_settings);

            finder.set_similarity(custom_settings.similar_images_sub_similarity as u32);
            let hash_alg = ALLOWED_IMAGE_HASH_ALG_VALUES[get_image_hash_alg_idx(&custom_settings.similar_images_sub_hash_alg).unwrap()].2;
            finder.set_hash_alg(hash_alg);
            finder.set_hash_size(custom_settings.similar_images_sub_hash_size);
            let resize_algorithm = ALLOWED_RESIZE_ALGORITHM_VALUES[get_resize_algorithm_idx(&custom_settings.similar_images_sub_resize_algorithm).unwrap()].2;
            finder.set_image_filter(resize_algorithm);
            finder.set_delete_outdated_cache(custom_settings.similar_images_delete_outdated_entries);
            finder.set_exclude_images_with_same_size(custom_settings.similar_images_sub_ignore_same_size);

            finder.find_similar_images(Some(&stop_receiver), Some(&progress_sender));

            let messages = finder.get_text_messages().create_messages_text();
            let hash_size = custom_settings.similar_images_sub_hash_size;

            let mut vector;
            if finder.get_use_reference() {
                vector = finder
                    .get_similar_images_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect::<Vec<_>>();
            } else {
                vector = finder.get_similar_images().iter().cloned().map(|items| (None, items)).collect::<Vec<_>>();
            }
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by_key(|e| e.similarity);
            }

            a.upgrade_in_event_loop(move |app| {
                write_similar_images_results(&app, vector, messages, hash_size);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1, fe.width as i32, fe.height as i32]);
    (data_model_str, data_model_int)
}

// Scan Similar Videos

fn scan_similar_videos(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = SimilarVideos::new();
            set_common_settings(&mut finder, &custom_settings);

            finder.set_tolerance(custom_settings.similar_videos_sub_similarity);
            finder.set_delete_outdated_cache(custom_settings.similar_videos_delete_outdated_entries);
            finder.set_exclude_videos_with_same_size(custom_settings.similar_videos_sub_ignore_same_size);

            finder.find_similar_videos(Some(&stop_receiver), Some(&progress_sender));

            let messages = finder.get_text_messages().create_messages_text();

            let mut vector;
            if finder.get_use_reference() {
                vector = finder
                    .get_similar_videos_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect::<Vec<_>>();
            } else {
                vector = finder.get_similar_videos().iter().cloned().map(|items| (None, items)).collect::<Vec<_>>();
            }
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));
            }

            a.upgrade_in_event_loop(move |app| {
                write_similar_videos_results(&app, vector, messages);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}
// Scan Similar Music
fn scan_similar_music(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = SameMusic::new();
            set_common_settings(&mut finder, &custom_settings);

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
                .unwrap();
                return Ok(());
            }

            finder.set_music_similarity(music_similarity);
            finder.set_maximum_difference(custom_settings.similar_music_sub_maximum_difference_value as f64);
            finder.set_minimum_segment_duration(custom_settings.similar_music_sub_minimal_fragment_duration_value);
            let audio_check_type = ALLOWED_AUDIO_CHECK_TYPE_VALUES[get_audio_check_type_idx(&custom_settings.similar_music_sub_audio_check_type).unwrap()].2;
            finder.set_check_type(audio_check_type);
            finder.set_approximate_comparison(custom_settings.similar_music_sub_approximate_comparison);

            finder.find_same_music(Some(&stop_receiver), Some(&progress_sender));

            let messages = finder.get_text_messages().create_messages_text();

            let mut vector;
            if finder.get_use_reference() {
                vector = finder
                    .get_similar_music_referenced()
                    .iter()
                    .cloned()
                    .map(|(original, others)| (Some(original), others))
                    .collect::<Vec<_>>();
            } else {
                vector = finder.get_duplicated_music_entries().iter().cloned().map(|items| (None, items)).collect::<Vec<_>>();
            }
            for (_first_entry, vec_fe) in &mut vector {
                vec_fe.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));
            }

            a.upgrade_in_event_loop(move |app| {
                write_similar_music_results(&app, vector, messages);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}
// Invalid Symlinks
fn scan_invalid_symlinks(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = InvalidSymlinks::new();
            set_common_settings(&mut finder, &custom_settings);

            finder.find_invalid_links(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = finder.get_invalid_symlinks().clone();
            let messages = finder.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            a.upgrade_in_event_loop(move |app| {
                write_invalid_symlinks_results(&app, vector, messages);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1]);
    (data_model_str, data_model_int)
} ////////////////////////////////////////// Temporary Files
fn scan_temporary_files(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = Temporary::new();
            set_common_settings(&mut finder, &custom_settings);

            finder.find_temporary_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = finder.get_temporary_files().clone();
            let messages = finder.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            a.upgrade_in_event_loop(move |app| {
                write_temporary_files_results(&app, vector, messages);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.modified_date as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1]);
    (data_model_str, data_model_int)
}
////////////////////////////////////////// Broken Files
fn scan_broken_files(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = BrokenFiles::new();
            set_common_settings(&mut finder, &custom_settings);

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
                .unwrap();
                return Ok(());
            }

            finder.set_checked_types(checked_types);
            finder.find_broken_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = finder.get_broken_files().clone();
            let messages = finder.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            a.upgrade_in_event_loop(move |app| {
                write_broken_files_results(&app, vector, messages);
            })
        })
        .unwrap();
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
        NaiveDateTime::from_timestamp_opt(fe.modified_date as i64, 0).unwrap().to_string().into(),
    ]);
    let modification_split = split_u64_into_i32s(fe.get_modified_date());
    let size_split = split_u64_into_i32s(fe.size);
    let data_model_int = VecModel::from_slice(&[modification_split.0, modification_split.1, size_split.0, size_split.1]);
    (data_model_str, data_model_int)
}
////////////////////////////////////////// Bad Extensions
fn scan_bad_extensions(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = BadExtensions::new();
            set_common_settings(&mut finder, &custom_settings);
            finder.find_bad_extensions_files(Some(&stop_receiver), Some(&progress_sender));

            let mut vector = finder.get_bad_extensions_files().clone();
            let messages = finder.get_text_messages().create_messages_text();

            vector.par_sort_unstable_by(|a, b| split_path_compare(a.path.as_path(), b.path.as_path()));

            a.upgrade_in_event_loop(move |app| {
                write_bad_extensions_results(&app, vector, messages);
            })
        })
        .unwrap();
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
    let data_model_str = VecModel::from_slice(&[file.into(), directory.into(), fe.current_extension.clone().into(), fe.proper_extensions.clone().into()]);
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
