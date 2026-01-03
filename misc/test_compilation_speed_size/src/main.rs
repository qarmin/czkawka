mod model;
mod new_chart;

use crate::model::{
    BuildConfig, BuildOrCheck,  Config,  Panic, Project, Results,
};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use walkdir::WalkDir;
// use crate::new_chart::create_chart;

const PROFILE_NAME: &str = "fff";
const RESULTS_FILE_NAME: &str = "compilation_results.txt";

fn main() {
    // create_chart(); // TODO currently is broken a little
    let Some(first_arg) = std::env::args().nth(1) else {
        eprintln!("Please provide a path to the configuration json file as the first argument.");
        exit(1);
    };

    let cargo_toml_path = Path::new("Cargo.toml");
    if !cargo_toml_path.is_file() {
        eprintln!("Cannot find Cargo.toml in the current directory. Please run this script from the root cargo directory(must be able to modify profiles).");
        exit(1);
    }

    clean_changes_to_project_files("Cargo.toml");

    let Ok(cargo_toml_content) = fs::read_to_string(&cargo_toml_path) else {
        eprintln!("Could not read content of Cargo.toml file");
        exit(1);
    };

    let Ok(config_json_content) = fs::read_to_string(&first_arg) else {
        eprintln!("Could not read content of the provided json file: {}", first_arg);
        exit(1);
    };

    let mut config = match serde_json::from_str::<Config>(&config_json_content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not parse content of the provided json file: {}. Error: {}", first_arg, e);
            exit(1);
        }
    };

    let mut results_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(RESULTS_FILE_NAME))
        .expect("Could not open results file");

    Results::write_header_to_file(&mut results_file).unwrap();

    config.build_config_converted = config.build_config.clone().into_iter().map(|e| e.into()).collect();
    let mut all_configs = Vec::new();
    for build_config in &config.build_config_converted {
        all_configs.push(build_config.clone());
    }

    println!("Found {} configurations to test", all_configs.len());

    // let mut results = Vec::new();
    for build_config in all_configs {
        let new_cargo_toml_content = format!("{cargo_toml_content}\n\n[profile.{PROFILE_NAME}]\n{}\n", build_config.to_str());
        fs::write(&cargo_toml_path, new_cargo_toml_content).expect("Could not write Cargo.toml file");
        let result = check_compilation_speed_and_size(&build_config, &config.project);
        // results.push(result.clone());
        result.save_to_file(&mut results_file).expect("Could not save results to file");
    }

    fs::write(&cargo_toml_path, cargo_toml_content).expect("Could not restore content of Cargo.toml file");
}

fn clean_cargo() {
    println!("Cleaning cargo...");
    let output = std::process::Command::new("cargo")
        .arg("clean")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .expect("Failed to execute cargo clean");

    if !output.status.success() {
        panic!("Cargo clean failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn run_cargo_build(build_config: &BuildConfig, project: &Project) {
    let build_check = if build_config.build_or_check == BuildOrCheck::Build { "build" } else { "check" };
    let mut command = std::process::Command::new("cargo");
    command.arg("+nightly");
    if build_config.cranelift {
        command.env("CARGO_PROFILE_DEV_CODEGEN_BACKEND", "cranelift");
        command.env("RUSTUP_TOOLCHAIN", "nightly");
        command.arg("-Zcodegen-backend");
    }
    let mut rust_flags = None;
    // TODO - not works currently
    // if mold {
    //     let to_add = "-C link-arg=-fuse-ld=mold";
    //     rust_flags = match rust_flags {
    //         None => Some(to_add.to_string()),
    //         Some(flags) => Some(format!("{flags} {to_add}")),
    //     };
    // }
    if build_config.build_std {
        if build_config.panic == Panic::Abort {
            command.args(["-Z", "build-std=std,panic_abort"]);
        } else {
            command.args(["-Z", "build-std=std"]);
        }
    }
    if build_config.native {
        let to_add = "-C target-cpu=native";
        rust_flags = match rust_flags {
            None => Some(to_add.to_string()),
            Some(flags) => Some(format!("{flags} {to_add}")),
        };
    }

    if let Some(rust_flags) = rust_flags {
        command.env("RUSTFLAGS", rust_flags);
    }

    // if threads_number > 0 {
    // Looks that not all steps uses this variable - but I may be wrong
    //     command.env("CARGO_BUILD_JOBS", threads_number.to_string());
    // }

    command
        .arg(build_check)
        .arg("--bin")
        .arg(&project.name)
        .arg("--profile")
        .arg(PROFILE_NAME)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    println!("Running cargo command: {:?}", command);

    let output = command.output().expect("Failed to execute cargo build");

    if !output.status.success() {
        panic!("Cargo build failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn clean_changes_to_project_files(path: &str) {
    let clean_command = std::process::Command::new("git")
        .arg("checkout")
        .arg(path)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .expect("Failed to execute git checkout");
    if !clean_command.status.success() {
        panic!("Git checkout failed: {}", String::from_utf8_lossy(&clean_command.stderr));
    }
}

fn add_empty_line_to_file(project: &Project) {
    let file_path = Path::new(&project.path_to_main_rs_file);
    let mut file = OpenOptions::new().append(true).open(&file_path).expect("Could not open main.rs file");
    if let Err(e) = writeln!(file, "// Absolutely nothing") {
        panic!("Could not write to main.rs file: {}", e);
    }
}

fn check_compilation_speed_and_size(build_config: &BuildConfig, project: &Project) -> Results {
    clean_cargo();
    clean_changes_to_project_files(&project.path_to_clean_with_git);

    let start_time = std::time::Instant::now();

    println!("Running cargo build for project: {}", project.name);
    println!("Build_config: {}", build_config.to_string_short());

    run_cargo_build(build_config, project);

    let compilation_time = start_time.elapsed();

    let output_file_size = get_size_of_output_file(project);
    let target_folder_size = get_size_of_target_folder();

    add_empty_line_to_file(project);

    let rebuild_time_start = std::time::Instant::now();
    run_cargo_build(build_config, project);
    let rebuild_time = rebuild_time_start.elapsed();

    clean_cargo();
    clean_changes_to_project_files(&project.path_to_clean_with_git);

    Results {
        output_file_size,
        target_folder_size,
        compilation_time,
        build_config: build_config.clone(),
        project: project.clone(),
        rebuild_time,
    }
}

fn get_size_of_output_file(project: &Project) -> u64 {
    let output_path = Path::new("target").join(PROFILE_NAME).join(&project.name);
    if output_path.exists() {
        output_path.metadata().map(|e| e.len()).unwrap_or_default()
    } else {
        0
    }
}

fn get_size_of_target_folder() -> u64 {
    let target_path = Path::new("target");
    get_size_of_files_in_folder(&target_path)
}

fn get_size_of_files_in_folder(folder: &Path) -> u64 {
    WalkDir::new(folder)
        .max_depth(999)
        .into_iter()
        .flatten()
        .map(|e| e.metadata().map(|e| e.len()).unwrap_or_default())
        .sum()
}
