use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;

use crossbeam_channel::{Receiver, Sender, unbounded};
use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::progress_data::{CurrentStage, ProgressData as CoreProgress};
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::similar_images::SimilarityPreset;

use crate::scanners::{
    scan_bad_extensions, scan_big_files, scan_broken_files, scan_duplicate_files, scan_empty_files, scan_empty_folders, scan_invalid_symlinks, scan_same_music,
    scan_similar_images, scan_temporary_files,
};

#[derive(Debug, Clone)]
pub struct FileItem {
    pub is_header: bool,
    pub name: String,
    pub path: String,
    pub size: String,
    pub extra: String,
}

#[derive(Debug, Clone, Default)]
pub struct CommonFilters {
    pub excluded_items: Vec<String>,
    pub allowed_extensions: Vec<String>,
    pub excluded_extensions: Vec<String>,
    pub min_file_size_bytes: u64,
}

#[derive(Debug)]
pub enum ScanRequest {
    DuplicateFiles {
        dirs: Vec<PathBuf>,
        check_method: CheckingMethod,
        hash_type: HashType,
        min_size_kb: i32,
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
        filters: CommonFilters,
    },
    InvalidSymlinks {
        dirs: Vec<PathBuf>,
        filters: CommonFilters,
    },
    BadExtensions {
        dirs: Vec<PathBuf>,
        filters: CommonFilters,
    },
    SameMusic {
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
    InvalidSymlinks(Vec<FileItem>),
    BadExtensions(Vec<FileItem>),
    SameMusic(Vec<FileItem>),
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
            move || worker_loop(req_rx, handler, stop_flag)
        })
        .expect("Failed to spawn scanner thread");
    (req_tx, stop_flag)
}

fn worker_loop<H: ScanResultHandler + Sync>(req_rx: Receiver<ScanRequest>, handler: H, stop_flag: Arc<AtomicBool>) {
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
                min_size_kb,
                use_cache,
                filters,
            } => {
                scan_id += 1;
                let items = scan_duplicate_files(dirs, check_method, hash_type, min_size_kb, use_cache, &filters, &stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::DuplicateFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::EmptyFolders { dirs, filters } => {
                scan_id += 1;
                let items = scan_empty_folders(dirs, &filters, &stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::EmptyFolders(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::SimilarImages {
                dirs,
                similarity_preset,
                hash_size,
                filters,
            } => {
                scan_id += 1;
                let items = scan_similar_images(dirs, similarity_preset, hash_size, &filters, &stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::SimilarImages(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::EmptyFiles { dirs, filters } => {
                scan_id += 1;
                let items = scan_empty_files(dirs, &filters, &stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::EmptyFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::TemporaryFiles { dirs, filters } => {
                scan_id += 1;
                let items = scan_temporary_files(dirs, &filters, &stop_flag, &handler, scan_id);
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
                let items = scan_big_files(dirs, search_mode, count, &filters, &stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::BigFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::BrokenFiles { dirs, filters } => {
                scan_id += 1;
                let items = scan_broken_files(dirs, &filters, &stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::BrokenFiles(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::InvalidSymlinks { dirs, filters } => {
                scan_id += 1;
                let items = scan_invalid_symlinks(dirs, &filters, &stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::InvalidSymlinks(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::BadExtensions { dirs, filters } => {
                scan_id += 1;
                let items = scan_bad_extensions(dirs, &filters, &stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::BadExtensions(items));
                handler.on_result(ScanResult::Finished(scan_id));
            }
            ScanRequest::SameMusic { dirs, filters } => {
                scan_id += 1;
                let items = scan_same_music(dirs, &filters, &stop_flag, &handler, scan_id);
                handler.on_result(ScanResult::SameMusic(items));
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

fn stage_label(stage: CurrentStage) -> &'static str {
    match stage {
        CurrentStage::CollectingFiles => "Zbieranie plików",
        CurrentStage::DuplicateScanningName => "Skanowanie po nazwie",
        CurrentStage::DuplicateScanningSizeName => "Skanowanie po nazwie i rozmiarze",
        CurrentStage::DuplicateScanningSize => "Skanowanie po rozmiarze",
        CurrentStage::DuplicatePreHashing => "Pre-hash",
        CurrentStage::DuplicateFullHashing => "Haszowanie",
        CurrentStage::DuplicateCacheLoading
        | CurrentStage::DuplicatePreHashCacheLoading
        | CurrentStage::SameMusicCacheLoadingTags
        | CurrentStage::SameMusicCacheLoadingFingerprints
        | CurrentStage::ExifRemoverCacheLoading => "Ładowanie cache",
        CurrentStage::DuplicateCacheSaving
        | CurrentStage::DuplicatePreHashCacheSaving
        | CurrentStage::SameMusicCacheSavingTags
        | CurrentStage::SameMusicCacheSavingFingerprints
        | CurrentStage::ExifRemoverCacheSaving => "Zapisywanie cache",
        CurrentStage::SimilarImagesCalculatingHashes => "Obliczanie hashy obrazów",
        CurrentStage::SimilarImagesComparingHashes => "Porównywanie obrazów",
        CurrentStage::SimilarVideosCalculatingHashes => "Obliczanie hashy wideo",
        CurrentStage::SimilarVideosCreatingThumbnails => "Tworzenie miniatur wideo",
        CurrentStage::BrokenFilesChecking => "Sprawdzanie plików",
        CurrentStage::BadExtensionsChecking => "Sprawdzanie rozszerzeń",
        CurrentStage::BadNamesChecking => "Sprawdzanie nazw",
        CurrentStage::SameMusicReadingTags => "Odczyt tagów muzycznych",
        CurrentStage::SameMusicComparingTags => "Porównywanie tagów",
        CurrentStage::SameMusicCalculatingFingerprints => "Obliczanie odcisków muzycznych",
        CurrentStage::SameMusicComparingFingerprints => "Porównywanie odcisków muzycznych",
        CurrentStage::ExifRemoverExtractingTags => "Odczyt tagów EXIF",
        CurrentStage::VideoOptimizerCreatingThumbnails => "Tworzenie miniatur wideo",
        CurrentStage::VideoOptimizerProcessingVideos => "Przetwarzanie wideo",
        CurrentStage::DeletingFiles => "Usuwanie plików",
        CurrentStage::RenamingFiles => "Zmiana nazw plików",
        CurrentStage::MovingFiles => "Przenoszenie plików",
        CurrentStage::HardlinkingFiles => "Tworzenie hardlinków",
        CurrentStage::SymlinkingFiles => "Tworzenie dowiązań",
        CurrentStage::OptimizingVideos => "Optymalizacja wideo",
        CurrentStage::CleaningExif => "Czyszczenie EXIF",
    }
}

fn stage_label_full(pd: &CoreProgress) -> String {
    let base = stage_label(pd.sstage);
    let label = if stage_uses_bytes(pd.sstage) && pd.bytes_to_check > 0 {
        format!("{base}  ({} / {})", fmt_size(pd.bytes_checked), fmt_size(pd.bytes_to_check))
    } else {
        base.to_string()
    };
    if pd.max_stage_idx > 0 {
        format!("{}/{}\u{2002}{label}", pd.current_stage_idx + 1, pd.max_stage_idx + 1)
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

pub(crate) fn file_name(p: &std::path::Path) -> String {
    p.file_name().unwrap_or_default().to_string_lossy().to_string()
}

pub(crate) fn parent_str(p: &std::path::Path) -> String {
    p.parent().map(|x| x.to_string_lossy().to_string()).unwrap_or_default()
}
