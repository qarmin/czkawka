use std::collections::BTreeSet;
use std::fs;
use std::process::Command;
use std::process::Stdio;

#[derive(Default, Clone, Debug)]
struct CollectedFiles {
    files: BTreeSet<String>,
    folders: BTreeSet<String>,
    symlinks: BTreeSet<String>,
}

const ATTEMPTS: u32 = 10;

// App runs - ./ci_tester PATH_TO_CZKAWKA
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_to_czkawka = args[1].clone();

    remove_test_dir();
    run_with_good_status(&["ls"], false);
    unzip_files();

    let all_files = collect_all_files_and_dirs("TestFiles").unwrap();
    remove_test_dir();

    for _ in 0..ATTEMPTS {
        test_empty_files(&path_to_czkawka, &all_files);
        test_smallest_files(&path_to_czkawka, &all_files);
        test_biggest_files(&path_to_czkawka, &all_files);
        test_empty_folders(&path_to_czkawka, &all_files);
        test_temporary_files(&path_to_czkawka, &all_files);
        test_symlinks_files(&path_to_czkawka, &all_files);
        test_remove_duplicates_one_oldest(&path_to_czkawka, &all_files);
        test_remove_duplicates_one_newest(&path_to_czkawka, &all_files);
        test_remove_duplicates_all_expect_newest(&path_to_czkawka, &all_files);
        test_remove_duplicates_all_expect_oldest(&path_to_czkawka, &all_files);
    }

    println!("Completed checking");
}
fn test_remove_duplicates_all_expect_oldest(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(
        path_to_czkawka,
        &["dup", "-d", "TestFiles", "-D", "AEO"],
        all_files,
        vec!["Images/A1.jpg", "Images/A5.jpg", "Music/M1.mp3", "Music/M2.mp3", "Videos/V1.mp4", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}
fn test_remove_duplicates_all_expect_newest(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(
        path_to_czkawka,
        &["dup", "-d", "TestFiles", "-D", "AEN"],
        all_files,
        vec!["Images/A2.jpg", "Images/A5.jpg", "Music/M1.mp3", "Music/M5.mp3", "Videos/V1.mp4", "Videos/V2.mp4"],
        vec![],
        vec![],
    );
}

fn test_remove_duplicates_one_newest(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(
        path_to_czkawka,
        &["dup", "-d", "TestFiles", "-D", "ON"],
        all_files,
        vec!["Images/A1.jpg", "Music/M2.mp3", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}
fn test_remove_duplicates_one_oldest(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(
        path_to_czkawka,
        &["dup", "-d", "TestFiles", "-D", "OO"],
        all_files,
        vec!["Images/A2.jpg", "Music/M5.mp3", "Videos/V2.mp4"],
        vec![],
        vec![],
    );
}

fn test_symlinks_files(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(
        path_to_czkawka,
        &["symlinks", "-d", "TestFiles", "-D"],
        all_files,
        vec![],
        vec![],
        vec!["Symlinks/EmptyFiles"],
    );
}
fn test_temporary_files(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(
        path_to_czkawka,
        &["temp", "-d", "TestFiles", "-D"],
        all_files,
        vec!["Temporary/Boczze.cache"],
        vec![],
        vec![],
    );
}
fn test_empty_folders(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(
        path_to_czkawka,
        &["empty-folders", "-d", "TestFiles", "-D"],
        all_files,
        vec![],
        vec!["EmptyFolders/One", "EmptyFolders/Two", "EmptyFolders/Two/TwoInside"],
        vec![],
    );
}

fn test_biggest_files(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(
        path_to_czkawka,
        &["big", "-d", "TestFiles", "-n", "6", "-D"],
        all_files,
        vec!["Music/M3.flac", "Music/M4.mp3", "Videos/V2.mp4", "Videos/V3.webm", "Videos/V1.mp4", "Videos/V5.mp4"],
        vec![],
        vec![],
    );
}

fn test_smallest_files(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(
        path_to_czkawka,
        &["big", "-d", "TestFiles", "-J", "-n", "5", "-D"],
        all_files,
        vec!["Broken/Br.jpg", "Broken/Br.mp3", "Broken/Br.pdf", "Broken/Br.zip", "EmptyFolders/ThreeButNot/KEKEKE"],
        vec![],
        vec![],
    );
}

fn test_empty_files(path_to_czkawka: &str, all_files: &CollectedFiles) {
    run_test(path_to_czkawka, &["empty-files", "-d", "TestFiles", "-D"], all_files, vec!["EmptyFile"], vec![], vec![]);
}

////////////////////////////////////
////////////////////////////////////
/////////HELPER FUNCTIONS///////////
////////////////////////////////////
////////////////////////////////////

fn run_test(
    path_to_czkawka: &str,
    arguments: &[&str],
    all_files: &CollectedFiles,
    expected_files_differences: Vec<&'static str>,
    expected_folders_differences: Vec<&'static str>,
    expected_symlinks_differences: Vec<&'static str>,
) {
    unzip_files();
    // Add path_to_czkawka to arguments
    let mut all_arguments = vec![];
    all_arguments.push(path_to_czkawka);
    all_arguments.extend_from_slice(arguments);
    run_with_good_status(&all_arguments, true);
    file_folder_diffs(&all_files, expected_files_differences, expected_folders_differences, expected_symlinks_differences);

    remove_test_dir();
}
fn unzip_files() {
    run_with_good_status(&["unzip", "TestFiles.zip", "-d", "TestFiles"], false);
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
    let status = com.spawn().expect("failed to execute process").wait().unwrap();
    assert!(status.success());
}

fn file_folder_diffs(
    all_files: &CollectedFiles,
    mut expected_files_differences: Vec<&'static str>,
    mut expected_folders_differences: Vec<&'static str>,
    mut expected_symlinks_differences: Vec<&'static str>,
) {
    let current_files = collect_all_files_and_dirs("TestFiles").unwrap();
    let mut diff_files = all_files
        .files
        .difference(&current_files.files)
        .map(|e| e.strip_prefix("TestFiles/").unwrap().to_string())
        .collect::<Vec<_>>();
    let mut diff_folders = all_files
        .folders
        .difference(&current_files.folders)
        .map(|e| e.strip_prefix("TestFiles/").unwrap().to_string())
        .collect::<Vec<_>>();
    let mut diff_symlinks = all_files
        .symlinks
        .difference(&current_files.symlinks)
        .map(|e| e.strip_prefix("TestFiles/").unwrap().to_string())
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
    while !folders_to_check.is_empty() {
        let folder = folders_to_check.pop().unwrap();
        let rd = fs::read_dir(folder)?;
        for entry in rd {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                folders.insert(path.display().to_string());
                folders_to_check.push(path.display().to_string());
            } else if path.is_symlink() {
                symlinks.insert(path.display().to_string());
            } else if path.is_file() {
                files.insert(path.display().to_string());
            } else {
                panic!("Unknown type of file {:?}", path);
            }
        }
    }

    folders.remove(dir);
    // println!("Found {} files, {} folders and {} symlinks", files.len(), folders.len(), symlinks.len());
    Ok(CollectedFiles { files, folders, symlinks })
}
