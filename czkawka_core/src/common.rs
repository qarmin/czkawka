#![allow(unused_imports)]
// I don't wanna fight with unused(heif) imports in this file, so simply ignore it to avoid too much complexity

use std::cmp::Ordering;
use std::ffi::OsString;
use std::fs::{DirEntry, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{atomic, Arc};
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, Instant, SystemTime};
use std::{fs, thread};

#[cfg(feature = "heif")]
use anyhow::Result;
use crossbeam_channel::Sender;
use directories_next::ProjectDirs;
use fun_time::fun_time;
use handsome_logger::{ColorChoice, ConfigBuilder, TerminalMode};
use image::{DynamicImage, ImageBuffer, Rgb};
use imagepipe::{ImageSource, Pipeline};
#[cfg(feature = "heif")]
use libheif_rs::{ColorSpace, HeifContext, RgbChroma};
#[cfg(feature = "libraw")]
use libraw::Processor;
use log::{debug, error, info, warn, LevelFilter, Record};
use rawloader::RawLoader;
use symphonia::core::conv::IntoSample;

// #[cfg(feature = "heif")]
// use libheif_rs::LibHeif;
use crate::common_dir_traversal::{CheckingMethod, ToolType};
use crate::common_directory::Directories;
use crate::common_items::{ExcludedItems, SingleExcludedItem};
use crate::common_messages::Messages;
use crate::common_tool::{CommonData, DeleteMethod};
use crate::common_traits::ResultEntry;
use crate::duplicate::make_hard_link;
use crate::progress_data::{CurrentStage, ProgressData};
use crate::CZKAWKA_VERSION;

static NUMBER_OF_THREADS: state::InitCell<usize> = state::InitCell::new();
static ALL_AVAILABLE_THREADS: state::InitCell<usize> = state::InitCell::new();
pub const DEFAULT_THREAD_SIZE: usize = 8 * 1024 * 1024; // 8 MB
pub const DEFAULT_WORKER_THREAD_SIZE: usize = 4 * 1024 * 1024; // 4 MB

pub fn get_number_of_threads() -> usize {
    let data = NUMBER_OF_THREADS.get();
    if *data >= 1 {
        *data
    } else {
        get_all_available_threads()
    }
}

fn filtering_messages(record: &Record) -> bool {
    if let Some(module_path) = record.module_path() {
        module_path.starts_with("czkawka") || module_path.starts_with("krokiet")
    } else {
        true
    }
}

pub fn setup_logger(disabled_printing: bool) {
    let log_level = if disabled_printing { LevelFilter::Off } else { LevelFilter::Info };

    let config = ConfigBuilder::default().set_level(log_level).set_message_filtering(Some(filtering_messages)).build();
    handsome_logger::TermLogger::init(config, TerminalMode::Mixed, ColorChoice::Always).expect("Cannot initialize logger");
}

pub fn get_all_available_threads() -> usize {
    *ALL_AVAILABLE_THREADS.get_or_init(|| {
        let available_threads = thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1);
        ALL_AVAILABLE_THREADS.set(available_threads);
        available_threads
    })
}

pub fn print_version_mode() {
    let rust_version = env!("RUST_VERSION_INTERNAL");
    let debug_release = if cfg!(debug_assertions) { "debug" } else { "release" };

    let processors = get_all_available_threads();

    let info = os_info::get();
    info!(
        "App version: {CZKAWKA_VERSION}, {debug_release} mode, rust {rust_version}, os {} {} [{} {}], {processors} cpu/threads",
        info.os_type(),
        info.version(),
        std::env::consts::ARCH,
        info.bitness(),
    );
    if cfg!(debug_assertions) {
        warn!("You are running debug version of app which is a lot of slower than release version.");
    }

    if option_env!("USING_CRANELIFT").is_some() {
        warn!("You are running app with cranelift which is intended only for fast compilation, not runtime performance.");
    }
}

pub fn set_default_number_of_threads() {
    set_number_of_threads(get_all_available_threads());
}

pub fn set_number_of_threads(thread_number: usize) {
    NUMBER_OF_THREADS.set(thread_number);

    let additional_message = if thread_number == 0 {
        " (0 - means that all available threads will be used)"
    } else {
        ""
    };
    debug!("Number of threads set to {thread_number}{additional_message}");

    rayon::ThreadPoolBuilder::new()
        .num_threads(get_number_of_threads())
        .stack_size(DEFAULT_WORKER_THREAD_SIZE)
        .build_global()
        .expect("Cannot set number of threads");
}

pub const RAW_IMAGE_EXTENSIONS: &[&str] = &[
    "mrw", "arw", "srf", "sr2", "mef", "orf", "srw", "erf", "kdc", "kdc", "dcs", "rw2", "raf", "dcr", "dng", "pef", "crw", "iiq", "3fr", "nrw", "nef", "mos", "cr2", "ari",
];
pub const IMAGE_RS_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "bmp", "tiff", "tif", "tga", "ff", "jif", "jfi", "webp", "gif", "ico", "exr", "qoi"];

pub const IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "bmp", "webp", "exr", "qoi"];

pub const IMAGE_RS_BROKEN_FILES_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "gif", "bmp", "ico", "jfif", "jpe", "pnz", "dib", "webp", "exr",
];
pub const HEIC_EXTENSIONS: &[&str] = &["heif", "heifs", "heic", "heics", "avci", "avcs", "avifs"];

pub const ZIP_FILES_EXTENSIONS: &[&str] = &["zip", "jar"];

pub const PDF_FILES_EXTENSIONS: &[&str] = &["pdf"];

pub const AUDIO_FILES_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "wav", "ogg", "m4a", "aac", "aiff", "pcm", "aif", "aiff", "aifc", "m3a", "mp2", "mp4a", "mp2a", "mpga", "wave", "weba", "wma", "oga",
];

pub const VIDEO_FILES_EXTENSIONS: &[&str] = &[
    "mp4", "mpv", "flv", "mp4a", "webm", "mpg", "mp2", "mpeg", "m4p", "m4v", "avi", "wmv", "qt", "mov", "swf", "mkv",
];

pub const LOOP_DURATION: u32 = 20; //ms
pub const SEND_PROGRESS_DATA_TIME_BETWEEN: u32 = 200; //ms

pub fn remove_folder_if_contains_only_empty_folders(path: impl AsRef<Path>, remove_to_trash: bool) -> Result<(), String> {
    let path = path.as_ref();
    if !path.is_dir() {
        return Err(format!("Trying to remove folder {path:?} which is not a directory",));
    }

    let mut entries_to_check = Vec::new();
    let Ok(initial_entry) = path.read_dir() else {
        return Err(format!("Cannot read directory {path:?}",));
    };
    for entry in initial_entry {
        if let Ok(entry) = entry {
            entries_to_check.push(entry);
        } else {
            return Err(format!("Cannot read entry from directory {path:?}"));
        }
    }
    loop {
        let Some(entry) = entries_to_check.pop() else {
            break;
        };
        let Some(file_type) = entry.file_type().ok() else {
            return Err(format!("Folder contains file with unknown type {:?} inside {path:?}", entry.path()));
        };

        if !file_type.is_dir() {
            return Err(format!("Folder contains file {:?} inside {path:?}", entry.path(),));
        }
        let Ok(internal_read_dir) = entry.path().read_dir() else {
            return Err(format!("Cannot read directory {:?} inside {path:?}", entry.path()));
        };
        for internal_elements in internal_read_dir {
            if let Ok(internal_element) = internal_elements {
                entries_to_check.push(internal_element);
            } else {
                return Err(format!("Cannot read entry from directory {:?} inside {path:?}", entry.path()));
            }
        }
    }

    if remove_to_trash {
        trash::delete(path).map_err(|e| format!("Cannot move folder {path:?} to trash, reason {e}"))
    } else {
        fs::remove_dir_all(path).map_err(|e| format!("Cannot remove directory {path:?}, reason {e}"))
    }
}

pub fn open_cache_folder(cache_file_name: &str, save_to_cache: bool, use_json: bool, warnings: &mut Vec<String>) -> Option<((Option<File>, PathBuf), (Option<File>, PathBuf))> {
    if let Some(proj_dirs) = ProjectDirs::from("pl", "Qarmin", "Czkawka") {
        let cache_dir = PathBuf::from(proj_dirs.cache_dir());
        let cache_file = cache_dir.join(cache_file_name);
        let cache_file_json = cache_dir.join(cache_file_name.replace(".bin", ".json"));

        let mut file_handler_default = None;
        let mut file_handler_json = None;

        if save_to_cache {
            if cache_dir.exists() {
                if !cache_dir.is_dir() {
                    warnings.push(format!("Config dir {cache_dir:?} is a file!"));
                    return None;
                }
            } else if let Err(e) = fs::create_dir_all(&cache_dir) {
                warnings.push(format!("Cannot create config dir {cache_dir:?}, reason {e}"));
                return None;
            }

            file_handler_default = Some(match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file) {
                Ok(t) => t,
                Err(e) => {
                    warnings.push(format!("Cannot create or open cache file {cache_file:?}, reason {e}"));
                    return None;
                }
            });
            if use_json {
                file_handler_json = Some(match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file_json) {
                    Ok(t) => t,
                    Err(e) => {
                        warnings.push(format!("Cannot create or open cache file {cache_file_json:?}, reason {e}"));
                        return None;
                    }
                });
            }
        } else {
            if let Ok(t) = OpenOptions::new().read(true).open(&cache_file) {
                file_handler_default = Some(t);
            } else {
                if use_json {
                    file_handler_json = Some(OpenOptions::new().read(true).open(&cache_file_json).ok()?);
                } else {
                    // messages.push(format!("Cannot find or open cache file {cache_file:?}")); // No error or warning
                    return None;
                }
            }
        };
        return Some(((file_handler_default, cache_file), (file_handler_json, cache_file_json)));
    }
    None
}

#[cfg(feature = "heif")]
pub fn get_dynamic_image_from_heic(path: &str) -> Result<DynamicImage> {
    // let libheif = LibHeif::new();
    let im = HeifContext::read_from_file(path)?;
    let handle = im.primary_image_handle()?;
    // let image = libheif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?; // Enable when using libheif 0.19
    let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    let width = image.width();
    let height = image.height();
    let planes = image.planes();
    let interleaved_plane = planes.interleaved.ok_or_else(|| anyhow::anyhow!("Failed to get interleaved plane"))?;
    ImageBuffer::from_raw(width, height, interleaved_plane.data.to_owned())
        .map(DynamicImage::ImageRgb8)
        .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))
}

#[cfg(feature = "libraw")]
pub fn get_dynamic_image_from_raw_image(path: impl AsRef<Path>) -> Option<DynamicImage> {
    let buf = fs::read(path.as_ref()).ok()?;

    let processor = Processor::new();
    let start_timer = Instant::now();
    let processed = processor.process_8bit(&buf).expect("processing successful");
    println!("Processing took {:?}", start_timer.elapsed());

    let width = processed.width();
    let height = processed.height();

    let data = processed.to_vec();

    let buffer = ImageBuffer::from_raw(width, height, data)?;
    // Utwórz DynamicImage z ImageBuffer
    Some(DynamicImage::ImageRgb8(buffer))
}

#[cfg(not(feature = "libraw"))]
pub fn get_dynamic_image_from_raw_image(path: impl AsRef<Path> + std::fmt::Debug) -> Option<DynamicImage> {
    let mut start_timer = Instant::now();
    let mut times = Vec::new();

    let loader = RawLoader::new();
    let raw = loader.decode_file(path.as_ref()).ok()?;

    times.push(("After decoding", start_timer.elapsed()));
    start_timer = Instant::now();

    let source = ImageSource::Raw(raw);

    times.push(("After creating source", start_timer.elapsed()));
    start_timer = Instant::now();

    let mut pipeline = Pipeline::new_from_source(source).ok()?;

    times.push(("After creating pipeline", start_timer.elapsed()));
    start_timer = Instant::now();

    pipeline.run(None);
    let image = pipeline.output_8bit(None).ok()?;

    times.push(("After creating image", start_timer.elapsed()));
    start_timer = Instant::now();

    let image = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(image.width as u32, image.height as u32, image.data)?;

    times.push(("After creating image buffer", start_timer.elapsed()));
    start_timer = Instant::now();
    // println!("Properly hashed {:?}", path);
    let res = Some(DynamicImage::ImageRgb8(image));
    times.push(("After creating dynamic image", start_timer.elapsed()));

    let str_timer = times.into_iter().map(|(name, time)| format!("{name}: {time:?}")).collect::<Vec<_>>().join(", ");
    debug!("Loading raw image --- {str_timer}");
    res
}

pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.to_string_lossy().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.to_string_lossy().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}

pub fn split_path_compare(path_a: &Path, path_b: &Path) -> Ordering {
    match path_a.parent().cmp(&path_b.parent()) {
        Ordering::Equal => path_a.file_name().cmp(&path_b.file_name()),
        other => other,
    }
}

pub fn create_crash_message(library_name: &str, file_path: &str, home_library_url: &str) -> String {
    format!("{library_name} library crashed when opening \"{file_path}\", please check if this is fixed with the latest version of {library_name} and if it is not fixed, please report bug here - {home_library_url}")
}

pub fn regex_check(expression_item: &SingleExcludedItem, directory_name: &str) -> bool {
    if expression_item.expression_splits.is_empty() {
        return true;
    }

    // Early checking if directory contains all parts needed by expression
    for split in &expression_item.unique_extensions_splits {
        if !directory_name.contains(split) {
            return false;
        }
    }

    // `git*` shouldn't be true for `/gitsfafasfs`
    if !expression_item.expression.starts_with('*')
        && directory_name
            .find(&expression_item.expression_splits[0])
            .expect("Cannot fail, because split must exists in directory_name")
            > 0
    {
        return false;
    }
    // `*home` shouldn't be true for `/homeowner`
    if !expression_item.expression.ends_with('*')
        && !directory_name.ends_with(expression_item.expression_splits.last().expect("Cannot fail, because at least one item is available"))
    {
        return false;
    }

    // At the end we check if parts between * are correctly positioned
    let mut last_split_point = directory_name.find(&expression_item.expression_splits[0]).expect("Cannot fail, because is checked earlier");
    let mut current_index: usize = 0;
    let mut found_index: usize;
    for spl in &expression_item.expression_splits[1..] {
        found_index = match directory_name[current_index..].find(spl) {
            Some(t) => t,
            None => return false,
        };
        current_index = last_split_point + spl.len();
        last_split_point = found_index + current_index;
    }
    true
}

pub fn normalize_windows_path(path_to_change: impl AsRef<Path>) -> PathBuf {
    let path = path_to_change.as_ref();

    // Don't do anything, because network path may be case intensive
    if path.to_string_lossy().starts_with('\\') {
        return path.to_path_buf();
    }

    match path.to_str() {
        Some(path) if path.is_char_boundary(1) => {
            let replaced = path.replace('/', "\\");
            let mut new_path = OsString::new();
            if replaced[1..].starts_with(':') {
                new_path.push(replaced[..1].to_ascii_uppercase());
                new_path.push(replaced[1..].to_ascii_lowercase());
            } else {
                new_path.push(replaced.to_ascii_lowercase());
            }
            PathBuf::from(new_path)
        }
        _ => path.to_path_buf(),
    }
}

pub fn check_folder_children(
    dir_result: &mut Vec<PathBuf>,
    warnings: &mut Vec<String>,
    entry_data: &DirEntry,
    recursive_search: bool,
    directories: &Directories,
    excluded_items: &ExcludedItems,
) {
    if !recursive_search {
        return;
    }

    let next_item = entry_data.path();
    if directories.is_excluded(&next_item) {
        return;
    }

    if excluded_items.is_excluded(&next_item) {
        return;
    }

    #[cfg(target_family = "unix")]
    if directories.exclude_other_filesystems() {
        match directories.is_on_other_filesystems(&next_item) {
            Ok(true) => return,
            Err(e) => warnings.push(e),
            _ => (),
        }
    }

    dir_result.push(next_item);
}

// Here we assume, that internal Vec<> have at least 1 object
#[allow(clippy::ptr_arg)]
pub fn delete_files_custom<T>(items: &Vec<&Vec<T>>, delete_method: &DeleteMethod, text_messages: &mut Messages, dry_run: bool) -> (u64, usize, usize)
where
    T: ResultEntry + Clone,
{
    let res = items
        .iter()
        .map(|values| {
            let mut gained_space: u64 = 0;
            let mut removed_files: usize = 0;
            let mut failed_to_remove_files: usize = 0;
            let mut infos = Vec::new();
            let mut errors = Vec::new();

            let mut all_values = (*values).clone();
            let len = all_values.len();

            // Sorted from smallest to biggest or oldest to newest
            all_values.sort_unstable_by_key(match delete_method {
                DeleteMethod::AllExceptBiggest | DeleteMethod::AllExceptSmallest | DeleteMethod::OneBiggest | DeleteMethod::OneSmallest => ResultEntry::get_size,
                _ => ResultEntry::get_modified_date,
            });

            if delete_method == &DeleteMethod::HardLink {
                let original_file = &all_values[0];
                for file_entry in &all_values[1..] {
                    if dry_run {
                        infos.push(format!(
                            "dry_run - would create hardlink from {:?} to {:?}",
                            original_file.get_path(),
                            file_entry.get_path()
                        ));
                    } else {
                        if dry_run {
                            infos.push(format!("Replace file {:?} with hard link to {:?}", original_file.get_path(), file_entry.get_path()));
                        } else {
                            if let Err(e) = make_hard_link(original_file.get_path(), file_entry.get_path()) {
                                errors.push(format!(
                                    "Cannot create hard link from {:?} to {:?} - {}",
                                    file_entry.get_path(),
                                    original_file.get_path(),
                                    e
                                ));
                                failed_to_remove_files += 1;
                            } else {
                                gained_space += 1;
                                removed_files += 1;
                            }
                        }
                    }
                }

                return (infos, errors, gained_space, removed_files, failed_to_remove_files);
            }

            let items = match delete_method {
                DeleteMethod::Delete => &all_values,
                DeleteMethod::AllExceptNewest | DeleteMethod::AllExceptBiggest => &all_values[..(len - 1)],
                DeleteMethod::AllExceptOldest | DeleteMethod::AllExceptSmallest => &all_values[1..],
                DeleteMethod::OneOldest | DeleteMethod::OneSmallest => &all_values[..1],
                DeleteMethod::OneNewest | DeleteMethod::OneBiggest => &all_values[(len - 1)..],
                DeleteMethod::HardLink | DeleteMethod::None => unreachable!("HardLink and None should be handled before"),
            };

            for i in items {
                if dry_run {
                    infos.push(format!("dry_run - would delete file: {:?}", i.get_path()));
                } else {
                    if let Err(e) = fs::remove_file(i.get_path()) {
                        errors.push(format!("Cannot delete file: {:?} - {e}", i.get_path()));
                        failed_to_remove_files += 1;
                    } else {
                        removed_files += 1;
                        gained_space += i.get_size();
                    }
                }
            }
            (infos, errors, gained_space, removed_files, failed_to_remove_files)
        })
        .collect::<Vec<_>>();

    let mut gained_space = 0;
    let mut removed_files = 0;
    let mut failed_to_remove_files = 0;
    for (infos, errors, gained_space_v, removed_files_v, failed_to_remove_files_v) in res {
        text_messages.messages.extend(infos);
        text_messages.errors.extend(errors);
        gained_space += gained_space_v;
        removed_files += removed_files_v;
        failed_to_remove_files += failed_to_remove_files_v;
    }

    (gained_space, removed_files, failed_to_remove_files)
}
pub fn filter_reference_folders_generic<T>(entries_to_check: Vec<Vec<T>>, directories: &Directories) -> Vec<(T, Vec<T>)>
where
    T: ResultEntry,
{
    entries_to_check
        .into_iter()
        .filter_map(|vec_file_entry| {
            let (mut files_from_referenced_folders, normal_files): (Vec<_>, Vec<_>) =
                vec_file_entry.into_iter().partition(|e| directories.is_in_referenced_directory(e.get_path()));

            if normal_files.is_empty() {
                None
            } else {
                files_from_referenced_folders.pop().map(|file| (file, normal_files))
            }
        })
        .collect::<Vec<(T, Vec<T>)>>()
}

pub fn prepare_thread_handler_common(
    progress_sender: Option<&Sender<ProgressData>>,
    sstage: CurrentStage,
    max_value: usize,
    test_type: (ToolType, CheckingMethod),
) -> (JoinHandle<()>, Arc<AtomicBool>, Arc<AtomicUsize>, AtomicBool) {
    let (tool_type, checking_method) = test_type;
    assert_ne!(tool_type, ToolType::None, "ToolType::None should not exist");
    let progress_thread_run = Arc::new(AtomicBool::new(true));
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let check_was_stopped = AtomicBool::new(false);
    let progress_thread_sender = if let Some(progress_sender) = progress_sender {
        let progress_send = progress_sender.clone();
        let progress_thread_run = progress_thread_run.clone();
        let atomic_counter = atomic_counter.clone();
        thread::spawn(move || {
            // Use earlier time, to send immediately first message
            let mut time_since_last_send = SystemTime::now() - Duration::from_secs(10u64);

            loop {
                if time_since_last_send.elapsed().expect("Cannot count time backwards").as_millis() > SEND_PROGRESS_DATA_TIME_BETWEEN as u128 {
                    let progress_data = ProgressData {
                        sstage,
                        checking_method,
                        current_stage_idx: sstage.get_current_stage(),
                        max_stage_idx: tool_type.get_max_stage(checking_method),
                        entries_checked: atomic_counter.load(atomic::Ordering::Relaxed),
                        entries_to_check: max_value,
                        tool_type,
                    };

                    progress_data.validate();

                    progress_send.send(progress_data).expect("Cannot send progress data");
                    time_since_last_send = SystemTime::now();
                }
                if !progress_thread_run.load(atomic::Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            }
        })
    } else {
        thread::spawn(|| {})
    };
    (progress_thread_sender, progress_thread_run, atomic_counter, check_was_stopped)
}

#[inline]
pub fn check_if_stop_received(stop_receiver: Option<&crossbeam_channel::Receiver<()>>) -> bool {
    if let Some(stop_receiver) = stop_receiver {
        if stop_receiver.try_recv().is_ok() {
            return true;
        }
    }
    false
}

#[fun_time(message = "send_info_and_wait_for_ending_all_threads", level = "debug")]
pub fn send_info_and_wait_for_ending_all_threads(progress_thread_run: &Arc<AtomicBool>, progress_thread_handle: JoinHandle<()>) {
    progress_thread_run.store(false, atomic::Ordering::Relaxed);
    progress_thread_handle.join().expect("Cannot join progress thread - quite fatal error, but happens rarely");
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    use tempfile::tempdir;

    use crate::common::{normalize_windows_path, regex_check, remove_folder_if_contains_only_empty_folders};
    use crate::common_items::new_excluded_item;

    #[test]
    fn test_remove_folder_if_contains_only_empty_folders() {
        let dir = tempdir().expect("Cannot create temporary directory");
        let sub_dir = dir.path().join("sub_dir");
        fs::create_dir(&sub_dir).expect("Cannot create directory");

        // Test with empty directory
        assert!(remove_folder_if_contains_only_empty_folders(&sub_dir, false).is_ok());
        assert!(!Path::new(&sub_dir).exists());

        // Test with directory containing an empty directory
        fs::create_dir(&sub_dir).expect("Cannot create directory");
        fs::create_dir(sub_dir.join("empty_sub_dir")).expect("Cannot create directory");
        assert!(remove_folder_if_contains_only_empty_folders(&sub_dir, false).is_ok());
        assert!(!Path::new(&sub_dir).exists());

        // Test with directory containing a file
        fs::create_dir(&sub_dir).expect("Cannot create directory");
        let mut file = fs::File::create(sub_dir.join("file.txt")).expect("Cannot create file");
        writeln!(file, "Hello, world!").expect("Cannot write to file");
        assert!(remove_folder_if_contains_only_empty_folders(&sub_dir, false).is_err());
        assert!(Path::new(&sub_dir).exists());
    }

    #[test]
    fn test_regex() {
        assert!(regex_check(&new_excluded_item("*"), "/home/rafal"));
        assert!(regex_check(&new_excluded_item("*home*"), "/home/rafal"));
        assert!(regex_check(&new_excluded_item("*home"), "/home"));
        assert!(regex_check(&new_excluded_item("*home/"), "/home/"));
        assert!(regex_check(&new_excluded_item("*home/*"), "/home/"));
        assert!(regex_check(&new_excluded_item("*.git*"), "/home/.git"));
        assert!(regex_check(&new_excluded_item("*/home/rafal*rafal*rafal*rafal*"), "/home/rafal/rafalrafalrafal"));
        assert!(regex_check(&new_excluded_item("AAA"), "AAA"));
        assert!(regex_check(&new_excluded_item("AAA*"), "AAABDGG/QQPW*"));
        assert!(!regex_check(&new_excluded_item("*home"), "/home/"));
        assert!(!regex_check(&new_excluded_item("*home"), "/homefasfasfasfasf/"));
        assert!(!regex_check(&new_excluded_item("*home"), "/homefasfasfasfasf"));
        assert!(!regex_check(&new_excluded_item("rafal*afal*fal"), "rafal"));
        assert!(!regex_check(&new_excluded_item("rafal*a"), "rafal"));
        assert!(!regex_check(&new_excluded_item("AAAAAAAA****"), "/AAAAAAAAAAAAAAAAA"));
        assert!(!regex_check(&new_excluded_item("*.git/*"), "/home/.git"));
        assert!(!regex_check(&new_excluded_item("*home/*koc"), "/koc/home/"));
        assert!(!regex_check(&new_excluded_item("*home/"), "/home"));
        assert!(!regex_check(&new_excluded_item("*TTT"), "/GGG"));
        assert!(regex_check(
            &new_excluded_item("*/home/*/.local/share/containers"),
            "/var/home/roman/.local/share/containers"
        ));

        if cfg!(target_family = "windows") {
            assert!(regex_check(&new_excluded_item("*\\home"), "C:\\home"));
            assert!(regex_check(&new_excluded_item("*/home"), "C:\\home"));
        }
    }

    #[test]
    fn test_windows_path() {
        assert_eq!(PathBuf::from("C:\\path.txt"), normalize_windows_path("c:/PATH.tXt"));
        assert_eq!(PathBuf::from("H:\\reka\\weza\\roman.txt"), normalize_windows_path("h:/RekA/Weza\\roMan.Txt"));
        assert_eq!(PathBuf::from("T:\\a"), normalize_windows_path("T:\\A"));
        assert_eq!(PathBuf::from("\\\\aBBa"), normalize_windows_path("\\\\aBBa"));
        assert_eq!(PathBuf::from("a"), normalize_windows_path("a"));
        assert_eq!(PathBuf::from(""), normalize_windows_path(""));
    }
}
