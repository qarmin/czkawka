use std::collections::BTreeSet;
use std::fs;
use std::process::{Command, Stdio};

use log::info;

#[derive(Default, Clone, Debug)]
struct CollectedFiles {
    files: BTreeSet<String>,
    folders: BTreeSet<String>,
    symlinks: BTreeSet<String>,
}

static CZKAWKA_PATH: state::InitCell<String> = state::InitCell::new();
static COLLECTED_FILES: state::InitCell<CollectedFiles> = state::InitCell::new();

const ATTEMPTS: u32 = 10;
const PRINT_MESSAGES_CZKAWKA: bool = true;

// App runs - ./ci_tester PATH_TO_CZKAWKA
fn main() {
    handsome_logger::init().expect("Should not fail in tests");
    let args: Vec<String> = std::env::args().collect();
    let path_to_czkawka = args[1].clone();
    CZKAWKA_PATH.set(path_to_czkawka);
    remove_test_dir();
    run_with_good_status(&["ls"], false);
    unzip_files();

    let all_files = collect_all_files_and_dirs("TestFiles").expect("Should not fail in tests");
    COLLECTED_FILES.set(all_files);
    remove_test_dir();

    for _ in 0..ATTEMPTS {
        test_empty_files();
        test_big_files();
        test_smallest_files();
        test_biggest_files();
        test_empty_folders();
        test_temporary_files();
        test_symlinks_files();
        test_remove_duplicates_one_oldest();
        test_remove_duplicates_one_newest();
        test_remove_duplicates_all_expect_newest();
        test_remove_duplicates_all_expect_oldest();
        test_remove_duplicates_one_smallest();
        test_remove_duplicates_one_biggest();
        test_remove_duplicates_all_expect_biggest();
        test_remove_duplicates_all_expect_smallest();
        test_remove_same_music_tags_one_oldest();
        test_remove_same_music_tags_one_newest();
        test_remove_same_music_tags_all_expect_oldest();
        test_remove_same_music_tags_all_expect_newest();
        test_remove_same_music_tags_one_smallest();
        test_remove_same_music_tags_one_biggest();
        test_remove_same_music_tags_all_expect_biggest();
        test_remove_same_music_tags_all_expect_smallest();
        test_remove_same_music_content_one_oldest();
        test_remove_same_music_content_all_expect_oldest();
        test_remove_same_music_content_one_newest();
        test_remove_same_music_content_all_expect_newest();
        test_remove_same_music_content_one_smallest();
        test_remove_same_music_content_one_biggest();
        test_remove_same_music_content_all_expect_biggest();
        test_remove_same_music_content_all_expect_smallest();
        test_remove_videos_one_oldest();
        test_remove_videos_one_newest();
        test_remove_videos_all_expect_oldest();
        test_remove_videos_all_expect_newest();
        test_remove_videos_one_smallest();
        test_remove_videos_one_biggest();
        test_remove_videos_all_expect_biggest();
        test_remove_videos_all_expect_smallest();
    }

    println!("Completed checking");
}
fn test_remove_videos_one_oldest() {
    info!("test_remove_videos_one_oldest");
    run_test(&["video", "-d", "TestFiles", "-D", "OO"], vec!["Videos/V3.webm"], vec![], vec![]);
}
fn test_remove_videos_one_newest() {
    info!("test_remove_videos_one_newest");
    run_test(&["video", "-d", "TestFiles", "-D", "ON"], vec!["Videos/V5.mp4"], vec![], vec![]);
}
fn test_remove_videos_all_expect_oldest() {
    info!("test_remove_videos_all_expect_oldest");
    run_test(
        &["video", "-d", "TestFiles", "-D", "AEO"],
        vec!["Videos/V1.mp4", "Videos/V2.mp4", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}
fn test_remove_videos_all_expect_newest() {
    info!("test_remove_videos_all_expect_newest");
    run_test(
        &["video", "-d", "TestFiles", "-D", "AEN"],
        vec!["Videos/V1.mp4", "Videos/V2.mp4", "Videos/V3.webm"],
        vec![],
        vec![],
    );
}
fn test_remove_videos_one_smallest() {
    info!("test_remove_videos_one_smallest");
    run_test(&["video", "-d", "TestFiles", "-D", "OS"], vec!["Videos/V2.mp4"], vec![], vec![]);
}
fn test_remove_videos_one_biggest() {
    info!("test_remove_videos_one_biggest");
    run_test(&["video", "-d", "TestFiles", "-D", "OB"], vec!["Videos/V3.webm"], vec![], vec![]);
}
fn test_remove_videos_all_expect_smallest() {
    info!("test_remove_videos_all_expect_smallest");
    run_test(
        &["video", "-d", "TestFiles", "-D", "AES"],
        vec!["Videos/V1.mp4", "Videos/V3.webm", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}
fn test_remove_videos_all_expect_biggest() {
    info!("test_remove_videos_all_expect_biggest");
    run_test(
        &["video", "-d", "TestFiles", "-D", "AEB"],
        vec!["Videos/V1.mp4", "Videos/V2.mp4", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}

fn test_remove_same_music_content_one_newest() {
    info!("test_remove_same_music_content_one_newest");
    run_test(
        &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "ON"],
        vec!["Music/M2.mp3"],
        vec![],
        vec![],
    );
}
fn test_remove_same_music_content_all_expect_newest() {
    info!("test_remove_same_music_content_all_expect_newest");
    run_test(
        &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AEN"],
        vec!["Music/M1.mp3", "Music/M3.flac", "Music/M5.mp3"],
        vec![],
        vec![],
    );
}

fn test_remove_same_music_content_all_expect_oldest() {
    info!("test_remove_same_music_content_all_expect_oldest");
    run_test(
        &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AEO"],
        vec!["Music/M1.mp3", "Music/M2.mp3", "Music/M3.flac"],
        vec![],
        vec![],
    );
}

fn test_remove_same_music_content_one_oldest() {
    info!("test_remove_same_music_content_one_oldest");
    run_test(
        &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "OO"],
        vec!["Music/M5.mp3"],
        vec![],
        vec![],
    );
}
fn test_remove_same_music_content_one_biggest() {
    info!("test_remove_same_music_content_one_biggest");
    run_test(
        &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "OB"],
        vec!["Music/M3.flac"],
        vec![],
        vec![],
    );
}
fn test_remove_same_music_content_all_expect_biggest() {
    info!("test_remove_same_music_content_all_expect_biggest");
    run_test(
        &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AEB"],
        vec!["Music/M1.mp3", "Music/M2.mp3", "Music/M5.mp3"],
        vec![],
        vec![],
    );
}

fn test_remove_same_music_content_all_expect_smallest() {
    info!("test_remove_same_music_content_all_expect_smallest");
    run_test(
        &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "AES"],
        vec!["Music/M1.mp3", "Music/M3.flac", "Music/M5.mp3"],
        vec![],
        vec![],
    );
}

fn test_remove_same_music_content_one_smallest() {
    info!("test_remove_same_music_content_one_smallest");
    run_test(
        &["music", "-d", "TestFiles", "-s", "CONTENT", "-l", "2.0", "-D", "OS"],
        vec!["Music/M2.mp3"],
        vec![],
        vec![],
    );
}
fn test_remove_same_music_tags_one_oldest() {
    info!("test_remove_same_music_one_oldest");
    run_test(&["music", "-d", "TestFiles", "-D", "OO"], vec!["Music/M5.mp3"], vec![], vec![]);
}
fn test_remove_same_music_tags_one_newest() {
    info!("test_remove_same_music_one_newest");
    run_test(&["music", "-d", "TestFiles", "-D", "ON"], vec!["Music/M2.mp3"], vec![], vec![]);
}
fn test_remove_same_music_tags_all_expect_oldest() {
    info!("test_remove_same_music_all_expect_oldest");
    run_test(
        &["music", "-d", "TestFiles", "-D", "AEO"],
        vec!["Music/M1.mp3", "Music/M2.mp3", "Music/M3.flac"],
        vec![],
        vec![],
    );
}
fn test_remove_same_music_tags_all_expect_newest() {
    info!("test_remove_same_music_all_expect_newest");
    run_test(
        &["music", "-d", "TestFiles", "-D", "AEN"],
        vec!["Music/M1.mp3", "Music/M3.flac", "Music/M5.mp3"],
        vec![],
        vec![],
    );
}
fn test_remove_same_music_tags_one_smallest() {
    info!("test_remove_same_music_one_smallest");
    run_test(&["music", "-d", "TestFiles", "-D", "OS"], vec!["Music/M1.mp3"], vec![], vec![]);
}
fn test_remove_same_music_tags_one_biggest() {
    info!("test_remove_same_music_one_biggest");
    run_test(&["music", "-d", "TestFiles", "-D", "OB"], vec!["Music/M3.flac"], vec![], vec![]);
}
fn test_remove_same_music_tags_all_expect_smallest() {
    info!("test_remove_same_music_all_expect_smallest");
    run_test(
        &["music", "-d", "TestFiles", "-D", "AES"],
        vec!["Music/M2.mp3", "Music/M3.flac", "Music/M5.mp3"],
        vec![],
        vec![],
    );
}
fn test_remove_same_music_tags_all_expect_biggest() {
    info!("test_remove_same_music_all_expect_biggest");
    run_test(
        &["music", "-d", "TestFiles", "-D", "AEB"],
        vec!["Music/M1.mp3", "Music/M2.mp3", "Music/M5.mp3"],
        vec![],
        vec![],
    );
}
fn test_remove_duplicates_all_expect_oldest() {
    info!("test_remove_duplicates_all_expect_oldest");
    run_test(
        &["dup", "-d", "TestFiles", "-D", "AEO"],
        vec!["Images/A1.jpg", "Images/A5.jpg", "Music/M1.mp3", "Music/M2.mp3", "Videos/V1.mp4", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}
fn test_remove_duplicates_all_expect_newest() {
    info!("test_remove_duplicates_all_expect_newest");
    run_test(
        &["dup", "-d", "TestFiles", "-D", "AEN"],
        vec!["Images/A2.jpg", "Images/A5.jpg", "Music/M1.mp3", "Music/M5.mp3", "Videos/V1.mp4", "Videos/V2.mp4"],
        vec![],
        vec![],
    );
}

fn test_remove_duplicates_one_newest() {
    info!("test_remove_duplicates_one_newest");
    run_test(
        &["dup", "-d", "TestFiles", "-D", "ON"],
        vec!["Images/A1.jpg", "Music/M2.mp3", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}
fn test_remove_duplicates_one_oldest() {
    info!("test_remove_duplicates_one_oldest");
    run_test(
        &["dup", "-d", "TestFiles", "-D", "OO"],
        vec!["Images/A2.jpg", "Music/M5.mp3", "Videos/V2.mp4"],
        vec![],
        vec![],
    );
}
fn test_remove_duplicates_all_expect_smallest() {
    info!("test_remove_duplicates_all_expect_smallest");
    run_test(
        &["dup", "-d", "TestFiles", "-D", "AES"],
        vec!["Images/A2.jpg", "Images/A5.jpg", "Music/M2.mp3", "Music/M5.mp3", "Videos/V2.mp4", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}
fn test_remove_duplicates_all_expect_biggest() {
    info!("test_remove_duplicates_all_expect_biggest");
    run_test(
        &["dup", "-d", "TestFiles", "-D", "AEN"],
        vec!["Images/A2.jpg", "Images/A5.jpg", "Music/M1.mp3", "Music/M5.mp3", "Videos/V1.mp4", "Videos/V2.mp4"],
        vec![],
        vec![],
    );
}

fn test_remove_duplicates_one_biggest() {
    info!("test_remove_duplicates_one_biggest");
    run_test(
        &["dup", "-d", "TestFiles", "-D", "ON"],
        vec!["Images/A1.jpg", "Music/M2.mp3", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}
fn test_remove_duplicates_one_smallest() {
    info!("test_remove_duplicates_one_smallest");
    run_test(
        &["dup", "-d", "TestFiles", "-D", "OS"],
        vec!["Images/A1.jpg", "Music/M1.mp3", "Videos/V1.mp4"],
        vec![],
        vec![],
    );
}

fn test_symlinks_files() {
    info!("test_symlinks_files");
    run_test(&["symlinks", "-d", "TestFiles", "-D"], vec![], vec![], vec!["Symlinks/EmptyFiles"]);
}
fn test_temporary_files() {
    info!("test_temporary_files");
    run_test(&["temp", "-d", "TestFiles", "-D"], vec!["Temporary/Boczze.cache"], vec![], vec![]);
}
fn test_empty_folders() {
    info!("test_empty_folders");
    run_test(
        &["empty-folders", "-d", "TestFiles", "-D"],
        vec![],
        vec!["EmptyFolders/One", "EmptyFolders/Two", "EmptyFolders/Two/TwoInside"],
        vec![],
    );
}

fn test_biggest_files() {
    info!("test_biggest_files");
    run_test(
        &["big", "-d", "TestFiles", "-n", "6", "-D"],
        vec!["Music/M3.flac", "Music/M4.mp3", "Videos/V2.mp4", "Videos/V3.webm", "Videos/V1.mp4", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}

fn test_smallest_files() {
    info!("test_smallest_files");
    run_test(
        &["big", "-d", "TestFiles", "-J", "-n", "5", "-D"],
        vec!["Broken/Br.jpg", "Broken/Br.mp3", "Broken/Br.pdf", "Broken/Br.zip", "EmptyFolders/ThreeButNot/KEKEKE"],
        vec![],
        vec![],
    );
}

fn test_empty_files() {
    info!("test_empty_files");
    run_test(&["empty-files", "-d", "TestFiles", "-D"], vec!["EmptyFile"], vec![], vec![]);
}

fn test_big_files() {
    info!("test_big_files");
    run_test(&["big", "-d", "TestFiles", "-n", "2", "-D"], vec!["Music/M4.mp3", "Videos/V3.webm"], vec![], vec![]);
}

////////////////////////////////////
////////////////////////////////////
/////////HELPER FUNCTIONS///////////
////////////////////////////////////
////////////////////////////////////

fn run_test(arguments: &[&str], expected_files_differences: Vec<&'static str>, expected_folders_differences: Vec<&'static str>, expected_symlinks_differences: Vec<&'static str>) {
    unzip_files();
    // Add path_to_czkawka to arguments
    let mut all_arguments = vec![];
    all_arguments.push(CZKAWKA_PATH.get().as_str());
    all_arguments.extend_from_slice(arguments);
    run_with_good_status(&all_arguments, PRINT_MESSAGES_CZKAWKA);
    file_folder_diffs(
        COLLECTED_FILES.get(),
        expected_files_differences,
        expected_folders_differences,
        expected_symlinks_differences,
    );

    remove_test_dir();
}
fn unzip_files() {
    run_with_good_status(&["unzip", "-X", "TestFiles.zip", "-d", "TestFiles"], false);
}
fn remove_test_dir() {
    let _ = fs::remove_dir_all("TestFiles");
}

fn run_with_good_status(str_command: &[&str], print_messages: bool) {
    let mut command = Command::new(str_command[0]);
    let mut com = command.args(&str_command[1..]);
    if !print_messages {
        com = com.stderr(Stdio::piped()).stdout(Stdio::piped());
    }
    let status = com.spawn().expect("failed to execute process").wait().expect("Should not fail in tests");
    assert!(status.success());
}

fn file_folder_diffs(
    all_files: &CollectedFiles,
    mut expected_files_differences: Vec<&'static str>,
    mut expected_folders_differences: Vec<&'static str>,
    mut expected_symlinks_differences: Vec<&'static str>,
) {
    let current_files = collect_all_files_and_dirs("TestFiles").expect("Should not fail in tests");
    let mut diff_files = all_files
        .files
        .difference(&current_files.files)
        .map(|e| e.strip_prefix("TestFiles/").expect("Should not fail in tests").to_string())
        .collect::<Vec<_>>();
    let mut diff_folders = all_files
        .folders
        .difference(&current_files.folders)
        .map(|e| e.strip_prefix("TestFiles/").expect("Should not fail in tests").to_string())
        .collect::<Vec<_>>();
    let mut diff_symlinks = all_files
        .symlinks
        .difference(&current_files.symlinks)
        .map(|e| e.strip_prefix("TestFiles/").expect("Should not fail in tests").to_string())
        .collect::<Vec<_>>();

    expected_symlinks_differences.sort();
    expected_folders_differences.sort();
    expected_files_differences.sort();

    diff_files.sort();
    diff_folders.sort();
    diff_symlinks.sort();

    assert_eq!(diff_files, expected_files_differences);
    assert_eq!(diff_folders, expected_folders_differences);
    assert_eq!(diff_symlinks, expected_symlinks_differences);
}

fn collect_all_files_and_dirs(dir: &str) -> std::io::Result<CollectedFiles> {
    let mut files = BTreeSet::new();
    let mut folders = BTreeSet::new();
    let mut symlinks = BTreeSet::new();

    let mut folders_to_check = vec![dir.to_string()];
    while let Some(folder) = folders_to_check.pop() {
        let rd = fs::read_dir(folder)?;
        for entry in rd {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path_str = entry.path().to_string_lossy().to_string();

            if file_type.is_dir() {
                folders.insert(path_str.clone());
                folders_to_check.push(path_str);
            } else if file_type.is_symlink() {
                symlinks.insert(path_str);
            } else if file_type.is_file() {
                files.insert(path_str);
            } else {
                panic!("Unknown type of file {path_str}");
            }
        }
    }

    for dir in &folders_to_check {
        println!("Folder \"{}\"", dir)
    }
    for symlink in &symlinks {
        println!("Symlink \"{}\"", symlink)
    }
    for file in &files {
        let metadata = fs::metadata(file)?;
        println!("File \"{}\" with size {} bytes", file, metadata.len());
    }

    folders.remove(dir);
    // println!("Found {} files, {} folders and {} symlinks", files.len(), folders.len(), symlinks.len());
    Ok(CollectedFiles { files, folders, symlinks })
}
