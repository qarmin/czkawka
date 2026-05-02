//! In-memory representation of the test file tree.
//!
//! Instead of shelling out to `unzip` we read TestFiles.zip once at startup,
//! hold every entry as raw bytes, and write them to disk on demand via
//! `default_test_files()`.  Adding new test assets is therefore as simple as:
//!
//!   * putting the file into TestFiles.zip, **or**
//!   * appending an entry to `add_synthetic_entries()` below.
//!
//! Rust decides which files go where – no external tooling is required.
//!
//! # File-size and mtime contract
//!
//! The sizes and modification timestamps of the bundled files drive several
//! test expectations.  The layout is intentional:
//!
//! ## Duplicate / similar-image groups (same byte content)
//! | File            | Size (B) | mtime (UTC approx)     | Role in tests          |
//! |-----------------|----------|------------------------|------------------------|
//! | Images/A1.jpg   | 8 772    | 2023-10-12 15:36       | newest in image group  |
//! | Images/A2.jpg   | 8 772    | 2023-10-12 15:21       | oldest jpg (tied A5)   |
//! | Images/A3.png   | 20 920   | 2023-10-11 19:47       | overall oldest, biggest|
//! | Images/A5.jpg   | 8 772    | 2023-10-12 15:21       | same mtime as A2       |
//!
//! ## Music duplicate group (M1 = M2 = M5 bytes)
//! | File            | Size (B)  | mtime (UTC approx)    | Role in tests          |
//! |-----------------|-----------|------------------------|------------------------|
//! | Music/M1.mp3    | 43 730    | 2023-10-12 15:23       | middle                 |
//! | Music/M2.mp3    | 43 730    | 2023-10-12 15:48       | newest                 |
//! | Music/M3.flac   | 450 079   | 2023-10-11 19:51       | not a dup, used by tags|
//! | Music/M5.mp3    | 43 730    | 2022-05-07 08:06       | oldest                 |
//!
//! ## Similar-video group (V1 = V2 = V5 bytes, V3 perceptually similar)
//! | File            | Size (B)  | mtime (UTC approx)    | Role in tests          |
//! |-----------------|-----------|------------------------|------------------------|
//! | Videos/V1.mp4   | 435 803   | 2023-10-12 15:31       | middle                 |
//! | Videos/V2.mp4   | 435 803   | 2023-10-12 15:22       | oldest among V1/V2/V5  |
//! | Videos/V3.webm  | 471 868   | 2006-07-24 03:21       | overall oldest (dup group) |
//! | Videos/V5.mp4   | 435 803   | 2023-10-12 15:48       | newest                 |

use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::Path;

use crate::CollectedFiles;

//  data types 

pub(crate) enum EntryKind {
    Dir,
    File,
    Symlink,
}

pub(crate) struct TestFileEntry {
    /// Relative path inside the test tree (no leading separator).
    pub path: String,
    pub kind: EntryKind,
    /// For `File`: raw file bytes.  For `Symlink`: the symlink target as UTF-8.
    /// Empty for `Dir`.
    pub data: Vec<u8>,
    /// Unix timestamp (seconds since epoch) from the zip's stored mtime.
    /// Applied to the file after writing so czkawka's time-based deletion
    /// strategies produce deterministic results.  `None` for synthetic entries
    /// that intentionally get the current timestamp.
    pub mtime_unix_secs: Option<u64>,
}

//  loading 

/// Read every entry from the zip and append additional synthetic entries.
/// Returns the complete list used by both `default_test_files` and
/// `baseline_from_entries`.
pub(crate) fn load_all_test_entries(zip_path: &str) -> Vec<TestFileEntry> {
    let mut entries = load_zip(zip_path);
    validate_known_file_sizes(&entries);
    add_synthetic_entries(&mut entries);
    entries
}

fn load_zip(zip_path: &str) -> Vec<TestFileEntry> {
    let file = fs::File::open(zip_path).unwrap_or_else(|e| panic!("Cannot open {zip_path}: {e}"));
    let mut archive = zip::ZipArchive::new(file).unwrap_or_else(|e| panic!("Cannot parse {zip_path}: {e}"));

    let mut entries = Vec::with_capacity(archive.len());

    for i in 0..archive.len() {
        let mut zf = archive.by_index(i).unwrap();

        // Zip stores directories with a trailing '/'; normalise it away.
        let raw = zf.name().to_owned();
        let path = raw.trim_end_matches('/').to_owned();
        if path.is_empty() {
            continue;
        }

        let mtime_unix_secs = zf.last_modified().and_then(zip_dt_to_unix_secs);

        // On Unix, symlinks are stored with file-type bits 0xA000 (S_IFLNK).
        let is_symlink = zf.unix_mode().map(|m| (m & 0xF000) == 0xA000).unwrap_or(false);

        if is_symlink {
            let mut data = Vec::new();
            zf.read_to_end(&mut data).unwrap();
            entries.push(TestFileEntry { path, kind: EntryKind::Symlink, data, mtime_unix_secs });
        } else if zf.is_dir() {
            entries.push(TestFileEntry { path, kind: EntryKind::Dir, data: vec![], mtime_unix_secs });
        } else {
            let mut data = Vec::new();
            zf.read_to_end(&mut data).unwrap();
            entries.push(TestFileEntry { path, kind: EntryKind::File, data, mtime_unix_secs });
        }
    }

    entries
}

/// Convert a `zip::DateTime` (Gregorian date/time, treated as UTC) into a
/// Unix timestamp (seconds since 1970-01-01 00:00:00 UTC).
///
/// Returns `None` when the stored date is clearly invalid (year < 1980, which
/// is the minimum possible in the ZIP/DOS timestamp format).
fn zip_dt_to_unix_secs(dt: zip::DateTime) -> Option<u64> {
    let y = dt.year() as i64;
    if y < 1980 {
        // ZIP epoch; file was stored without a real timestamp.
        return None;
    }
    let m = dt.month() as i64;
    let d = dt.day() as i64;
    let h = dt.hour() as i64;
    let mi = dt.minute() as i64;
    let s = dt.second() as i64;

    // Compute the Julian Day Number using the standard proleptic Gregorian formula.
    let a = (14 - m) / 12;
    let y2 = y + 4800 - a;
    let m2 = m + 12 * a - 3;
    let jdn = d + (153 * m2 + 2) / 5 + 365 * y2 + y2 / 4 - y2 / 100 + y2 / 400 - 32045;

    // JDN of the Unix epoch (1970-01-01) is 2440588.
    let days_since_epoch = jdn - 2440588;
    Some((days_since_epoch * 86400 + h * 3600 + mi * 60 + s) as u64)
}

/// Validate that key files in the loaded zip have the sizes documented in the
/// module header.  This acts as both documentation and a guard against an
/// accidentally corrupt or wrong version of TestFiles.zip.
fn validate_known_file_sizes(entries: &[TestFileEntry]) {
    /// (relative path, expected size in bytes)
    const KNOWN_SIZES: &[(&str, usize)] = &[
        //  Images 
        // A1 = A2 = A5 (identical bytes, 8 772 B each).
        // A3.png is perceptually similar to A1/A2/A5 (same visual content).
        ("Images/A1.jpg", 8772),
        ("Images/A2.jpg", 8772),
        ("Images/A3.png", 20920),
        ("Images/A4.jpg", 25105),
        ("Images/A5.jpg", 8772),
        //  Music 
        // M1 = M2 = M5 (identical bytes, 43 730 B each).
        ("Music/M1.mp3", 43730),
        ("Music/M2.mp3", 43730),
        ("Music/M3.flac", 450079),
        ("Music/M4.mp3", 3754941),
        ("Music/M5.mp3", 43730),
        //  Videos 
        // V1 = V2 = V5 (identical bytes, 435 803 B each).
        // V3.webm is perceptually similar (same visual content, different format).
        ("Videos/V1.mp4", 435803),
        ("Videos/V2.mp4", 435803),
        ("Videos/V3.webm", 471868),
        ("Videos/V4.mp4", 323521),
        ("Videos/V5.mp4", 435803),
        //  Bad extension 
        // BE.jpg is actually an MP4 (ftypisom magic bytes).
        ("BadExtensions/BE.jpg", 126240),
        //  Broken files 
        ("Broken/Br.jpg", 6),
        ("Broken/Br.mp3", 7),
        ("Broken/Br.pdf", 7),
        ("Broken/Br.zip", 9),
    ];

    let file_map: std::collections::HashMap<&str, usize> = entries
        .iter()
        .filter_map(|e| match e.kind {
            EntryKind::File => Some((e.path.as_str(), e.data.len())),
            _ => None,
        })
        .collect();

    for &(path, expected_size) in KNOWN_SIZES {
        let actual = file_map.get(path).copied().unwrap_or_else(|| {
            panic!("TestFiles.zip is missing expected file '{path}'")
        });
        assert_eq!(
            actual, expected_size,
            "TestFiles.zip: '{path}' has size {actual} B but expected {expected_size} B – wrong zip version?"
        );
    }
}

/// Extend the entry list with files that are NOT in TestFiles.zip but are
/// needed for specific test cases.  This is the preferred way to add small,
/// purpose-built test assets without touching the zip.
fn add_synthetic_entries(entries: &mut Vec<TestFileEntry>) {
    //  Bad-names test assets 
    // A JPEG file whose extension is all-uppercase (.JPG).  The `bad-names -u`
    // flag will detect and rename it to photo.jpg, letting the test verify that
    // BadNames/photo.JPG disappears from the directory tree.
    //
    // mtime_unix_secs = None so the file gets the current timestamp, which is
    // fine – bad-names checks the *name*, not the modification time.
    entries.push(TestFileEntry {
        path: "BadNames".to_string(),
        kind: EntryKind::Dir,
        data: vec![],
        mtime_unix_secs: None,
    });
    entries.push(TestFileEntry {
        path: "BadNames/photo.JPG".to_string(),
        kind: EntryKind::File,
        // Minimal valid JPEG SOI/APP0/EOI triplet (bad-names checks name, not content).
        data: b"\xff\xd8\xff\xe0\x00\x10JFIF\x00\x01\x01\x00\x00\x01\x00\x01\x00\x00\xff\xd9".to_vec(),
        mtime_unix_secs: None,
    });
}

//  baseline 

/// Derive the `CollectedFiles` baseline directly from the in-memory entry list.
/// This avoids the write-then-scan cycle that was previously needed.
pub(crate) fn baseline_from_entries(entries: &[TestFileEntry]) -> CollectedFiles {
    let mut files = BTreeSet::new();
    let mut folders = BTreeSet::new();
    let mut symlinks = BTreeSet::new();

    for e in entries {
        match e.kind {
            EntryKind::Dir => {
                folders.insert(e.path.clone());
            }
            EntryKind::File => {
                files.insert(e.path.clone());
            }
            EntryKind::Symlink => {
                symlinks.insert(e.path.clone());
            }
        }
    }

    CollectedFiles { files, folders, symlinks }
}

//  writing to disk 

/// Recreate the full test directory tree under `dir`.
///
/// This is the Rust-native replacement for `unzip -qq -X TestFiles.zip -d <dir>`.
/// Both zip-based and synthetic entries are written, so the on-disk layout is
/// identical to what `baseline_from_entries` describes.
///
/// The `-X` flag of the old `unzip` command preserved modification timestamps;
/// we replicate that behaviour by calling `filetime::set_file_mtime` after
/// each file write.  This is critical for tests that rely on time-based
/// deletion strategies (OO / ON / AEO / AEN).
pub(crate) fn default_test_files(dir: &str, entries: &[TestFileEntry]) -> Result<(), String> {
    for entry in entries {
        let full = format!("{dir}/{}", entry.path);
        match entry.kind {
            EntryKind::Dir => {
                fs::create_dir_all(&full).map_err(|e| format!("mkdir {full}: {e}"))?;
            }
            EntryKind::File => {
                if let Some(parent) = Path::new(&full).parent() {
                    fs::create_dir_all(parent).map_err(|e| format!("mkdir {parent:?}: {e}"))?;
                }
                fs::write(&full, &entry.data).map_err(|e| format!("write {full}: {e}"))?;

                // Restore the original mtime so that time-ordering tests are deterministic.
                if let Some(secs) = entry.mtime_unix_secs {
                    let ft = filetime::FileTime::from_unix_time(secs as i64, 0);
                    filetime::set_file_mtime(&full, ft)
                        .map_err(|e| format!("set_file_mtime {full}: {e}"))?;
                }
            }
            EntryKind::Symlink => {
                if let Some(parent) = Path::new(&full).parent() {
                    fs::create_dir_all(parent).map_err(|e| format!("mkdir {parent:?}: {e}"))?;
                }
                // The symlink target is stored as raw bytes in the zip.
                let target = std::str::from_utf8(&entry.data)
                    .map_err(|e| format!("symlink target utf8 {full}: {e}"))?
                    .trim_end_matches('\0');

                #[cfg(unix)]
                std::os::unix::fs::symlink(target, &full)
                    .map_err(|e| format!("symlink {target:?} → {full}: {e}"))?;

                #[cfg(not(unix))]
                {
                    // Symlinks are Unix-only; create an empty placeholder on Windows.
                    let _ = target;
                    fs::write(&full, b"").map_err(|e| format!("write stub {full}: {e}"))?;
                }
            }
        }
    }
    Ok(())
}
