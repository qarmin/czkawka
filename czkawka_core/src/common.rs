use std::ffi::OsString;
use std::fs::{DirEntry, File, OpenOptions};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, SystemTime};
use std::{fs, thread};

#[cfg(feature = "heif")]
use anyhow::Result;
use directories_next::ProjectDirs;
use fun_time::fun_time;
use futures::channel::mpsc::UnboundedSender;
use handsome_logger::{ColorChoice, ConfigBuilder, TerminalMode};
use image::{DynamicImage, ImageBuffer, Rgb};
use imagepipe::{ImageSource, Pipeline};
#[cfg(feature = "heif")]
use libheif_rs::{ColorSpace, HeifContext, RgbChroma};
use log::{info, LevelFilter, Record};

// #[cfg(feature = "heif")]
// use libheif_rs::LibHeif;
use crate::common_dir_traversal::{CheckingMethod, ProgressData, ToolType};
use crate::common_directory::Directories;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_tool::DeleteMethod;
use crate::common_traits::ResultEntry;
use crate::duplicate::make_hard_link;
use crate::CZKAWKA_VERSION;

static NUMBER_OF_THREADS: state::InitCell<usize> = state::InitCell::new();

pub fn get_number_of_threads() -> usize {
    let data = NUMBER_OF_THREADS.get();
    if *data >= 1 {
        *data
    } else {
        get_default_number_of_threads()
    }
}

fn filtering_messages(record: &Record) -> bool {
    if let Some(module_path) = record.module_path() {
        module_path.starts_with("czkawka")
    } else {
        true
    }
}

pub fn setup_logger(disabled_printing: bool) {
    let log_level = if disabled_printing { LevelFilter::Off } else { LevelFilter::Info };

    let config = ConfigBuilder::default().set_level(log_level).set_message_filtering(Some(filtering_messages)).build();
    handsome_logger::TermLogger::init(config, TerminalMode::Mixed, ColorChoice::Always).unwrap();
}

pub fn print_version_mode() {
    info!(
        "Czkawka version: {}, was compiled with {} mode",
        CZKAWKA_VERSION,
        if cfg!(debug_assertions) { "debug" } else { "release" }
    );
}

pub fn set_default_number_of_threads() {
    set_number_of_threads(get_default_number_of_threads());
}

pub fn get_default_number_of_threads() -> usize {
    thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1)
}

pub fn set_number_of_threads(thread_number: usize) {
    NUMBER_OF_THREADS.set(thread_number);

    rayon::ThreadPoolBuilder::new().num_threads(get_number_of_threads()).build_global().unwrap();
}

pub const RAW_IMAGE_EXTENSIONS: &[&str] = &[
    ".mrw", ".arw", ".srf", ".sr2", ".mef", ".orf", ".srw", ".erf", ".kdc", ".kdc", ".dcs", ".rw2", ".raf", ".dcr", ".dng", ".pef", ".crw", ".iiq", ".3fr", ".nrw", ".nef", ".mos",
    ".cr2", ".ari",
];
pub const IMAGE_RS_EXTENSIONS: &[&str] = &[
    ".jpg", ".jpeg", ".png", ".bmp", ".tiff", ".tif", ".tga", ".ff", ".jif", ".jfi", ".webp", ".gif", ".ico", ".exr", ".qoi",
];

pub const IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS: &[&str] = &[".jpg", ".jpeg", ".png", ".tiff", ".tif", ".tga", ".ff", ".jif", ".jfi", ".bmp", ".webp", ".exr", ".qoi"];

pub const IMAGE_RS_BROKEN_FILES_EXTENSIONS: &[&str] = &[
    ".jpg", ".jpeg", ".png", ".tiff", ".tif", ".tga", ".ff", ".jif", ".jfi", ".gif", ".bmp", ".ico", ".jfif", ".jpe", ".pnz", ".dib", ".webp", ".exr",
];
pub const HEIC_EXTENSIONS: &[&str] = &[".heif", ".heifs", ".heic", ".heics", ".avci", ".avcs", ".avifs"];

pub const ZIP_FILES_EXTENSIONS: &[&str] = &[".zip", ".jar"];

pub const PDF_FILES_EXTENSIONS: &[&str] = &[".pdf"];

pub const AUDIO_FILES_EXTENSIONS: &[&str] = &[
    ".mp3", ".flac", ".wav", ".ogg", ".m4a", ".aac", ".aiff", ".pcm", ".aif", ".aiff", ".aifc", ".m3a", ".mp2", ".mp4a", ".mp2a", ".mpga", ".wave", ".weba", ".wma", ".oga",
];

pub const VIDEO_FILES_EXTENSIONS: &[&str] = &[
    ".mp4", ".mpv", ".flv", ".mp4a", ".webm", ".mpg", ".mp2", ".mpeg", ".m4p", ".m4v", ".avi", ".wmv", ".qt", ".mov", ".swf", ".mkv",
];

pub const LOOP_DURATION: u32 = 20; //ms
pub const SEND_PROGRESS_DATA_TIME_BETWEEN: u32 = 200; //ms

pub struct Common();

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
                    warnings.push(format!("Config dir {} is a file!", cache_dir.display()));
                    return None;
                }
            } else if let Err(e) = fs::create_dir_all(&cache_dir) {
                warnings.push(format!("Cannot create config dir {}, reason {}", cache_dir.display(), e));
                return None;
            }

            file_handler_default = Some(match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file) {
                Ok(t) => t,
                Err(e) => {
                    warnings.push(format!("Cannot create or open cache file {}, reason {}", cache_file.display(), e));
                    return None;
                }
            });
            if use_json {
                file_handler_json = Some(match OpenOptions::new().truncate(true).write(true).create(true).open(&cache_file_json) {
                    Ok(t) => t,
                    Err(e) => {
                        warnings.push(format!("Cannot create or open cache file {}, reason {}", cache_file_json.display(), e));
                        return None;
                    }
                });
            }
        } else {
            if let Ok(t) = OpenOptions::new().read(true).open(&cache_file) {
                file_handler_default = Some(t);
            } else {
                if use_json {
                    file_handler_json = Some(match OpenOptions::new().read(true).open(&cache_file_json) {
                        Ok(t) => t,
                        Err(_) => return None,
                    });
                } else {
                    // messages.push(format!("Cannot find or open cache file {}", cache_file.display())); // No error or warning
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
    let interleaved_plane = planes.interleaved.unwrap();
    ImageBuffer::from_raw(width, height, interleaved_plane.data.to_owned())
        .map(DynamicImage::ImageRgb8)
        .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))
}

pub fn get_dynamic_image_from_raw_image(path: impl AsRef<Path> + std::fmt::Debug) -> Option<DynamicImage> {
    let file_handler = match OpenOptions::new().read(true).open(&path) {
        Ok(t) => t,
        Err(_e) => {
            return None;
        }
    };

    let mut reader = BufReader::new(file_handler);
    let raw = match rawloader::decode(&mut reader) {
        Ok(raw) => raw,
        Err(_e) => {
            return None;
        }
    };

    let source = ImageSource::Raw(raw);

    let mut pipeline = match Pipeline::new_from_source(source) {
        Ok(pipeline) => pipeline,
        Err(_e) => {
            return None;
        }
    };

    pipeline.run(None);
    let image = match pipeline.output_8bit(None) {
        Ok(image) => image,
        Err(_e) => {
            return None;
        }
    };

    let Some(image) = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(image.width as u32, image.height as u32, image.data) else {
        return None;
    };

    // println!("Properly hashed {:?}", path);
    Some(DynamicImage::ImageRgb8(image))
}

pub fn split_path(path: &Path) -> (String, String) {
    match (path.parent(), path.file_name()) {
        (Some(dir), Some(file)) => (dir.display().to_string(), file.to_string_lossy().into_owned()),
        (Some(dir), None) => (dir.display().to_string(), String::new()),
        (None, _) => (String::new(), String::new()),
    }
}

pub fn create_crash_message(library_name: &str, file_path: &str, home_library_url: &str) -> String {
    format!("{library_name} library crashed when opening \"{file_path}\", please check if this is fixed with the latest version of {library_name} (e.g. with https://github.com/qarmin/crates_tester) and if it is not fixed, please report bug here - {home_library_url}")
}

impl Common {
    pub fn regex_check(expression: &str, directory: impl AsRef<Path>) -> bool {
        if expression == "*" {
            return true;
        }

        let temp_splits: Vec<&str> = expression.split('*').collect();
        let mut splits: Vec<&str> = Vec::new();
        for i in temp_splits {
            if !i.is_empty() {
                splits.push(i);
            }
        }
        if splits.is_empty() {
            return false;
        }

        // Get rid of non unicode characters
        let directory = directory.as_ref().to_string_lossy();

        // Early checking if directory contains all parts needed by expression
        for split in &splits {
            if !directory.contains(split) {
                return false;
            }
        }

        let mut position_of_splits: Vec<usize> = Vec::new();

        // `git*` shouldn't be true for `/gitsfafasfs`
        if !expression.starts_with('*') && directory.find(splits[0]).unwrap() > 0 {
            return false;
        }
        // `*home` shouldn't be true for `/homeowner`
        if !expression.ends_with('*') && !directory.ends_with(splits.last().unwrap()) {
            return false;
        }

        // At the end we check if parts between * are correctly positioned
        position_of_splits.push(directory.find(splits[0]).unwrap());
        let mut current_index: usize;
        let mut found_index: usize;
        for i in splits[1..].iter().enumerate() {
            current_index = *position_of_splits.get(i.0).unwrap() + i.1.len();
            found_index = match directory[current_index..].find(i.1) {
                Some(t) => t,
                None => return false,
            };
            position_of_splits.push(found_index + current_index);
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
}

pub fn check_folder_children(
    dir_result: &mut Vec<PathBuf>,
    warnings: &mut Vec<String>,
    current_folder: &Path,
    entry_data: &DirEntry,
    recursive_search: bool,
    directories: &Directories,
    excluded_items: &ExcludedItems,
) {
    if !recursive_search {
        return;
    }

    let next_folder = current_folder.join(entry_data.file_name());
    if directories.is_excluded(&next_folder) {
        return;
    }

    if excluded_items.is_excluded(&next_folder) {
        return;
    }

    #[cfg(target_family = "unix")]
    if directories.exclude_other_filesystems() {
        match directories.is_on_other_filesystems(&next_folder) {
            Ok(true) => return,
            Err(e) => warnings.push(e),
            _ => (),
        }
    }

    dir_result.push(next_folder);
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

            // Sorted from oldest to newest - from smallest value to bigger
            all_values.sort_unstable_by_key(ResultEntry::get_modified_date);

            if delete_method == &DeleteMethod::HardLink {
                let original_file = &all_values[0];
                for file_entry in &all_values[1..] {
                    if dry_run {
                        infos.push(format!(
                            "dry_run - would create hardlink from {:?} to {:?}",
                            original_file.get_path(),
                            original_file.get_path()
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
                DeleteMethod::AllExceptNewest => &all_values[..(len - 1)],
                DeleteMethod::AllExceptOldest => &all_values[1..],
                DeleteMethod::OneOldest => &all_values[..1],
                DeleteMethod::OneNewest => &all_values[(len - 1)..],
                DeleteMethod::HardLink | DeleteMethod::None => unreachable!("HardLink and None should be handled before"),
            };

            for i in items {
                if dry_run {
                    infos.push(format!("dry_run - would delete file: {:?}", i.get_path()));
                } else {
                    if let Err(e) = std::fs::remove_file(i.get_path()) {
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

            if files_from_referenced_folders.is_empty() || normal_files.is_empty() {
                None
            } else {
                Some((files_from_referenced_folders.pop().unwrap(), normal_files))
            }
        })
        .collect::<Vec<(T, Vec<T>)>>()
}

pub fn prepare_thread_handler_common(
    progress_sender: Option<&UnboundedSender<ProgressData>>,
    current_stage: u8,
    max_stage: u8,
    max_value: usize,
    checking_method: CheckingMethod,
    tool_type: ToolType,
) -> (JoinHandle<()>, Arc<AtomicBool>, Arc<AtomicUsize>, AtomicBool) {
    let progress_thread_run = Arc::new(AtomicBool::new(true));
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let check_was_stopped = AtomicBool::new(false);
    let progress_thread_sender = if let Some(progress_sender) = progress_sender {
        let progress_send = progress_sender.clone();
        let progress_thread_run = progress_thread_run.clone();
        let atomic_counter = atomic_counter.clone();
        thread::spawn(move || {
            let mut time_since_last_send = SystemTime::now();

            loop {
                if time_since_last_send.elapsed().unwrap().as_millis() > SEND_PROGRESS_DATA_TIME_BETWEEN as u128 {
                    progress_send
                        .unbounded_send(ProgressData {
                            checking_method,
                            current_stage,
                            max_stage,
                            entries_checked: atomic_counter.load(Ordering::Relaxed),
                            entries_to_check: max_value,
                            tool_type,
                        })
                        .unwrap();
                    time_since_last_send = SystemTime::now();
                }
                if !progress_thread_run.load(Ordering::Relaxed) {
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
    progress_thread_run.store(false, Ordering::Relaxed);
    progress_thread_handle.join().unwrap();
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::common::Common;

    #[test]
    fn test_regex() {
        assert!(Common::regex_check("*home*", "/home/rafal"));
        assert!(Common::regex_check("*home", "/home"));
        assert!(Common::regex_check("*home/", "/home/"));
        assert!(Common::regex_check("*home/*", "/home/"));
        assert!(Common::regex_check("*.git*", "/home/.git"));
        assert!(Common::regex_check("*/home/rafal*rafal*rafal*rafal*", "/home/rafal/rafalrafalrafal"));
        assert!(Common::regex_check("AAA", "AAA"));
        assert!(Common::regex_check("AAA*", "AAABDGG/QQPW*"));
        assert!(!Common::regex_check("*home", "/home/"));
        assert!(!Common::regex_check("*home", "/homefasfasfasfasf/"));
        assert!(!Common::regex_check("*home", "/homefasfasfasfasf"));
        assert!(!Common::regex_check("rafal*afal*fal", "rafal"));
        assert!(!Common::regex_check("rafal*a", "rafal"));
        assert!(!Common::regex_check("AAAAAAAA****", "/AAAAAAAAAAAAAAAAA"));
        assert!(!Common::regex_check("*.git/*", "/home/.git"));
        assert!(!Common::regex_check("*home/*koc", "/koc/home/"));
        assert!(!Common::regex_check("*home/", "/home"));
        assert!(!Common::regex_check("*TTT", "/GGG"));

        #[cfg(target_family = "windows")]
        {
            assert!(Common::regex_check("*\\home", "C:\\home"));
            assert!(Common::regex_check("*/home", "C:\\home"));
        }
    }

    #[test]
    fn test_windows_path() {
        assert_eq!(PathBuf::from("C:\\path.txt"), Common::normalize_windows_path("c:/PATH.tXt"));
        assert_eq!(PathBuf::from("H:\\reka\\weza\\roman.txt"), Common::normalize_windows_path("h:/RekA/Weza\\roMan.Txt"));
        assert_eq!(PathBuf::from("T:\\a"), Common::normalize_windows_path("T:\\A"));
        assert_eq!(PathBuf::from("\\\\aBBa"), Common::normalize_windows_path("\\\\aBBa"));
        assert_eq!(PathBuf::from("a"), Common::normalize_windows_path("a"));
        assert_eq!(PathBuf::from(""), Common::normalize_windows_path(""));
    }
}
