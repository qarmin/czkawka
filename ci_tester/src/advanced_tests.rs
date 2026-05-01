use std::fs;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

pub struct AdvancedTestCase {
    pub name: &'static str,
    /// Receives the isolated test directory. Responsible for creating its own
    /// file structure, running czkawka, and asserting results.
    /// Cleanup is handled by the caller.
    pub run: fn(&str) -> Result<(), String>,
}

pub fn all_advanced_test_cases() -> Vec<AdvancedTestCase> {
    vec![
        // Regression: "Would hardlink file to itself" bug (GitHub issue)
        AdvancedTestCase {
            name: "hardlink_two_dirs",
            run: test_hardlink_two_dirs,
        },
        AdvancedTestCase {
            name: "hardlink_two_dirs_size_only",
            run: test_hardlink_two_dirs_size_only,
        },
        // Regression: hardlink with reference dir produced no hardlink
        AdvancedTestCase {
            name: "hardlink_reference_dir",
            run: test_hardlink_reference_dir,
        },
        // Basic: two copies in the same directory
        AdvancedTestCase {
            name: "hardlink_same_dir",
            run: test_hardlink_same_dir,
        },
        // Re-running HARD on already-hardlinked files must not error
        AdvancedTestCase {
            name: "hardlink_already_linked",
            run: test_hardlink_already_linked,
        },
        // Manually copy a test file and verify czkawka detects + deletes it
        AdvancedTestCase {
            name: "dup_detect_manual_copy",
            run: test_dup_detect_manual_copy,
        },
    ]
}

// ─── helpers ─────────────────────────────────────────────────────────────────

fn write_file(path: &str, content: &[u8]) -> Result<(), String> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("create_dir_all({parent:?}): {e}"))?;
    }
    fs::write(path, content).map_err(|e| format!("write({path}): {e}"))
}

fn ensure_exists(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        Err(format!("'{path}' does not exist but should"))
    } else {
        Ok(())
    }
}

fn ensure_missing(path: &str) -> Result<(), String> {
    if Path::new(path).exists() {
        Err(format!("'{path}' exists but should have been removed"))
    } else {
        Ok(())
    }
}

#[cfg(unix)]
fn check_same_inode(path_a: &str, path_b: &str) -> Result<(), String> {
    let ino_a = fs::metadata(path_a)
        .map_err(|e| format!("metadata({path_a}): {e}"))?.ino();
    let ino_b = fs::metadata(path_b)
        .map_err(|e| format!("metadata({path_b}): {e}"))?.ino();
    if ino_a != ino_b {
        Err(format!(
            "Hardlink not created: '{path_a}' (inode {ino_a}) and '{path_b}' (inode {ino_b}) differ"
        ))
    } else {
        Ok(())
    }
}

#[cfg(unix)]
fn check_nlink(path: &str, expected: u64) -> Result<(), String> {
    let nlink = fs::metadata(path)
        .map_err(|e| format!("metadata({path}): {e}"))?.nlink();
    if nlink != expected {
        Err(format!("'{path}' has nlink={nlink}, expected {expected}"))
    } else {
        Ok(())
    }
}

#[cfg(not(unix))]
fn check_same_inode(_a: &str, _b: &str) -> Result<(), String> { Ok(()) }
#[cfg(not(unix))]
fn check_nlink(_path: &str, _expected: u64) -> Result<(), String> { Ok(()) }

// Unique content per test so we don't accidentally match other files.
const CONTENT_HL_TWO_DIRS: &[u8]       = b"ci_tester:hardlink_two_dirs:v1 - unique payload 111111111";
const CONTENT_HL_SIZE_ONLY: &[u8]      = b"ci_tester:hardlink_size_only:v1 - unique payload 222222222";
const CONTENT_HL_REF: &[u8]            = b"ci_tester:hardlink_ref_dir:v1 - unique payload 333333333";
const CONTENT_HL_SAME_DIR: &[u8]       = b"ci_tester:hardlink_same_dir:v1 - unique payload 444444444";
const CONTENT_HL_ALREADY: &[u8]        = b"ci_tester:hardlink_already:v1  - unique payload 555555555";

// ─── individual tests ─────────────────────────────────────────────────────────

/// Regression test for: duplicates in two separate directories were
/// "hardlinked to themselves" instead of to each other.
///
/// Structure:
///   dir/a/file1.bin  ─┐  (same content)
///   dir/b/file2.bin  ─┘
///
/// Expected after `dup -D HARD -W`: both files exist and share the same inode.
fn test_hardlink_two_dirs(dir: &str) -> Result<(), String> {
    let file_a = format!("{dir}/a/file1.bin");
    let file_b = format!("{dir}/b/file2.bin");
    write_file(&file_a, CONTENT_HL_TWO_DIRS)?;
    write_file(&file_b, CONTENT_HL_TWO_DIRS)?;

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-d", dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&file_a)?;
    ensure_exists(&file_b)?;
    check_same_inode(&file_a, &file_b)?;
    check_nlink(&file_a, 2)
}

/// Same scenario as above but using SIZE-only comparison (`-s SIZE`).
/// This matches the exact flags used in the original bug report.
fn test_hardlink_two_dirs_size_only(dir: &str) -> Result<(), String> {
    let file_a = format!("{dir}/a/file1.bin");
    let file_b = format!("{dir}/b/file2.bin");
    write_file(&file_a, CONTENT_HL_SIZE_ONLY)?;
    write_file(&file_b, CONTENT_HL_SIZE_ONLY)?;

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "SIZE", "-d", dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&file_a)?;
    ensure_exists(&file_b)?;
    check_same_inode(&file_a, &file_b)?;
    check_nlink(&file_a, 2)
}

/// Regression test for: no hardlink was performed when a reference directory
/// was specified (`-r ref_dir -d scan_dir -D HARD`).
///
/// Structure:
///   dir/ref_dir/original.bin  (reference – must not be deleted)
///   dir/scan_dir/duplicate.bin (scan target – should be hardlinked to original)
///
/// Expected: duplicate.bin gets the same inode as original.bin.
fn test_hardlink_reference_dir(dir: &str) -> Result<(), String> {
    let file_ref = format!("{dir}/ref_dir/original.bin");
    let file_dup = format!("{dir}/scan_dir/duplicate.bin");
    write_file(&file_ref, CONTENT_HL_REF)?;
    write_file(&file_dup, CONTENT_HL_REF)?;

    let ref_dir  = format!("{dir}/ref_dir");
    let scan_dir = format!("{dir}/scan_dir");

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&file_ref)?;
    ensure_exists(&file_dup)?;
    check_same_inode(&file_ref, &file_dup)?;
    check_nlink(&file_ref, 2)
}

/// Two copies of the same file in a single directory.
///
/// Structure:
///   dir/file1.bin  ─┐  (same content)
///   dir/file2.bin  ─┘
fn test_hardlink_same_dir(dir: &str) -> Result<(), String> {
    let file1 = format!("{dir}/file1.bin");
    let file2 = format!("{dir}/file2.bin");
    write_file(&file1, CONTENT_HL_SAME_DIR)?;
    write_file(&file2, CONTENT_HL_SAME_DIR)?;

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-d", dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&file1)?;
    ensure_exists(&file2)?;
    check_same_inode(&file1, &file2)?;
    check_nlink(&file1, 2)
}

/// Re-running `dup -D HARD` on files that are already hardlinked must succeed
/// without errors and must not change the inode count.
fn test_hardlink_already_linked(dir: &str) -> Result<(), String> {
    let file1 = format!("{dir}/file1.bin");
    let file2 = format!("{dir}/file2.bin");
    write_file(&file1, CONTENT_HL_ALREADY)?;
    // Create file2 as a hardlink of file1 from the start
    fs::hard_link(&file1, &file2).map_err(|e| format!("hard_link: {e}"))?;

    check_same_inode(&file1, &file2)?;
    check_nlink(&file1, 2)?;

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    // Running again must not error out
    crate::run_with_good_status(
        &[czkawka, "dup", "-d", dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&file1)?;
    ensure_exists(&file2)?;
    check_same_inode(&file1, &file2)?;
    check_nlink(&file1, 2)
}

/// Manually copy a file from the standard TestFiles set and confirm that
/// czkawka's duplicate finder detects and removes it.
///
/// A1_copy.jpg is created with the current timestamp, making it the newest
/// member of the A1/A2/A5 duplicate group. `dup -D ON` (one newest per group)
/// must therefore delete A1_copy.jpg while leaving the originals untouched.
fn test_dup_detect_manual_copy(dir: &str) -> Result<(), String> {
    crate::unzip_files(dir)?;

    let original = format!("{dir}/Images/A1.jpg");
    let copy     = format!("{dir}/Images/A1_copy.jpg");
    fs::copy(&original, &copy).map_err(|e| format!("copy {original} → {copy}: {e}"))?;

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-d", dir, "-H", "-D", "ON", "-W"],
        false,
    )?;

    // The copy (newest) must have been removed
    ensure_missing(&copy)?;
    // The original must still be present
    ensure_exists(&original)
}
