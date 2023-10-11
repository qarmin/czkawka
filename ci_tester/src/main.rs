use std::collections::BTreeSet;
use std::fs;
use std::process::Command;
use std::process::Stdio;

// App runs - ./ci_tester PATH_TO_CZKAWKA
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_to_czkawka = args[1].clone();

    remove_test_dir();
    run_with_good_status(&["ls"], false);
    unzip_files();

    let (files, folders, symlinks) = collect_all_files_and_dirs("TestFiles").unwrap();
    remove_test_dir();

    for _ in 0..5 {
        test_empty_files(&path_to_czkawka, &files, &folders, &symlinks);
        test_smallest_files(&path_to_czkawka, &files, &folders, &symlinks);
        test_biggest_files(&path_to_czkawka, &files, &folders, &symlinks);
    }

    println!("Completed checking");
}

fn test_biggest_files(path_to_czkawka: &str, all_files: &BTreeSet<String>, all_folders: &BTreeSet<String>, all_symlinks: &BTreeSet<String>) {
    unzip_files();
    run_with_good_status(&[path_to_czkawka, "big", "-d", "TestFiles", "-n", "5", "-D"], true);

    file_folder_diffs(
        &all_files,
        &all_folders,
        &all_symlinks,
        vec![
            "Music/M4.mp3".to_string(),
            "Videos/V3.webm".to_string(),
            "Music/M3.flac".to_string(),
            "Videos/V2.mp4".to_string(),
            "Videos/V4.mp4".to_string(),
        ],
        vec![],
        vec![],
    );
    remove_test_dir();
}

fn test_smallest_files(path_to_czkawka: &str, all_files: &BTreeSet<String>, all_folders: &BTreeSet<String>, all_symlinks: &BTreeSet<String>) {
    unzip_files();
    run_with_good_status(&[path_to_czkawka, "big", "-d", "TestFiles", "-J", "-n", "5", "-D"], true);

    file_folder_diffs(
        &all_files,
        &all_folders,
        &all_symlinks,
        vec![
            "Broken/Br.jpg".to_string(),
            "Broken/Br.mp3".to_string(),
            "Broken/Br.pdf".to_string(),
            "Broken/Br.zip".to_string(),
            "EmptyFolders/ThreeButNot/KEKEKE".to_string(),
        ],
        vec![],
        vec![],
    );
    remove_test_dir();
}

fn test_empty_files(path_to_czkawka: &str, all_files: &BTreeSet<String>, all_folders: &BTreeSet<String>, all_symlinks: &BTreeSet<String>) {
    unzip_files();
    run_with_good_status(&[path_to_czkawka, "empty-files", "-d", "TestFiles", "-D"], true);

    file_folder_diffs(&all_files, &all_folders, &all_symlinks, vec!["EmptyFile".to_string()], vec![], vec![]);
    remove_test_dir();
}

////////////////////////////////////
////////////////////////////////////
/////////HELPER FUNCTIONS///////////
////////////////////////////////////
////////////////////////////////////

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
    file: &BTreeSet<String>,
    folder: &BTreeSet<String>,
    symlinks: &BTreeSet<String>,
    mut expected_files_differences: Vec<String>,
    mut expected_folders_differences: Vec<String>,
    mut expected_symlinks_differences: Vec<String>,
) {
    let (current_files, current_folders, current_symlinks) = collect_all_files_and_dirs("TestFiles").unwrap();
    let mut diff_files = file
        .difference(&current_files)
        .map(|e| e.strip_prefix("TestFiles/").unwrap().to_string())
        .collect::<Vec<_>>();
    let mut diff_folders = folder
        .difference(&current_folders)
        .map(|e| e.strip_prefix("TestFiles/").unwrap().to_string())
        .collect::<Vec<_>>();
    let mut diff_symlinks = symlinks
        .difference(&current_symlinks)
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

fn collect_all_files_and_dirs(dir: &str) -> std::io::Result<(BTreeSet<String>, BTreeSet<String>, BTreeSet<String>)> {
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
    Ok((files, folders, symlinks))
}
