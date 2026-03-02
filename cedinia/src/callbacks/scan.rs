use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crossbeam_channel::Sender;
use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::similar_images::SimilarityPreset;
use slint::{ComponentHandle, SharedString};

use crate::scan_runner::{CommonFilters, ScanRequest};
use crate::settings::gui_settings_values::StringComboBoxItems;
use crate::{ActiveTool, AppState, BigFilesSettings, DuplicateSettings, GeneralSettings, MainWindow, ScanState, SimilarImagesSettings};

pub(crate) fn wire_scan(
    window: &MainWindow,
    stop_flag: Arc<AtomicBool>,
    scan_tx: Rc<Sender<ScanRequest>>,
    included_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
    scan_gen: Arc<AtomicU32>,
) {
    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let stop = stop_flag.clone();
        let tx = scan_tx.clone();
        let scan_gen2 = scan_gen.clone();
        window.global::<AppState>().on_scan_requested(move || {
            let win = weak.unwrap();
            scan_gen2.fetch_add(1, Ordering::SeqCst);
            win.global::<AppState>().set_scan_state(ScanState::Scanning);
            win.global::<AppState>().set_status_message(SharedString::from("Skanowanie…"));
            stop.store(false, Ordering::Relaxed);
            let dirs = inc.borrow().clone();
            let tool = win.global::<AppState>().get_active_tool();
            let req = build_scan_request(&win, tool, dirs);
            let _ = tx.send(req);
        });
    }
    {
        let weak = window.as_weak();
        let stop = stop_flag;
        let tx = scan_tx;
        window.global::<AppState>().on_stop_requested(move || {
            let win = weak.unwrap();
            stop.store(true, Ordering::Relaxed);
            let _ = tx.send(ScanRequest::Stop);
            win.global::<AppState>().set_scan_state(ScanState::Stopping);
        });
    }
    window.global::<AppState>().on_tool_changed(|_| {});
}

fn build_common_filters(win: &MainWindow) -> CommonFilters {
    let g = win.global::<GeneralSettings>();
    let min_file_size_bytes = match g.get_min_file_size_idx() {
        0 => 0,
        1 => 1024,
        2 => 8 * 1024,
        3 => 64 * 1024,
        4 => 1024 * 1024,
        _ => 0,
    };
    let split_csv = |s: slint::SharedString| -> Vec<String> { s.as_str().split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect() };
    let mut excluded_items = split_csv(g.get_excluded_items());
    let cache_dir = crate::thumbnail_loader::thumbnail_cache_dir();
    if let Some(s) = cache_dir.to_str() {
        excluded_items.push(format!("{}/*", s));
    }
    if g.get_ignore_hidden() {
        excluded_items.push("*/.*".to_string());
        excluded_items.push("*/.*/*".to_string());
    }
    CommonFilters {
        excluded_items,
        allowed_extensions: split_csv(g.get_allowed_extensions()),
        excluded_extensions: split_csv(g.get_excluded_extensions()),
        min_file_size_bytes,
    }
}

fn build_scan_request(win: &MainWindow, tool: ActiveTool, dirs: Vec<PathBuf>) -> ScanRequest {
    let filters = build_common_filters(win);
    let items = StringComboBoxItems::new();

    let duplicate_request = || {
        let d = win.global::<DuplicateSettings>();
        ScanRequest::DuplicateFiles {
            dirs: dirs.clone(),
            check_method: StringComboBoxItems::value_from_config_name(&d.get_check_method_value(), &items.duplicates_check_method, CheckingMethod::Hash),
            hash_type: StringComboBoxItems::value_from_config_name(&d.get_hash_type_value(), &items.duplicates_hash_type, HashType::Blake3),
            min_size_kb: min_size_kb_from_idx(d.get_min_size_kb_idx()),
            use_cache: win.global::<GeneralSettings>().get_use_cache(),
            filters: filters.clone(),
        }
    };

    match tool {
        ActiveTool::DuplicateFiles => duplicate_request(),
        ActiveTool::EmptyFolders => ScanRequest::EmptyFolders { dirs, filters },
        ActiveTool::SimilarImages => {
            let s = win.global::<SimilarImagesSettings>();
            ScanRequest::SimilarImages {
                dirs,
                similarity_preset: StringComboBoxItems::value_from_config_name(&s.get_similarity_preset_value(), &items.similarity_preset, SimilarityPreset::Medium),
                hash_size: StringComboBoxItems::value_from_config_name(&s.get_hash_size_value(), &items.hash_size, 16),
                filters,
            }
        }
        ActiveTool::EmptyFiles => ScanRequest::EmptyFiles { dirs, filters },
        ActiveTool::TemporaryFiles => ScanRequest::TemporaryFiles { dirs, filters },
        ActiveTool::BigFiles => {
            let b = win.global::<BigFilesSettings>();
            ScanRequest::BigFiles {
                dirs,
                search_mode: StringComboBoxItems::value_from_config_name(&b.get_search_mode_value(), &items.biggest_files_method, SearchMode::BiggestFiles),
                count: StringComboBoxItems::value_from_config_name(&b.get_count_value(), &items.big_files_count, 50),
                filters,
            }
        }
        ActiveTool::BrokenFiles => ScanRequest::BrokenFiles { dirs, filters },
        ActiveTool::InvalidSymlinks => ScanRequest::InvalidSymlinks { dirs, filters },
        ActiveTool::BadExtensions => ScanRequest::BadExtensions { dirs, filters },
        ActiveTool::SameMusic => ScanRequest::SameMusic { dirs, filters },
        ActiveTool::Home | ActiveTool::Directories | ActiveTool::Settings => duplicate_request(),
    }
}

fn min_size_kb_from_idx(idx: i32) -> i32 {
    match idx {
        0 => 1,
        1 => 8,
        2 => 64,
        3 => 1024,
        _ => 8,
    }
}
