use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use tempfile::TempDir;

use crate::common::cache::save_cache_to_file_generalized;
use crate::common::config_cache_path::set_config_cache_path_test;
use crate::common::model::HashType;
use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::duplicate::DuplicateEntry;
use crate::tools::duplicate::core::get_duplicate_cache_file;
use crate::tools::folder_compare::{DiffNode, DiffStatus, FolderCompare, FolderCompareMode, FolderCompareParameters, FolderGroup, GroupRelation, MatchRole};

fn write(root: &Path, relative: &str, content: &[u8]) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn params() -> FolderCompareParameters {
    FolderCompareParameters::new(HashType::Blake3)
}

fn find_duplicates(root: &Path, params: FolderCompareParameters) -> FolderCompare {
    let mut tool = FolderCompare::new(params);
    tool.set_use_cache(false);
    tool.set_included_paths(vec![root.to_path_buf()]);
    tool.search(&Arc::new(AtomicBool::new(false)), None);
    tool
}

fn compare(left: &Path, right: &Path) -> FolderCompare {
    let mut tool = FolderCompare::new(params());
    tool.set_use_cache(false);
    tool.set_included_paths(vec![left.to_path_buf()]);
    tool.set_reference_paths(vec![right.to_path_buf()]);
    tool.search(&Arc::new(AtomicBool::new(false)), None);
    tool
}

fn folder_names(group: &FolderGroup) -> Vec<String> {
    group.folders.iter().map(|folder| folder.path.file_name().unwrap().to_string_lossy().into_owned()).collect()
}

fn node<'a>(nodes: &'a [DiffNode], name: &str) -> &'a DiffNode {
    nodes.iter().find(|node| node.name == name).unwrap_or_else(|| panic!("no diff node {name}"))
}

#[test]
fn identical_folders_are_grouped() {
    let temp_dir = TempDir::new().unwrap();
    for folder in ["a", "b"] {
        write(temp_dir.path(), &format!("{folder}/x.txt"), b"hello");
        write(temp_dir.path(), &format!("{folder}/y.txt"), b"world!");
    }
    write(temp_dir.path(), "c/x.txt", b"hello");
    write(temp_dir.path(), "c/y.txt", b"WORLD!");

    let tool = find_duplicates(temp_dir.path(), params());

    assert_eq!(tool.get_mode(), FolderCompareMode::Duplicates);
    let groups = tool.get_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].relation, GroupRelation::Identical);
    assert_eq!(folder_names(&groups[0]), ["a", "b"]);
    assert_eq!(groups[0].wasted_size(), 11);
}

#[test]
fn big_files_differing_only_in_the_middle_are_not_identical() {
    let temp_dir = TempDir::new().unwrap();
    let mut first = vec![1_u8; 20_000];
    let second = first.clone();
    first[10_000] = 2;
    write(temp_dir.path(), "a/big.bin", &first);
    write(temp_dir.path(), "a/small.txt", b"s");
    write(temp_dir.path(), "b/big.bin", &second);
    write(temp_dir.path(), "b/small.txt", b"s");

    let tool = find_duplicates(temp_dir.path(), FolderCompareParameters { find_subsets: false, ..params() });

    assert!(tool.get_groups().is_empty());
}

#[test]
fn renamed_files_are_identical_only_when_names_are_ignored() {
    let temp_dir = TempDir::new().unwrap();
    write(temp_dir.path(), "a/x.txt", b"hello");
    write(temp_dir.path(), "a/y.txt", b"world!");
    write(temp_dir.path(), "b/p.txt", b"hello");
    write(temp_dir.path(), "b/q.txt", b"world!");

    let names_matter = find_duplicates(temp_dir.path(), FolderCompareParameters { find_subsets: false, ..params() });
    let names_ignored = find_duplicates(temp_dir.path(), FolderCompareParameters { ignore_names: true, ..params() });

    assert!(names_matter.get_groups().is_empty());
    assert_eq!(names_ignored.get_groups().len(), 1);
    assert_eq!(folder_names(&names_ignored.get_groups()[0]), ["a", "b"]);
}

#[test]
fn folder_with_all_files_of_another_is_reported_as_container() {
    let temp_dir = TempDir::new().unwrap();
    write(temp_dir.path(), "old/x.txt", b"hello");
    write(temp_dir.path(), "old/y.txt", b"world!");
    write(temp_dir.path(), "new/x.txt", b"hello");
    write(temp_dir.path(), "new/y.txt", b"world!");
    write(temp_dir.path(), "new/z.txt", b"extra");

    let tool = find_duplicates(temp_dir.path(), params());

    let groups = tool.get_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].relation, GroupRelation::Subset);
    assert_eq!(folder_names(&groups[0]), ["old", "new"]);
    assert_eq!(groups[0].folders[0].role, MatchRole::Contained);
    assert_eq!(groups[0].folders[1].role, MatchRole::Container);
    assert_eq!(groups[0].folders[1].extra_files, 1);
}

#[test]
fn subsets_are_not_reported_when_disabled() {
    let temp_dir = TempDir::new().unwrap();
    write(temp_dir.path(), "old/x.txt", b"hello");
    write(temp_dir.path(), "old/y.txt", b"world!");
    write(temp_dir.path(), "new/x.txt", b"hello");
    write(temp_dir.path(), "new/y.txt", b"world!");
    write(temp_dir.path(), "new/z.txt", b"extra");

    let tool = find_duplicates(temp_dir.path(), FolderCompareParameters { find_subsets: false, ..params() });

    assert!(tool.get_groups().is_empty());
}

#[test]
fn folders_below_minimal_file_count_and_empty_folders_are_skipped() {
    let temp_dir = TempDir::new().unwrap();
    write(temp_dir.path(), "a/x.txt", b"hello");
    write(temp_dir.path(), "b/x.txt", b"hello");
    fs::create_dir_all(temp_dir.path().join("empty1")).unwrap();
    fs::create_dir_all(temp_dir.path().join("empty2")).unwrap();

    let tool = find_duplicates(temp_dir.path(), params());
    let permissive = find_duplicates(temp_dir.path(), FolderCompareParameters { min_files: 1, ..params() });

    assert!(tool.get_groups().is_empty());
    assert_eq!(permissive.get_groups().len(), 1);
    assert_eq!(folder_names(&permissive.get_groups()[0]), ["a", "b"]);
}

#[test]
fn nested_identical_folders_are_reported_only_at_the_top() {
    let temp_dir = TempDir::new().unwrap();
    for folder in ["a", "b"] {
        write(temp_dir.path(), &format!("{folder}/sub/x.txt"), b"hello");
        write(temp_dir.path(), &format!("{folder}/sub/y.txt"), b"world!");
    }

    let tool = find_duplicates(temp_dir.path(), params());

    let groups = tool.get_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(folder_names(&groups[0]), ["a", "b"]);
}

#[test]
fn diff_marks_each_kind_of_difference() {
    let temp_dir = TempDir::new().unwrap();
    let (left, right) = (temp_dir.path().join("left"), temp_dir.path().join("right"));
    write(&left, "same.txt", b"hello");
    write(&right, "same.txt", b"hello");
    write(&left, "changed.txt", b"aaaa");
    write(&right, "changed.txt", b"bbbb");
    write(&left, "left_only.txt", b"l");
    write(&right, "right_only.txt", b"r");
    write(&left, "both/x.txt", b"x");
    write(&right, "both/x.txt", b"x");
    write(&right, "both/extra.txt", b"extra");
    write(&left, "left_dir/a.txt", b"a");

    let tool = compare(&left, &right);

    assert_eq!(tool.get_mode(), FolderCompareMode::Diff);
    let entries = &tool.get_diff().entries;
    assert_eq!(node(entries, "same.txt").status, DiffStatus::Identical);
    assert_eq!(node(entries, "changed.txt").status, DiffStatus::Different);
    assert_eq!(node(entries, "left_only.txt").status, DiffStatus::OnlyLeft);
    assert_eq!(node(entries, "right_only.txt").status, DiffStatus::OnlyRight);
    assert_eq!(node(entries, "left_dir").status, DiffStatus::OnlyLeft);
    assert!(node(entries, "left_dir").children.is_empty());
    let both = node(entries, "both");
    assert_eq!(both.status, DiffStatus::RightHasMore);
    assert_eq!(node(&both.children, "x.txt").status, DiffStatus::Identical);
    assert_eq!(node(&both.children, "extra.txt").status, DiffStatus::OnlyRight);
    assert_eq!(tool.get_information().number_of_different_entries, 5);
}

#[test]
fn identical_folders_collapse_in_diff() {
    let temp_dir = TempDir::new().unwrap();
    let (left, right) = (temp_dir.path().join("left"), temp_dir.path().join("right"));
    for side in [&left, &right] {
        write(side, "dir/sub/x.txt", b"x");
    }

    let tool = compare(&left, &right);

    let dir = node(&tool.get_diff().entries, "dir");
    assert_eq!(dir.status, DiffStatus::Identical);
    assert!(dir.children.is_empty());
    assert!(!tool.found_any_items());
}

#[test]
fn diff_honors_excluded_paths_on_the_right_side() {
    let temp_dir = TempDir::new().unwrap();
    let (left, right) = (temp_dir.path().join("left"), temp_dir.path().join("right"));
    write(&left, "x.txt", b"x");
    write(&right, "x.txt", b"x");
    write(&right, "skipped/y.txt", b"y");

    let mut tool = FolderCompare::new(params());
    tool.set_included_paths(vec![left]);
    tool.set_reference_paths(vec![right.clone()]);
    tool.set_excluded_paths(vec![right.join("skipped")]);
    tool.search(&Arc::new(AtomicBool::new(false)), None);

    assert!(tool.get_diff().entries.iter().all(|entry| entry.name != "skipped"));
}

#[test]
fn overlapping_sides_are_rejected() {
    let temp_dir = TempDir::new().unwrap();
    write(temp_dir.path(), "inner/x.txt", b"x");

    let tool = compare(temp_dir.path(), &temp_dir.path().join("inner"));

    assert!(tool.get_text_messages().critical.is_some());
}

#[test]
fn stop_flag_stops_the_search() {
    let temp_dir = TempDir::new().unwrap();
    write(temp_dir.path(), "a/x.txt", b"x");
    let mut tool = FolderCompare::new(params());
    tool.set_use_cache(false);
    tool.set_included_paths(vec![temp_dir.path().to_path_buf()]);

    tool.search(&Arc::new(AtomicBool::new(true)), None);

    assert!(tool.get_stopped_search());
}

/// Both cache tests read and write the same cache file of the process.
static CACHE_TEST_LOCK: Mutex<()> = Mutex::new(());

fn modified_seconds(path: &Path) -> u64 {
    fs::metadata(path).unwrap().modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}

fn cache_entry(path: &Path, hash: &str) -> (String, DuplicateEntry) {
    let entry = DuplicateEntry {
        path: path.to_path_buf(),
        modified_date: modified_seconds(path),
        size: fs::metadata(path).unwrap().len(),
        hash: hash.to_string(),
    };
    (path.to_string_lossy().into_owned(), entry)
}

#[test]
fn cached_full_hashes_are_used_and_new_ones_are_stored_for_duplicates() {
    let _guard = CACHE_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let cache_dir = TempDir::new().unwrap();
    set_config_cache_path_test(cache_dir.path().to_path_buf(), cache_dir.path().to_path_buf());
    std::mem::forget(cache_dir);

    let temp_dir = TempDir::new().unwrap();
    let mut first = vec![1_u8; 20_000];
    let second = first.clone();
    first[10_000] = 2;
    write(temp_dir.path(), "a/big.bin", &first);
    write(temp_dir.path(), "a/small.txt", b"s");
    write(temp_dir.path(), "b/big.bin", &second);
    write(temp_dir.path(), "b/small.txt", b"s");
    let canonical = temp_dir.path().canonicalize().unwrap();
    let cache_file = get_duplicate_cache_file(HashType::Blake3, false);
    let fake_cache = [cache_entry(&canonical.join("a/big.bin"), "fake"), cache_entry(&canonical.join("b/big.bin"), "fake")]
        .into_iter()
        .collect();
    assert!(save_cache_to_file_generalized(&cache_file, &fake_cache, false, 0).warnings.is_empty());

    let mut tool = FolderCompare::new(FolderCompareParameters { find_subsets: false, ..params() });
    tool.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    tool.search(&Arc::new(AtomicBool::new(false)), None);

    assert_eq!(tool.get_groups().len(), 1, "differing files with the same cached hash must be treated as identical");
}

#[test]
fn computed_full_hashes_are_saved_to_the_cache_of_duplicates() {
    let _guard = CACHE_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let cache_dir = TempDir::new().unwrap();
    set_config_cache_path_test(cache_dir.path().to_path_buf(), cache_dir.path().to_path_buf());
    std::mem::forget(cache_dir);

    let temp_dir = TempDir::new().unwrap();
    let content = vec![7_u8; 300_000];
    write(temp_dir.path(), "a/big.bin", &content);
    write(temp_dir.path(), "b/big.bin", &content);
    let canonical = temp_dir.path().canonicalize().unwrap();

    let mut tool = FolderCompare::new(FolderCompareParameters { min_files: 1, ..params() });
    tool.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    tool.search(&Arc::new(AtomicBool::new(false)), None);

    let key = canonical.join("a/big.bin").to_string_lossy().into_owned();
    let used = [(key.clone(), cache_entry(&canonical.join("a/big.bin"), "").1)].into_iter().collect();
    let (_, loaded) = crate::common::cache::load_cache_from_file_generalized_by_path(&get_duplicate_cache_file(HashType::Blake3, false), false, &used);
    assert!(loaded.is_some_and(|loaded| loaded.get(&key).is_some_and(|entry| !entry.hash.is_empty())));
}

fn reported_stages(mut tool: FolderCompare) -> Vec<crate::common::progress_data::ToolStage> {
    let (sender, receiver) = crossbeam_channel::unbounded();
    tool.set_use_cache(false);
    tool.search(&Arc::new(AtomicBool::new(false)), Some(&sender));
    drop(sender);
    let mut stages: Vec<_> = receiver.iter().map(|progress| progress.stage).collect();
    stages.dedup();
    stages
}

#[test]
fn nothing_to_hash_reports_only_the_scanning_stage() {
    let temp_dir = TempDir::new().unwrap();
    write(temp_dir.path(), "a/x.txt", b"1");
    write(temp_dir.path(), "b/y.txt", b"22");
    let mut tool = FolderCompare::new(params());
    tool.set_included_paths(vec![temp_dir.path().to_path_buf()]);

    let stages = reported_stages(tool);

    assert_eq!(
        stages,
        [crate::common::progress_data::ToolStage::CollectingFiles(crate::common::model::CheckingMethod::None)]
    );
}

#[test]
fn comparing_two_sides_reports_the_scanning_stage_once() {
    let temp_dir = TempDir::new().unwrap();
    let (left, right) = (temp_dir.path().join("left"), temp_dir.path().join("right"));
    write(&left, "x.txt", b"1");
    write(&right, "y.txt", b"22");
    let mut tool = FolderCompare::new(params());
    tool.set_included_paths(vec![left]);
    tool.set_reference_paths(vec![right]);

    let stages = reported_stages(tool);

    assert_eq!(
        stages,
        [crate::common::progress_data::ToolStage::CollectingFiles(crate::common::model::CheckingMethod::None)]
    );
}
