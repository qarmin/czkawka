pub mod core;
#[cfg(test)]
mod tests;
pub mod traits;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Debug;
#[cfg(target_family = "unix")]
use std::fs;
use std::fs::File;
use std::hash::Hasher;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use static_assertions::const_assert;
use xxhash_rust::xxh3::Xxh3;

use crate::common::model::{CheckingMethod, FileEntry, HashType};
use crate::common::progress_stop_handler::check_if_stop_received;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;
use crate::flc;

pub const PREHASHING_BUFFER_SIZE: u64 = 4 * 1024;
pub const THREAD_BUFFER_SIZE: usize = 2 * 1024 * 1024;

thread_local! {
    static THREAD_BUFFER: RefCell<Vec<u8>> = RefCell::new(vec![0u8; THREAD_BUFFER_SIZE]);
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DuplicateEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub hash: String,
}
impl ResultEntry for DuplicateEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
    fn get_size(&self) -> u64 {
        self.size
    }
}

impl FileEntry {
    fn into_duplicate_entry(self) -> DuplicateEntry {
        DuplicateEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,
            hash: String::new(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Info {
    pub number_of_groups_by_size: usize,
    pub number_of_duplicated_files_by_size: usize,
    pub number_of_groups_by_hash: usize,
    pub number_of_duplicated_files_by_hash: usize,
    pub number_of_groups_by_name: usize,
    pub number_of_duplicated_files_by_name: usize,
    pub number_of_groups_by_size_name: usize,
    pub number_of_duplicated_files_by_size_name: usize,
    pub lost_space_by_size: u64,
    pub lost_space_by_hash: u64,
    pub scanning_time: Duration,
}

#[derive(Clone)]
pub struct DuplicateFinderParameters {
    pub check_method: CheckingMethod,
    pub hash_type: HashType,
    pub use_prehash_cache: bool,
    pub minimal_cache_file_size: u64,
    pub minimal_prehash_cache_file_size: u64,
    pub case_sensitive_name_comparison: bool,
}

impl DuplicateFinderParameters {
    pub fn new(
        check_method: CheckingMethod,
        hash_type: HashType,
        use_prehash_cache: bool,
        minimal_cache_file_size: u64,
        minimal_prehash_cache_file_size: u64,
        case_sensitive_name_comparison: bool,
    ) -> Self {
        Self {
            check_method,
            hash_type,
            use_prehash_cache,
            minimal_cache_file_size,
            minimal_prehash_cache_file_size,
            case_sensitive_name_comparison,
        }
    }
}

pub struct DuplicateFinder {
    common_data: CommonToolData,
    information: Info,
    // File Size, File Entry
    files_with_identical_names: BTreeMap<String, Vec<DuplicateEntry>>,
    // File (Size, Name), File Entry
    files_with_identical_size_names: BTreeMap<(u64, String), Vec<DuplicateEntry>>,
    // File Size, File Entry
    files_with_identical_size: BTreeMap<u64, Vec<DuplicateEntry>>,
    // File Size, next grouped by file size, next grouped by hash
    files_with_identical_hashes: BTreeMap<u64, Vec<Vec<DuplicateEntry>>>,
    // File Size, File Entry
    files_with_identical_names_referenced: BTreeMap<String, (DuplicateEntry, Vec<DuplicateEntry>)>,
    // File (Size, Name), File Entry
    files_with_identical_size_names_referenced: BTreeMap<(u64, String), (DuplicateEntry, Vec<DuplicateEntry>)>,
    // File Size, File Entry
    files_with_identical_size_referenced: BTreeMap<u64, (DuplicateEntry, Vec<DuplicateEntry>)>,
    // File Size, next grouped by file size, next grouped by hash
    files_with_identical_hashes_referenced: BTreeMap<u64, Vec<(DuplicateEntry, Vec<DuplicateEntry>)>>,
    params: DuplicateFinderParameters,
}

#[cfg(target_family = "windows")]
fn filter_hard_links(vec_file_entry: Vec<FileEntry>) -> Vec<FileEntry> {
    let mut inodes: IndexSet<u128> = IndexSet::with_capacity(vec_file_entry.len());
    let mut identical: Vec<FileEntry> = Vec::with_capacity(vec_file_entry.len());
    for f in vec_file_entry {
        if let Ok(meta) = file_id::get_high_res_file_id(&f.path) {
            if let file_id::FileId::HighRes { file_id, .. } = meta {
                if !inodes.insert(file_id) {
                    continue;
                }
            }
        }
        identical.push(f);
    }
    identical
}

#[cfg(target_family = "unix")]
fn filter_hard_links(vec_file_entry: Vec<FileEntry>) -> Vec<FileEntry> {
    let mut inodes: IndexSet<u64> = IndexSet::with_capacity(vec_file_entry.len());
    let mut identical: Vec<FileEntry> = Vec::with_capacity(vec_file_entry.len());
    for f in vec_file_entry {
        if let Ok(meta) = fs::metadata(&f.path)
            && !inodes.insert(meta.ino())
        {
            continue;
        }
        identical.push(f);
    }
    identical
}

pub trait MyHasher {
    fn update(&mut self, bytes: &[u8]);
    fn finalize(&self) -> String;
}

impl DuplicateFinder {
    pub fn get_params(&self) -> &DuplicateFinderParameters {
        &self.params
    }

    pub const fn get_files_sorted_by_names(&self) -> &BTreeMap<String, Vec<DuplicateEntry>> {
        &self.files_with_identical_names
    }

    pub const fn get_files_sorted_by_size(&self) -> &BTreeMap<u64, Vec<DuplicateEntry>> {
        &self.files_with_identical_size
    }

    pub const fn get_files_sorted_by_size_name(&self) -> &BTreeMap<(u64, String), Vec<DuplicateEntry>> {
        &self.files_with_identical_size_names
    }

    pub const fn get_files_sorted_by_hash(&self) -> &BTreeMap<u64, Vec<Vec<DuplicateEntry>>> {
        &self.files_with_identical_hashes
    }

    pub const fn get_information(&self) -> Info {
        self.information
    }

    pub fn set_dry_run(&mut self, dry_run: bool) {
        self.common_data.dry_run = dry_run;
    }

    pub fn get_use_reference(&self) -> bool {
        self.common_data.use_reference_folders
    }

    pub fn get_files_with_identical_hashes_referenced(&self) -> &BTreeMap<u64, Vec<(DuplicateEntry, Vec<DuplicateEntry>)>> {
        &self.files_with_identical_hashes_referenced
    }

    pub fn get_files_with_identical_name_referenced(&self) -> &BTreeMap<String, (DuplicateEntry, Vec<DuplicateEntry>)> {
        &self.files_with_identical_names_referenced
    }

    pub fn get_files_with_identical_size_referenced(&self) -> &BTreeMap<u64, (DuplicateEntry, Vec<DuplicateEntry>)> {
        &self.files_with_identical_size_referenced
    }

    pub fn get_files_with_identical_size_names_referenced(&self) -> &BTreeMap<(u64, String), (DuplicateEntry, Vec<DuplicateEntry>)> {
        &self.files_with_identical_size_names_referenced
    }
}

pub(crate) fn hash_calculation_limit(buffer: &mut [u8], file_entry: &DuplicateEntry, hash_type: HashType, limit: u64, size_counter: &Arc<AtomicU64>) -> Result<String, String> {
    // This function is used only to calculate hash of file with limit
    // We must ensure that buffer is big enough to store all data
    // We don't need to check that each time
    const_assert!(PREHASHING_BUFFER_SIZE <= THREAD_BUFFER_SIZE as u64);

    let mut file_handler = match File::open(&file_entry.path) {
        Ok(t) => t,
        Err(e) => {
            size_counter.fetch_add(limit, Ordering::Relaxed);
            return Err(flc!(
                "core_unable_check_hash_of_file",
                file = file_entry.path.to_string_lossy().to_string(),
                reason = e.to_string()
            ));
        }
    };
    let hasher = &mut *hash_type.hasher();
    #[expect(clippy::indexing_slicing)] // Safe, because limit is always <= buffer size
    let n = match file_handler.read(&mut buffer[..limit as usize]) {
        Ok(t) => t,
        Err(e) => return Err(flc!("core_error_checking_hash_of_file", file = file_entry.path.to_string_lossy(), reason = e.to_string())),
    };

    #[expect(clippy::indexing_slicing)] // Safe, because we read only n bytes, which is always <= limit <= buffer size
    hasher.update(&buffer[..n]);
    size_counter.fetch_add(n as u64, Ordering::Relaxed);
    Ok(hasher.finalize())
}

pub fn hash_calculation(
    buffer: &mut [u8],
    file_entry: &DuplicateEntry,
    hash_type: HashType,
    size_counter: &Arc<AtomicU64>,
    stop_flag: &Arc<AtomicBool>,
) -> Result<Option<String>, String> {
    let mut file_handler = match File::open(&file_entry.path) {
        Ok(t) => t,
        Err(e) => {
            size_counter.fetch_add(file_entry.size, Ordering::Relaxed);
            return Err(flc!("core_unable_check_hash_of_file", file = file_entry.path.to_string_lossy(), reason = e.to_string()));
        }
    };
    let hasher = &mut *hash_type.hasher();
    loop {
        let n = match file_handler.read(buffer) {
            Ok(0) => break,
            Ok(t) => t,
            Err(e) => return Err(flc!("core_error_checking_hash_of_file", file = file_entry.path.to_string_lossy(), reason = e.to_string())),
        };

        #[expect(clippy::indexing_slicing)] // Safe, because we read only n bytes, which is always <= buffer size
        hasher.update(&buffer[..n]);
        size_counter.fetch_add(n as u64, Ordering::Relaxed);
        if check_if_stop_received(stop_flag) {
            return Ok(None);
        }
    }
    Ok(Some(hasher.finalize()))
}

impl MyHasher for blake3::Hasher {
    fn update(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
    fn finalize(&self) -> String {
        self.finalize().to_hex().to_string()
    }
}

impl MyHasher for crc32fast::Hasher {
    fn update(&mut self, bytes: &[u8]) {
        self.write(bytes);
    }
    fn finalize(&self) -> String {
        self.finish().to_string()
    }
}

impl MyHasher for Xxh3 {
    fn update(&mut self, bytes: &[u8]) {
        self.write(bytes);
    }
    fn finalize(&self) -> String {
        self.finish().to_string()
    }
}

#[cfg(test)]
mod tests2 {
    use std::fs::File;
    use std::io;

    use super::*;
    use crate::common::model::FileEntry;
    use crate::tools::duplicate::filter_hard_links;

    #[test]
    fn test_filter_hard_links_empty() {
        let expected: Vec<FileEntry> = Default::default();
        assert_eq!(expected, filter_hard_links(Vec::new()));
    }

    #[cfg(target_family = "unix")]
    #[test]
    fn test_filter_hard_links() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        File::create(&src)?;
        fs::hard_link(src.clone(), dst.clone())?;
        let e1 = FileEntry { path: src, ..Default::default() };
        let e2 = FileEntry { path: dst, ..Default::default() };
        let actual = filter_hard_links(vec![e1.clone(), e2]);
        assert_eq!(vec![e1], actual);
        Ok(())
    }

    #[test]
    fn test_filter_hard_links_regular_files() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let (src, dst) = (dir.path().join("a"), dir.path().join("b"));
        File::create(&src)?;
        File::create(&dst)?;
        let e1 = FileEntry { path: src, ..Default::default() };
        let e2 = FileEntry { path: dst, ..Default::default() };
        let actual = filter_hard_links(vec![e1.clone(), e2.clone()]);
        assert_eq!(vec![e1, e2], actual);
        Ok(())
    }

    #[test]
    fn test_hash_calculation() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let mut buf = [0u8; 1 << 10];
        let src = dir.path().join("a");
        let mut file = File::create(&src)?;
        file.write_all(b"aaAAAAAAAAAAAAAAFFFFFFFFFFFFFFFFFFFFGGGGGGGGG")?;
        let e = DuplicateEntry { path: src, ..Default::default() };
        let size_counter = Arc::new(AtomicU64::new(0));
        let r = hash_calculation(&mut buf, &e, HashType::Blake3, &size_counter, &Arc::default())
            .expect("hash_calculation failed")
            .expect("hash_calculation returned None");
        assert!(!r.is_empty());
        assert_eq!(size_counter.load(Ordering::Relaxed), 45);
        Ok(())
    }

    #[test]
    fn test_hash_calculation_limit() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let mut buf = [0u8; 1000];
        let src = dir.path().join("a");
        let mut file = File::create(&src)?;
        file.write_all(b"aa")?;
        let e = DuplicateEntry { path: src, ..Default::default() };
        let size_counter_1 = Arc::new(AtomicU64::new(0));
        let size_counter_2 = Arc::new(AtomicU64::new(0));
        let size_counter_3 = Arc::new(AtomicU64::new(0));
        let r1 = hash_calculation_limit(&mut buf, &e, HashType::Blake3, 1, &size_counter_1).expect("hash_calculation failed");
        let r2 = hash_calculation_limit(&mut buf, &e, HashType::Blake3, 2, &size_counter_2).expect("hash_calculation failed");
        let r3 = hash_calculation_limit(&mut buf, &e, HashType::Blake3, 1000, &size_counter_3).expect("hash_calculation failed");
        assert_ne!(r1, r2);
        assert_eq!(r2, r3);

        assert_eq!(1, size_counter_1.load(Ordering::Relaxed));
        assert_eq!(2, size_counter_2.load(Ordering::Relaxed));
        assert_eq!(2, size_counter_3.load(Ordering::Relaxed));

        Ok(())
    }

    #[test]
    fn test_hash_calculation_invalid_file() -> io::Result<()> {
        let dir = tempfile::Builder::new().tempdir()?;
        let mut buf = [0u8; 1 << 10];
        let src = dir.path().join("a");
        let e = DuplicateEntry { path: src, ..Default::default() };
        let r = hash_calculation(&mut buf, &e, HashType::Blake3, &Arc::default(), &Arc::default()).expect_err("hash_calculation succeeded");
        assert!(!r.is_empty());
        Ok(())
    }
}
