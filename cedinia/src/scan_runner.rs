use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;

use crossbeam_channel::{Receiver, Sender, unbounded};
use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::progress_data::ProgressData as CoreProgress;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::re_exported::{FilterType, HashAlg};
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::similar_images::{GeometricInvariance, SimilarityPreset};

use crate::scanners::{
    scan_bad_extensions, scan_bad_names, scan_big_files, scan_broken_files, scan_duplicate_files, scan_empty_files, scan_empty_folders, scan_exif_remover, scan_same_music,
    scan_similar_images, scan_similar_videos, scan_temporary_files,
};

#[derive(Debug, Clone)]
pub struct FileItem {
    pub is_header: bool,

    pub is_reference: bool,
    pub val_str: Vec<String>,
    pub val_int: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct CommonFilters {
    pub excluded_items: Vec<String>,
    pub allowed_extensions: Vec<String>,
    pub excluded_extensions: Vec<String>,
    pub excluded_paths: Vec<PathBuf>,
    pub min_file_size_bytes: u64,
    pub max_file_size_bytes: Option<u64>,
    pub recursive_search: bool,
    pub use_cache: bool,
    pub hide_hard_links: bool,
    pub delete_outdated_cache: bool,
    pub save_also_as_json: bool,

    pub referenced_dirs: Vec<PathBuf>,
}

impl Default for CommonFilters {
    fn default() -> Self {
        Self {
            excluded_items: Vec::new(),
            allowed_extensions: Vec::new(),
            excluded_extensions: Vec::new(),
            excluded_paths: Vec::new(),
            min_file_size_bytes: 0,
            max_file_size_bytes: None,
            recursive_search: true,
            use_cache: true,
            hide_hard_links: false,
            delete_outdated_cache: true,
            save_also_as_json: false,
            referenced_dirs: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum ScanRequest {
    DuplicateFiles {
        dirs: Vec<PathBuf>,
        check_method: CheckingMethod,
        hash_type: HashType,
        filters: CommonFilters,
    },
    EmptyFolders {
        dirs: Vec<PathBuf>,
        filters: CommonFilters,
    },
    SimilarImages {
        dirs: Vec<PathBuf>,
        similarity_preset: SimilarityPreset,
        hash_size: u8,
        hash_alg: HashAlg,
        image_filter: FilterType,
        geometric_invariance: GeometricInvariance,
        ignore_same_size: bool,
        ignore_same_resolution: bool,
        filters: CommonFilters,
    },
    EmptyFiles {
        dirs: Vec<PathBuf>,
        filters: CommonFilters,
    },
    TemporaryFiles {
        dirs: Vec<PathBuf>,
        extensions: Vec<String>,
        filters: CommonFilters,
    },
    BigFiles {
        dirs: Vec<PathBuf>,
        search_mode: SearchMode,
        count: usize,
        filters: CommonFilters,
    },
    BrokenFiles {
        dirs: Vec<PathBuf>,
        checked_types: u32,
        filters: CommonFilters,
    },
    BadExtensions {
        dirs: Vec<PathBuf>,
        filters: CommonFilters,
    },
    SameMusic {
        dirs: Vec<PathBuf>,
        music_similarity: u32,
        approximate: bool,
        check_method: CheckingMethod,
        filters: CommonFilters,
    },
    BadNames {
        dirs: Vec<PathBuf>,
        filters: CommonFilters,
        uppercase_extension: bool,
        emoji_used: bool,
        space_at_start_or_end: bool,
        non_ascii_graphical: bool,
        remove_duplicated_non_alpha: bool,
    },
    ExifRemover {
        dirs: Vec<PathBuf>,
        filters: CommonFilters,
    },
    SimilarVideos {
        dirs: Vec<PathBuf>,
        filters: CommonFilters,
        audio_similarity_percent: f64,
        audio_maximum_difference: f64,
        audio_length_ratio: f64,
        audio_min_duration_seconds: u32,
    },
    Stop,
}

#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    pub step_name: String,
    pub all_progress: i32,
    pub current_progress: i32,
    pub is_indeterminate: bool,
    pub scan_id: u32,
}

#[derive(Debug)]
pub enum ScanResult {
    Progress(ProgressUpdate),
    DuplicateFiles(Vec<FileItem>),
    EmptyFolders(Vec<FileItem>),
    SimilarImages(Vec<FileItem>),
    EmptyFiles(Vec<FileItem>),
    TemporaryFiles(Vec<FileItem>),
    BigFiles(Vec<FileItem>),
    BrokenFiles(Vec<FileItem>),
    BadExtensions(Vec<FileItem>),
    SameMusic(Vec<FileItem>),
    BadNames(Vec<FileItem>),
    ExifRemover(Vec<FileItem>),
    SimilarVideos(Vec<FileItem>),
    Finished(u32),
}

pub trait ScanResultHandler: Send + Sync + 'static {
    fn on_result(&self, result: ScanResult);
}

/// Holds a WakeLock for the scan's duration so Android doesn't throttle the worker thread.
#[cfg(target_os = "android")]
struct ScanWakeLock;

#[cfg(target_os = "android")]
impl ScanWakeLock {
    fn acquire() -> Self {
        crate::file_picker_android::acquire_wakelock();
        ScanWakeLock
    }
}

#[cfg(target_os = "android")]
impl Drop for ScanWakeLock {
    fn drop(&mut self) {
        crate::file_picker_android::release_wakelock();
    }
}

pub fn start_worker<H: ScanResultHandler>(handler: H) -> (Sender<ScanRequest>, Arc<AtomicBool>) {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let (req_tx, req_rx) = unbounded::<ScanRequest>();
    thread::Builder::new()
        .name("cedinia-scanner".into())
        .spawn({
            let stop_flag = Arc::clone(&stop_flag);
            move || worker_loop(&req_rx, handler, &stop_flag)
        })
        .expect("Failed to spawn scanner thread");
    (req_tx, stop_flag)
}

fn worker_loop<H: ScanResultHandler + Sync>(req_rx: &Receiver<ScanRequest>, handler: H, stop_flag: &Arc<AtomicBool>) {
    use std::sync::atomic::Ordering;
    let mut scan_id: u32 = 0;

    let handler = Arc::new(handler);

    while let Ok(req) = req_rx.recv() {
        if matches!(req, ScanRequest::Stop) {
            stop_flag.store(true, Ordering::Relaxed);
            continue;
        }

        scan_id += 1;
        #[cfg(target_os = "android")]
        let _wakelock = ScanWakeLock::acquire();

        match req {
            ScanRequest::Stop => unreachable!(),
            ScanRequest::DuplicateFiles {
                dirs,
                check_method,
                hash_type,
                filters,
            } => {
                let items = scan_duplicate_files(dirs, check_method, hash_type, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::DuplicateFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::EmptyFolders { dirs, filters } => {
                let items = scan_empty_folders(dirs, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::EmptyFolders(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::SimilarImages {
                dirs,
                similarity_preset,
                hash_size,
                hash_alg,
                image_filter,
                geometric_invariance,
                ignore_same_size,
                ignore_same_resolution,
                filters,
            } => {
                let items = scan_similar_images(
                    dirs,
                    similarity_preset,
                    hash_size,
                    hash_alg,
                    image_filter,
                    geometric_invariance,
                    ignore_same_size,
                    ignore_same_resolution,
                    &filters,
                    stop_flag,
                    &handler,
                    scan_id,
                );
                handler.on_result(ScanResult::SimilarImages(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::EmptyFiles { dirs, filters } => {
                let items = scan_empty_files(dirs, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::EmptyFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::TemporaryFiles { dirs, extensions, filters } => {
                let items = scan_temporary_files(dirs, extensions, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::TemporaryFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::BigFiles {
                dirs,
                search_mode,
                count,
                filters,
            } => {
                let items = scan_big_files(dirs, search_mode, count, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::BigFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::BrokenFiles { dirs, checked_types, filters } => {
                let items = scan_broken_files(dirs, checked_types, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::BrokenFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::BadExtensions { dirs, filters } => {
                let items = scan_bad_extensions(dirs, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::BadExtensions(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::SameMusic {
                dirs,
                music_similarity,
                approximate,
                check_method,
                filters,
            } => {
                let items = scan_same_music(dirs, music_similarity, approximate, check_method, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::SameMusic(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::BadNames {
                dirs,
                filters,
                uppercase_extension,
                emoji_used,
                space_at_start_or_end,
                non_ascii_graphical,
                remove_duplicated_non_alpha,
            } => {
                let items = scan_bad_names(
                    dirs,
                    &filters,
                    stop_flag,
                    &handler,
                    scan_id,
                    uppercase_extension,
                    emoji_used,
                    space_at_start_or_end,
                    non_ascii_graphical,
                    remove_duplicated_non_alpha,
                );
                handler.on_result(ScanResult::BadNames(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::ExifRemover { dirs, filters } => {
                let items = scan_exif_remover(dirs, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::ExifRemover(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::SimilarVideos {
                dirs,
                filters,
                audio_similarity_percent,
                audio_maximum_difference,
                audio_length_ratio,
                audio_min_duration_seconds,
            } => {
                let items = scan_similar_videos(
                    dirs,
                    &filters,
                    audio_similarity_percent,
                    audio_maximum_difference,
                    audio_length_ratio,
                    audio_min_duration_seconds,
                    stop_flag,
                    &handler,
                    scan_id,
                );
                handler.on_result(ScanResult::SimilarVideos(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
        }
    }
}

pub(crate) fn apply_filters<T: CommonData>(tool: &mut T, filters: &CommonFilters) {
    if !filters.excluded_items.is_empty() {
        tool.set_excluded_items(filters.excluded_items.clone());
    }
    if !filters.allowed_extensions.is_empty() {
        tool.set_allowed_extensions(filters.allowed_extensions.clone());
    }
    if !filters.excluded_extensions.is_empty() {
        tool.set_excluded_extensions(filters.excluded_extensions.clone());
    }
    if !filters.excluded_paths.is_empty() {
        tool.set_excluded_paths(filters.excluded_paths.clone());
    }
    if filters.min_file_size_bytes > 0 {
        tool.set_minimal_file_size(filters.min_file_size_bytes);
    }
    if let Some(max) = filters.max_file_size_bytes {
        tool.set_maximal_file_size(max);
    }
    tool.set_recursive_search(filters.recursive_search);
    tool.set_use_cache(filters.use_cache);
    tool.set_hide_hard_links(filters.hide_hard_links);
    tool.set_delete_outdated_cache(filters.delete_outdated_cache);
    tool.set_save_also_as_json(filters.save_also_as_json);
    tool.set_reference_paths(filters.referenced_dirs.clone());
}

pub(crate) fn spawn_progress_forwarder<H: ScanResultHandler + Sync>(handler: Arc<H>, scan_id: u32) -> (Sender<CoreProgress>, thread::JoinHandle<()>) {
    let (ptx, prx) = unbounded::<CoreProgress>();
    let handle = thread::spawn(move || {
        while let Ok(pd) = prx.recv() {
            let display = pd.to_display();
            let update = ProgressUpdate {
                step_name: display.label,
                all_progress: display.all_progress.max(0),
                current_progress: display.current_progress.unwrap_or(0),
                is_indeterminate: display.current_progress.is_none(),
                scan_id,
            };
            handler.on_result(ScanResult::Progress(update));
        }
    });
    (ptx, handle)
}

pub(crate) fn fmt_size(bytes: u64) -> String {
    humansize::format_size(bytes, humansize::BINARY)
}

pub(crate) fn fmt_date(unix_secs: u64) -> String {
    use chrono::{Local, TimeZone, Utc};
    let dt_local = Utc.timestamp_opt(unix_secs as i64, 0).single().unwrap_or_default().with_timezone(&Local);
    dt_local.format("%Y-%m-%d %H:%M").to_string()
}

pub(crate) fn size_to_hi_lo(size: u64) -> (i32, i32) {
    let hi = (size >> 32) as i32;
    let lo = (size & 0xFFFF_FFFF) as i32;
    (hi, lo)
}

pub(crate) fn file_name(p: &std::path::Path) -> String {
    p.file_name().unwrap_or_default().to_string_lossy().to_string()
}

pub(crate) fn parent_str(p: &std::path::Path) -> String {
    p.parent().map(|x| x.to_string_lossy().to_string()).unwrap_or_default()
}
