use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crossbeam_channel::Sender;
use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::re_exported::{FilterType, HashAlg};
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::similar_images::SimilarityPreset;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::model::count_checked;
use crate::scan_runner::{CommonFilters, ScanRequest};
use crate::settings::gui_settings_values::{AudioPresetParams, StringComboBoxItems};
use crate::{
    ActiveTool, AppState, BadNamesSettings, BigFilesSettings, BrokenFilesSettings, DuplicateSettings, FileEntry, GeneralSettings, MainWindow, SameMusicSettings, ScanState,
    SimilarGroupCard, SimilarImagesSettings, SimilarVideosSettings, TemporaryFilesSettings, flc,
};

pub(crate) fn wire_scan(
    window: &MainWindow,
    stop_flag: Arc<AtomicBool>,
    scan_tx: Rc<Sender<ScanRequest>>,
    included_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
    excluded_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
    referenced_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>,
    scan_gen: Arc<AtomicU32>,
) {
    {
        let weak = window.as_weak();
        let inc = included_dirs;
        let exc = excluded_dirs;
        let refr = referenced_dirs;
        let stop = stop_flag.clone();
        let tx = scan_tx.clone();
        let scan_gen2 = scan_gen;
        window.global::<AppState>().on_scan_requested(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_scan_requested");
            scan_gen2.fetch_add(1, Ordering::SeqCst);
            win.global::<AppState>().set_scan_state(ScanState::Scanning);
            win.global::<AppState>().set_status_message(SharedString::from(flc!("scanning_fallback")));
            stop.store(false, Ordering::Relaxed);
            let dirs = inc.borrow().clone();
            let excluded = exc.borrow().clone();
            let refs = refr.borrow().clone();
            let tool = win.global::<AppState>().get_active_tool();
            clear_tool_results(&win, tool);
            let req = build_scan_request(&win, tool, dirs, excluded, refs);
            let _ = tx.send(req);
        });
    }
    {
        let weak = window.as_weak();
        let stop = stop_flag;
        let tx = scan_tx;
        window.global::<AppState>().on_stop_requested(move || {
            let win = weak.upgrade().expect("MainWindow dropped in on_stop_requested");
            stop.store(true, Ordering::Relaxed);
            let _ = tx.send(ScanRequest::Stop);
            win.global::<AppState>().set_scan_state(ScanState::Stopping);
        });
    }
    {
        let weak = window.as_weak();
        window.global::<AppState>().on_tool_changed(move |tool| {
            let win = weak.upgrade().expect("MainWindow dropped in on_tool_changed");

            match tool {
                ActiveTool::Home | ActiveTool::Directories | ActiveTool::Settings => {
                    win.global::<AppState>().set_selected_count(0);
                }
                _ => {
                    let model = super::selection::get_model_for_tool(&win, tool);
                    let count = count_checked(&model);
                    win.global::<AppState>().set_selected_count(count);
                    win.global::<AppState>().set_status_message(SharedString::default());
                }
            }
        });
    }
}

fn empty_entries() -> ModelRc<FileEntry> {
    ModelRc::new(VecModel::from(Vec::new()))
}

fn clear_tool_results(win: &MainWindow, tool: ActiveTool) {
    match tool {
        ActiveTool::DuplicateFiles => win.set_duplicate_files_model(empty_entries()),
        ActiveTool::EmptyFolders => win.set_empty_folder_model(empty_entries()),
        ActiveTool::SimilarImages => {
            win.set_similar_images_model(empty_entries());
            win.set_similar_images_groups(ModelRc::new(VecModel::<SimilarGroupCard>::from(Vec::new())));
        }
        ActiveTool::EmptyFiles => win.set_empty_files_model(empty_entries()),
        ActiveTool::TemporaryFiles => win.set_temporary_files_model(empty_entries()),
        ActiveTool::BigFiles => win.set_big_files_model(empty_entries()),
        ActiveTool::BrokenFiles => win.set_broken_files_model(empty_entries()),
        ActiveTool::BadExtensions => win.set_bad_extensions_model(empty_entries()),
        ActiveTool::SameMusic => win.set_same_music_model(empty_entries()),
        ActiveTool::BadNames => win.set_bad_names_model(empty_entries()),
        ActiveTool::ExifRemover => win.set_exif_remover_model(empty_entries()),
        ActiveTool::SimilarVideos => win.set_similar_videos_model(empty_entries()),
        ActiveTool::Home | ActiveTool::Directories | ActiveTool::Settings => {}
    }
    win.global::<AppState>().set_selected_count(0);
}

fn build_common_filters(win: &MainWindow, excluded_paths: Vec<PathBuf>, referenced_dirs: Vec<PathBuf>) -> CommonFilters {
    let g = win.global::<GeneralSettings>();
    let items = StringComboBoxItems::new();
    let min_file_size_bytes = items.min_file_size.get(g.get_min_file_size_idx() as usize).map_or(0, |e| e.value.to_bytes());
    let max_file_size_bytes = items.max_file_size.get(g.get_max_file_size_idx() as usize).and_then(|e| e.value.to_bytes());
    let split_csv = |s: slint::SharedString| -> Vec<String> { s.as_str().split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect() };
    let mut excluded_items = split_csv(g.get_excluded_items());
    let cache_dir = crate::thumbnail_loader::thumbnail_cache_dir();
    if let Some(s) = cache_dir.to_str() {
        excluded_items.push(format!("{s}/*"));
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
        max_file_size_bytes,
        recursive_search: true,
        use_cache: g.get_use_cache(),
        hide_hard_links: false,
        delete_outdated_cache: true,
        save_also_as_json: false,
        excluded_paths,
        referenced_dirs,
    }
}

fn build_scan_request(win: &MainWindow, tool: ActiveTool, dirs: Vec<PathBuf>, excluded_paths: Vec<PathBuf>, referenced_dirs: Vec<PathBuf>) -> ScanRequest {
    let filters = build_common_filters(win, excluded_paths, referenced_dirs);
    let items = StringComboBoxItems::new();

    let duplicate_request = || {
        let d = win.global::<DuplicateSettings>();
        ScanRequest::DuplicateFiles {
            dirs: dirs.clone(),
            check_method: StringComboBoxItems::value_from_idx(&items.duplicates_check_method, d.get_check_method(), CheckingMethod::Hash),
            hash_type: StringComboBoxItems::value_from_idx(&items.duplicates_hash_type, d.get_hash_type(), HashType::Blake3),
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
                similarity_preset: StringComboBoxItems::value_from_idx(&items.similarity_preset, s.get_similarity_preset(), SimilarityPreset::Medium),
                hash_size: StringComboBoxItems::value_from_idx(&items.hash_size, s.get_hash_size_idx(), 16),
                hash_alg: StringComboBoxItems::value_from_idx(&items.hash_alg, s.get_hash_alg_idx(), HashAlg::Mean),
                image_filter: StringComboBoxItems::value_from_idx(&items.image_filter, s.get_image_filter_idx(), FilterType::Triangle),
                ignore_same_size: s.get_ignore_same_size(),
                ignore_same_resolution: s.get_ignore_same_resolution(),
                filters,
            }
        }
        ActiveTool::EmptyFiles => ScanRequest::EmptyFiles { dirs, filters },
        ActiveTool::TemporaryFiles => {
            let t = win.global::<TemporaryFilesSettings>();
            let extensions: Vec<String> = t
                .get_extensions()
                .as_str()
                .split(',')
                .map(|s| s.trim().to_ascii_lowercase())
                .filter(|s| !s.is_empty())
                .collect();
            let extensions = if extensions.is_empty() {
                czkawka_core::tools::temporary::TEMP_EXTENSIONS.iter().map(|s| s.to_string()).collect()
            } else {
                extensions
            };
            ScanRequest::TemporaryFiles { dirs, extensions, filters }
        }
        ActiveTool::BigFiles => {
            let b = win.global::<BigFilesSettings>();
            ScanRequest::BigFiles {
                dirs,
                search_mode: StringComboBoxItems::value_from_idx(&items.biggest_files_method, b.get_search_mode_idx(), SearchMode::BiggestFiles),
                count: StringComboBoxItems::value_from_idx(&items.big_files_count, b.get_count_idx(), 50),
                filters,
            }
        }
        ActiveTool::BrokenFiles => {
            let b = win.global::<BrokenFilesSettings>();
            use czkawka_core::tools::broken_files::CheckedTypes;
            let mut types = CheckedTypes::empty();
            if b.get_check_audio() {
                types |= CheckedTypes::AUDIO;
            }
            if b.get_check_pdf() {
                types |= CheckedTypes::PDF;
            }
            if b.get_check_archive() {
                types |= CheckedTypes::ARCHIVE;
            }
            if b.get_check_image() {
                types |= CheckedTypes::IMAGE;
            }
            if b.get_check_font() {
                types |= CheckedTypes::FONT;
            }
            if b.get_check_markup() {
                types |= CheckedTypes::MARKUP;
            }
            ScanRequest::BrokenFiles {
                dirs,
                filters,
                checked_types: types.bits(),
            }
        }
        ActiveTool::BadExtensions => ScanRequest::BadExtensions { dirs, filters },
        ActiveTool::SameMusic => {
            let m = win.global::<SameMusicSettings>();
            use czkawka_core::tools::same_music::MusicSimilarity;
            let mut sim = MusicSimilarity::NONE;
            if m.get_title() {
                sim |= MusicSimilarity::TRACK_TITLE;
            }
            if m.get_artist() {
                sim |= MusicSimilarity::TRACK_ARTIST;
            }
            if m.get_year() {
                sim |= MusicSimilarity::YEAR;
            }
            if m.get_length() {
                sim |= MusicSimilarity::LENGTH;
            }
            if m.get_genre() {
                sim |= MusicSimilarity::GENRE;
            }
            if m.get_bitrate() {
                sim |= MusicSimilarity::BITRATE;
            }
            if sim.is_empty() {
                sim = MusicSimilarity::TRACK_TITLE | MusicSimilarity::TRACK_ARTIST;
            }
            ScanRequest::SameMusic {
                dirs,
                filters,
                music_similarity: sim.bits(),
                approximate: m.get_approximate(),
                check_method: StringComboBoxItems::value_from_idx(&items.same_music_check_method, m.get_check_method_idx(), CheckingMethod::AudioTags),
            }
        }
        ActiveTool::BadNames => {
            let bn = win.global::<BadNamesSettings>();
            ScanRequest::BadNames {
                dirs,
                filters,
                uppercase_extension: bn.get_uppercase_extension(),
                emoji_used: bn.get_emoji_used(),
                space_at_start_or_end: bn.get_space_at_start_or_end(),
                non_ascii_graphical: bn.get_non_ascii_graphical(),
                remove_duplicated_non_alpha: bn.get_remove_duplicated_non_alpha(),
            }
        }
        ActiveTool::ExifRemover => ScanRequest::ExifRemover { dirs, filters },
        ActiveTool::SimilarVideos => {
            let sv = win.global::<SimilarVideosSettings>();
            let preset = StringComboBoxItems::value_from_idx(
                &items.similar_videos_audio_preset,
                sv.get_audio_preset_idx(),
                AudioPresetParams {
                    similarity_percent: 80.0,
                    maximum_difference: 3.0,
                    length_ratio: 0.05,
                    min_duration_seconds: 10,
                },
            );
            ScanRequest::SimilarVideos {
                dirs,
                filters,
                audio_similarity_percent: preset.similarity_percent,
                audio_maximum_difference: preset.maximum_difference,
                audio_length_ratio: preset.length_ratio,
                audio_min_duration_seconds: preset.min_duration_seconds,
            }
        }
        ActiveTool::Home | ActiveTool::Directories | ActiveTool::Settings => {
            unreachable!("scan cannot be triggered from Home/Directories/Settings tab")
        }
    }
}
