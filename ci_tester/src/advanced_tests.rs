use std::fs;
use std::path::Path;
use std::process::Command;

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
        //  Hardlink regressions 
        // Bug: "Would hardlink file to itself"
        AdvancedTestCase { name: "hardlink_two_dirs",          run: test_hardlink_two_dirs },
        AdvancedTestCase { name: "hardlink_two_dirs_size_only",run: test_hardlink_two_dirs_size_only },
        // Bug: no hardlink performed when reference dir was given
        AdvancedTestCase { name: "hardlink_reference_dir",     run: test_hardlink_reference_dir },
        AdvancedTestCase { name: "hardlink_same_dir",          run: test_hardlink_same_dir },
        AdvancedTestCase { name: "hardlink_already_linked",    run: test_hardlink_already_linked },

        //  Manual-copy detection 
        AdvancedTestCase { name: "dup_detect_manual_copy",     run: test_dup_detect_manual_copy },

        //  Reference directory edge-cases 
        // Reference files must NEVER be deleted, regardless of -D method
        AdvancedTestCase { name: "ref_never_deleted_aeo",      run: test_ref_never_deleted_aeo },
        // When ALL duplicates live in reference dirs the scan dir is untouched
        AdvancedTestCase { name: "ref_all_dups_in_ref_no_scan_deletions",
                           run: test_ref_all_dups_in_ref_no_scan_deletions },
        // AEN applied only to non-reference copies; ref file always kept
        AdvancedTestCase { name: "ref_aen_keeps_newest_nonref",run: test_ref_aen_keeps_newest_nonref },
        // -r + -d with the SAME content: non-ref copy is replaced by hardlink to ref
        AdvancedTestCase { name: "ref_hardlink_replaces_nonref_copy",
                           run: test_ref_hardlink_replaces_nonref_copy },
        // Multiple -r dirs: non-ref files that match ANY reference are deleted
        AdvancedTestCase { name: "ref_multiple_ref_dirs",      run: test_ref_multiple_ref_dirs },

        //  Filtering 
        // -e excludes a whole directory; files inside must survive
        AdvancedTestCase { name: "dup_excluded_dir",           run: test_dup_excluded_dir },
        // -E glob pattern keeps matching files out of results
        AdvancedTestCase { name: "dup_excluded_glob",          run: test_dup_excluded_glob },
        // -R (non-recursive) must not descend into sub-directories
        AdvancedTestCase { name: "dup_non_recursive",          run: test_dup_non_recursive },

        //  Hash-mode correctness 
        // Default HASH mode must not treat same-size, different-content files as duplicates
        AdvancedTestCase { name: "dup_hash_no_false_positive", run: test_dup_hash_no_false_positive },
        // -s NAME mode: same file name, different content → treated as "duplicates" by name
        AdvancedTestCase { name: "dup_name_mode_same_name",    run: test_dup_name_mode_same_name },

        //  Reference + hardlink per checking method
        AdvancedTestCase { name: "ref_hardlink_hash",      run: test_ref_hardlink_hash },
        AdvancedTestCase { name: "ref_hardlink_size",      run: test_ref_hardlink_size },
        AdvancedTestCase { name: "ref_hardlink_name",      run: test_ref_hardlink_name },
        AdvancedTestCase { name: "ref_hardlink_size_name", run: test_ref_hardlink_size_name },

        //  Reference + delete per checking method
        AdvancedTestCase { name: "ref_delete_hash",      run: test_ref_delete_hash },
        AdvancedTestCase { name: "ref_delete_size",      run: test_ref_delete_size },
        AdvancedTestCase { name: "ref_delete_name",      run: test_ref_delete_name },
        AdvancedTestCase { name: "ref_delete_size_name", run: test_ref_delete_size_name },

        //  broken-files detection
        // A file with a .zip extension but garbage content must be detected
        AdvancedTestCase { name: "broken_invalid_archive",     run: test_broken_invalid_archive },
        // A structurally valid image that is completely truncated must be detected
        AdvancedTestCase { name: "broken_truncated_image",     run: test_broken_truncated_image },
    ]
}

//  helpers 

/// Sets the modification time of a file using `touch -t <ts>`.
/// `ts` format: `[[CC]YY]MMDDhhmm[.ss]`  (e.g. `"202001010000"` = 2020-01-01)
fn set_mtime(path: &str, ts: &str) -> Result<(), String> {
    let status = Command::new("touch")
        .args(["-t", ts, path])
        .status()
        .map_err(|e| format!("touch: {e}"))?;
    if !status.success() {
        return Err(format!("touch -t {ts} {path} failed"));
    }
    Ok(())
}

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

/// Returns 16 KiB of deterministic content that is unique per `tag`.
/// Files must exceed the default dup minimum size of 8 192 bytes to be
/// detected without `-m 1`.
fn make_content(tag: u8) -> Vec<u8> {
    (0..16_384usize).map(|i| tag ^ ((i & 0xff) as u8)).collect()
}

//  individual tests 

/// Regression test for: duplicates in two separate directories were
/// "hardlinked to themselves" instead of to each other.
///
/// Structure:
///   dir/a/file1.bin  ┐  (same content)
///   dir/b/file2.bin  ┘
///
/// Expected after `dup -D HARD -W`: both files exist and share the same inode.
fn test_hardlink_two_dirs(dir: &str) -> Result<(), String> {
    let content = make_content(1);
    let file_a = format!("{dir}/a/file1.bin");
    let file_b = format!("{dir}/b/file2.bin");
    write_file(&file_a, &content)?;
    write_file(&file_b, &content)?;

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
    let content = make_content(2);
    let file_a = format!("{dir}/a/file1.bin");
    let file_b = format!("{dir}/b/file2.bin");
    write_file(&file_a, &content)?;
    write_file(&file_b, &content)?;

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
    let content = make_content(3);
    let file_ref = format!("{dir}/ref_dir/original.bin");
    let file_dup = format!("{dir}/scan_dir/duplicate.bin");
    write_file(&file_ref, &content)?;
    write_file(&file_dup, &content)?;

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
///   dir/file1.bin  ┐  (same content)
///   dir/file2.bin  ┘
fn test_hardlink_same_dir(dir: &str) -> Result<(), String> {
    let content = make_content(4);
    let file1 = format!("{dir}/file1.bin");
    let file2 = format!("{dir}/file2.bin");
    write_file(&file1, &content)?;
    write_file(&file2, &content)?;

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
    let content = make_content(5);
    let file1 = format!("{dir}/file1.bin");
    let file2 = format!("{dir}/file2.bin");
    write_file(&file1, &content)?;
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

//  reference directory edge-cases 

/// Reference files are NEVER subject to deletion, regardless of the -D strategy.
 /// Furthermore, when a reference directory is active, czkawka unconditionally
/// deletes ALL non-reference duplicates (the deletion strategy such as AEO/AEN
/// is bypassed entirely for ref-mode groups).
///
/// Layout:
///   ref/master.bin          (old mtime – reference, always protected)
///   scan/old_copy.bin       (mtime 2021 – non-ref)
///   scan/new_copy.bin       (mtime 2024 – non-ref)
///
/// With -D AEO and a reference dir:
///   - ref/master.bin  → kept (reference, untouchable)
///   - scan/old_copy.bin → deleted (ALL non-ref copies removed unconditionally)
///   - scan/new_copy.bin → deleted (same)
fn test_ref_never_deleted_aeo(dir: &str) -> Result<(), String> {
    let content   = make_content(6);
    let ref_file  = format!("{dir}/ref/master.bin");
    let scan_old  = format!("{dir}/scan/old_copy.bin");
    let scan_new  = format!("{dir}/scan/new_copy.bin");

    write_file(&ref_file, &content)?;
    write_file(&scan_old, &content)?;
    write_file(&scan_new, &content)?;

    set_mtime(&ref_file, "202001010000")?;   // 2020 – oldest overall
    set_mtime(&scan_old, "202101010000")?;   // 2021
    set_mtime(&scan_new, "202401010000")?;   // 2024 – newest

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "AEO", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;    // ref: always kept
    ensure_missing(&scan_old)?;   // non-ref: unconditionally deleted in ref mode
    ensure_missing(&scan_new)     // non-ref: unconditionally deleted in ref mode
}

/// When every duplicate in a group lives in a reference directory, there are
/// no non-reference files to act on: the scan directory must be left intact.
///
/// Layout:
///   ref/copy_a.bin   ┐  (same content, both reference)
///   ref/copy_b.bin   ┘
///   scan/unique.bin      (different content – no duplicates outside ref)
fn test_ref_all_dups_in_ref_no_scan_deletions(dir: &str) -> Result<(), String> {
    let dup_content    = make_content(7);
    let unique_content = make_content(8);  // distinct content

    write_file(&format!("{dir}/ref/copy_a.bin"),  &dup_content)?;
    write_file(&format!("{dir}/ref/copy_b.bin"),  &dup_content)?;
    write_file(&format!("{dir}/scan/unique.bin"), &unique_content)?;

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "AEO", "-W"],
        false,
    )?;

    // Nothing in scan should be touched
    ensure_exists(&format!("{dir}/scan/unique.bin"))?;
    // Reference copies survive regardless
    ensure_exists(&format!("{dir}/ref/copy_a.bin"))?;
    ensure_exists(&format!("{dir}/ref/copy_b.bin"))
}

/// With a reference directory, ALL non-ref duplicates are unconditionally deleted,
/// even when a "keep one" strategy like AEN is used.  The strategy is bypassed
/// in ref mode; only HARD is handled specially (hardlink instead of delete).
///
/// Layout:
///   ref/master.bin           (mtime: 2024 – newest overall, but it's a ref)
///   scan/older_copy.bin      (mtime: 2021 – non-ref)
///   scan/newer_copy.bin      (mtime: 2023 – non-ref, newer among non-ref)
///
/// With -D AEN and a reference dir:
///   - ref/master.bin    → kept (reference, untouchable)
///   - scan/older_copy.bin → deleted (ALL non-ref copies removed unconditionally)
///   - scan/newer_copy.bin → deleted (same – NOT kept even though it's "newest non-ref")
fn test_ref_aen_keeps_newest_nonref(dir: &str) -> Result<(), String> {
    let content = make_content(9);
    let ref_file   = format!("{dir}/ref/master.bin");
    let scan_older = format!("{dir}/scan/older_copy.bin");
    let scan_newer = format!("{dir}/scan/newer_copy.bin");

    write_file(&ref_file,   &content)?;
    write_file(&scan_older, &content)?;
    write_file(&scan_newer, &content)?;

    set_mtime(&ref_file,   "202401010000")?;  // 2024 (ref)
    set_mtime(&scan_older, "202101010000")?;  // 2021
    set_mtime(&scan_newer, "202301010000")?;  // 2023 – newest non-ref

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "AEN", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;    // ref: never touched
    ensure_missing(&scan_older)?; // non-ref: unconditionally deleted in ref mode
    ensure_missing(&scan_newer)   // non-ref: unconditionally deleted in ref mode
}

/// With -r + -D HARD the non-reference copy should be hardlinked to the
/// reference original (extended coverage for the hardlink regression).
///
/// Layout:
///   ref/original.bin
///   scan/copy1.bin    (same content)
///   scan/copy2.bin    (same content)
///
/// After HARD: both scan copies share an inode with ref/original.
fn test_ref_hardlink_replaces_nonref_copy(dir: &str) -> Result<(), String> {
    let content = make_content(10);
    let ref_orig = format!("{dir}/ref/original.bin");
    let scan_c1  = format!("{dir}/scan/copy1.bin");
    let scan_c2  = format!("{dir}/scan/copy2.bin");

    write_file(&ref_orig, &content)?;
    write_file(&scan_c1,  &content)?;
    write_file(&scan_c2,  &content)?;

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&ref_orig)?;
    ensure_exists(&scan_c1)?;
    ensure_exists(&scan_c2)?;

    // Every scan copy must now point to the same inode as the reference
    check_same_inode(&ref_orig, &scan_c1)?;
    check_same_inode(&ref_orig, &scan_c2)
}

/// Multiple reference directories: non-ref files matching ANY reference are acted on.
///
/// Layout:
///   ref1/file_a.bin    (content A)
///   ref2/file_b.bin    (content B)
///   scan/dup_a.bin     (content A → duplicate of ref1/file_a)
///   scan/dup_b.bin     (content B → duplicate of ref2/file_b)
///   scan/unique.bin    (content C → no reference counterpart)
///
/// With -D OO (one oldest per group): one copy per group is deleted.
/// ref files are never deleted; scan/unique.bin has no duplicates → stays.
fn test_ref_multiple_ref_dirs(dir: &str) -> Result<(), String> {
    let content_a  = make_content(11);
    let content_b  = make_content(12);
    let content_c  = make_content(13); // unique

    write_file(&format!("{dir}/ref1/file_a.bin"),  &content_a)?;
    write_file(&format!("{dir}/ref2/file_b.bin"),  &content_b)?;
    write_file(&format!("{dir}/scan/dup_a.bin"),   &content_a)?;
    write_file(&format!("{dir}/scan/dup_b.bin"),   &content_b)?;
    write_file(&format!("{dir}/scan/unique.bin"),  &content_c)?;

    // Make ref files older so OO would "want" to delete them – but refs are
    // always preserved, so they must still be here after the run.
    set_mtime(&format!("{dir}/ref1/file_a.bin"), "202001010000")?;
    set_mtime(&format!("{dir}/ref2/file_b.bin"), "202001010000")?;
    set_mtime(&format!("{dir}/scan/dup_a.bin"),  "202401010000")?;
    set_mtime(&format!("{dir}/scan/dup_b.bin"),  "202401010000")?;

    let ref1_dir = format!("{dir}/ref1");
    let ref2_dir = format!("{dir}/ref2");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup",
          "-r", &ref1_dir, "-r", &ref2_dir,
          "-d", &scan_dir,
          "-H", "-D", "OO", "-W"],
        false,
    )?;

    // Reference files are always preserved
    ensure_exists(&format!("{dir}/ref1/file_a.bin"))?;
    ensure_exists(&format!("{dir}/ref2/file_b.bin"))?;

    // Non-ref duplicates of reference files must have been deleted (OO = one oldest,
    // but here refs are oldest → the non-ref copy IS the file that gets removed
    // because the group has only one non-ref entry which gets deleted by OO).
    ensure_missing(&format!("{dir}/scan/dup_a.bin"))?;
    ensure_missing(&format!("{dir}/scan/dup_b.bin"))?;

    // Unique file in scan directory must be untouched
    ensure_exists(&format!("{dir}/scan/unique.bin"))
}

//  filtering 

/// -e (excluded directory): files inside the excluded subtree must never be
/// processed, so they survive even when duplicates exist elsewhere.
///
/// Layout:
///   scan/active/file.bin    (content D)
///   scan/excluded/file.bin  (content D – excluded via -e)
///
/// Without the exclusion czkawka would see two duplicates and delete one.
/// With -e scan/excluded the excluded copy is invisible, leaving only one
/// file in the active area → no duplicates → no deletions.
fn test_dup_excluded_dir(dir: &str) -> Result<(), String> {
    let content  = make_content(14);
    let active   = format!("{dir}/scan/active/file.bin");
    let excluded = format!("{dir}/scan/excluded/file.bin");

    write_file(&active,   &content)?;
    write_file(&excluded, &content)?;

    let scan_dir     = format!("{dir}/scan");
    let excluded_dir = format!("{dir}/scan/excluded");
    let czkawka      = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-d", &scan_dir, "-e", &excluded_dir, "-H", "-D", "AEO", "-W"],
        false,
    )?;

    // Only one copy was visible → no duplicates → both files survive
    ensure_exists(&active)?;
    ensure_exists(&excluded)
}

/// -E (glob pattern exclusion): files whose path matches the pattern are
/// invisible to czkawka; the remaining copies form the deduplication pool.
///
/// Layout:
///   scan/keep/file.bin     (content E – excluded by pattern)
///   scan/trash1/file.bin   (content E)
///   scan/trash2/file.bin   (content E)
///
/// -E */keep/* hides scan/keep/file.bin.
/// czkawka sees only trash1 and trash2 → AEO: oldest kept, newer deleted.
fn test_dup_excluded_glob(dir: &str) -> Result<(), String> {
    let content = make_content(15);
    let keep    = format!("{dir}/scan/keep/file.bin");
    let trash1  = format!("{dir}/scan/trash1/file.bin");
    let trash2  = format!("{dir}/scan/trash2/file.bin");

    write_file(&keep,   &content)?;
    write_file(&trash1, &content)?;
    write_file(&trash2, &content)?;

    set_mtime(&trash1, "202101010000")?;  // older  → kept by AEO
    set_mtime(&trash2, "202401010000")?;  // newer  → deleted by AEO

    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-d", &scan_dir, "-E", "*/keep/*", "-H", "-D", "AEO", "-W"],
        false,
    )?;

    ensure_exists(&keep)?;    // excluded → untouched
    ensure_exists(&trash1)?;  // oldest visible copy → kept by AEO
    ensure_missing(&trash2)   // newer visible copy → deleted by AEO
}

/// -R (non-recursive) must not descend into sub-directories.
///
/// Layout:
///   scan/file.bin        (content F – at top level)
///   scan/sub/file.bin    (content F – in sub-directory, must be invisible)
///
/// With -R, only the top level is scanned: one copy seen → no duplicate → no deletion.
fn test_dup_non_recursive(dir: &str) -> Result<(), String> {
    let content = make_content(16);
    let top_file = format!("{dir}/scan/file.bin");
    let sub_file = format!("{dir}/scan/sub/file.bin");

    write_file(&top_file, &content)?;
    write_file(&sub_file, &content)?;

    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-d", &scan_dir, "-R", "-H", "-D", "AEO", "-W"],
        false,
    )?;

    // Sub-directory was not scanned → only one file visible → no deletion
    ensure_exists(&top_file)?;
    ensure_exists(&sub_file)
}

//  hash-mode correctness 

/// The default HASH mode must not produce false positives for files that
/// share the same size but have different content.
///
/// Layout:
///   a/file.bin   (content G – 16 KiB)
///   b/file.bin   (content H – 16 KiB, different content)
///
/// Same size, different hash → must NOT be treated as duplicates.
fn test_dup_hash_no_false_positive(dir: &str) -> Result<(), String> {
    let content_g = make_content(17);
    let content_h = make_content(18); // different content, same length

    let file_a = format!("{dir}/a/file.bin");
    let file_b = format!("{dir}/b/file.bin");
    write_file(&file_a, &content_g)?;
    write_file(&file_b, &content_h)?;

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-d", dir, "-H", "-D", "AEO", "-W"],
        false,
    )?;

    // No duplicate detected → both files must survive
    ensure_exists(&file_a)?;
    ensure_exists(&file_b)
}

/// With `-s NAME` czkawka groups files that share the same filename, regardless
/// of content.  Two identically-named files in different directories with
/// different content ARE detected as "duplicates" in NAME mode.
///
/// Layout:
///   a/report.bin   (content I – 16 KiB)
///   b/report.bin   (content J – 16 KiB, different)
///
/// -s NAME + -D ON (one newest) must delete the newer copy.
fn test_dup_name_mode_same_name(dir: &str) -> Result<(), String> {
    let content_i = make_content(19);
    let content_j = make_content(20);

    let file_a = format!("{dir}/a/report.bin");
    let file_b = format!("{dir}/b/report.bin");
    write_file(&file_a, &content_i)?;
    write_file(&file_b, &content_j)?;

    set_mtime(&file_a, "202101010000")?;  // older
    set_mtime(&file_b, "202401010000")?;  // newer → deleted by ON

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "NAME", "-d", dir, "-H", "-D", "ON", "-W"],
        false,
    )?;

    ensure_exists(&file_a)?;   // older → kept
    ensure_missing(&file_b)    // newer → deleted
}

//  Reference + hardlink per checking method
//
// Each test creates ref/file + scan/copy with the same content and verifies
// that `-D HARD` replaces the scan copy with a hardlink to the reference file.

/// HASH mode (default): same content → same hash → hardlink
fn test_ref_hardlink_hash(dir: &str) -> Result<(), String> {
    let content = make_content(30);
    let ref_file = format!("{dir}/ref/file.bin");
    let scan_dup = format!("{dir}/scan/dup.bin");
    write_file(&ref_file, &content)?;
    write_file(&scan_dup, &content)?;

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "HASH", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;
    ensure_exists(&scan_dup)?;
    check_same_inode(&ref_file, &scan_dup)?;
    check_nlink(&ref_file, 2)
}

/// SIZE mode: same size → hardlink (content may differ but we use identical content
/// to also ensure hardlink actually works)
fn test_ref_hardlink_size(dir: &str) -> Result<(), String> {
    let content = make_content(31);
    let ref_file = format!("{dir}/ref/file.bin");
    let scan_dup = format!("{dir}/scan/dup.bin");
    write_file(&ref_file, &content)?;
    write_file(&scan_dup, &content)?;

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "SIZE", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;
    ensure_exists(&scan_dup)?;
    check_same_inode(&ref_file, &scan_dup)?;
    check_nlink(&ref_file, 2)
}

/// NAME mode: same filename → hardlink.  Files live in different directories
/// so they can share a name.  Content differs (NAME doesn't care about content).
fn test_ref_hardlink_name(dir: &str) -> Result<(), String> {
    let content_a = make_content(32);
    let content_b = make_content(33);
    let ref_file = format!("{dir}/ref/shared.bin");
    let scan_dup = format!("{dir}/scan/shared.bin");
    write_file(&ref_file, &content_a)?;
    write_file(&scan_dup, &content_b)?;

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "NAME", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;
    ensure_exists(&scan_dup)?;
    check_same_inode(&ref_file, &scan_dup)?;
    check_nlink(&ref_file, 2)
}

/// SIZE_NAME mode: same size + same filename → hardlink
fn test_ref_hardlink_size_name(dir: &str) -> Result<(), String> {
    let content = make_content(34);
    let ref_file = format!("{dir}/ref/shared.bin");
    let scan_dup = format!("{dir}/scan/shared.bin");
    write_file(&ref_file, &content)?;
    write_file(&scan_dup, &content)?;

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "SIZE_NAME", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "HARD", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;
    ensure_exists(&scan_dup)?;
    check_same_inode(&ref_file, &scan_dup)?;
    check_nlink(&ref_file, 2)
}

//  Reference + delete per checking method
//
// Each test creates ref/master + scan/copy1 + scan/copy2 and verifies that
// `-D AEO` with a reference directory deletes ALL non-reference copies
// while keeping the reference file.

fn test_ref_delete_hash(dir: &str) -> Result<(), String> {
    let content  = make_content(35);
    let ref_file = format!("{dir}/ref/master.bin");
    let scan_c1  = format!("{dir}/scan/copy1.bin");
    let scan_c2  = format!("{dir}/scan/copy2.bin");
    write_file(&ref_file, &content)?;
    write_file(&scan_c1,  &content)?;
    write_file(&scan_c2,  &content)?;

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "HASH", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "AEO", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;
    ensure_missing(&scan_c1)?;
    ensure_missing(&scan_c2)
}

fn test_ref_delete_size(dir: &str) -> Result<(), String> {
    let content  = make_content(36);
    let ref_file = format!("{dir}/ref/master.bin");
    let scan_c1  = format!("{dir}/scan/copy1.bin");
    let scan_c2  = format!("{dir}/scan/copy2.bin");
    write_file(&ref_file, &content)?;
    write_file(&scan_c1,  &content)?;
    write_file(&scan_c2,  &content)?;

    let ref_dir  = format!("{dir}/ref");
    let scan_dir = format!("{dir}/scan");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "SIZE", "-r", &ref_dir, "-d", &scan_dir, "-H", "-D", "AEO", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;
    ensure_missing(&scan_c1)?;
    ensure_missing(&scan_c2)
}

fn test_ref_delete_name(dir: &str) -> Result<(), String> {
    let content_a = make_content(37);
    let content_b = make_content(38);
    let content_c = make_content(39);
    // NAME mode groups by filename; all three share the name "shared.bin"
    let ref_file = format!("{dir}/ref/shared.bin");
    let scan_c1  = format!("{dir}/scan1/shared.bin");
    let scan_c2  = format!("{dir}/scan2/shared.bin");
    write_file(&ref_file, &content_a)?;
    write_file(&scan_c1,  &content_b)?;
    write_file(&scan_c2,  &content_c)?;

    let ref_dir  = format!("{dir}/ref");
    let scan1    = format!("{dir}/scan1");
    let scan2    = format!("{dir}/scan2");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "NAME", "-r", &ref_dir, "-d", &scan1, "-d", &scan2, "-H", "-D", "AEO", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;
    ensure_missing(&scan_c1)?;
    ensure_missing(&scan_c2)
}

fn test_ref_delete_size_name(dir: &str) -> Result<(), String> {
    let content  = make_content(40);
    // SIZE_NAME groups by (size, name); all three share the same size+name
    let ref_file = format!("{dir}/ref/shared.bin");
    let scan_c1  = format!("{dir}/scan1/shared.bin");
    let scan_c2  = format!("{dir}/scan2/shared.bin");
    write_file(&ref_file, &content)?;
    write_file(&scan_c1,  &content)?;
    write_file(&scan_c2,  &content)?;

    let ref_dir  = format!("{dir}/ref");
    let scan1    = format!("{dir}/scan1");
    let scan2    = format!("{dir}/scan2");
    let czkawka  = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "dup", "-s", "SIZE_NAME", "-r", &ref_dir, "-d", &scan1, "-d", &scan2, "-H", "-D", "AEO", "-W"],
        false,
    )?;

    ensure_exists(&ref_file)?;
    ensure_missing(&scan_c1)?;
    ensure_missing(&scan_c2)
}

//  broken-file detection

/// A file with a .zip extension but garbage content must be detected by
/// `broken -c ARCHIVE` and deleted with `-D`.
fn test_broken_invalid_archive(dir: &str) -> Result<(), String> {
    let corrupt_zip = format!("{dir}/corrupt.zip");
    // A valid ZIP starts with the PK magic (0x50 0x4B 0x03 0x04); this doesn't.
    write_file(&corrupt_zip, b"THIS_IS_NOT_A_VALID_ZIP_FILE_AT_ALL_GARBAGE_CONTENT_PADDING")?;

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "broken", "-d", dir, "-H", "-c", "ARCHIVE", "-D", "-W"],
        false,
    )?;

    ensure_missing(&corrupt_zip)
}

/// A file named .jpg that contains zero valid image data (all zeroes, much too
/// short) must be detected by `broken -c IMAGE` and deleted.
fn test_broken_truncated_image(dir: &str) -> Result<(), String> {
    let bad_jpg = format!("{dir}/truncated.jpg");
    // A real JPEG starts with FF D8 FF; 4 zero bytes will never decode
    write_file(&bad_jpg, &[0u8; 4])?;

    let czkawka = crate::CZKAWKA_PATH.get().as_str();
    crate::run_with_good_status(
        &[czkawka, "broken", "-d", dir, "-H", "-c", "IMAGE", "-D", "-W"],
        false,
    )?;

    ensure_missing(&bad_jpg)
}
