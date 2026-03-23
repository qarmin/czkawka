use std::env;
use std::path::PathBuf;

fn main() {
    slint_build::compile_with_config("ui/main_window.slint", slint_build::CompilerConfiguration::new().with_style("material".into()))
        .expect("Unable to compile Slint UI files for Cedinia");

    // Compile Java helper classes to DEX when building for Android.
    // This follows the same pattern used by the Slint android-activity backend.
    if env::var("TARGET").unwrap_or_default().contains("android") {
        compile_android_java();
    }
}

fn compile_android_java() {
    use android_build::{Dexer, JavaBuild};

    let java_files = ["java/CediniaActivity.java", "java/CediniaFilePicker.java", "java/CediniaPickerFragment.java"];

    let out_dir: PathBuf = env::var_os("OUT_DIR").expect("OUT_DIR environment variable not set").into();
    let out_class_dir = out_dir.join("java_classes");

    if out_class_dir.try_exists().unwrap_or(false) {
        let _ = std::fs::remove_dir_all(&out_class_dir);
    }
    std::fs::create_dir_all(&out_class_dir).unwrap_or_else(|e| panic!("Cannot create output directory {out_class_dir:?} - {e}"));

    let android_jar = android_build::android_jar(None).expect("No Android platform SDK found; install SDK");

    let release_mode = env::var("PROFILE").as_ref().map(|s| s.as_str()) == Ok("release");

    // Compile all Java sources to .class files.
    let mut build = JavaBuild::new();
    for f in &java_files {
        build.file(f);
    }
    let status = build
        .class_path(&android_jar)
        .classes_out_dir(&out_class_dir)
        .java_source_version(8)
        .java_target_version(8)
        .debug_info(android_build::DebugInfo {
            line_numbers: !release_mode,
            variables: !release_mode,
            source_files: !release_mode,
        })
        .command()
        .unwrap_or_else(|e| panic!("Could not generate javac command: {e}"))
        .args(["-encoding", "UTF-8"])
        .output()
        .unwrap_or_else(|e| panic!("Could not run javac: {e}"));

    assert!(status.status.success(), "Java compilation failed:\n{}", String::from_utf8_lossy(&status.stderr));

    // Convert .class files to a single classes.dex.
    let dex_out = Dexer::new()
        .android_jar(&android_jar)
        .class_path(&out_class_dir)
        .collect_classes(&out_class_dir)
        .expect("Failed to collect compiled Java classes for DEX conversion")
        .release(release_mode)
        .android_min_api(21)
        .out_dir(&out_dir)
        .command()
        .unwrap_or_else(|e| panic!("Could not generate D8 command: {e}"))
        .output()
        .unwrap_or_else(|e| panic!("Error running D8: {e}"));

    assert!(dex_out.status.success(), "Dex conversion failed:\n{}", String::from_utf8_lossy(&dex_out.stderr));

    for f in &java_files {
        println!("cargo:rerun-if-changed={f}");
    }
}
