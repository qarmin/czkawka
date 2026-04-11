use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;

use crossbeam_channel::{Receiver, Sender, unbounded};
use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::progress_data::{CurrentStage, ProgressData as CoreProgress};
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::re_exported::{FilterType, HashAlg};
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::similar_images::SimilarityPreset;

use crate::flc;
use crate::scanners::{
    scan_bad_extensions, scan_bad_names, scan_big_files, scan_broken_files, scan_duplicate_files, scan_empty_files, scan_empty_folders, scan_exif_remover, scan_same_music,
    scan_similar_images, scan_temporary_files,
};

#[derive(Debug, Clone)]
pub struct FileItem {
    pub is_header: bool,

    pub is_reference: bool,
    pub val_str: Vec<String>,
    pub val_int: Vec<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct CommonFilters {
    pub excluded_items: Vec<String>,
    pub allowed_extensions: Vec<String>,
    pub excluded_extensions: Vec<String>,
    pub min_file_size_bytes: u64,
    pub max_file_size_bytes: Option<u64>,

    pub referenced_dirs: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum ScanRequest {
    DuplicateFiles {
        dirs: Vec<PathBuf>,
        check_method: CheckingMethod,
        hash_type: HashType,
        use_cache: bool,
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
        ignore_same_size: bool,
        filters: CommonFilters,
    },
    EmptyFiles {
        dirs: Vec<PathBuf>,
        filters: CommonFilters,
    },
    TemporaryFiles {
        dirs: Vec<PathBuf>,
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
    Stop,
}

#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    pub step_name: String,
    pub current: i32,
    pub all: i32,
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
    Finished(u32),
}

pub trait ScanResultHandler: Send + Sync + 'static {
    fn on_result(&self, result: ScanResult);
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
        match req {
            ScanRequest::Stop => {
                stop_flag.store(true, Ordering::Relaxed);
            }
            ScanRequest::DuplicateFiles {
                dirs,
                check_method,
                hash_type,
                use_cache,
                filters,
            } => {
                scan_id += 1;
                let items = scan_duplicate_files(dirs, check_method, hash_type, use_cache, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::DuplicateFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::EmptyFolders { dirs, filters } => {
                scan_id += 1;
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
                ignore_same_size,
                filters,
            } => {
                scan_id += 1;
                let items = scan_similar_images(
                    dirs,
                    similarity_preset,
                    hash_size,
                    hash_alg,
                    image_filter,
                    ignore_same_size,
                    &filters,
                    stop_flag,
                    &handler,
                    scan_id,
                );
                handler.on_result(ScanResult::SimilarImages(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::EmptyFiles { dirs, filters } => {
                scan_id += 1;
                let items = scan_empty_files(dirs, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::EmptyFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::TemporaryFiles { dirs, filters } => {
                scan_id += 1;
                let items = scan_temporary_files(dirs, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::TemporaryFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::BigFiles {
                dirs,
                search_mode,
                count,
                filters,
            } => {
                scan_id += 1;
                let items = scan_big_files(dirs, search_mode, count, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::BigFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::BrokenFiles { dirs, checked_types, filters } => {
                scan_id += 1;
                let items = scan_broken_files(dirs, checked_types, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::BrokenFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::BadExtensions { dirs, filters } => {
                scan_id += 1;
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
                scan_id += 1;
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
                scan_id += 1;
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
                scan_id += 1;
                let items = scan_exif_remover(dirs, &filters, stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::ExifRemover(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
        }
    }
}

fn stage_uses_bytes(stage: CurrentStage) -> bool {
    matches!(
        stage,
        CurrentStage::DuplicatePreHashing | CurrentStage::DuplicateFullHashing | CurrentStage::SimilarImagesCalculatingHashes | CurrentStage::SameMusicCalculatingFingerprints
    )
}

fn stage_label(stage: CurrentStage) -> String {
    match stage {
        CurrentStage::CollectingFiles => flc!("stage_collecting_files"),
        CurrentStage::DuplicateScanningName => flc!("stage_scanning_name"),
        CurrentStage::DuplicateScanningSizeName => flc!("stage_scanning_size_name"),
        CurrentStage::DuplicateScanningSize => flc!("stage_scanning_size"),
        CurrentStage::DuplicatePreHashing => flc!("stage_pre_hash"),
        CurrentStage::DuplicateFullHashing => flc!("stage_full_hash"),
        CurrentStage::DuplicateCacheLoading
        | CurrentStage::DuplicatePreHashCacheLoading
        | CurrentStage::SameMusicCacheLoadingTags
        | CurrentStage::SameMusicCacheLoadingFingerprints
        | CurrentStage::ExifRemoverCacheLoading => flc!("stage_loading_cache"),
        CurrentStage::DuplicateCacheSaving
        | CurrentStage::DuplicatePreHashCacheSaving
        | CurrentStage::SameMusicCacheSavingTags
        | CurrentStage::SameMusicCacheSavingFingerprints
        | CurrentStage::ExifRemoverCacheSaving => flc!("stage_saving_cache"),
        CurrentStage::SimilarImagesCalculatingHashes => flc!("stage_calculating_image_hashes"),
        CurrentStage::SimilarImagesComparingHashes => flc!("stage_comparing_images"),
        CurrentStage::SimilarVideosCalculatingHashes => flc!("stage_calculating_video_hashes"),
        CurrentStage::BrokenFilesChecking => flc!("stage_checking_files"),
        CurrentStage::BadExtensionsChecking => flc!("stage_checking_extensions"),
        CurrentStage::BadNamesChecking => flc!("stage_checking_names"),
        CurrentStage::SameMusicReadingTags => flc!("stage_reading_music_tags"),
        CurrentStage::SameMusicComparingTags => flc!("stage_comparing_tags"),
        CurrentStage::SameMusicCalculatingFingerprints => flc!("stage_calculating_music_fingerprints"),
        CurrentStage::SameMusicComparingFingerprints => flc!("stage_comparing_fingerprints"),
        CurrentStage::ExifRemoverExtractingTags => flc!("stage_extracting_exif"),
        CurrentStage::VideoOptimizerCreatingThumbnails | CurrentStage::SimilarVideosCreatingThumbnails => flc!("stage_creating_video_thumbnails"),
        CurrentStage::VideoOptimizerProcessingVideos => flc!("stage_processing_videos"),
        CurrentStage::DeletingFiles => flc!("stage_deleting"),
        CurrentStage::RenamingFiles => flc!("stage_renaming"),
        CurrentStage::MovingFiles => flc!("stage_moving"),
        CurrentStage::HardlinkingFiles => flc!("stage_hardlinking"),
        CurrentStage::SymlinkingFiles => flc!("stage_symlinking"),
        CurrentStage::OptimizingVideos => flc!("stage_optimizing_videos"),
        CurrentStage::CleaningExif => flc!("stage_cleaning_exif"),
        CurrentStage::DuplicateHidingHardLinks | CurrentStage::SimilarImagesHidingHardLinks | CurrentStage::SimilarVideosHidingHardLinks => flc!("stage_all_hiding_links"),
    }
}

fn stage_label_full(pd: &CoreProgress) -> String {
    let base = stage_label(pd.sstage);
    let label = if stage_uses_bytes(pd.sstage) && pd.bytes_to_check > 0 {
        format!("{base}  ({} / {})", fmt_size(pd.bytes_checked), fmt_size(pd.bytes_to_check))
    } else {
        base
    };
    if pd.max_stage_idx > 0 {
        format!("{}/{}  {label}", pd.current_stage_idx + 1, pd.max_stage_idx + 1)
    } else {
        label
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
    if filters.min_file_size_bytes > 0 {
        tool.set_minimal_file_size(filters.min_file_size_bytes);
    }
    if let Some(max) = filters.max_file_size_bytes {
        tool.set_maximal_file_size(max);
    }

    tool.set_reference_paths(filters.referenced_dirs.clone());
}

pub(crate) fn spawn_progress_forwarder<H: ScanResultHandler + Sync>(handler: Arc<H>, scan_id: u32) -> (Sender<CoreProgress>, thread::JoinHandle<()>) {
    let (ptx, prx) = unbounded::<CoreProgress>();
    let handle = thread::spawn(move || {
        while let Ok(pd) = prx.recv() {
            let is_indeterminate = pd.sstage.check_if_loading_saving_cache();
            let update = ProgressUpdate {
                step_name: stage_label_full(&pd),
                current: pd.entries_checked as i32,
                all: pd.entries_to_check as i32,
                is_indeterminate,
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
    let secs = unix_secs;
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;

    let min = mins % 60;
    let hour = hours % 24;

    let mut remaining_days = days;
    let mut year = 1970u64;
    loop {
        let days_in_year = if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) {
            366
        } else {
            365
        };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let months_days: [u64; 12] = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut month = 1u64;
    for &md in &months_days {
        if remaining_days < md {
            break;
        }
        remaining_days -= md;
        month += 1;
    }
    let day = remaining_days + 1;

    format!("{year}-{month:02}-{day:02} {hour:02}:{min:02}")
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
