use std::process;

use log::{error, warn};

use crate::common::config_cache_path::get_config_cache_path;
use crate::common::model::ToolType;
use crate::{CZKAWKA_VERSION, flc};

#[derive(Clone, Debug)]
pub struct CliResult {
    pub included_items: Vec<String>,
    pub excluded_items: Vec<String>,
    pub referenced_items: Vec<String>,
    pub tool: Option<ToolType>,
    pub preset: Option<i32>,
    pub start_scan: bool,
    pub exit_after_scan: bool,
}

const TOOL_NAME_MAPPING: &[(&str, ToolType)] = &[
    ("duplicates", ToolType::Duplicate),
    ("empty-folders", ToolType::EmptyFolders),
    ("biggest-files", ToolType::BigFile),
    ("empty-files", ToolType::EmptyFiles),
    ("temporary", ToolType::TemporaryFiles),
    ("similar-images", ToolType::SimilarImages),
    ("similar-videos", ToolType::SimilarVideos),
    ("same-music", ToolType::SameMusic),
    ("invalid-symlinks", ToolType::InvalidSymlinks),
    ("broken-files", ToolType::BrokenFiles),
    ("bad-extensions", ToolType::BadExtensions),
    ("bad-names", ToolType::BadNames),
    ("exif-remover", ToolType::ExifRemover),
    ("video-optimizer", ToolType::VideoOptimizer),
];

fn tool_type_from_cli_name(name: &str) -> Option<ToolType> {
    TOOL_NAME_MAPPING.iter().find(|(candidate, _)| *candidate == name).map(|(_, tool)| *tool)
}

enum ExpectedArgs {
    Include,
    Exclude,
    Referenced,
    Tool,
    Preset,
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
        println!(
            "  -t TOOL, --tool TOOL  Select the active tool. One of: {}",
            TOOL_NAME_MAPPING.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(", ")
        );
        println!("  -p NUMBER, --preset NUMBER       Select which settings preset to load");
        println!("  -s, --scan            Automatically start a scan on launch");
        println!("  -x, --exit            Close the app once the auto-started scan finishes (requires --scan)");
        println!("  --cache, -c           Opens the cache folder");
        println!("  --config, -C          Opens the config folder");
        println!("  --help, -h            Show this help message");
        println!("  --version, -v         Show version information");
        println!("Examples:");
        println!("  {app_exec} /path/absolute/to/folder -e relative_path/2 -r /path/to/referenced");
        println!("  {app_exec} . folder2 folder3");
        println!("  {app_exec} -t similar-images -s -x /path/to/folder");
        println!("If none of the above are specified, the program launches normally with saved settings.");
        process::exit(0);
    }
    if ["--version", "-v"].iter().any(|&arg| args.contains(&arg.to_string())) {
        let git_commit = env!("CZKAWKA_GIT_COMMIT_SHORT");
        let official_build = if env!("CZKAWKA_OFFICIAL_BUILD") == "1" {
            "O" // Official build
        } else {
            "U" // Unofficial build
        };
        let git_date = env!("CZKAWKA_GIT_COMMIT_DATE");
        println!("{app_display} version {CZKAWKA_VERSION}({git_commit} {official_build} {git_date})");
        process::exit(0);
    }

    let mut expected_arg = ExpectedArgs::Include;
    let mut cli_result = CliResult {
        included_items: Vec::new(),
        excluded_items: Vec::new(),
        referenced_items: Vec::new(),
        tool: None,
        preset: None,
        start_scan: false,
        exit_after_scan: false,
    };
    let mut errors = Vec::new();

    for arg in args {
        if arg.starts_with("-") {
            match arg.as_str() {
                "-e" | "--exclude" => expected_arg = ExpectedArgs::Exclude,
                "-r" | "--referenced" => expected_arg = ExpectedArgs::Referenced,
                "-t" | "--tool" => expected_arg = ExpectedArgs::Tool,
                "-p" | "--preset" => expected_arg = ExpectedArgs::Preset,
                "-s" | "--scan" => cli_result.start_scan = true,
                "-x" | "--exit" => cli_result.exit_after_scan = true,
                "-c" | "--cache" => {
                    if let Some(cfg) = get_config_cache_path() {
                        if let Err(e) = open::that(&cfg.cache_folder) {
                            error!("Failed to open cache folder \"{}\": {e}", cfg.cache_folder.to_string_lossy());
                            process::exit(1);
                        }
                        process::exit(0);
                    } else {
                        error!("Failed to get cache folder path");
                        process::exit(1);
                    }
                }
                "-C" | "--config" => {
                    if let Some(cfg) = get_config_cache_path() {
                        if let Err(e) = open::that(&cfg.config_folder) {
                            error!("Failed to open config folder \"{}\": {e}", cfg.config_folder.to_string_lossy());
                            process::exit(1);
                        }
                        process::exit(0);
                    } else {
                        error!("Failed to get config folder path");
                        process::exit(1);
                    }
                }
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
                ExpectedArgs::Tool => match tool_type_from_cli_name(&arg) {
                    Some(tool) => cli_result.tool = Some(tool),
                    None => {
                        let valid_names = TOOL_NAME_MAPPING.iter().map(|(name, _)| *name).collect::<Vec<_>>().join(", ");
                        eprintln!("Unknown tool: {arg}. Valid values: {valid_names}");
                        process::exit(1);
                    }
                },
                ExpectedArgs::Preset => match arg.parse::<i32>() {
                    Ok(preset) if preset >= 1 => cli_result.preset = Some(preset),
                    _ => {
                        eprintln!("Invalid preset number: {arg}. Must be a positive integer.");
                        process::exit(1);
                    }
                },
            }
            expected_arg = ExpectedArgs::Include;
        }
    }

    if cli_result.exit_after_scan && !cli_result.start_scan {
        eprintln!("--exit can only be used together with --scan");
        process::exit(1);
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

    if cli_result.included_items.is_empty()
        && cli_result.excluded_items.is_empty()
        && cli_result.referenced_items.is_empty()
        && cli_result.tool.is_none()
        && cli_result.preset.is_none()
        && !cli_result.start_scan
        && !cli_result.exit_after_scan
    {
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
        return Err(flc!("core_folder_does_not_exist", folder = folder));
    }
    if !path.is_dir() {
        return Err(flc!("core_path_not_directory", folder = folder));
    }
    let canonical_path = dunce::canonicalize(path).map_err(|e| format!("Failed to canonicalize path: {folder}. Error: {e}"))?;

    Ok(canonical_path.to_string_lossy().to_string())
}

#[cfg(test)]
fn check_if_folder_is_valid(folder: &str) -> Result<String, String> {
    if folder.contains("test_error") {
        return Err(flc!("core_test_error_for_folder", folder = folder));
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
        let args = Vec::new();
        let result = process_cli_args("A", "B", args);
        assert!(result.is_none());
    }

    #[test]
    fn processes_tool_selection() {
        let args = vec!["--tool".to_string(), "similar-images".to_string()];
        let result = process_cli_args("A", "B", args).expect("TEST");
        assert_eq!(result.tool, Some(ToolType::SimilarImages));
    }

    #[test]
    fn processes_preset_selection() {
        let args = vec!["-p".to_string(), "3".to_string()];
        let result = process_cli_args("A", "B", args).expect("TEST");
        assert_eq!(result.preset, Some(3));
    }

    #[test]
    fn processes_scan_and_exit_flags() {
        let args = vec!["-s".to_string(), "-x".to_string()];
        let result = process_cli_args("A", "B", args).expect("TEST");
        assert!(result.start_scan);
        assert!(result.exit_after_scan);
    }

    #[test]
    fn scan_alone_does_not_imply_exit() {
        let args = vec!["--scan".to_string()];
        let result = process_cli_args("A", "B", args).expect("TEST");
        assert!(result.start_scan);
        assert!(!result.exit_after_scan);
    }

    #[test]
    fn tool_and_preset_default_to_none() {
        let args = vec!["/valid/folder".to_string()];
        let result = process_cli_args("A", "B", args).expect("TEST");
        assert_eq!(result.tool, None);
        assert_eq!(result.preset, None);
    }
}
