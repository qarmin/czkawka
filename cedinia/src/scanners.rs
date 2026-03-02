use std::cmp::Reverse;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;

use crate::scan_runner::{CommonFilters, FileItem, ScanResultHandler, apply_filters, file_name, fmt_size, parent_str, spawn_progress_forwarder};

pub(crate) fn scan_duplicate_files<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    check_method: czkawka_core::common::model::CheckingMethod,
    hash_type: czkawka_core::common::model::HashType,
    min_size_kb: i32,
    use_cache: bool,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = DuplicateFinderParameters::new(check_method, hash_type, use_cache, 8 * 1024, 0, false);
    let mut tool = DuplicateFinder::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    let min_bytes = (min_size_kb.max(1) as u64) * 1024;
    tool.set_minimal_file_size(min_bytes);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();

    let mut items: Vec<FileItem> = Vec::new();
    for groups in tool.get_files_sorted_by_hash().values().rev() {
        let mut sorted_groups: Vec<&Vec<_>> = groups.iter().collect();
        sorted_groups.sort_by_key(|g| Reverse(g.len()));
        for group in sorted_groups {
            if group.len() < 2 {
                continue;
            }
            let file_size = group[0].size;
            let total = fmt_size(file_size * group.len() as u64);
            let per = fmt_size(file_size);
            items.push(FileItem {
                is_header: true,
                name: format!("{} pliki  ×  {} / plik  =  {} łącznie", group.len(), per, total),
                path: String::new(),
                size: String::new(),
                extra: String::new(),
            });
            for fe in group {
                items.push(FileItem {
                    is_header: false,
                    name: file_name(&fe.path),
                    path: parent_str(&fe.path),
                    size: fmt_size(fe.size),
                    extra: String::new(),
                });
            }
        }
    }
    items
}

pub(crate) fn scan_empty_folders<H: ScanResultHandler>(dirs: Vec<PathBuf>, filters: &CommonFilters, stop: &Arc<AtomicBool>, handler: &Arc<H>, scan_id: u32) -> Vec<FileItem> {
    use czkawka_core::tools::empty_folder::EmptyFolder;
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let mut tool = EmptyFolder::new();
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();
    let mut items: Vec<FileItem> = tool
        .get_empty_folder_list()
        .keys()
        .map(|p| {
            let path = std::path::Path::new(p.as_str());
            FileItem {
                is_header: false,
                name: file_name(path),
                path: parent_str(path),
                size: String::new(),
                extra: String::new(),
            }
        })
        .collect();
    items.sort_by(|a, b| a.path.cmp(&b.path).then(a.name.cmp(&b.name)));
    items
}

pub(crate) fn scan_similar_images<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    similarity_preset: czkawka_core::tools::similar_images::SimilarityPreset,
    hash_size: u8,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::re_exported::{FilterType, HashAlg};
    use czkawka_core::tools::similar_images::{SimilarImages, SimilarImagesParameters, return_similarity_from_similarity_preset};

    let max_diff = return_similarity_from_similarity_preset(similarity_preset, hash_size);

    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = SimilarImagesParameters::new(max_diff, hash_size, HashAlg::Mean, FilterType::Lanczos3, false);
    let mut tool = SimilarImages::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();

    let raw_groups: &Vec<Vec<_>> = tool.get_similar_images();
    let mut groups_with_size: Vec<(&Vec<_>, u64)> = raw_groups
        .iter()
        .filter(|g| g.len() >= 2)
        .map(|g| {
            let total: u64 = g.iter().map(|img| img.size).sum();
            (g, total)
        })
        .collect();
    groups_with_size.sort_by_key(|&(_, total)| Reverse(total));

    let mut items: Vec<FileItem> = Vec::new();
    for (group, _) in groups_with_size {
        items.push(FileItem {
            is_header: true,
            name: format!("{} podobnych obrazów", group.len()),
            path: String::new(),
            size: String::new(),
            extra: String::new(),
        });
        for img in group {
            let dims = format!("{}×{}  Δ{}", img.width, img.height, img.difference);
            items.push(FileItem {
                is_header: false,
                name: file_name(&img.path),
                path: parent_str(&img.path),
                size: fmt_size(img.size),
                extra: dims,
            });
        }
    }
    items
}

pub(crate) fn scan_empty_files<H: ScanResultHandler>(dirs: Vec<PathBuf>, filters: &CommonFilters, stop: &Arc<AtomicBool>, handler: &Arc<H>, scan_id: u32) -> Vec<FileItem> {
    use czkawka_core::tools::empty_files::EmptyFiles;
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let mut tool = EmptyFiles::new();
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();
    let mut items: Vec<FileItem> = tool
        .get_empty_files()
        .iter()
        .map(|fe| FileItem {
            is_header: false,
            name: file_name(&fe.path),
            path: parent_str(&fe.path),
            size: fmt_size(fe.size),
            extra: String::new(),
        })
        .collect();
    items.sort_by(|a, b| a.path.cmp(&b.path).then(a.name.cmp(&b.name)));
    items
}

pub(crate) fn scan_temporary_files<H: ScanResultHandler>(dirs: Vec<PathBuf>, filters: &CommonFilters, stop: &Arc<AtomicBool>, handler: &Arc<H>, scan_id: u32) -> Vec<FileItem> {
    use czkawka_core::tools::temporary::Temporary;
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let mut tool = Temporary::new();
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();
    let mut items: Vec<FileItem> = tool
        .get_temporary_files()
        .iter()
        .map(|fe| FileItem {
            is_header: false,
            name: file_name(&fe.path),
            path: parent_str(&fe.path),
            size: fmt_size(fe.size),
            extra: String::new(),
        })
        .collect();
    items.sort_by(|a, b| a.path.cmp(&b.path).then(a.name.cmp(&b.name)));
    items
}

pub(crate) fn scan_big_files<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    search_mode: czkawka_core::tools::big_file::SearchMode,
    count: usize,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::tools::big_file::{BigFile, BigFileParameters};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = BigFileParameters::new(count, search_mode);
    let mut tool = BigFile::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();
    tool.get_big_files()
        .iter()
        .map(|fe| FileItem {
            is_header: false,
            name: file_name(&fe.path),
            path: parent_str(&fe.path),
            size: fmt_size(fe.size),
            extra: String::new(),
        })
        .collect()
}

pub(crate) fn scan_broken_files<H: ScanResultHandler>(dirs: Vec<PathBuf>, filters: &CommonFilters, stop: &Arc<AtomicBool>, handler: &Arc<H>, scan_id: u32) -> Vec<FileItem> {
    use czkawka_core::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = BrokenFilesParameters::new(CheckedTypes::PDF | CheckedTypes::AUDIO | CheckedTypes::IMAGE | CheckedTypes::ARCHIVE);
    let mut tool = BrokenFiles::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();
    let mut items: Vec<FileItem> = tool
        .get_broken_files()
        .iter()
        .map(|be| FileItem {
            is_header: false,
            name: file_name(&be.path),
            path: parent_str(&be.path),
            size: fmt_size(be.size),
            extra: be.error_string.clone(),
        })
        .collect();
    items.sort_by(|a, b| a.path.cmp(&b.path).then(a.name.cmp(&b.name)));
    items
}

pub(crate) fn scan_invalid_symlinks<H: ScanResultHandler>(dirs: Vec<PathBuf>, filters: &CommonFilters, stop: &Arc<AtomicBool>, handler: &Arc<H>, scan_id: u32) -> Vec<FileItem> {
    use czkawka_core::tools::invalid_symlinks::InvalidSymlinks;
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let mut tool = InvalidSymlinks::new();
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();
    let mut items: Vec<FileItem> = tool
        .get_invalid_symlinks()
        .iter()
        .map(|se| {
            let dest = se.symlink_info.destination_path.to_string_lossy().to_string();
            FileItem {
                is_header: false,
                name: file_name(&se.path),
                path: parent_str(&se.path),
                size: String::new(),
                extra: dest,
            }
        })
        .collect();
    items.sort_by(|a, b| a.path.cmp(&b.path).then(a.name.cmp(&b.name)));
    items
}

pub(crate) fn scan_bad_extensions<H: ScanResultHandler>(dirs: Vec<PathBuf>, filters: &CommonFilters, stop: &Arc<AtomicBool>, handler: &Arc<H>, scan_id: u32) -> Vec<FileItem> {
    use czkawka_core::tools::bad_extensions::{BadExtensions, BadExtensionsParameters};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = BadExtensionsParameters::new();
    let mut tool = BadExtensions::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();
    let mut items: Vec<FileItem> = tool
        .get_bad_extensions_files()
        .iter()
        .map(|be| FileItem {
            is_header: false,
            name: file_name(&be.path),
            path: parent_str(&be.path),
            size: fmt_size(be.size),
            extra: format!(".{} → .{}", be.current_extension, be.proper_extension),
        })
        .collect();
    items.sort_by(|a, b| a.path.cmp(&b.path).then(a.name.cmp(&b.name)));
    items
}

pub(crate) fn scan_same_music<H: ScanResultHandler>(dirs: Vec<PathBuf>, filters: &CommonFilters, stop: &Arc<AtomicBool>, handler: &Arc<H>, scan_id: u32) -> Vec<FileItem> {
    use czkawka_core::common::model::CheckingMethod;
    use czkawka_core::tools::same_music::{MusicSimilarity, SameMusic, SameMusicParameters};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = SameMusicParameters::new(
        MusicSimilarity::TRACK_TITLE | MusicSimilarity::TRACK_ARTIST,
        false,
        CheckingMethod::AudioTags,
        0.0,
        0.0,
        false,
    );
    let mut tool = SameMusic::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().ok();

    let raw_groups = tool.get_duplicated_music_entries();
    let mut groups_with_size: Vec<(&Vec<_>, u64)> = raw_groups
        .iter()
        .filter(|g| g.len() >= 2)
        .map(|g| {
            let total: u64 = g.iter().map(|me| me.size).sum();
            (g, total)
        })
        .collect();
    groups_with_size.sort_by_key(|&(_, total)| Reverse(total));

    let mut items: Vec<FileItem> = Vec::new();
    for (group, _) in groups_with_size {
        items.push(FileItem {
            is_header: true,
            name: format!("{} podobnych utworów", group.len()),
            path: String::new(),
            size: String::new(),
            extra: String::new(),
        });
        for me in group {
            let artist = if me.track_artist.is_empty() { "?" } else { &me.track_artist };
            let title = if me.track_title.is_empty() { "?" } else { &me.track_title };
            items.push(FileItem {
                is_header: false,
                name: file_name(&me.path),
                path: parent_str(&me.path),
                size: fmt_size(me.size),
                extra: format!("{artist} – {title}"),
            });
        }
    }
    items
}
