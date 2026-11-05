//! What the remembered encoder configurations are for, end to end.
//!
//! Driven through the real binary rather than the library, because the profile library is
//! process-global: a unit test that loaded a store would leave those configurations behind for
//! every other test in the binary, and "this file is unrecoverable without a reference" is an
//! assertion several of them make. A subprocess starts with an empty library every time.

use std::path::{Path, PathBuf};
use std::process::Command;

fn sample() -> PathBuf {
    PathBuf::from("tmp/samples/clip_hevc.mp4")
}

fn ffmpeg_is_available() -> bool {
    Command::new("ffprobe")
        .arg("-version")
        .output()
        .is_ok_and(|output| output.status.success())
}

/// A copy with its last 40% cut off, which for this sample takes the whole index with it.
fn index_less_copy(directory: &Path) -> PathBuf {
    let data = std::fs::read(sample()).expect("the sample is readable");
    let path = directory.join("cut.mp4");
    std::fs::write(&path, data.get(..data.len() * 6 / 10).expect("a prefix")).expect("write");
    path
}

fn run(arguments: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_file_fixer"))
        .args(arguments)
        .output()
        .expect("the binary runs");
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// The headline of the whole feature: scan a camera's healthy footage once, and a recording it
/// produced becomes repairable later with nothing named on the command line.
#[test]
fn a_remembered_configuration_repairs_a_file_no_reference_was_given_for() {
    if !ffmpeg_is_available() || !sample().exists() {
        return;
    }
    let work = tempfile::tempdir().expect("temp dir");
    let store = work.path().join("profiles.json");
    let damaged = index_less_copy(work.path());
    let out = work.path().join("out");
    let store = store.to_string_lossy().into_owned();
    let damaged = damaged.to_string_lossy().into_owned();
    let out = out.to_string_lossy().into_owned();

    // Without anything remembered there is no way to read the stream, and it must say so
    // rather than invent something.
    let before = run(&[
        &damaged,
        "--mode",
        "aggressive",
        "-o",
        &out,
        "--force",
        "--profile-store",
        &store,
    ]);
    assert!(before.contains("unrecoverable"), "{before}");

    let scanned = run(&["scan-profiles", "tmp/samples", "--profile-store", &store]);
    assert!(scanned.contains("configuration(s) remembered"), "{scanned}");

    let after = run(&[
        &damaged,
        "--mode",
        "aggressive",
        "-o",
        &out,
        "--force",
        "--profile-store",
        &store,
    ]);
    assert!(after.contains("recalling"), "the store has to be consulted: {after}");
    assert!(
        after.contains("mp4_rebuild_index"),
        "the remembered configuration has to make the rebuild possible: {after}"
    );
}

/// A store nobody has written to yet is not an error, and listing it says so plainly.
#[test]
fn listing_an_empty_store_is_not_a_failure() {
    let work = tempfile::tempdir().expect("temp dir");
    let store = work.path().join("nothing-here.json");
    let printed = run(&["scan-profiles", "--list", "--profile-store", &store.to_string_lossy()]);

    assert!(printed.contains("nothing remembered yet"), "{printed}");
}
