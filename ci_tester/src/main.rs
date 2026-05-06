mod advanced_tests;
mod test_cases;
mod test_file_system;

use advanced_tests::{AdvancedTestCase, all_advanced_test_cases};
use test_cases::{TestCase, all_test_cases};
use test_file_system::TestFileEntry;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::process::{Command, Output, Stdio};
use std::env;
use log::info;
use rayon::prelude::*;

//  shared state 

#[derive(Default, Clone, Debug)]
pub(crate) struct CollectedFiles {
    pub(crate) files: BTreeSet<String>,
    pub(crate) folders: BTreeSet<String>,
    pub(crate) symlinks: BTreeSet<String>,
}

pub(crate) static CZKAWKA_PATH: state::InitCell<String> = state::InitCell::new();
static COLLECTED_FILES: state::InitCell<CollectedFiles> = state::InitCell::new();
/// All test file entries held in memory – both zip-derived and synthetic.
/// `unzip_files()` reads from this to recreate the tree on disk.
static ZIP_ENTRIES: state::InitCell<Vec<TestFileEntry>> = state::InitCell::new();

const ATTEMPTS: u32 = 5;

struct Failure {
    name: String,
    error: String,
}

//  entry point 

// App runs - ./ci_tester PATH_TO_CZKAWKA
fn main() {
    handsome_logger::init().expect("Should not fail in tests");
    let args: Vec<String> = env::args().collect();
    let path_to_czkawka = args[1].clone();
    CZKAWKA_PATH.set(path_to_czkawka);

    test_args();

    // Load the zip into memory once, add any synthetic entries, then derive
    // the baseline from the in-memory list – no on-disk extraction needed here.
    let mut entries = test_file_system::load_all_test_entries("TestFiles.zip");
    // Sort for deterministic baseline ordering.
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    let baseline = test_file_system::baseline_from_entries(&entries);
    COLLECTED_FILES.set(baseline);
    ZIP_ENTRIES.set(entries);

    let test_cases     = all_test_cases();
    let advanced_cases = all_advanced_test_cases();

    println!(
        "Running {} standard test cases + {} advanced tests × {} attempts...",
        test_cases.len(),
        advanced_cases.len(),
        ATTEMPTS,
    );

    let mut all_failures: Vec<Failure> = Vec::new();

    for attempt in 0..ATTEMPTS {
        let standard_failures: Vec<Failure> = test_cases
            .par_iter()
            .filter_map(|tc| {
                info!("[{}/{}] {}", attempt + 1, ATTEMPTS, tc.name);
                run_standard_test(tc).err().map(|e| Failure {
                    name: tc.name.to_string(),
                    error: e,
                })
            })
            .collect();

        let advanced_failures: Vec<Failure> = advanced_cases
            .par_iter()
            .filter_map(|tc| {
                info!("[{}/{}] adv:{}", attempt + 1, ATTEMPTS, tc.name);
                run_advanced_test(tc).err().map(|e| Failure {
                    name: format!("adv:{}", tc.name),
                    error: e,
                })
            })
            .collect();

        let n_fail = standard_failures.len() + advanced_failures.len();
        let n_total = test_cases.len() + advanced_cases.len();
        println!(
            "Attempt {}/{}: {}/{} passed",
            attempt + 1, ATTEMPTS,
            n_total - n_fail, n_total,
        );

        all_failures.extend(standard_failures);
        all_failures.extend(advanced_failures);
    }

    if all_failures.is_empty() {
        println!(
            "\nAll {} tests passed ({} attempts).",
            test_cases.len() + advanced_cases.len(),
            ATTEMPTS,
        );
    } else {
        // Group by test name so repeated failures across attempts are shown once.
        let mut by_name: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for f in all_failures {
            by_name.entry(f.name).or_default().push(f.error);
        }

        let total_failures: usize = by_name.values().map(|v| v.len()).sum();
        eprintln!(
            "\n====== {} UNIQUE FAILURE(S) ({} total across {} attempts) ======",
            by_name.len(), total_failures, ATTEMPTS,
        );
        for (name, errors) in &by_name {
            eprintln!("  [{}/{}] {}", errors.len(), ATTEMPTS, name);
            // Indent the first error message as an example
            let example = errors[0].replace('\n', "\n      ");
            eprintln!("      {example}");
            // Warn if errors varied across attempts
            let unique: BTreeSet<_> = errors.iter().collect();
            if unique.len() > 1 {
                eprintln!("      ({} distinct error messages across attempts)", unique.len());
            }
        }
        std::process::exit(1);
    }
}

fn test_args() {
    let modes = [
        "dup", "big", "empty-folders", "empty-files", "temp", "image",
        "symlinks", "broken", "ext", "video", "music",
    ];
    for mode in modes {
        info!("Testing mode args: {mode}");
        let _ = fs::remove_dir_all("RandomDirWithoutContent");
        fs::create_dir_all("RandomDirWithoutContent").expect("Should not fail in tests");
        run_with_good_status(
            &[CZKAWKA_PATH.get().as_str(), mode, "-d", "RandomDirWithoutContent", "-H", "-W"],
            false,
        )
        .expect("test_args failed");
    }
    let _ = fs::remove_dir_all("RandomDirWithoutContent");
}

//  test runners 

fn run_standard_test(tc: &TestCase) -> Result<(), String> {
    let dir = format!("TestFiles_{}", tc.name);
    let result = run_standard_test_inner(tc, &dir);
    let _ = fs::remove_dir_all(&dir);
    result
}

fn run_standard_test_inner(tc: &TestCase, dir: &str) -> Result<(), String> {
    let _ = fs::remove_dir_all(dir);
    unzip_files(dir)?;

    // Build the czkawka command: replace the "TestFiles" placeholder and append -H.
    let mut cmd: Vec<&str> = vec![CZKAWKA_PATH.get().as_str()];
    for &arg in tc.args {
        cmd.push(if arg == "TestFiles" { dir } else { arg });
    }
    cmd.push("-H");

    run_with_good_status(&cmd, false).map_err(|e| format!("czkawka failed: {e}"))?;

    file_folder_diffs(
        COLLECTED_FILES.get(),
        dir,
        tc.expected_files,
        tc.expected_folders,
        tc.expected_symlinks,
    )
}

fn run_advanced_test(tc: &AdvancedTestCase) -> Result<(), String> {
    let dir = format!("TestFiles_adv_{}", tc.name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).map_err(|e| format!("create_dir_all({dir}): {e}"))?;
    let result = (tc.run)(&dir);
    let _ = fs::remove_dir_all(&dir);
    result
}

//  infrastructure (pub(crate) so advanced_tests.rs can use them) 

/// Write the full test file tree to `dir` using the in-memory snapshot.
/// Replaces the previous `unzip -qq -X TestFiles.zip -d <dir>` shell call.
pub(crate) fn unzip_files(dir: &str) -> Result<(), String> {
    test_file_system::default_test_files(dir, ZIP_ENTRIES.get())
}

pub(crate) fn run_with_good_status(str_command: &[&str], print_output: bool) -> Result<(), String> {
    let mut command = Command::new(str_command[0]);
    let mut com = command.args(&str_command[1..]);
    com.env("ENABLE_TERMINAL_LOGS_IN_CLI", "1");
    com.env("RUST_BACKTRACE", "1");

    if !print_output {
        com = com.stderr(Stdio::piped()).stdout(Stdio::piped());
    }

    let output = com.spawn().unwrap().wait_with_output().unwrap();
    if output.status.success() {
        return Ok(());
    }

    let all_output = collect_output(&output);
    let command_str = str_command.join(" ");
    Err(format!(
        "Command \"{command_str}\" failed (exit {:?}) in {:?}\n\nOutput:\n{all_output}",
        output.status.code(),
        env::current_dir(),
    ))
}

fn file_folder_diffs(
    baseline: &CollectedFiles,
    dir: &str,
    expected_files: &[&str],
    expected_folders: &[&str],
    expected_symlinks: &[&str],
) -> Result<(), String> {
    let current = collect_all_files_and_dirs(dir)
        .map_err(|e| format!("collect_all_files_and_dirs({dir}): {e}"))?;

    let mut diff_files: Vec<&str> = baseline.files.difference(&current.files).map(|s| s.as_str()).collect();
    let mut diff_folders: Vec<&str> = baseline.folders.difference(&current.folders).map(|s| s.as_str()).collect();
    let mut diff_symlinks: Vec<&str> = baseline.symlinks.difference(&current.symlinks).map(|s| s.as_str()).collect();

    diff_files.sort_unstable();
    diff_folders.sort_unstable();
    diff_symlinks.sort_unstable();

    let mut exp_files   = expected_files.to_vec();
    let mut exp_folders = expected_folders.to_vec();
    let mut exp_symlinks = expected_symlinks.to_vec();

    exp_files.sort_unstable();
    exp_folders.sort_unstable();
    exp_symlinks.sort_unstable();

    let mut errors: Vec<String> = Vec::new();
    if diff_files != exp_files {
        errors.push(format!(
            "files mismatch\n    expected: {exp_files:?}\n    got:      {diff_files:?}"
        ));
    }
    if diff_folders != exp_folders {
        errors.push(format!(
            "folders mismatch\n    expected: {exp_folders:?}\n    got:      {diff_folders:?}"
        ));
    }
    if diff_symlinks != exp_symlinks {
        errors.push(format!(
            "symlinks mismatch\n    expected: {exp_symlinks:?}\n    got:      {diff_symlinks:?}"
        ));
    }

    if errors.is_empty() { Ok(()) } else { Err(errors.join("\n")) }
}

/// Collects all entries under `dir` and returns them as *relative* paths
/// (without the leading `dir/` prefix). This makes the baseline reusable
/// regardless of the concrete directory name chosen per test.
pub(crate) fn collect_all_files_and_dirs(dir: &str) -> std::io::Result<CollectedFiles> {
    let prefix = format!("{dir}/");
    let mut files   = BTreeSet::new();
    let mut folders = BTreeSet::new();
    let mut symlinks = BTreeSet::new();

    let mut dirs_to_scan = vec![dir.to_string()];
    while let Some(current_dir) = dirs_to_scan.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let full_path = entry.path().to_string_lossy().to_string();
            let rel_path = full_path.strip_prefix(&prefix).unwrap_or(&full_path).to_string();

            if file_type.is_dir() {
                folders.insert(rel_path);
                dirs_to_scan.push(full_path);
            } else if file_type.is_symlink() {
                symlinks.insert(rel_path);
            } else if file_type.is_file() {
                files.insert(rel_path);
            } else {
                panic!("Unknown file type: {full_path}");
            }
        }
    }

    Ok(CollectedFiles { files, folders, symlinks })
}

fn collect_output(output: &Output) -> String {
    format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}
