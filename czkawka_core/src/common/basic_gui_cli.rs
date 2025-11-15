use std::process;

use log::warn;

use crate::CZKAWKA_VERSION;

#[derive(Clone, Debug)]
pub struct CliResult {
    pub included_items: Vec<String>,
    pub excluded_items: Vec<String>,
    pub referenced_items: Vec<String>,
}

enum ExpectedArgs {
    Include,
    Exclude,
    Referenced,
}

// Manual processing of CLI arguments, because Clap would be too heavy for this simple task

#[expect(clippy::print_stdout)]
#[expect(clippy::print_stderr)]
pub fn process_cli_args(app_display: &str, app_exec: &str, args: Vec<String>) -> Option<CliResult> {
    if ["--help", "-h"].iter().any(|&arg| args.contains(&arg.to_string())) {
        println!("{app_display}");
        println!("{app_display} allows you to specify folders to search for files via the CLI, and also to exclude or reference folders.");
        println!("If used, it will automatically apply the last preset and load its options.");
        println!("Running the app without arguments will launch the {app_display} with default or saved options.");
        println!("Usage: {app_exec} [OPTIONS] [FOLDERS...]");
        println!("Options:");
        println!("  FOLDER                Include a folder in the search");
        println!("  -e FOLDER, --exclude FOLDER      Exclude a folder from the search");
        println!("  -r FOLDER, --referenced FOLDER   Include a folder and set it as referenced");
        println!("  --help, -h            Show this help message");
        println!("  --version, -v         Show version information");
        println!("Examples:");
        println!("  {app_exec} /path/absolute/to/folder -e relative_path/2 -r /path/to/referenced");
        println!("  {app_exec} . folder2 folder3");
        println!("If no folders are specified, the program will exit without doing anything.");
        process::exit(0);
    }
    if ["--version", "-v"].iter().any(|&arg| args.contains(&arg.to_string())) {
        println!("{app_display} version {CZKAWKA_VERSION}");
        process::exit(0);
    }

    let mut expected_arg = ExpectedArgs::Include;
    let mut cli_result = CliResult {
        included_items: Vec::new(),
        excluded_items: Vec::new(),
        referenced_items: Vec::new(),
    };
    let mut errors = Vec::new();

    for arg in args {
        if arg.starts_with("-") {
            match arg.as_str() {
                "-e" | "--exclude" => expected_arg = ExpectedArgs::Exclude,
                "-r" | "--referenced" => expected_arg = ExpectedArgs::Referenced,
                _ => {
                    eprintln!("Unknown option: {arg}");
                    process::exit(1);
                }
            }
        } else {
            match expected_arg {
                ExpectedArgs::Include => match check_if_folder_is_valid(&arg) {
                    Ok(folder) => cli_result.included_items.push(folder),
                    Err(e) => errors.push(e),
                },
                ExpectedArgs::Exclude => match check_if_folder_is_valid(&arg) {
                    Ok(folder) => cli_result.excluded_items.push(folder),
                    Err(e) => errors.push(e),
                },
                ExpectedArgs::Referenced => match check_if_folder_is_valid(&arg) {
                    Ok(folder) => {
                        cli_result.included_items.push(folder.clone());
                        cli_result.referenced_items.push(folder);
                    }
                    Err(e) => errors.push(e),
                },
            }
            expected_arg = ExpectedArgs::Include;
        }
    }

    deduplicate_folders(&mut cli_result.included_items);
    deduplicate_folders(&mut cli_result.excluded_items);
    deduplicate_folders(&mut cli_result.referenced_items);

    if !errors.is_empty() {
        warn!("Errors encountered while processing CLI arguments:");
    }
    for error in &errors {
        warn!("{error}");
    }

    if cli_result.included_items.is_empty() && cli_result.excluded_items.is_empty() && cli_result.referenced_items.is_empty() {
        None
    } else {
        Some(cli_result)
    }
}

fn deduplicate_folders(folder_list: &mut Vec<String>) {
    folder_list.sort();
    folder_list.dedup();
}

#[cfg(not(test))]
fn check_if_folder_is_valid(folder: &str) -> Result<String, String> {
    let path = std::path::Path::new(folder);
    if !path.exists() {
        return Err(format!("Folder does not exist: {folder}"));
    }
    if !path.is_dir() {
        return Err(format!("Path is not a directory: {folder}"));
    }
    let canonical_path = dunce::canonicalize(path).map_err(|e| format!("Failed to canonicalize path: {folder}. Error: {e}"))?;

    Ok(canonical_path.to_string_lossy().to_string())
}

#[cfg(test)]
fn check_if_folder_is_valid(folder: &str) -> Result<String, String> {
    if folder.contains("test_error") {
        return Err(format!("Test error for folder: {folder}"));
    }
    Ok(folder.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processes_include_folder() {
        let args = vec!["/valid/folder".to_string()];
        let result = process_cli_args("A", "B", args).expect("TEST");
        assert_eq!(result.included_items, vec!["/valid/folder".to_string()]);
        assert!(result.excluded_items.is_empty());
        assert!(result.referenced_items.is_empty());
    }

    #[test]
    fn processes_exclude_folder() {
        let args = vec!["-e".to_string(), "/valid/folder".to_string()];
        let result = process_cli_args("A", "B", args).expect("TEST");
        assert!(result.included_items.is_empty());
        assert_eq!(result.excluded_items, vec!["/valid/folder".to_string()]);
        assert!(result.referenced_items.is_empty());
    }

    #[test]
    fn processes_referenced_folder() {
        let args = vec!["-r".to_string(), "/valid/folder".to_string()];
        let result = process_cli_args("A", "B", args).expect("TEST");
        assert_eq!(result.included_items, vec!["/valid/folder".to_string()]);
        assert!(result.excluded_items.is_empty());
        assert_eq!(result.referenced_items, vec!["/valid/folder".to_string()]);
    }

    #[test]
    fn processes_multiple_same_folder() {
        let args = [
            "-r",
            "/valid/folder",
            "-r",
            "/valid/folder",
            "normal_folder",
            "abcd",
            "abcd",
            "-e",
            "/exclu",
            "normal_folder",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let result = process_cli_args("A", "B", args).expect("TEST");
        assert_eq!(result.included_items, vec!["/valid/folder".to_string(), "abcd".to_string(), "normal_folder".to_string()]);
        assert_eq!(result.excluded_items, vec!["/exclu".to_string()]);
        assert_eq!(result.referenced_items, vec!["/valid/folder".to_string()]);
    }

    #[test]
    fn handles_invalid_folder() {
        let args = vec!["/invalid/test_error".to_string()];
        let result = process_cli_args("A", "B", args);
        assert!(result.is_none());
    }

    #[test]
    fn handles_no_arguments() {
        let args = vec![];
        let result = process_cli_args("A", "B", args);
        assert!(result.is_none());
    }
}
