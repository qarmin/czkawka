mod model;

use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;
use walkdir::WalkDir;

use crate::model::{BuildOrCheck, CodegenUnits, Config, Debugg, Incremental, OptLevel, OverflowChecks, Panic, Results, SplitDebug, LTO};

const START_CONFIG_TOML: &str = r#"[workspace]
members = [
    "czkawka_core",
    "czkawka_cli",
    "czkawka_gui",
    "krokiet"
]
exclude = [
    "misc/test_read_perf",
    "misc/test_image_perf",
    "misc/test_compilation_speed_size",
    "ci_tester",
]
resolver = "3""#;

fn main() {
    let first_arg = std::env::args().nth(1).expect("Please provide the Czkawka root path");
    let config_toml_path = Path::new(&first_arg).join("Cargo.toml");
    let config_toml_content = fs::read_to_string(&config_toml_path).unwrap_or_else(|err| {
        panic!("Could not read config.toml file at {}: {}", config_toml_path.display(), err);
    });
    if !config_toml_content.starts_with(START_CONFIG_TOML) {
        panic!("The config.toml file does not start with the expected content. Please use czkawka repo.");
    }

    let mut new_content_base = START_CONFIG_TOML.to_string();
    new_content_base.push_str("\n\n\n[profile.fff]\ninherits=\"dev\"");

    let mut results_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(&first_arg).join("compilation_results.txt"))
        .expect("Could not open results file");

    Results::write_header_to_file(&mut results_file).unwrap();

    // ALL configs
    const USE_MOLD: &[bool] = &[false];
    const CRANELIFT: &[bool] = &[true, false];
    const PROJECTS: &[&str] = &["krokiet"];
    const THREADS_NUMBERS: &[u32] = &[0];

    // TEST config
    // const USE_MOLD: &[bool] = &[true];
    // const CRANELIFT: &[bool] = &[false];
    // const PROJECTS: &[&str] = &["krokiet"];
    // const THREADS_NUMBERS: &[u32] = &[24];

    for cranelift in CRANELIFT {
        for config in get_configs(*cranelift) {
            for use_mold in USE_MOLD {
                for project in PROJECTS {
                    for threads_number in THREADS_NUMBERS {
                        let new_content = format!("{new_content_base}\n{}\n", config.to_str());
                        fs::write(&config_toml_path, new_content).expect("Could not write config file");

                        let result = check_compilation_speed_and_size(&first_arg, project, config.clone(), *threads_number, *cranelift, *use_mold);
                        result.save_to_file(&mut results_file).expect("Could not save results to file");
                    }
                }
            }
        }
    }
}

fn get_configs(cranelift: bool) -> Vec<Config> {
    //[profile.dev]
    // opt-level = 0
    // debug = true
    // split-debuginfo = '...'  # Platform-specific.
    // strip = "none"
    // debug-assertions = true
    // overflow-checks = true
    // lto = false
    // panic = 'unwind'
    // incremental = true
    // codegen-units = 256
    // rpath = false

    // [profile.release]
    // opt-level = 3
    // debug = false
    // split-debuginfo = '...'  # Platform-specific.
    // strip = "none"
    // debug-assertions = false
    // overflow-checks = false
    // lto = false
    // panic = 'unwind'
    // incremental = false
    // codegen-units = 16
    // rpath = false
    let debug_base = Config {
        name: "debug",
        lto: LTO::Off,
        debug: Debugg::Full,
        opt_level: OptLevel::Zero,
        build_or_check: BuildOrCheck::Build,
        codegen_units: CodegenUnits::Default,
        panic: Panic::Unwind,
        split_debug: SplitDebug::Off,
        overflow_checks: OverflowChecks::On,
        incremental: Incremental::On,
        build_std: false,
    };
    let release_base = Config {
        name: "release",
        lto: LTO::Off,
        debug: Debugg::None,
        opt_level: OptLevel::Three,
        build_or_check: BuildOrCheck::Build,
        codegen_units: CodegenUnits::Sixteen,
        panic: Panic::Unwind,
        split_debug: SplitDebug::Off,
        overflow_checks: OverflowChecks::Off,
        incremental: Incremental::Off,
        build_std: false,
    };

    let mut debug_fast_check = debug_base.clone();
    debug_fast_check.name = "debug + debug disabled";
    debug_fast_check.debug = Debugg::None;

    let mut check_fast_check = debug_base.clone();
    check_fast_check.name = "check";
    check_fast_check.build_or_check = BuildOrCheck::Check;

    let mut release_thin_lto = release_base.clone();
    release_thin_lto.name = "release + thin lto";
    release_thin_lto.lto = LTO::Thin;

    let mut release_optimize_size = release_base.clone();
    release_optimize_size.name = "release + optimize size";
    release_optimize_size.opt_level = OptLevel::S;

    let mut release_full_lto = release_base.clone();
    release_full_lto.name = "release + fat lto";
    release_full_lto.lto = LTO::Fat;

    let mut release_codegen_units = release_base.clone();
    release_codegen_units.name = "release + codegen units 1";
    release_codegen_units.codegen_units = CodegenUnits::One;

    let mut release_panic_abort = release_base.clone();
    release_panic_abort.name = "release + panic abort";
    release_panic_abort.panic = Panic::Abort;

    let mut release_std = release_base.clone();
    release_std.name = "release + build-std";
    release_std.build_std = true;

    let mut release_fastest = release_base.clone();
    release_fastest.name = "release + fat lto + codegen units 1 + panic abort";
    release_fastest.lto = LTO::Fat;
    release_fastest.codegen_units = CodegenUnits::One;
    release_fastest.panic = Panic::Abort;
    // release_fastest.build_std = true; // I would use it, but fails to compile with lto enabled

    let configs = vec![
        debug_base,
        debug_fast_check,
        check_fast_check,
        release_base,
        release_codegen_units,
        release_std,
        release_panic_abort,
        release_optimize_size,
        release_thin_lto,
        release_full_lto,
        release_fastest,
    ];

    // For cranelift filter out configs with lto which is not supported
    // also build-std panics
    if cranelift {
        configs.into_iter().filter(|config| config.lto == LTO::Off || config.build_std).collect()
    } else {
        configs
    }
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

fn run_cargo_build(project: &str, threads_number: u32, build: BuildOrCheck, cranelift: bool, use_mold: bool, build_std: bool) {
    let build_check = if build == BuildOrCheck::Build { "build" } else { "check" };
    let mut command = std::process::Command::new("cargo");
    command.arg("+nightly");
    if cranelift {
        command.env("CARGO_PROFILE_DEV_CODEGEN_BACKEND", "cranelift");
        command.env("RUSTUP_TOOLCHAIN", "nightly");
        command.arg("-Zcodegen-backend");
    }
    let mut rust_flags = None;
    if use_mold {
        rust_flags = match rust_flags {
            None => Some("-C link-arg=-fuse-ld=mold".to_string()),
            Some(flags) => Some(format!("{} -C link-arg=-fuse-ld=mold", flags)),
        };
    }
    if build_std {
        command.args(&["-Z", "build-std=std"]);
    }

    if let Some(rust_flags) = rust_flags {
        command.env("RUSTFLAGS", rust_flags);
    }

    if threads_number > 0 {
        command.env("CARGO_BUILD_JOBS", threads_number.to_string());
    }

    command
        .arg(build_check)
        .arg("--package")
        .arg(project)
        .arg("--profile")
        .arg("fff")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    println!("Running cargo command: {:?}", command);

    let output = command.output().expect("Failed to execute cargo build");

    if !output.status.success() {
        panic!("Cargo build failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn clean_changes_to_project_files(project: &str) {
    let clean_command = std::process::Command::new("git")
        .arg("checkout")
        .arg(project)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .output()
        .expect("Failed to execute git checkout");
    if !clean_command.status.success() {
        panic!("Git checkout failed: {}", String::from_utf8_lossy(&clean_command.stderr));
    }
}

fn add_empty_line_to_file(project: &str) {
    let file_path = Path::new(project).join("src").join("main.rs");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .expect("Could not open main.rs file");
    if let Err(e) = writeln!(file, "// Absolutelly nothing") {
        panic!("Could not write to main.rs file: {}", e);
    }
}

fn check_compilation_speed_and_size(base: &str, project: &str, config: Config, threads_number: u32, cranelift: bool, use_mold: bool) -> Results {
    clean_cargo();
    clean_changes_to_project_files(project);

    let start_time = std::time::Instant::now();

    println!("Running cargo build for project: {}", project);

    println!("Project: {project}, threads: {threads_number}, {}", config.to_string_short());


    run_cargo_build(project, threads_number, config.build_or_check, cranelift, use_mold, config.build_std);

    let compilation_time = start_time.elapsed();

    let output_file_size = get_size_of_output_file(base, project);
    let target_folder_size = get_size_of_target_folder(base);

    add_empty_line_to_file(project);
    let rebuild_time_start = std::time::Instant::now();
    run_cargo_build(project, threads_number, config.build_or_check, cranelift, use_mold, config.build_std);
    let rebuild_time = rebuild_time_start.elapsed();

    Results {
        output_file_size,
        target_folder_size,
        compilation_time,
        config,
        threads_number,
        project: project.to_string(),
        cranelift,
        use_mold,
        rebuild_time
    }
}

fn get_size_of_output_file(base: &str, project: &str) -> u64 {
    let output_path = Path::new(base).join("target").join("fff").join(project);
    if output_path.exists() {
        output_path.metadata().map(|e| e.len()).unwrap_or_default()
    } else {
        0
    }
}

fn get_size_of_target_folder(base: &str) -> u64 {
    let target_path = Path::new(base).join("target");
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
