mod model;

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use walkdir::WalkDir;

use crate::model::{BuildOrCheck, CodegenUnits, BuildConfig, Debugg, Incremental, Lto, OptLevel, OverflowChecks, Panic, Results, SplitDebug, GeneralConfig, Config, Cranelift, Project};

const PROFILE_NAME: &str = "fff";

fn main() {
    let cargo_toml_path = Path::new("Cargo.toml");
    let canonicalize = cargo_toml_path.canonicalize().unwrap();
    if !cargo_toml_path.is_file() {
        eprintln!("Cannot find Cargo.toml in the current directory. Please run this script from the root cargo directory(must be able to modify profiles).");
        std::process::exit(1);
    }

    let Ok(cargo_toml_content) = fs::read_to_string(&cargo_toml_path) else {
        eprintln!("Could not read contnet of Cargo.toml file");
        std::process::exit(1);
    };

    let Some(first_arg) = std::env::args().nth(1) else {
        eprintln!("Please provide the path to json config info as first argument.");
        std::process::exit(1);
    };
    let Ok(config_json_content) = fs::read_to_string(&first_arg) else {
        eprintln!("Could not read content of the provided json file: {}", first_arg);
        std::process::exit(1);
    };

    let Ok( mut config) = serde_json::from_str::<Config>(&config_json_content) else {
        eprintln!("Could not parse content of the provided json file: {}", first_arg);
        std::process::exit(1);
    };

    let mut results_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(&first_arg).join("compilation_results.txt"))
        .expect("Could not open results file");

    Results::write_header_to_file(&mut results_file).unwrap();

    config.build_config_converted = config.build_config.clone().into_iter().map(|e|e.into()).collect();
    let g = config.general_config.clone();
    let mut all_configs = vec![];
    for mold in g.mold.to_options() {
        for cranelift in g.cranelift.to_options() {
            for  build_config in  &config.build_config_converted {
                all_configs.push((mold, cranelift, build_config.clone()));
            }
        }
    }

    println!("Found {} configurations to test", all_configs.len());

    // let mut results = vec![];
    for (mold, cranelift, build_config) in all_configs {
        let new_cargo_toml_content = format!("[profile.{PROFILE_NAME}]\n{cargo_toml_content}\n{}\n", build_config.to_str());
        fs::write(&cargo_toml_path, new_cargo_toml_content).expect("Could not write Cargo.toml file");
        let result = check_compilation_speed_and_size(*mold, *cranelift, &build_config, &config.project);
        // results.push(result.clone());
        result.save_to_file(&mut results_file).expect("Could not save results to file");
    }

    fs::write(&cargo_toml_path, cargo_toml_content).expect("Could not restore content of Cargo.toml file");

    // for cranelift in CRANELIFT {
    //     for config in get_configs(*cranelift) {
    //         for use_mold in USE_MOLD {
    //             for project in PROJECTS {
    //                 for threads_number in THREADS_NUMBERS {
    //                     let new_content = format!("{new_content_base}\n{}\n", config.to_str());
    //                     fs::write(&config_toml_path, new_content).expect("Could not write config file");
    //
    //                     let result = check_compilation_speed_and_size(&first_arg, project, config.clone(), *threads_number, *cranelift, *use_mold);
    //                     result.save_to_file(&mut results_file).expect("Could not save results to file");
    //                 }
    //             }
    //         }
    //     }
    // }
}

fn get_configs(cranelift: bool) -> Vec<BuildConfig> {
    vec![]
    // //[profile.dev]
    // // opt-level = 0
    // // debug = true
    // // split-debuginfo = '...'  # Platform-specific.
    // // strip = "none"
    // // debug-assertions = true
    // // overflow-checks = true
    // // lto = false
    // // panic = 'unwind'
    // // incremental = true
    // // codegen-units = 256
    // // rpath = false
    //
    // // [profile.release]
    // // opt-level = 3
    // // debug = false
    // // split-debuginfo = '...'  # Platform-specific.
    // // strip = "none"
    // // debug-assertions = false
    // // overflow-checks = false
    // // lto = false
    // // panic = 'unwind'
    // // incremental = false
    // // codegen-units = 16
    // // rpath = false
    // let debug_base = BuildConfig {
    //     name: "debug",
    //     lto: Lto::Off,
    //     debug: Debugg::Full,
    //     opt_level: OptLevel::Zero,
    //     build_or_check: BuildOrCheck::Build,
    //     codegen_units: CodegenUnits::Default,
    //     panic: Panic::Unwind,
    //     split_debug: SplitDebug::Off,
    //     overflow_checks: OverflowChecks::On,
    //     incremental: Incremental::On,
    //     build_std: false,
    // };
    // let release_base = BuildConfig {
    //     name: "release",
    //     lto: Lto::Off,
    //     debug: Debugg::None,
    //     opt_level: OptLevel::Three,
    //     build_or_check: BuildOrCheck::Build,
    //     codegen_units: CodegenUnits::Sixteen,
    //     panic: Panic::Unwind,
    //     split_debug: SplitDebug::Off,
    //     overflow_checks: OverflowChecks::Off,
    //     incremental: Incremental::Off,
    //     build_std: false,
    // };
    //
    // let mut debug_fast_check = debug_base.clone();
    // debug_fast_check.name = "debug + debug disabled";
    // debug_fast_check.debug = Debugg::None;
    //
    // let mut check_fast_check = debug_base.clone();
    // check_fast_check.name = "check";
    // check_fast_check.build_or_check = BuildOrCheck::Check;
    //
    // // Debug split is strange, because resulted binary was same as debug without split debug
    // // Need to verify if it works with this configuration
    // // let mut debug_split_debug = debug_base.clone();
    // // debug_split_debug.name = "debug + split debug";
    // // debug_split_debug.split_debug = SplitDebug::Unpacked;
    //
    // let mut release_with_debug = release_base.clone();
    // release_with_debug.name = "release + debug info";
    // release_with_debug.debug = Debugg::Full;
    //
    // let mut release_o2 = release_base.clone();
    // release_o2.name = "release + opt o2";
    // release_o2.opt_level = OptLevel::Two;
    //
    // let mut release_o1 = release_base.clone();
    // release_o1.name = "release + opt o1";
    // release_o1.opt_level = OptLevel::One;
    //
    // let mut release_thin_lto = release_base.clone();
    // release_thin_lto.name = "release + thin lto";
    // release_thin_lto.lto = Lto::Thin;
    //
    // let mut release_optimize_size = release_base.clone();
    // release_optimize_size.name = "release + optimize size";
    // release_optimize_size.opt_level = OptLevel::S;
    //
    // let mut release_full_lto = release_base.clone();
    // release_full_lto.name = "release + fat lto";
    // release_full_lto.lto = Lto::Fat;
    //
    // let mut release_codegen_units = release_base.clone();
    // release_codegen_units.name = "release + cu 1";
    // release_codegen_units.codegen_units = CodegenUnits::One;
    //
    // let mut release_panic_abort = release_base.clone();
    // release_panic_abort.name = "release + panic abort";
    // release_panic_abort.panic = Panic::Abort;
    //
    // let mut release_std = release_base.clone();
    // release_std.name = "release + build-std";
    // release_std.build_std = true;
    //
    // let mut release_fastest = release_base.clone();
    // release_fastest.name = "release + fat lto + cu 1 + panic abort";
    // release_fastest.lto = Lto::Fat;
    // release_fastest.codegen_units = CodegenUnits::One;
    // release_fastest.panic = Panic::Abort;
    //
    // let mut release_fastest_with_build_std = release_base.clone();
    // release_fastest_with_build_std.name = "release + fat lto + cu 1 + panic abort + build-std";
    // release_fastest_with_build_std.lto = Lto::Fat;
    // release_fastest_with_build_std.codegen_units = CodegenUnits::One;
    // release_fastest_with_build_std.panic = Panic::Abort;
    // release_fastest_with_build_std.build_std = true;
    //
    // let mut release_incremental = release_base.clone();
    // release_incremental.name = "release + incremental";
    // release_incremental.incremental = Incremental::On;
    //
    // let mut release_incremental_lto = release_base.clone();
    // release_incremental_lto.name = "release + incremental + fat lto";
    // release_incremental_lto.incremental = Incremental::On;
    // release_incremental_lto.lto = Lto::Fat;
    //
    // let configs = vec![
    //     // debug_split_debug,
    //     debug_base,
    //     debug_fast_check,
    //     check_fast_check,
    //     release_base,
    //     release_codegen_units,
    //     release_with_debug,
    //     release_std,
    //     release_panic_abort,
    //     release_optimize_size,
    //     release_thin_lto,
    //     release_full_lto,
    //     release_fastest,
    //     release_o2,
    //     release_o1,
    //     release_fastest_with_build_std,
    //     release_incremental,
    //     release_incremental_lto
    // ];
    //
    // // For cranelift filter out configs with lto which is not supported
    // // also build-std panics
    // if cranelift {
    //     configs.into_iter().filter(|config| config.lto == Lto::Off && !config.build_std).collect()
    // } else {
    //     configs
    // }
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

fn run_cargo_build(
    mold: bool,
    cranelift: bool,
    build_config: &BuildConfig,
    project: &Project,
) {
    return; // TODO only for tests
    let build_check = if build_config.build_or_check == BuildOrCheck::Build { "build" } else { "check" };
    let mut command = std::process::Command::new("cargo");
    command.arg("+nightly");
    if cranelift {
        command.env("CARGO_PROFILE_DEV_CODEGEN_BACKEND", "cranelift");
        command.env("RUSTUP_TOOLCHAIN", "nightly");
        command.arg("-Zcodegen-backend");
    }
    let mut rust_flags = None;
    if mold {
        rust_flags = match rust_flags {
            None => Some("-C link-arg=-fuse-ld=mold".to_string()),
            Some(flags) => Some(format!("{} -C link-arg=-fuse-ld=mold", flags)),
        };
    }
    if build_config.build_std {
        if build_config.panic == Panic::Abort {
            command.args(["-Z", "build-std=std,panic_abort"]);
        } else {
            command.args(["-Z", "build-std=std"]);
        }
    }

    if let Some(rust_flags) = rust_flags {
        command.env("RUSTFLAGS", rust_flags);
    }

    // if threads_number > 0 {
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

fn clean_changes_to_project_files(project: &Project) {
    let clean_command = std::process::Command::new("git")
        .arg("checkout")
        .arg(&project.path_to_clean_with_git)
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
    if let Err(e) = writeln!(file, "// Absolutelly nothing") {
        panic!("Could not write to main.rs file: {}", e);
    }
}

fn check_compilation_speed_and_size(mold: bool, cranelift: bool, build_config: &BuildConfig, project: &Project) -> Results {
    clean_cargo();
    clean_changes_to_project_files();

    let start_time = std::time::Instant::now();

    println!("Running cargo build for project: {}", project.name);

    run_cargo_build(mold, cranelift, build_config, project);

    let compilation_time = start_time.elapsed();

    let output_file_size = get_size_of_output_file(project);
    let target_folder_size = get_size_of_target_folder();

    add_empty_line_to_file(project);
    let rebuild_time_start = std::time::Instant::now();
    run_cargo_build(mold, cranelift, build_config, project);
    let rebuild_time = rebuild_time_start.elapsed();

    Results {
        output_file_size,
        target_folder_size,
        compilation_time,
        build_config: build_config.clone(),
        project: project.clone(),
        cranelift,
        mold,
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
