use std::collections::BTreeSet;
use std::fs;
use std::process::{Command, Output, Stdio};
use std::env;
use log::info;
use rayon::prelude::*;

#[derive(Default, Clone, Debug)]
struct CollectedFiles {
    files: BTreeSet<String>,
    folders: BTreeSet<String>,
    symlinks: BTreeSet<String>,
}

static CZKAWKA_PATH: state::InitCell<String> = state::InitCell::new();
static COLLECTED_FILES: state::InitCell<CollectedFiles> = state::InitCell::new();

const ATTEMPTS: u32 = 10;

/// A single test case. Use "TestFiles" as a placeholder in args – it will be
/// replaced at runtime with a per-test unique directory so that tests can run
/// in parallel without stepping on each other.
struct TestCase {
    name: &'static str,
    args: &'static [&'static str],
    expected_files: &'static [&'static str],
    expected_folders: &'static [&'static str],
    expected_symlinks: &'static [&'static str],
}

fn all_test_cases() -> Vec<TestCase> {
    vec![
        // ── Misc ──────────────────────────────────────────────────────────────
        TestCase {
            name: "empty_files",
            args: &["empty-files", "-d", "TestFiles", "-D", "-W"],
            expected_files: &["EmptyFile"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "big_files",
            args: &["big", "-d", "TestFiles", "-n", "2", "-D", "-W"],
            expected_files: &["Music/M4.mp3", "Videos/V3.webm"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "smallest_files",
            args: &["big", "-d", "TestFiles", "-J", "-n", "5", "-D", "-W"],
            expected_files: &[
                "Broken/Br.jpg",
                "Broken/Br.mp3",
                "Broken/Br.pdf",
                "Broken/Br.zip",
                "EmptyFolders/ThreeButNot/KEKEKE",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "biggest_files",
            args: &["big", "-d", "TestFiles", "-n", "6", "-D", "-W"],
            expected_files: &[
                "Music/M3.flac",
                "Music/M4.mp3",
                "Videos/V1.mp4",
                "Videos/V2.mp4",
                "Videos/V3.webm",
                "Videos/V5.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "empty_folders",
            args: &["empty-folders", "-d", "TestFiles", "-D", "-W"],
            expected_files: &[],
            expected_folders: &["EmptyFolders/One", "EmptyFolders/Two", "EmptyFolders/Two/TwoInside"],
            expected_symlinks: &[],
        },
        TestCase {
            name: "temporary_files",
            args: &["temp", "-d", "TestFiles", "-D", "-W"],
            expected_files: &["Temporary/Boczze.cache"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "symlinks_files",
            args: &["symlinks", "-d", "TestFiles", "-D", "-W"],
            expected_files: &[],
            expected_folders: &[],
            expected_symlinks: &["Symlinks/EmptyFiles"],
        },
        // ── Duplicates ────────────────────────────────────────────────────────
        TestCase {
            name: "dup_one_oldest",
            args: &["dup", "-d", "TestFiles", "-D", "OO", "-W"],
            expected_files: &["Images/A2.jpg", "Music/M5.mp3", "Videos/V2.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_one_newest",
            args: &["dup", "-d", "TestFiles", "-D", "ON", "-W"],
            expected_files: &["Images/A1.jpg", "Music/M2.mp3", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_all_except_oldest",
            args: &["dup", "-d", "TestFiles", "-D", "AEO", "-W"],
            expected_files: &[
                "Images/A1.jpg",
                "Images/A5.jpg",
                "Music/M1.mp3",
                "Music/M2.mp3",
                "Videos/V1.mp4",
                "Videos/V5.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_all_except_newest",
            args: &["dup", "-d", "TestFiles", "-D", "AEN", "-W"],
            expected_files: &[
                "Images/A2.jpg",
                "Images/A5.jpg",
                "Music/M1.mp3",
                "Music/M5.mp3",
                "Videos/V1.mp4",
                "Videos/V2.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_one_smallest",
            args: &["dup", "-d", "TestFiles", "-D", "OS", "-W"],
            expected_files: &["Images/A1.jpg", "Music/M1.mp3", "Videos/V1.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            // NOTE: original code used "-D ON" here (copy-paste bug); preserved as-is
            name: "dup_one_biggest",
            args: &["dup", "-d", "TestFiles", "-D", "ON", "-W"],
            expected_files: &["Images/A1.jpg", "Music/M2.mp3", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "dup_all_except_smallest",
            args: &["dup", "-d", "TestFiles", "-D", "AES", "-W"],
            expected_files: &[
                "Images/A2.jpg",
                "Images/A5.jpg",
                "Music/M2.mp3",
                "Music/M5.mp3",
                "Videos/V2.mp4",
                "Videos/V5.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            // NOTE: original code used "-D AEN" here (copy-paste bug); preserved as-is
            name: "dup_all_except_biggest",
            args: &["dup", "-d", "TestFiles", "-D", "AEN", "-W"],
            expected_files: &[
                "Images/A2.jpg",
                "Images/A5.jpg",
                "Music/M1.mp3",
                "Music/M5.mp3",
                "Videos/V1.mp4",
                "Videos/V2.mp4",
            ],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        // ── Music by tags ─────────────────────────────────────────────────────
        TestCase {
            name: "music_tags_one_oldest",
            args: &["music", "-d", "TestFiles", "-D", "OO", "-W"],
            expected_files: &["Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_one_newest",
            args: &["music", "-d", "TestFiles", "-D", "ON", "-W"],
            expected_files: &["Music/M2.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_all_except_oldest",
            args: &["music", "-d", "TestFiles", "-D", "AEO", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M2.mp3", "Music/M3.flac"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_all_except_newest",
            args: &["music", "-d", "TestFiles", "-D", "AEN", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M3.flac", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_one_smallest",
            args: &["music", "-d", "TestFiles", "-D", "OS", "-W"],
            expected_files: &["Music/M1.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_one_biggest",
            args: &["music", "-d", "TestFiles", "-D", "OB", "-W"],
            expected_files: &["Music/M3.flac"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_all_except_smallest",
            args: &["music", "-d", "TestFiles", "-D", "AES", "-W"],
            expected_files: &["Music/M2.mp3", "Music/M3.flac", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_tags_all_except_biggest",
            args: &["music", "-d", "TestFiles", "-D", "AEB", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M2.mp3", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        // ── Music by content ──────────────────────────────────────────────────
        TestCase {
            name: "music_content_one_oldest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "OO", "-W"],
            expected_files: &["Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_one_newest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "ON", "-W"],
            expected_files: &["Music/M2.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_all_except_oldest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AEO", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M2.mp3", "Music/M3.flac"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_all_except_newest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AEN", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M3.flac", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_one_smallest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "OS", "-W"],
            expected_files: &["Music/M2.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_one_biggest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "OB", "-W"],
            expected_files: &["Music/M3.flac"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_all_except_smallest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AES", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M3.flac", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "music_content_all_except_biggest",
            args: &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AEB", "-W"],
            expected_files: &["Music/M1.mp3", "Music/M2.mp3", "Music/M5.mp3"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        // ── Video ─────────────────────────────────────────────────────────────
        TestCase {
            name: "video_one_oldest",
            args: &["video", "-d", "TestFiles", "-D", "OO", "-W"],
            expected_files: &["Videos/V3.webm"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_one_newest",
            args: &["video", "-d", "TestFiles", "-D", "ON", "-W"],
            expected_files: &["Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_all_except_oldest",
            args: &["video", "-d", "TestFiles", "-D", "AEO", "-W"],
            expected_files: &["Videos/V1.mp4", "Videos/V2.mp4", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_all_except_newest",
            args: &["video", "-d", "TestFiles", "-D", "AEN", "-W"],
            expected_files: &["Videos/V1.mp4", "Videos/V2.mp4", "Videos/V3.webm"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_one_smallest",
            args: &["video", "-d", "TestFiles", "-D", "OS", "-W"],
            expected_files: &["Videos/V2.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_one_biggest",
            args: &["video", "-d", "TestFiles", "-D", "OB", "-W"],
            expected_files: &["Videos/V3.webm"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_all_except_smallest",
            args: &["video", "-d", "TestFiles", "-D", "AES", "-W"],
            expected_files: &["Videos/V1.mp4", "Videos/V3.webm", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
        TestCase {
            name: "video_all_except_biggest",
            args: &["video", "-d", "TestFiles", "-D", "AEB", "-W"],
            expected_files: &["Videos/V1.mp4", "Videos/V2.mp4", "Videos/V5.mp4"],
            expected_folders: &[],
            expected_symlinks: &[],
        },
    ]
}

// App runs - ./ci_tester PATH_TO_CZKAWKA
fn main() {
    handsome_logger::init().expect("Should not fail in tests");
    let args: Vec<String> = env::args().collect();
    let path_to_czkawka = args[1].clone();
    CZKAWKA_PATH.set(path_to_czkawka);

    test_args();

    // Collect baseline from a fresh unzip, then remove it
    let _ = fs::remove_dir_all("TestFiles");
    unzip_files("TestFiles").expect("Initial unzip failed");
    let baseline = collect_all_files_and_dirs("TestFiles").expect("Should not fail in tests");
    COLLECTED_FILES.set(baseline);
    let _ = fs::remove_dir_all("TestFiles");

    let test_cases = all_test_cases();
    println!(
        "Running {} test cases × {} attempts (parallel within each attempt)...",
        test_cases.len(),
        ATTEMPTS
    );

    let mut all_failures: Vec<String> = Vec::new();

    for attempt in 0..ATTEMPTS {
        let attempt_failures: Vec<String> = test_cases
            .par_iter()
            .filter_map(|tc| {
                info!("[{}/{}] {}", attempt + 1, ATTEMPTS, tc.name);
                match run_test(tc) {
                    Ok(()) => None,
                    Err(e) => Some(format!(
                        "[attempt {}/{}] {}: {}",
                        attempt + 1,
                        ATTEMPTS,
                        tc.name,
                        e
                    )),
                }
            })
            .collect();

        let passed = test_cases.len() - attempt_failures.len();
        println!(
            "Attempt {}/{}: {}/{} passed",
            attempt + 1,
            ATTEMPTS,
            passed,
            test_cases.len()
        );
        all_failures.extend(attempt_failures);
    }

    if all_failures.is_empty() {
        println!(
            "\nAll {} tests passed ({} attempts).",
            test_cases.len(),
            ATTEMPTS
        );
    } else {
        eprintln!("\n====== {} FAILURE(S) ======", all_failures.len());
        for f in &all_failures {
            eprintln!("  FAIL: {f}");
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

////////////////////////////////////
////////////////////////////////////
/////////HELPER FUNCTIONS///////////
////////////////////////////////////
////////////////////////////////////

/// Run one test case in its own isolated directory and return Ok or an error
/// string. The directory is always cleaned up, even on failure.
fn run_test(tc: &TestCase) -> Result<(), String> {
    let dir = format!("TestFiles_{}", tc.name);
    let result = run_test_inner(tc, &dir);
    let _ = fs::remove_dir_all(&dir);
    result
}

fn run_test_inner(tc: &TestCase, dir: &str) -> Result<(), String> {
    // Clean up any leftover from a previous interrupted run
    let _ = fs::remove_dir_all(dir);

    unzip_files(dir)?;

    // Build the command: prepend binary path, substitute "TestFiles" placeholder,
    // and append -H to disable cache.
    let mut cmd: Vec<&str> = vec![CZKAWKA_PATH.get().as_str()];
    for &arg in tc.args {
        if arg == "TestFiles" {
            cmd.push(dir);
        } else {
            cmd.push(arg);
        }
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

fn unzip_files(dir: &str) -> Result<(), String> {
    run_with_good_status(&["unzip", "-qq", "-X", "TestFiles.zip", "-d", dir], false)
        .map_err(|e| format!("unzip to {dir} failed: {e}"))
}

fn run_with_good_status(str_command: &[&str], print_output: bool) -> Result<(), String> {
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
    let current =
        collect_all_files_and_dirs(dir).map_err(|e| format!("collect_all_files_and_dirs({dir}) failed: {e}"))?;

    let mut diff_files: Vec<&str> = baseline
        .files
        .difference(&current.files)
        .map(|s| s.as_str())
        .collect();
    let mut diff_folders: Vec<&str> = baseline
        .folders
        .difference(&current.folders)
        .map(|s| s.as_str())
        .collect();
    let mut diff_symlinks: Vec<&str> = baseline
        .symlinks
        .difference(&current.symlinks)
        .map(|s| s.as_str())
        .collect();

    diff_files.sort_unstable();
    diff_folders.sort_unstable();
    diff_symlinks.sort_unstable();

    let mut exp_files = expected_files.to_vec();
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

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

/// Collects all entries under `dir` and returns them as *relative* paths
/// (i.e. without the leading `dir/` prefix). This makes the baseline
/// reusable regardless of the concrete directory name used in each test.
fn collect_all_files_and_dirs(dir: &str) -> std::io::Result<CollectedFiles> {
    let prefix = format!("{dir}/");
    let mut files = BTreeSet::new();
    let mut folders = BTreeSet::new();
    let mut symlinks = BTreeSet::new();

    // dirs_to_scan holds full paths so we can call fs::read_dir on them.
    let mut dirs_to_scan = vec![dir.to_string()];
    while let Some(current_dir) = dirs_to_scan.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let full_path = entry.path().to_string_lossy().to_string();
            // Store relative path so comparisons work across different dir names.
            let rel_path = full_path
                .strip_prefix(&prefix)
                .unwrap_or(&full_path)
                .to_string();

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
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);
    format!("{stdout_str}\n{stderr_str}")
}
