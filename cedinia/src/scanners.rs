use std::cmp::Reverse;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;

use crate::common::{
    INT_BASE_COUNT, INT_IDX_SIZE_HI, INT_IDX_SIZE_LO, MAX_INT_DATA_EXIF_REMOVER, MAX_INT_DATA_SIMILAR_IMAGES, MAX_STR_DATA_BAD_EXTENSIONS, MAX_STR_DATA_BAD_NAMES,
    MAX_STR_DATA_BROKEN_FILES, MAX_STR_DATA_SAME_MUSIC, MAX_STR_DATA_SIMILAR_IMAGES, MAX_STR_DATA_SIMILAR_VIDEOS, STR_BASE_COUNT, STR_IDX_NAME, STR_IDX_PATH,
};
use crate::scan_runner::{CommonFilters, FileItem, ScanResultHandler, apply_filters, file_name, fmt_date, fmt_size, parent_str, size_to_hi_lo, spawn_progress_forwarder};

fn base_item(name: String, path: String, size_str: String, modified_str: String, mod_secs: u64, size_bytes: u64) -> FileItem {
    let (mod_hi, mod_lo) = size_to_hi_lo(mod_secs);
    let (size_hi, size_lo) = size_to_hi_lo(size_bytes);
    let val_str: [String; STR_BASE_COUNT] = [name, path, size_str, modified_str];
    let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, size_hi, size_lo];
    FileItem {
        is_header: false,
        is_reference: false,
        val_str: val_str.into(),
        val_int: val_int.into(),
    }
}

fn ref_base_item(name: String, path: String, size_str: String, modified_str: String, mod_secs: u64, size_bytes: u64) -> FileItem {
    FileItem {
        is_reference: true,
        ..base_item(name, path, size_str, modified_str, mod_secs, size_bytes)
    }
}

fn header_item(label: String) -> FileItem {
    let val_str: [String; STR_BASE_COUNT] = [label, String::new(), String::new(), String::new()];
    let val_int: [i32; INT_BASE_COUNT] = [0, 0, 0, 0];
    FileItem {
        is_header: true,
        is_reference: false,
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
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::common::model::CheckingMethod;
    use czkawka_core::tools::duplicate::{DuplicateEntry, DuplicateFinder, DuplicateFinderParameters};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = DuplicateFinderParameters::new(check_method, hash_type, filters.use_cache, 8 * 1024, 0, false);
    let mut tool = DuplicateFinder::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");

    let use_ref = tool.get_use_reference();

    let mut groups: Vec<(Option<DuplicateEntry>, Vec<DuplicateEntry>)> = if use_ref {
        match check_method {
            CheckingMethod::Hash => tool
                .get_files_with_identical_hashes_referenced()
                .values()
                .flatten()
                .cloned()
                .map(|(orig, others)| (Some(orig), others))
                .collect(),
            CheckingMethod::Name => tool
                .get_files_with_identical_name_referenced()
                .values()
                .cloned()
                .map(|(orig, others)| (Some(orig), others))
                .collect(),
            CheckingMethod::Size => tool
                .get_files_with_identical_size_referenced()
                .values()
                .cloned()
                .map(|(orig, others)| (Some(orig), others))
                .collect(),
            CheckingMethod::SizeName => tool
                .get_files_with_identical_size_names_referenced()
                .values()
                .cloned()
                .map(|(orig, others)| (Some(orig), others))
                .collect(),
            _ => Vec::new(),
        }
    } else {
        match check_method {
            CheckingMethod::Hash => tool.get_files_sorted_by_hash().values().flatten().cloned().map(|group| (None, group)).collect(),
            CheckingMethod::Name => tool.get_files_sorted_by_names().values().cloned().map(|group| (None, group)).collect(),
            CheckingMethod::Size => tool.get_files_sorted_by_size().values().cloned().map(|group| (None, group)).collect(),
            CheckingMethod::SizeName => tool.get_files_sorted_by_size_name().values().cloned().map(|group| (None, group)).collect(),
            _ => Vec::new(),
        }
    };

    groups.sort_by_key(|(ref_f, dups)| {
        let total: u64 = dups.iter().map(|f| f.size).sum::<u64>() + ref_f.as_ref().map_or(0, |f| f.size);
        Reverse(total)
    });

    let mut items: Vec<FileItem> = Vec::new();
    for (ref_file, dup_files) in groups {
        let group_len = dup_files.len() + usize::from(ref_file.is_some());
        if group_len < 2 {
            continue;
        }
        let file_size = ref_file.as_ref().map(|f| f.size).or_else(|| dup_files.first().map(|f| f.size)).unwrap_or(0);
        let total = fmt_size(file_size * group_len as u64);
        let per = fmt_size(file_size);
        items.push(header_item(crate::flc!(
            "duplicates_group_header",
            count = group_len,
            per_file = per.as_str(),
            total = total.as_str()
        )));
        if let Some(ref_fe) = ref_file {
            items.push(ref_base_item(
                file_name(&ref_fe.path),
                parent_str(&ref_fe.path),
                fmt_size(ref_fe.size),
                fmt_date(ref_fe.modified_date),
                ref_fe.modified_date,
                ref_fe.size,
            ));
        }
        for fe in &dup_files {
            items.push(base_item(
                file_name(&fe.path),
                parent_str(&fe.path),
                fmt_size(fe.size),
                fmt_date(fe.modified_date),
                fe.modified_date,
                fe.size,
            ));
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
        .map(|fe| base_item(file_name(&fe.path), parent_str(&fe.path), String::new(), fmt_date(fe.modified_date), fe.modified_date, 0))
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
    ignore_same_resolution: bool,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::tools::similar_images::{ImagesEntry, SimilarImages, SimilarImagesParameters, return_similarity_from_similarity_preset};
    let max_diff = return_similarity_from_similarity_preset(similarity_preset, hash_size);
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = SimilarImagesParameters::new(max_diff, hash_size, hash_alg, image_filter, ignore_same_size, ignore_same_resolution);
    let mut tool = SimilarImages::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");

    let use_ref = tool.get_use_reference();
    let mut groups: Vec<(Option<_>, Vec<_>)> = if use_ref {
        tool.get_similar_images_referenced().iter().cloned().map(|(orig, others)| (Some(orig), others)).collect()
    } else {
        tool.get_similar_images().iter().cloned().map(|g| (None, g)).collect()
    };

    groups.sort_by_key(|(ref_img, imgs)| {
        let total: u64 = imgs.iter().map(|i| i.size).sum::<u64>() + ref_img.as_ref().map_or(0, |i| i.size);
        Reverse(total)
    });

    let mut items: Vec<FileItem> = Vec::new();
    for (ref_img, dup_imgs) in groups {
        let group_len = dup_imgs.len() + usize::from(ref_img.is_some());
        if group_len < 2 {
            continue;
        }
        items.push(header_item(crate::flc!("similar_images_group_header", count = group_len)));

        let make_img_item = |img: &ImagesEntry, is_reference: bool| {
            let dims = format!("{}×{}  Δ{}", img.width, img.height, img.difference);
            let (mod_hi, mod_lo) = size_to_hi_lo(img.modified_date);
            let (size_hi, size_lo) = size_to_hi_lo(img.size);
            let val_str: [String; MAX_STR_DATA_SIMILAR_IMAGES] = [file_name(&img.path), parent_str(&img.path), fmt_size(img.size), fmt_date(img.modified_date), dims];
            let val_int: [i32; MAX_INT_DATA_SIMILAR_IMAGES] = [mod_hi, mod_lo, size_hi, size_lo, img.width as i32, img.height as i32, img.difference as i32];
            FileItem {
                is_header: false,
                is_reference,
                val_str: val_str.into(),
                val_int: val_int.into(),
            }
        };

        if let Some(ref_i) = ref_img.as_ref() {
            items.push(make_img_item(ref_i, true));
        }
        for img in &dup_imgs {
            items.push(make_img_item(img, false));
        }
    }
    items
}

pub(crate) fn scan_empty_files<H: ScanResultHandler>(dirs: Vec<PathBuf>, filters: &CommonFilters, stop: &Arc<AtomicBool>, handler: &Arc<H>, scan_id: u32) -> Vec<FileItem> {
    use czkawka_core::tools::empty_files::EmptyFiles;
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let mut tool = EmptyFiles::default();
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
    let mut items: Vec<FileItem> = tool
        .get_empty_files()
        .iter()
        .map(|fe| {
            base_item(
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

pub(crate) fn scan_temporary_files<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    extensions: Vec<String>,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::tools::temporary::{Temporary, TemporaryParameters};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let mut tool = Temporary::new(TemporaryParameters { extensions });
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
    let mut items: Vec<FileItem> = tool
        .get_temporary_files()
        .iter()
        .map(|fe| {
            base_item(
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
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
    tool.get_big_files()
        .iter()
        .map(|fe| {
            base_item(
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
                be.get_error_string(),
            ];
            let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, 0, 0];
            FileItem {
                is_header: false,
                is_reference: false,
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
                format!(".{} → .{}", be.current_extension, be.proper_extension),
                be.proper_extension.clone(),
            ];
            let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, size_hi, size_lo];
            FileItem {
                is_header: false,
                is_reference: false,
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
                is_reference: false,
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
    let params = ExifRemoverParameters::new(Vec::new());
    let mut tool = ExifRemover::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
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
                    is_reference: false,
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
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");
    use czkawka_core::tools::same_music::MusicEntry;
    let use_ref = tool.get_use_reference();
    let mut groups: Vec<(Option<MusicEntry>, Vec<MusicEntry>)> = if use_ref {
        tool.get_similar_music_referenced().iter().cloned().map(|(orig, others)| (Some(orig), others)).collect()
    } else {
        tool.get_duplicated_music_entries().iter().cloned().map(|items| (None, items)).collect()
    };

    groups.sort_by_key(|(ref_me, dups)| {
        let total: u64 = dups.iter().map(|me| me.size).sum::<u64>() + ref_me.as_ref().map_or(0, |me| me.size);
        Reverse(total)
    });

    let make_music_item = |me: &MusicEntry, is_reference: bool| {
        let artist = if me.track_artist.is_empty() { "?" } else { &me.track_artist };
        let title = if me.track_title.is_empty() { "?" } else { &me.track_title };
        let (mod_hi, mod_lo) = size_to_hi_lo(me.modified_date);
        let (size_hi, size_lo) = size_to_hi_lo(me.size);
        let val_str: [String; MAX_STR_DATA_SAME_MUSIC] = [
            file_name(&me.path),
            parent_str(&me.path),
            fmt_size(me.size),
            fmt_date(me.modified_date),
            format!("{artist} - {title}"),
            title.to_string(),
        ];
        let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, size_hi, size_lo];
        FileItem {
            is_header: false,
            is_reference,
            val_str: val_str.into(),
            val_int: val_int.into(),
        }
    };

    let mut items: Vec<FileItem> = Vec::new();
    for (ref_me, dup_mes) in &groups {
        let group_len = dup_mes.len() + usize::from(ref_me.is_some());
        if group_len < 2 {
            continue;
        }
        items.push(header_item(crate::flc!("same_music_group_header", count = group_len)));
        if let Some(ref_m) = ref_me.as_ref() {
            items.push(make_music_item(ref_m, true));
        }
        for me in dup_mes {
            items.push(make_music_item(me, false));
        }
    }
    items
}

pub(crate) fn scan_similar_videos<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    filters: &CommonFilters,
    audio_similarity_percent: f64,
    audio_maximum_difference: f64,
    audio_length_ratio: f64,
    audio_min_duration_seconds: u32,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> Vec<FileItem> {
    use czkawka_core::tools::similar_videos::{DEFAULT_CROP_DETECT, DEFAULT_SKIP_FORWARD_AMOUNT, DEFAULT_VID_HASH_DURATION, SimilarVideos, SimilarVideosParameters, VideosEntry};
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let params = SimilarVideosParameters::new(
        10,    // tolerance (not used in audio mode)
        false, // exclude_videos_with_same_size
        false, // exclude_videos_with_same_resolution
        DEFAULT_SKIP_FORWARD_AMOUNT,
        DEFAULT_VID_HASH_DURATION,
        DEFAULT_CROP_DETECT,
        false, // generate_thumbnails
        10,    // thumbnail_video_percentage_from_start
        false, // generate_thumbnail_grid_instead_of_single
        2,     // thumbnail_grid_tiles_per_side
        true,  // check_audio_content – audio-only mode, no FFmpeg needed
        audio_similarity_percent,
        audio_maximum_difference,
        audio_length_ratio,
        audio_min_duration_seconds,
    );
    let mut tool = SimilarVideos::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");

    let use_ref = tool.get_use_reference();
    let mut groups: Vec<(Option<VideosEntry>, Vec<VideosEntry>)> = if use_ref {
        tool.get_similar_videos_referenced().iter().cloned().map(|(orig, others)| (Some(orig), others)).collect()
    } else {
        tool.get_similar_videos().iter().cloned().map(|g| (None, g)).collect()
    };

    groups.sort_by_key(|(ref_v, vs)| {
        let total: u64 = vs.iter().map(|v| v.size).sum::<u64>() + ref_v.as_ref().map_or(0, |v| v.size);
        Reverse(total)
    });

    fn fmt_duration(secs: Option<f64>) -> String {
        match secs {
            Some(s) if s >= 60.0 => {
                let mins = (s / 60.0) as u64;
                let rem = s as u64 % 60;
                format!("{mins}:{rem:02}")
            }
            Some(s) => format!("{s:.0} s"),
            None => String::new(),
        }
    }

    let make_video_item = |v: &VideosEntry, is_reference: bool| {
        let (mod_hi, mod_lo) = size_to_hi_lo(v.modified_date);
        let (size_hi, size_lo) = size_to_hi_lo(v.size);
        let val_str: [String; MAX_STR_DATA_SIMILAR_VIDEOS] = [
            file_name(&v.path),
            parent_str(&v.path),
            fmt_size(v.size),
            fmt_date(v.modified_date),
            fmt_duration(v.duration),
        ];
        let val_int: [i32; INT_BASE_COUNT] = [mod_hi, mod_lo, size_hi, size_lo];
        FileItem {
            is_header: false,
            is_reference,
            val_str: val_str.into(),
            val_int: val_int.into(),
        }
    };

    let mut items: Vec<FileItem> = Vec::new();
    for (ref_v, dup_vs) in groups {
        let group_len = dup_vs.len() + usize::from(ref_v.is_some());
        if group_len < 2 {
            continue;
        }
        items.push(header_item(crate::flc!("similar_videos_group_header", count = group_len)));
        if let Some(ref_video) = ref_v.as_ref() {
            items.push(make_video_item(ref_video, true));
        }
        for v in &dup_vs {
            items.push(make_video_item(v, false));
        }
    }
    items
}
