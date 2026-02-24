use chrono::{NaiveDate, Utc};
use slint::{Model, ModelRc, SharedString, VecModel};

use crate::common::{
    IntDataBigFiles, IntDataBrokenFiles, IntDataDuplicateFiles, IntDataEmptyFiles, IntDataEmptyFolders, IntDataExifRemover, IntDataInvalidSymlinks, IntDataSimilarImages,
    IntDataSimilarMusic, IntDataSimilarVideos, IntDataTemporaryFiles, IntDataVideoOptimizer, StrDataBadExtensions, StrDataBadNames, StrDataBigFiles, StrDataBrokenFiles,
    StrDataDuplicateFiles, StrDataEmptyFiles, StrDataEmptyFolders, StrDataExifRemover, StrDataInvalidSymlinks, StrDataSimilarImages, StrDataSimilarMusic, StrDataSimilarVideos,
    StrDataTemporaryFiles, StrDataVideoOptimizer, connect_i32_into_u64,
};
use crate::{ActiveTab, ColumnType, CustomSelectColumnModel, SingleMainListModel, flk};
pub(super) type SelectionResult = (u64, u64, ModelRc<SingleMainListModel>);
macro_rules! col_str {
    ($name:expr, $idx:expr) => {
        CustomSelectColumnModel {
            column_name: SharedString::from($name),
            column_idx: $idx as i32,
            column_type: ColumnType::Str,
            is_pair: false,
            enabled: false,
            filter_value: SharedString::default(),
        }
    };
}

macro_rules! col_int {
    ($name:expr, $idx:expr) => {
        CustomSelectColumnModel {
            column_name: SharedString::from($name),
            column_idx: $idx as i32,
            column_type: ColumnType::Int,
            is_pair: false,
            enabled: false,
            filter_value: SharedString::default(),
        }
    };
}

macro_rules! col_int_pair {
    ($name:expr, $idx:expr) => {
        CustomSelectColumnModel {
            column_name: SharedString::from($name),
            column_idx: $idx as i32,
            column_type: ColumnType::Int,
            is_pair: true,
            enabled: false,
            filter_value: SharedString::default(),
        }
    };
}

macro_rules! col_date {
    ($name:expr, $idx:expr) => {
        CustomSelectColumnModel {
            column_name: SharedString::from($name),
            column_idx: $idx as i32,
            column_type: ColumnType::Date,
            is_pair: false,
            enabled: false,
            filter_value: SharedString::default(),
        }
    };
}

macro_rules! col_full_path {
    ($name:expr) => {
        CustomSelectColumnModel {
            column_name: SharedString::from($name),
            column_idx: 0,
            column_type: ColumnType::FullPath,
            is_pair: false,
            enabled: false,
            filter_value: SharedString::default(),
        }
    };
}
pub(super) fn build_custom_select_columns(active_tab: ActiveTab) -> Vec<CustomSelectColumnModel> {
    let size = flk!("column_size");
    let file_name = flk!("column_file_name");
    let path = flk!("column_path");
    let mod_date = flk!("column_modification_date");
    let similarity = flk!("column_similarity");
    let dimensions = flk!("column_dimensions");
    let title = flk!("column_title");
    let artist = flk!("column_artist");
    let year = flk!("column_year");
    let bitrate = flk!("column_bitrate");
    let length = flk!("column_length");
    let genre = flk!("column_genre");
    let fps = flk!("column_fps");
    let codec = flk!("column_codec");
    let duration = flk!("column_duration");
    let type_of_error = flk!("column_type_of_error");
    let symlink_name = flk!("column_symlink_name");
    let symlink_folder = flk!("column_symlink_folder");
    let destination_path = flk!("column_destination_path");
    let current_extension = flk!("column_current_extension");
    let proper_extension = flk!("column_proper_extension");
    let exif_tags = flk!("column_exif_tags");
    let new_name = flk!("column_new_name");
    let full_path = flk!("column_full_path");

    match active_tab {
        ActiveTab::DuplicateFiles => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataDuplicateFiles::Name),
            col_str!(&path, StrDataDuplicateFiles::Path),
            col_int_pair!(format!("{} [KB]", size), IntDataDuplicateFiles::SizePart1),
            col_date!(&mod_date, IntDataDuplicateFiles::ModificationDatePart1),
        ],
        ActiveTab::EmptyFolders => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataEmptyFolders::Name),
            col_str!(&path, StrDataEmptyFolders::Path),
            col_date!(&mod_date, IntDataEmptyFolders::ModificationDatePart1),
        ],
        ActiveTab::BigFiles => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataBigFiles::Name),
            col_str!(&path, StrDataBigFiles::Path),
            col_int_pair!(format!("{} [KB]", size), IntDataBigFiles::SizePart1),
            col_date!(&mod_date, IntDataBigFiles::ModificationDatePart1),
        ],
        ActiveTab::EmptyFiles => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataEmptyFiles::Name),
            col_str!(&path, StrDataEmptyFiles::Path),
            col_date!(&mod_date, IntDataEmptyFiles::ModificationDatePart1),
        ],
        ActiveTab::TemporaryFiles => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataTemporaryFiles::Name),
            col_str!(&path, StrDataTemporaryFiles::Path),
            col_date!(&mod_date, IntDataTemporaryFiles::ModificationDatePart1),
        ],
        ActiveTab::SimilarImages => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataSimilarImages::Name),
            col_str!(&path, StrDataSimilarImages::Path),
            col_int!(&similarity, IntDataSimilarImages::SimilarityValue),
            col_int_pair!(format!("{} [KB]", size), IntDataSimilarImages::SizePart1),
            col_int!(format!("{} [px]", dimensions), IntDataSimilarImages::PixelCount),
            col_date!(&mod_date, IntDataSimilarImages::ModificationDatePart1),
        ],
        ActiveTab::SimilarVideos => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataSimilarVideos::Name),
            col_str!(&path, StrDataSimilarVideos::Path),
            col_str!(&codec, StrDataSimilarVideos::Codec),
            col_int_pair!(format!("{} [KB]", size), IntDataSimilarVideos::SizePart1),
            col_int!(format!("{} [s]", duration), IntDataSimilarVideos::Duration),
            col_int_pair!(format!("{} [kbps]", bitrate), IntDataSimilarVideos::BitratePart1),
            col_int!(&fps, IntDataSimilarVideos::Fps),
            col_date!(&mod_date, IntDataSimilarVideos::ModificationDatePart1),
        ],
        ActiveTab::SimilarMusic => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataSimilarMusic::Name),
            col_str!(&path, StrDataSimilarMusic::Path),
            col_str!(&title, StrDataSimilarMusic::Title),
            col_str!(&artist, StrDataSimilarMusic::Artist),
            col_str!(&year, StrDataSimilarMusic::Year),
            col_str!(&genre, StrDataSimilarMusic::Genre),
            col_int_pair!(format!("{} [KB]", size), IntDataSimilarMusic::SizePart1),
            col_int!(format!("{} [kbps]", bitrate), IntDataSimilarMusic::Bitrate),
            col_int!(format!("{} [s]", length), IntDataSimilarMusic::Length),
            col_date!(&mod_date, IntDataSimilarMusic::ModificationDatePart1),
        ],
        ActiveTab::InvalidSymlinks => vec![
            col_str!(&symlink_name, StrDataInvalidSymlinks::SymlinkName),
            col_str!(&symlink_folder, StrDataInvalidSymlinks::SymlinkFolder),
            col_str!(&destination_path, StrDataInvalidSymlinks::DestinationPath),
            col_str!(&type_of_error, StrDataInvalidSymlinks::TypeOfError),
            col_date!(&mod_date, IntDataInvalidSymlinks::ModificationDatePart1),
        ],
        ActiveTab::BrokenFiles => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataBrokenFiles::Name),
            col_str!(&path, StrDataBrokenFiles::Path),
            col_str!(&type_of_error, StrDataBrokenFiles::TypeOfError),
            col_int_pair!(format!("{} [KB]", size), IntDataBrokenFiles::SizePart1),
            col_date!(&mod_date, IntDataBrokenFiles::ModificationDatePart1),
        ],
        ActiveTab::BadExtensions => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataBadExtensions::Name),
            col_str!(&path, StrDataBadExtensions::Path),
            col_str!(&current_extension, StrDataBadExtensions::CurrentExtension),
            col_str!(&proper_extension, StrDataBadExtensions::ProperExtensionsGroup),
            col_str!(&proper_extension, StrDataBadExtensions::ProperExtension),
        ],
        ActiveTab::BadNames => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataBadNames::Name),
            col_str!(&new_name, StrDataBadNames::NewName),
            col_str!(&path, StrDataBadNames::Path),
        ],
        ActiveTab::ExifRemover => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataExifRemover::Name),
            col_str!(&path, StrDataExifRemover::Path),
            col_str!(&exif_tags, StrDataExifRemover::ExifTags),
            col_int_pair!(format!("{} [KB]", size), IntDataExifRemover::SizePart1),
            col_int!(&exif_tags, IntDataExifRemover::ExifTagsCount),
            col_date!(&mod_date, IntDataExifRemover::ModificationDatePart1),
        ],
        ActiveTab::VideoOptimizer => vec![
            col_full_path!(&full_path),
            col_str!(&file_name, StrDataVideoOptimizer::Name),
            col_str!(&path, StrDataVideoOptimizer::Path),
            col_str!(&codec, StrDataVideoOptimizer::Codec),
            col_int_pair!(format!("{} [KB]", size), IntDataVideoOptimizer::SizePart1),
            col_date!(&mod_date, IntDataVideoOptimizer::ModificationDatePart1),
        ],
        ActiveTab::Settings | ActiveTab::About => vec![],
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
enum CmpOp {
    Gte,
    Lte,
    Gt,
    Lt,
    Eq,
}
fn parse_op_and_value(filter: &str) -> Option<(CmpOp, u64)> {
    let filter = filter.trim();
    let (op, rest) = if let Some(r) = filter.strip_prefix(">=") {
        (CmpOp::Gte, r)
    } else if let Some(r) = filter.strip_prefix("<=") {
        (CmpOp::Lte, r)
    } else if let Some(r) = filter.strip_prefix('>') {
        (CmpOp::Gt, r)
    } else if let Some(r) = filter.strip_prefix('<') {
        (CmpOp::Lt, r)
    } else if let Some(r) = filter.strip_prefix('=') {
        (CmpOp::Eq, r)
    } else {
        (CmpOp::Eq, filter)
    };
    let val: u64 = rest.trim().parse().ok()?;
    Some((op, val))
}
fn eval_op(actual: u64, op: CmpOp, threshold: u64) -> bool {
    match op {
        CmpOp::Gte => actual >= threshold,
        CmpOp::Lte => actual <= threshold,
        CmpOp::Gt => actual > threshold,
        CmpOp::Lt => actual < threshold,
        CmpOp::Eq => actual == threshold,
    }
}

fn read_u64_pair(item: &SingleMainListModel, base_idx: usize) -> u64 {
    let mut iter = item.val_int.iter();
    let hi = iter.nth(base_idx).expect("base_idx out of bounds for int pair column");
    let lo = iter.next().expect("base_idx+1 out of bounds for int pair column");
    connect_i32_into_u64(hi, lo)
}

fn read_int_single(item: &SingleMainListModel, idx: usize) -> u64 {
    item.val_int.iter().nth(idx).expect("idx out of bounds for int single column") as u64
}
fn matches_int_filter(item: &SingleMainListModel, col: &CustomSelectColumnModel) -> bool {
    let base_idx = col.column_idx as usize;
    let filter = col.filter_value.as_str().trim();
    let Some((op, threshold)) = parse_op_and_value(filter) else { return false };
    let actual: u64 = if col.is_pair {
        let bytes = read_u64_pair(item, base_idx);
        bytes / 1024
    } else {
        read_int_single(item, base_idx)
    };
    eval_op(actual, op, threshold)
}
fn parse_date(s: &str) -> Option<u64> {
    let s = s.trim();

    let (date_part, time_part) = match s.split_once(' ') {
        Some((a, b)) => (a, Some(b.trim())),
        None => (s, None),
    };

    let d = NaiveDate::parse_from_str(date_part, "%d-%m-%Y")
        .or_else(|_| NaiveDate::parse_from_str(date_part, "%Y-%m-%d"))
        .ok()?;

    let (h, m, sec) = if let Some(t) = time_part {
        let parts: Vec<&str> = t.splitn(3, ':').collect();
        let h: u32 = parts.first().and_then(|v| v.parse().ok()).unwrap_or(0);
        let m: u32 = parts.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);
        let sec: u32 = parts.get(2).and_then(|v| v.parse().ok()).unwrap_or(0);
        (h, m, sec)
    } else {
        (0, 0, 0)
    };

    let naive_dt = d.and_hms_opt(h, m, sec)?;
    let dt = chrono::DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);

    let ts = dt.timestamp();
    if ts >= 86400 { Some((ts - 86400) as u64) } else { Some(ts as u64) }
}
fn matches_date_filter(item: &SingleMainListModel, col: &CustomSelectColumnModel) -> bool {
    let base_idx = col.column_idx as usize;
    let filter = col.filter_value.as_str().trim();

    let (op, date_str) = if let Some(r) = filter.strip_prefix(">=") {
        (CmpOp::Gte, r.trim())
    } else if let Some(r) = filter.strip_prefix("<=") {
        (CmpOp::Lte, r.trim())
    } else if let Some(r) = filter.strip_prefix('>') {
        (CmpOp::Gt, r.trim())
    } else if let Some(r) = filter.strip_prefix('<') {
        (CmpOp::Lt, r.trim())
    } else if let Some(r) = filter.strip_prefix('=') {
        (CmpOp::Eq, r.trim())
    } else {
        (CmpOp::Eq, filter)
    };
    let Some(threshold) = parse_date(date_str) else { return false };
    let actual = read_u64_pair(item, base_idx);
    eval_op(actual, op, threshold)
}
fn matches_str_filter(raw_value: &str, filter: &str, case_sensitive: bool) -> bool {
    use czkawka_core::common::items::new_excluded_item;
    use czkawka_core::common::regex_check;
    let (value_cmp, filter_cmp) = if case_sensitive {
        (raw_value.to_owned(), filter.to_owned())
    } else {
        (raw_value.to_lowercase(), filter.to_lowercase())
    };
    let excluded = new_excluded_item(&filter_cmp);
    regex_check(&excluded, &value_cmp)
}

fn matches_full_path_filter(col: &CustomSelectColumnModel, path_idx: usize, name_idx: usize, case_sensitive: bool, val_strs: &[SharedString]) -> bool {
    let path = val_strs.get(path_idx).map_or("", |s| s.as_str());
    let name = val_strs.get(name_idx).map_or("", |s| s.as_str());
    let full = format!("{path}/{name}");
    matches_str_filter(&full, col.filter_value.as_str(), case_sensitive)
}

pub(super) fn select_custom_columns(
    model: &ModelRc<SingleMainListModel>,
    active_tab: ActiveTab,
    select_mode: bool,
    columns: &[CustomSelectColumnModel],
    case_sensitive: bool,
    leave_one_in_group: bool,
) -> SelectionResult {
    let mut checked_items = 0u64;
    let mut unchecked_items = 0u64;
    let mut old_data = model.iter().collect::<Vec<_>>();
    let active_columns: Vec<&CustomSelectColumnModel> = columns.iter().filter(|c| c.enabled && !c.filter_value.is_empty()).collect();
    if active_columns.is_empty() {
        return (0, 0, ModelRc::new(VecModel::from(old_data)));
    }
    let is_header_mode = active_tab.get_is_header_mode();
    let path_idx = active_tab.get_str_path_idx();
    let name_idx = active_tab.get_str_name_idx();
    let item_matches = |item: &SingleMainListModel| -> bool {
        let val_strs: Vec<SharedString> = item.val_str.iter().collect();
        for col in &active_columns {
            let matches = match col.column_type {
                ColumnType::Str => {
                    let idx = col.column_idx as usize;
                    let raw = val_strs.get(idx).map_or("", |s| s.as_str());
                    matches_str_filter(raw, col.filter_value.as_str(), case_sensitive)
                }
                ColumnType::Int => matches_int_filter(item, col),
                ColumnType::Date => matches_date_filter(item, col),
                ColumnType::FullPath => matches_full_path_filter(col, path_idx, name_idx, case_sensitive, &val_strs),
            };
            if !matches {
                return false;
            }
        }
        true
    };
    if !is_header_mode {
        for item in &mut old_data {
            if item.header_row {
                continue;
            }
            let matches = item_matches(item);
            if select_mode {
                if matches && !item.checked {
                    item.checked = true;
                    checked_items += 1;
                }
            } else if matches && item.checked {
                item.checked = false;
                unchecked_items += 1;
            }
        }
    } else {
        let headers_idx: Vec<usize> = old_data.iter().enumerate().filter_map(|(idx, m)| if m.header_row { Some(idx) } else { None }).collect();
        for i in 0..headers_idx.len() {
            let start_idx = headers_idx[i] + 1;
            let end_idx = if i + 1 < headers_idx.len() { headers_idx[i + 1] } else { old_data.len() };
            if start_idx >= end_idx {
                continue;
            }
            let mut items_to_change: Vec<usize> = Vec::new();
            let mut already_selected = 0usize;
            let total_in_group = end_idx - start_idx;
            for (j, item) in old_data.iter().enumerate().skip(start_idx).take(end_idx - start_idx) {
                if item.header_row {
                    continue;
                }
                if select_mode {
                    if item.checked {
                        already_selected += 1;
                    } else if item_matches(item) {
                        items_to_change.push(j);
                    }
                } else if item.checked && item_matches(item) {
                    items_to_change.push(j);
                }
            }
            if select_mode {
                if leave_one_in_group && (total_in_group - already_selected == items_to_change.len()) && !items_to_change.is_empty() {
                    items_to_change.pop();
                }
                for &idx in &items_to_change {
                    old_data[idx].checked = true;
                    checked_items += 1;
                }
            } else {
                for &idx in &items_to_change {
                    old_data[idx].checked = false;
                    unchecked_items += 1;
                }
            }
        }
    }
    (checked_items, unchecked_items, ModelRc::new(VecModel::from(old_data)))
}

#[cfg(test)]
mod tests {
    use slint::VecModel;

    use super::*;
    use crate::common::{IntDataDuplicateFiles, MAX_INT_DATA_DUPLICATE_FILES, MAX_STR_DATA_DUPLICATE_FILES, create_model_from_model_vec, split_u64_into_i32s};
    use crate::test_common::get_model_vec;

    fn make_item(val_str: &[&str], val_int: &[i32]) -> SingleMainListModel {
        SingleMainListModel {
            checked: false,
            filled_header_row: false,
            header_row: false,
            selected_row: false,
            val_str: ModelRc::new(VecModel::from(val_str.iter().map(|s| SharedString::from(*s)).collect::<Vec<_>>())),
            val_int: ModelRc::new(VecModel::from(val_int.to_vec())),
        }
    }

    fn make_header() -> SingleMainListModel {
        SingleMainListModel {
            header_row: true,
            filled_header_row: false,
            ..make_item(&[], &[])
        }
    }

    fn dup_item(size_bytes: u64, name: &str, path: &str, mod_ts: u64) -> SingleMainListModel {
        let size_str = format!("{size_bytes} B");
        let mod_str = "2020-01-01 00:00:00".to_string();
        let val_str: [SharedString; MAX_STR_DATA_DUPLICATE_FILES] = [
            SharedString::from(size_str.as_str()),
            SharedString::from(name),
            SharedString::from(path),
            SharedString::from(mod_str.as_str()),
        ];
        let (sz1, sz2) = split_u64_into_i32s(size_bytes);
        let (md1, md2) = split_u64_into_i32s(mod_ts);
        let val_int: [i32; MAX_INT_DATA_DUPLICATE_FILES] = [md1, md2, sz1, sz2];
        SingleMainListModel {
            checked: false,
            filled_header_row: false,
            header_row: false,
            selected_row: false,
            val_str: ModelRc::new(VecModel::from(val_str.to_vec())),
            val_int: ModelRc::new(VecModel::from(val_int.to_vec())),
        }
    }

    fn enabled_col(mut col: CustomSelectColumnModel, filter: &str) -> CustomSelectColumnModel {
        col.enabled = true;
        col.filter_value = SharedString::from(filter);
        col
    }

    #[test]
    fn parse_op_and_val() {
        let (op, val) = parse_op_and_value("42").unwrap();
        assert_eq!(op, CmpOp::Eq);
        assert_eq!(val, 42);

        let (op, val) = parse_op_and_value(">= 100").unwrap();
        assert_eq!(op, CmpOp::Gte);
        assert_eq!(val, 100);

        let (op, val) = parse_op_and_value("<= 200").unwrap();
        assert_eq!(op, CmpOp::Lte);
        assert_eq!(val, 200);

        let (op, val) = parse_op_and_value("> 0").unwrap();
        assert_eq!(op, CmpOp::Gt);
        assert_eq!(val, 0);

        let (op, val) = parse_op_and_value("< 512").unwrap();
        assert_eq!(op, CmpOp::Lt);
        assert_eq!(val, 512);

        let (op, val) = parse_op_and_value("= 7").unwrap();
        assert_eq!(op, CmpOp::Eq);
        assert_eq!(val, 7);

        assert!(parse_op_and_value("").is_none());

        assert!(parse_op_and_value(">= abc").is_none());
    }

    #[test]
    fn parse_dat() {
        let ts = parse_date("15-01-2020").unwrap();
        assert_eq!(ts, 1578960000);

        let ts = parse_date("2020-01-15").unwrap();
        assert_eq!(ts, 1578960000);

        let ts1 = parse_date("15-01-2020").unwrap();
        let ts2 = parse_date("2020-01-15").unwrap();
        assert_eq!(ts1, ts2);

        let base = parse_date("2020-01-15").unwrap();
        let with_time = parse_date("2020-01-15 12:30:45").unwrap();
        assert_eq!(with_time, base + 12 * 3600 + 30 * 60 + 45);

        let base = parse_date("2020-01-15").unwrap();
        let with_time = parse_date("2020-01-15 08:00").unwrap();
        assert_eq!(with_time, base + 8 * 3600);

        let ts1 = parse_date("15-01-2020 06:00:00").unwrap();
        let ts2 = parse_date("2020-01-15 06:00:00").unwrap();
        assert_eq!(ts1, ts2);

        assert!(parse_date("not-a-date").is_none());
        assert!(parse_date("32-01-2020").is_none());
        assert!(parse_date("").is_none());
    }

    #[test]
    fn matches_str_filter_exact_case_sensitive() {
        assert!(matches_str_filter("hello.rs", "hello.rs", true));
        assert!(!matches_str_filter("Hello.rs", "hello.rs", true));
        assert!(matches_str_filter("Hello.RS", "hello.rs", false));
        assert!(matches_str_filter("photo_2024.jpg", "*.jpg", false));
        assert!(!matches_str_filter("photo_2024.png", "*.jpg", false));
        assert!(matches_str_filter("my_backup_file.tar", "*backup*", false));
        assert!(!matches_str_filter("my_document.pdf", "*backup*", false));
        assert!(matches_str_filter("anything", "", false));
    }

    fn str_col(idx: usize, filter: &str) -> CustomSelectColumnModel {
        CustomSelectColumnModel {
            column_name: SharedString::from(flk!("column_file_name")),
            column_idx: idx as i32,
            column_type: ColumnType::Str,
            is_pair: false,
            enabled: true,
            filter_value: SharedString::from(filter),
        }
    }

    #[test]
    fn select_custom_columns_no_active_columns_returns_unchanged() {
        let mut rows = get_model_vec(3);
        rows[0].checked = true;
        let model = create_model_from_model_vec(&rows);
        let cols: Vec<CustomSelectColumnModel> = vec![];

        let (checked, unchecked, new_model) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, false, false);

        assert_eq!(checked, 0);
        assert_eq!(unchecked, 0);

        assert!(new_model.row_data(0).unwrap().checked);
        assert!(!new_model.row_data(1).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_flat_selects_matching_rows() {
        let rows: Vec<SingleMainListModel> = vec![
            make_item(&["photo.jpg", "/home"], &[]),
            make_item(&["backup.tar", "/home"], &[]),
            make_item(&["photo_old.jpg", "/tmp"], &[]),
        ];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![str_col(0, "*.jpg")];

        let (checked, unchecked, new_model) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, false, false);

        assert_eq!(checked, 2);
        assert_eq!(unchecked, 0);
        assert!(new_model.row_data(0).unwrap().checked);
        assert!(!new_model.row_data(1).unwrap().checked);
        assert!(new_model.row_data(2).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_flat_unselects_matching_rows() {
        let rows: Vec<SingleMainListModel> = vec![
            {
                let mut r = make_item(&["photo.jpg"], &[]);
                r.checked = true;
                r
            },
            {
                let mut r = make_item(&["backup.tar"], &[]);
                r.checked = true;
                r
            },
        ];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![str_col(0, "*.jpg")];

        let (checked, unchecked, new_model) = select_custom_columns(&model, ActiveTab::BigFiles, false, &cols, false, false);

        assert_eq!(checked, 0);
        assert_eq!(unchecked, 1);
        assert!(!new_model.row_data(0).unwrap().checked);
        assert!(new_model.row_data(1).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_flat_skips_header_rows() {
        let rows: Vec<SingleMainListModel> = vec![
            {
                let mut r = make_item(&["photo.jpg"], &[]);
                r.header_row = true;
                r
            },
            make_item(&["photo.jpg"], &[]),
        ];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![str_col(0, "*.jpg")];

        let (checked, _unchecked, new_model) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, false, false);

        assert_eq!(checked, 1);
        assert!(!new_model.row_data(0).unwrap().checked);
        assert!(new_model.row_data(1).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_flat_case_sensitive_no_match() {
        let rows = vec![make_item(&["Photo.JPG"], &[])];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![str_col(0, "*.jpg")];

        let (checked, _, _) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, true, false);
        assert_eq!(checked, 0);
    }

    #[test]
    fn select_custom_columns_flat_case_insensitive_matches() {
        let rows = vec![make_item(&["Photo.JPG"], &[])];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![str_col(0, "*.jpg")];

        let (checked, _, _) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, false, false);
        assert_eq!(checked, 1);
    }

    fn grouped_model(names: &[(&str, bool)]) -> (Vec<SingleMainListModel>, ModelRc<SingleMainListModel>) {
        let rows: Vec<SingleMainListModel> = names.iter().map(|(n, is_hdr)| if *is_hdr { make_header() } else { make_item(&[n], &[]) }).collect();
        let model = create_model_from_model_vec(&rows);
        (rows, model)
    }

    #[test]
    fn select_custom_columns_header_mode_selects_matching_in_group() {
        let (_rows, model) = grouped_model(&[("", true), ("a.jpg", false), ("b.tar", false), ("", true), ("c.jpg", false)]);
        let cols = vec![str_col(0, "*.jpg")];

        let (checked, unchecked, new_model) = select_custom_columns(&model, ActiveTab::DuplicateFiles, true, &cols, false, false);

        assert_eq!(checked, 2);
        assert_eq!(unchecked, 0);
        assert!(!new_model.row_data(0).unwrap().checked);
        assert!(new_model.row_data(1).unwrap().checked);
        assert!(!new_model.row_data(2).unwrap().checked);
        assert!(!new_model.row_data(3).unwrap().checked);
        assert!(new_model.row_data(4).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_header_mode_leave_one_in_group_keeps_one_unselected() {
        let (_rows, model) = grouped_model(&[("", true), ("a.jpg", false), ("b.jpg", false)]);
        let cols = vec![str_col(0, "*.jpg")];

        let (checked, _, new_model) = select_custom_columns(&model, ActiveTab::DuplicateFiles, true, &cols, false, true);

        assert_eq!(checked, 1);
        assert!(new_model.row_data(1).unwrap().checked);
        assert!(!new_model.row_data(2).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_header_mode_leave_one_in_group_no_effect_when_already_partially_selected() {
        let rows: Vec<SingleMainListModel> = vec![
            make_header(),
            {
                let mut r = make_item(&["a.jpg"], &[]);
                r.checked = true;
                r
            },
            make_item(&["b.jpg"], &[]),
            make_item(&["c.jpg"], &[]),
        ];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![str_col(0, "*.jpg")];

        let (checked, _, new_model) = select_custom_columns(&model, ActiveTab::DuplicateFiles, true, &cols, false, true);

        assert_eq!(checked, 1);
        assert!(new_model.row_data(1).unwrap().checked);

        let b = new_model.row_data(2).unwrap().checked;
        let c = new_model.row_data(3).unwrap().checked;
        assert!(b ^ c, "exactly one of b/c should be selected by leave_one_in_group");
    }

    #[test]
    fn select_custom_columns_header_mode_unselect() {
        let rows: Vec<SingleMainListModel> = vec![
            make_header(),
            {
                let mut r = make_item(&["a.jpg"], &[]);
                r.checked = true;
                r
            },
            {
                let mut r = make_item(&["b.jpg"], &[]);
                r.checked = true;
                r
            },
            make_item(&["c.tar"], &[]),
        ];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![str_col(0, "*.jpg")];

        let (checked, unchecked, new_model) = select_custom_columns(&model, ActiveTab::DuplicateFiles, false, &cols, false, false);

        assert_eq!(checked, 0);
        assert_eq!(unchecked, 2);
        assert!(!new_model.row_data(1).unwrap().checked);
        assert!(!new_model.row_data(2).unwrap().checked);
        assert!(!new_model.row_data(3).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_int_pair_size_filter() {
        let small = dup_item(512 * 1024, "small.bin", "/tmp", 0);
        let large = dup_item(3 * 1024 * 1024, "large.bin", "/tmp", 0);

        let rows = vec![make_header(), small, large];
        let model = create_model_from_model_vec(&rows);

        let size_col = enabled_col(
            CustomSelectColumnModel {
                column_name: SharedString::from(format!("{} [KB]", flk!("column_size"))),
                column_idx: IntDataDuplicateFiles::SizePart1 as i32,
                column_type: ColumnType::Int,
                is_pair: true,
                enabled: false,
                filter_value: SharedString::default(),
            },
            ">= 1024",
        );

        let (checked, _, new_model) = select_custom_columns(&model, ActiveTab::DuplicateFiles, true, &[size_col], false, false);

        assert_eq!(checked, 1);
        assert!(!new_model.row_data(1).unwrap().checked);
        assert!(new_model.row_data(2).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_multiple_columns_requires_all_to_match() {
        let rows: Vec<SingleMainListModel> = vec![
            make_item(&["photo.jpg", "/home/user"], &[]),
            make_item(&["photo.jpg", "/tmp"], &[]),
            make_item(&["backup.tar", "/home/user"], &[]),
        ];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![str_col(0, "*.jpg"), str_col(1, "/home/*")];

        let (checked, _, new_model) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, false, false);

        assert_eq!(checked, 1);
        assert!(new_model.row_data(0).unwrap().checked);
        assert!(!new_model.row_data(1).unwrap().checked);
        assert!(!new_model.row_data(2).unwrap().checked);
    }

    fn make_full_path_col(filter: &str) -> CustomSelectColumnModel {
        CustomSelectColumnModel {
            column_name: SharedString::from(flk!("column_full_path")),
            column_idx: 0,
            column_type: ColumnType::FullPath,
            is_pair: false,
            enabled: true,
            filter_value: SharedString::from(filter),
        }
    }

    // BigFiles: StrData layout: Size=0, Name=1, Path=2, ModificationDate=3
    // get_str_path_idx() = 2, get_str_name_idx() = 1
    // full_path = "{val_str[2]}/{val_str[1]}"
    fn bigfiles_item_with_path(name: &str, path: &str) -> SingleMainListModel {
        make_item(&["0 B", name, path, "2020-01-01"], &[0, 0, 0, 0])
    }

    #[test]
    fn select_custom_columns_full_path_matches_path_slash_name() {
        let rows = vec![
            bigfiles_item_with_path("photo.jpg", "/home/user"),
            bigfiles_item_with_path("backup.tar", "/tmp"),
            bigfiles_item_with_path("notes.jpg", "/home/user/docs"),
        ];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![make_full_path_col("*.jpg")];

        let (checked, _, new_model) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, false, false);

        assert_eq!(checked, 2);
        assert!(new_model.row_data(0).unwrap().checked);
        assert!(!new_model.row_data(1).unwrap().checked);
        assert!(new_model.row_data(2).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_full_path_filters_by_directory() {
        let rows = vec![
            bigfiles_item_with_path("file.rs", "/home/user/project"),
            bigfiles_item_with_path("file.rs", "/tmp"),
            bigfiles_item_with_path("other.rs", "/home/user/project"),
        ];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![make_full_path_col("/home/user/*")];

        let (checked, _, new_model) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, false, false);

        assert_eq!(checked, 2);
        assert!(new_model.row_data(0).unwrap().checked);
        assert!(!new_model.row_data(1).unwrap().checked);
        assert!(new_model.row_data(2).unwrap().checked);
    }

    #[test]
    fn select_custom_columns_full_path_case_insensitive() {
        let rows = vec![bigfiles_item_with_path("Photo.JPG", "/HOME/User")];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![make_full_path_col("*.jpg")];

        let (checked, _, _) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, false, false);
        assert_eq!(checked, 1);
    }

    #[test]
    fn select_custom_columns_full_path_case_sensitive_no_match() {
        let rows = vec![bigfiles_item_with_path("Photo.JPG", "/HOME/User")];
        let model = create_model_from_model_vec(&rows);
        let cols = vec![make_full_path_col("*.jpg")];

        let (checked, _, _) = select_custom_columns(&model, ActiveTab::BigFiles, true, &cols, true, false);
        assert_eq!(checked, 0);
    }
}
