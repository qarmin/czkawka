use std::cmp::Reverse;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;

use crate::common::{
    INT_BASE_COUNT, INT_IDX_SIZE_HI, INT_IDX_SIZE_LO, MAX_INT_DATA_EXIF_REMOVER, MAX_INT_DATA_SIMILAR_IMAGES, MAX_STR_DATA_BAD_EXTENSIONS, MAX_STR_DATA_BAD_NAMES,
    MAX_STR_DATA_BROKEN_FILES, MAX_STR_DATA_SAME_MUSIC, MAX_STR_DATA_SIMILAR_IMAGES, STR_BASE_COUNT, STR_IDX_NAME, STR_IDX_PATH,
};
use crate::scan_runner::{CommonFilters, FileItem, ScanResultHandler, apply_filters, file_name, fmt_date, fmt_size, parent_str, size_to_hi_lo, spawn_progress_forwarder};

fn base_item(is_header: bool, name: String, path: String, size_str: String, modified_str: String, mod_secs: u64, size_bytes: u64) -> FileItem {
    let (mod_hi, mod_lo) = size_to_hi_lo(mod_secs);
    let (size_hi, size_lo) = size_to_hi_lo(size_bytes);
    let val_str: [String; STR_BASE_COUNT] = [name, path, size_str, modified_str];
    let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, size_hi, size_lo];
    FileItem {
        is_header,
        val_str: val_str.into(),
        val_int: val_int.into(),
    }
}

fn header_item(label: String) -> FileItem {
    let val_str: [String; STR_BASE_COUNT] = [label, String::new(), String::new(), String::new()];
    let val_int: [i32; INT_BASE_COUNT] = [0, 0, 0, 0];
    FileItem {
        is_header: true,
        val_str: val_str.into(),
        val_int: val_int.into(),
    }
}

fn item_name(item: &FileItem) -> &str {
    &item.val_str[STR_IDX_NAME]
}

fn item_path(item: &FileItem) -> &str {
    &item.val_str[STR_IDX_PATH]
}

fn item_size_u64(item: &FileItem) -> u64 {
    let hi = item.val_int[INT_IDX_SIZE_HI] as u64;
    let lo = item.val_int[INT_IDX_SIZE_LO] as u64;
    (hi << 32) | (lo & 0xFFFF_FFFF)
}

pub(crate) fn scan_duplicate_files<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    check_method: czkawka_core::common::model::CheckingMethod,
    hash_type: czkawka_core::common::model::HashType,
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
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
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
            items.push(header_item(format!("{} pliki  \u{00d7}  {} / plik  =  {} \u{0142}\u{0105}cznie", group.len(), per, total)));
            for fe in group {
                items.push(base_item(
                    false,
                    file_name(&fe.path),
                    parent_str(&fe.path),
                    fmt_size(fe.size),
                    fmt_date(fe.modified_date),
                    fe.modified_date,
                    fe.size,
                ));
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
    fwd.join().expect("Failed to join progress forwarder thread");
    let mut items: Vec<FileItem> = tool
        .get_empty_folder_list()
        .values()
        .map(|fe| {
            base_item(
                false,
                file_name(&fe.path),
                parent_str(&fe.path),
                String::new(),
                fmt_date(fe.modified_date),
                fe.modified_date,
                0,
            )
        })
        .collect();
    items.sort_by(|a, b| item_path(a).cmp(item_path(b)).then(item_name(a).cmp(item_name(b))));
    items
}

pub(crate) fn scan_similar_images<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    similarity_preset: czkawka_core::tools::similar_images::SimilarityPreset,
    hash_size: u8,
    hash_alg: czkawka_core::re_exported::HashAlg,
    image_filter: czkawka_core::re_exported::FilterType,
    ignore_same_size: bool,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::tools::similar_images::{SimilarImages, SimilarImagesParameters, return_similarity_from_similarity_preset};
    let max_diff = return_similarity_from_similarity_preset(similarity_preset, hash_size);
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = SimilarImagesParameters::new(max_diff, hash_size, hash_alg, image_filter, ignore_same_size);
    let mut tool = SimilarImages::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
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
        items.push(header_item(format!("{} podobnych obraz\u{00f3}w", group.len())));
        for img in group {
            let dims = format!("{}\u{00d7}{}  \u{0394}{}", img.width, img.height, img.difference);
            let (mod_hi, mod_lo) = size_to_hi_lo(img.modified_date);
            let (size_hi, size_lo) = size_to_hi_lo(img.size);
            let val_str: [String; MAX_STR_DATA_SIMILAR_IMAGES] = [file_name(&img.path), parent_str(&img.path), fmt_size(img.size), fmt_date(img.modified_date), dims];
            let val_int: [i32; MAX_INT_DATA_SIMILAR_IMAGES] = [mod_hi, mod_lo, size_hi, size_lo, img.width as i32, img.height as i32, img.difference as i32];
            items.push(FileItem {
                is_header: false,
                val_str: val_str.into(),
                val_int: val_int.into(),
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
    fwd.join().expect("Failed to join progress forwarder thread");
    let mut items: Vec<FileItem> = tool
        .get_empty_files()
        .iter()
        .map(|fe| {
            base_item(
                false,
                file_name(&fe.path),
                parent_str(&fe.path),
                fmt_size(fe.size),
                fmt_date(fe.modified_date),
                fe.modified_date,
                fe.size,
            )
        })
        .collect();
    items.sort_by(|a, b| item_path(a).cmp(item_path(b)).then(item_name(a).cmp(item_name(b))));
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
    fwd.join().expect("Failed to join progress forwarder thread");
    let mut items: Vec<FileItem> = tool
        .get_temporary_files()
        .iter()
        .map(|fe| {
            base_item(
                false,
                file_name(&fe.path),
                parent_str(&fe.path),
                fmt_size(fe.size),
                fmt_date(fe.modified_date),
                fe.modified_date,
                fe.size,
            )
        })
        .collect();
    items.sort_by(|a, b| item_path(a).cmp(item_path(b)).then(item_name(a).cmp(item_name(b))));
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
    fwd.join().expect("Failed to join progress forwarder thread");
    tool.get_big_files()
        .iter()
        .map(|fe| {
            base_item(
                false,
                file_name(&fe.path),
                parent_str(&fe.path),
                fmt_size(fe.size),
                fmt_date(fe.modified_date),
                fe.modified_date,
                fe.size,
            )
        })
        .collect()
}

pub(crate) fn scan_broken_files<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    checked_types: u32,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = BrokenFilesParameters::new(CheckedTypes::from_bits_truncate(checked_types));
    let mut tool = BrokenFiles::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
    let mut items: Vec<FileItem> = tool
        .get_broken_files()
        .iter()
        .map(|be| {
            let (mod_hi, mod_lo) = size_to_hi_lo(be.modified_date);
            let val_str: [String; MAX_STR_DATA_BROKEN_FILES] = [
                file_name(&be.path),
                parent_str(&be.path),
                fmt_size(be.size),
                fmt_date(be.modified_date),
                be.error_string.clone(),
            ];
            let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, 0, 0];
            FileItem {
                is_header: false,
                val_str: val_str.into(),
                val_int: val_int.into(),
            }
        })
        .collect();
    items.sort_by(|a, b| item_path(a).cmp(item_path(b)).then(item_name(a).cmp(item_name(b))));
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
    fwd.join().expect("Failed to join progress forwarder thread");
    let mut items: Vec<FileItem> = tool
        .get_bad_extensions_files()
        .iter()
        .map(|be| {
            let (mod_hi, mod_lo) = size_to_hi_lo(be.modified_date);
            let (size_hi, size_lo) = size_to_hi_lo(be.size);
            let val_str: [String; MAX_STR_DATA_BAD_EXTENSIONS] = [
                file_name(&be.path),
                parent_str(&be.path),
                fmt_size(be.size),
                fmt_date(be.modified_date),
                format!(".{} \u{2192} .{}", be.current_extension, be.proper_extension),
                be.proper_extension.clone(),
            ];
            let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, size_hi, size_lo];
            FileItem {
                is_header: false,
                val_str: val_str.into(),
                val_int: val_int.into(),
            }
        })
        .collect();
    items.sort_by(|a, b| {
        item_size_u64(b)
            .cmp(&item_size_u64(a))
            .then(item_path(a).cmp(item_path(b)))
            .then(item_name(a).cmp(item_name(b)))
    });
    items
}

pub(crate) fn scan_bad_names<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
    uppercase_extension: bool,
    emoji_used: bool,
    space_at_start_or_end: bool,
    non_ascii_graphical: bool,
    remove_duplicated_non_alpha: bool,
) -> Vec<FileItem> {
    use czkawka_core::tools::bad_names::{BadNames, BadNamesParameters, NameIssues};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = BadNamesParameters::new(NameIssues {
        uppercase_extension,
        emoji_used,
        space_at_start_or_end,
        non_ascii_graphical,
        restricted_charset_allowed: if non_ascii_graphical { Some(vec!['_', '-', ' ', '.']) } else { None },
        remove_duplicated_non_alphanumeric: remove_duplicated_non_alpha,
    });
    let mut tool = BadNames::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
    let mut items: Vec<FileItem> = tool
        .get_bad_names_files()
        .iter()
        .map(|bn| {
            let (mod_hi, mod_lo) = size_to_hi_lo(bn.modified_date);
            let val_str: [String; MAX_STR_DATA_BAD_NAMES] = [
                file_name(&bn.path),
                parent_str(&bn.path),
                fmt_size(bn.size),
                fmt_date(bn.modified_date),
                bn.new_name.clone(),
            ];
            let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, 0, 0];
            FileItem {
                is_header: false,
                val_str: val_str.into(),
                val_int: val_int.into(),
            }
        })
        .collect();
    items.sort_by(|a, b| item_path(a).cmp(item_path(b)).then(item_name(a).cmp(item_name(b))));
    items
}

pub(crate) fn scan_exif_remover<H: ScanResultHandler>(dirs: Vec<PathBuf>, filters: &CommonFilters, stop: &Arc<AtomicBool>, handler: &Arc<H>, scan_id: u32) -> Vec<FileItem> {
    use czkawka_core::tools::exif_remover::{ExifRemover, ExifRemoverParameters};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = ExifRemoverParameters::new(vec![]);
    let mut tool = ExifRemover::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
    let mut items: Vec<(u64, FileItem)> = tool
        .get_exif_files()
        .iter()
        .map(|ee| {
            let (mod_hi, mod_lo) = size_to_hi_lo(ee.modified_date);
            let (size_hi, size_lo) = size_to_hi_lo(ee.size);
            let val_str: [String; STR_BASE_COUNT] = [file_name(&ee.path), parent_str(&ee.path), fmt_size(ee.size), fmt_date(ee.modified_date)];
            let val_int: [i32; MAX_INT_DATA_EXIF_REMOVER] = [mod_hi, mod_lo, size_hi, size_lo, ee.exif_tags.len() as i32];
            (
                ee.size,
                FileItem {
                    is_header: false,
                    val_str: val_str.into(),
                    val_int: val_int.into(),
                },
            )
        })
        .collect();
    items.sort_by_key(|(size, _)| std::cmp::Reverse(*size));
    items.into_iter().map(|(_, item)| item).collect()
}

pub(crate) fn scan_same_music<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    music_similarity: u32,
    approximate: bool,
    check_method: czkawka_core::common::model::CheckingMethod,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::tools::same_music::{MusicSimilarity, SameMusic, SameMusicParameters};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = SameMusicParameters::new(MusicSimilarity::from_bits_truncate(music_similarity), approximate, check_method, 0.0, 0.0, false);
    let mut tool = SameMusic::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.set_recursive_search(true);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
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
        items.push(header_item(format!("{} podobnych utw\u{00f3}r\u{00f3}w", group.len())));
        for me in group {
            let artist = if me.track_artist.is_empty() { "?" } else { &me.track_artist };
            let title = if me.track_title.is_empty() { "?" } else { &me.track_title };
            let (mod_hi, mod_lo) = size_to_hi_lo(me.modified_date);
            let (size_hi, size_lo) = size_to_hi_lo(me.size);
            let val_str: [String; MAX_STR_DATA_SAME_MUSIC] = [
                file_name(&me.path),
                parent_str(&me.path),
                fmt_size(me.size),
                fmt_date(me.modified_date),
                format!("{artist} \u{2013} {title}"),
                title.to_string(),
            ];
            let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, size_hi, size_lo];
            items.push(FileItem {
                is_header: false,
                val_str: val_str.into(),
                val_int: val_int.into(),
            });
        }
    }
    items
}
