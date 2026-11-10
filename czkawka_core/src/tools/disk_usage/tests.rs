use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tempfile::TempDir;

use crate::common::tool_data::CommonData;
use crate::common::traits::Search;
use crate::tools::disk_usage::category::FileCategory;
use crate::tools::disk_usage::report::build_report;
use crate::tools::disk_usage::tree::{DiskNode, DiskTree, NodeKind};
use crate::tools::disk_usage::{ContentDetection, DiskUsage, DiskUsageParameters};

fn scan(path: &Path, excluded: Vec<std::path::PathBuf>) -> DiskUsage {
    let mut tool = DiskUsage::new(DiskUsageParameters::new(ContentDetection::ExtensionOnly));
    tool.set_included_paths(vec![path.to_path_buf()]);
    tool.set_excluded_paths(excluded);
    tool.search(&Arc::new(AtomicBool::new(false)), None);
    tool
}

fn sample_tree() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let root = temp_dir.path();
    fs::create_dir_all(root.join("music")).unwrap();
    fs::create_dir_all(root.join("docs/nested")).unwrap();
    fs::write(root.join("music/song.mp3"), vec![0; 100]).unwrap();
    fs::write(root.join("docs/readme.txt"), vec![0; 10]).unwrap();
    fs::write(root.join("docs/nested/notes.md"), vec![0; 5]).unwrap();
    fs::write(root.join("top.bin"), vec![0; 20]).unwrap();
    temp_dir
}

fn index_of(node: &DiskNode, name: &str) -> u32 {
    node.children.iter().position(|child| &*child.name == name).unwrap_or_else(|| panic!("no child {name}")) as u32
}

fn child<'a>(node: &'a DiskNode, name: &str) -> &'a DiskNode {
    node.children.iter().find(|child| &*child.name == name).unwrap_or_else(|| panic!("no child {name}"))
}

#[test]
fn directory_sizes_are_sums_of_descendants() {
    let temp_dir = sample_tree();
    let tool = scan(temp_dir.path(), Vec::new());

    let root = &tool.get_tree().roots[0];
    assert_eq!(root.size, 135);
    assert_eq!(child(root, "music").size, 100);
    assert_eq!(child(root, "docs").size, 15);
    assert_eq!(child(child(root, "docs"), "nested").size, 5);
    assert_eq!(tool.get_information().number_of_files, 4);
    assert_eq!(tool.get_information().number_of_directories, 4);
    assert_eq!(tool.get_information().total_size, 135);
}

#[test]
fn children_are_sorted_by_size_descending() {
    let temp_dir = sample_tree();
    let tool = scan(temp_dir.path(), Vec::new());

    let names = tool.get_tree().roots[0].children.iter().map(|child| &*child.name).collect::<Vec<_>>();
    assert_eq!(names, ["music", "top.bin", "docs"]);
}

#[test]
fn categories_come_from_extensions_and_directories_get_dominant_one() {
    let temp_dir = sample_tree();
    let tool = scan(temp_dir.path(), Vec::new());

    let root = &tool.get_tree().roots[0];
    assert_eq!(child(child(root, "music"), "song.mp3").category, FileCategory::Audio);
    assert_eq!(child(root, "top.bin").category, FileCategory::Binary);
    assert_eq!(child(root, "music").category, FileCategory::Audio);
    assert_eq!(root.category, FileCategory::Audio);
    let sizes = root.category_sizes.as_ref().unwrap();
    assert_eq!(sizes[FileCategory::Audio.index()], 100);
    assert_eq!(sizes[FileCategory::Document.index()], 15);
    assert_eq!(sizes[FileCategory::Binary.index()], 20);
}

#[test]
fn excluded_directory_is_not_counted() {
    let temp_dir = sample_tree();
    let tool = scan(temp_dir.path(), vec![temp_dir.path().join("music")]);

    let root = &tool.get_tree().roots[0];
    assert_eq!(root.size, 35);
    assert!(root.children.iter().all(|child| &*child.name != "music"));
}

#[test]
fn single_file_can_be_a_root() {
    let temp_dir = sample_tree();
    let mut tool = DiskUsage::new(DiskUsageParameters::new(ContentDetection::ExtensionOnly));
    tool.set_included_paths(vec![temp_dir.path().join("top.bin")]);
    tool.search(&Arc::new(AtomicBool::new(false)), None);

    assert_eq!(tool.get_tree().roots.len(), 1);
    assert_eq!(tool.get_tree().roots[0].size, 20);
    assert_eq!(tool.get_tree().roots[0].kind, NodeKind::File);
}

#[test]
fn stopped_scan_is_marked_as_stopped() {
    let temp_dir = sample_tree();
    let mut tool = DiskUsage::new(DiskUsageParameters::new(ContentDetection::ExtensionOnly));
    tool.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    tool.search(&Arc::new(AtomicBool::new(true)), None);

    assert!(tool.get_cd().stopped_search);
    assert_eq!(tool.get_tree().total_size(), 0);
}

#[cfg(unix)]
#[test]
fn symlinks_are_not_followed() {
    let temp_dir = sample_tree();
    std::os::unix::fs::symlink(temp_dir.path().join("music"), temp_dir.path().join("link")).unwrap();
    let tool = scan(temp_dir.path(), Vec::new());

    let root = &tool.get_tree().roots[0];
    assert_eq!(root.size, 135);
    assert_eq!(child(root, "link").kind, NodeKind::Symlink);
    assert_eq!(child(root, "link").size, 0);
}

#[test]
fn address_resolves_node_and_path() {
    let temp_dir = sample_tree();
    let tool = scan(temp_dir.path(), Vec::new());
    let tree = tool.get_tree();

    let docs = index_of(&tree.roots[0], "docs");
    let nested = index_of(&tree.roots[0].children[docs as usize], "nested");
    let address = [0, docs, nested, 0];
    assert_eq!(&*tree.node(&address).unwrap().name, "notes.md");
    assert!(tree.path(&address).unwrap().ends_with("docs/nested/notes.md"));
    assert!(tree.node(&[0, 99]).is_none());
    assert!(tree.path(&[]).is_none());
}

#[test]
fn removing_node_updates_ancestor_sizes_and_categories() {
    let temp_dir = sample_tree();
    let tool = scan(temp_dir.path(), Vec::new());
    let mut tree = tool.get_tree().clone();

    let music = index_of(&tree.roots[0], "music");
    let removed = tree.remove(&[0, music]).unwrap();

    assert_eq!(&*removed.name, "music");
    assert_eq!(tree.roots[0].size, 35);
    let sizes = tree.roots[0].category_sizes.as_ref().unwrap();
    assert_eq!(sizes[FileCategory::Audio.index()], 0);
    assert_eq!(tree.roots[0].category, FileCategory::Binary);
    assert!(tree.remove(&[0, 99]).is_none());
}

#[test]
fn removing_root_leaves_other_roots() {
    let mut tree = DiskTree::new(vec![DiskNode::file("a", 5, FileCategory::Other), DiskNode::file("b", 10, FileCategory::Other)]);

    assert_eq!(&*tree.remove(&[0]).unwrap().name, "b");
    assert_eq!(tree.total_size(), 5);
}

#[test]
fn report_respects_depth_and_entry_limits() {
    let temp_dir = sample_tree();
    let tool = scan(temp_dir.path(), Vec::new());

    let report = build_report(tool.get_tree(), 2, 2);
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].children.len(), 2);
    assert_eq!(report[0].children[0].name, "music");
    assert!(report[0].children.iter().all(|child| child.children.is_empty()));
    assert!((report[0].children[0].percent - 100.0 * 100.0 / 135.0).abs() < 1e-9);

    let top_level_only = build_report(tool.get_tree(), 1, 10);
    assert!(top_level_only[0].children.is_empty());
}

#[test]
fn extension_categories_are_case_insensitive() {
    assert_eq!(FileCategory::from_extension("JPG"), FileCategory::Image);
    for extension in ["rw2", "raf", "orf", "3fr", "fff", "pef", "srw", "rwl", "x3f"] {
        assert_eq!(FileCategory::from_extension(extension), FileCategory::Image, "{extension}");
    }
    assert_eq!(FileCategory::from_extension("Rs"), FileCategory::Code);
    assert_eq!(FileCategory::from_extension("rlib"), FileCategory::Binary);
    assert_eq!(FileCategory::from_extension("gd"), FileCategory::Code);
    assert_eq!(FileCategory::from_extension("unknown"), FileCategory::Other);
    assert_eq!(FileCategory::from_path(Path::new("no_extension")), FileCategory::Other);
    assert_eq!(FileCategory::from_index(FileCategory::Video.index()), Some(FileCategory::Video));
    assert_eq!(FileCategory::from_index(99), None);
}

fn scan_paths(paths: Vec<std::path::PathBuf>) -> DiskUsage {
    let mut tool = DiskUsage::new(DiskUsageParameters::new(ContentDetection::ExtensionOnly));
    tool.set_included_paths(paths);
    tool.search(&Arc::new(AtomicBool::new(false)), None);
    tool
}

#[test]
fn nested_included_paths_are_not_counted_twice() {
    let temp_dir = sample_tree();

    let tool = scan_paths(vec![temp_dir.path().to_path_buf(), temp_dir.path().join("music")]);

    assert_eq!(tool.get_tree().roots.len(), 1);
    assert_eq!(tool.get_tree().total_size(), 135);
}

#[test]
fn the_same_included_path_twice_is_counted_once() {
    let temp_dir = sample_tree();

    let tool = scan_paths(vec![temp_dir.path().to_path_buf(), temp_dir.path().to_path_buf()]);

    assert_eq!(tool.get_tree().roots.len(), 1);
    assert_eq!(tool.get_tree().total_size(), 135);
}

#[test]
fn files_of_equal_size_are_ordered_by_name_whatever_the_read_order() {
    let temp_dir = TempDir::new().unwrap();
    for name in ["delta", "alpha", "charlie", "bravo"] {
        fs::write(temp_dir.path().join(name), vec![0; 10]).unwrap();
    }

    let tool = scan(temp_dir.path(), Vec::new());

    let names = tool.get_tree().roots[0].children.iter().map(|child| child.name.to_string()).collect::<Vec<_>>();
    assert_eq!(names, ["alpha", "bravo", "charlie", "delta"]);
}

#[test]
fn content_detection_reads_headers_by_file_size() {
    let large = ContentDetection::LARGE_FILE_SIZE;
    assert!(!ContentDetection::ExtensionOnly.should_read_header(large));
    assert!(!ContentDetection::LargeFiles.should_read_header(large - 1));
    assert!(ContentDetection::LargeFiles.should_read_header(large));
    assert!(ContentDetection::AllFiles.should_read_header(0));
    assert_eq!(ContentDetection::default(), ContentDetection::LargeFiles);
}

fn scan_with_excluded_items(ignore_excluded_items: bool, ignore_excluded_directories: bool) -> u64 {
    let temp_dir = sample_tree();
    fs::create_dir_all(temp_dir.path().join("node_modules")).unwrap();
    fs::write(temp_dir.path().join("node_modules/dep.js"), vec![0; 1000]).unwrap();

    let params = DiskUsageParameters {
        ignore_excluded_items,
        ignore_excluded_directories,
        ..DiskUsageParameters::new(ContentDetection::ExtensionOnly)
    };
    let mut tool = DiskUsage::new(params);
    tool.set_included_paths(vec![temp_dir.path().to_path_buf()]);
    tool.set_excluded_items(vec!["*/node_modules/*".to_string()]);
    tool.set_excluded_paths(vec![temp_dir.path().join("music")]);
    tool.search(&Arc::new(AtomicBool::new(false)), None);
    tool.get_tree().total_size()
}

#[test]
fn excluded_items_and_paths_can_be_ignored_independently() {
    assert_eq!(scan_with_excluded_items(true, false), 10 + 5 + 20 + 1000);
    assert_eq!(scan_with_excluded_items(false, false), 10 + 5 + 20);
    assert_eq!(scan_with_excluded_items(true, true), 100 + 10 + 5 + 20 + 1000);
    assert_eq!(scan_with_excluded_items(false, true), 100 + 10 + 5 + 20);
}
