use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use png::{BitDepth, ColorType, Encoder};
use tempfile::TempDir;

use super::*;
use crate::common::config_cache_path::set_config_cache_path_test;
use crate::file_fixer::corrupt::{DamageKind, DamageSpec, Fraction, apply_damage};
use crate::file_fixer::formats::png_chunks::split_idat;

fn noisy_png() -> Vec<u8> {
    let (width, height) = (64u32, 200u32);
    let mut state = 12345u32;
    let pixels: Vec<u8> = (0..width * height * 3)
        .map(|_| {
            state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (state >> 24) as u8
        })
        .collect();
    let mut out = Vec::new();
    let mut encoder = Encoder::new(Cursor::new(&mut out), width, height);
    encoder.set_color(ColorType::Rgb);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
    writer.finish().unwrap();
    split_idat(&out, 2048)
}

fn write_truncated_png(directory: &Path) -> PathBuf {
    let keep = Fraction::new(0.6).unwrap();
    let damaged = apply_damage(&noisy_png(), &DamageSpec::new(DamageKind::TruncateTail { keep }, 1));
    let path = directory.join("photo.png");
    std::fs::write(&path, damaged.data).unwrap();
    path
}

fn not_stopped() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}

fn fix_parameters(output_dir: Option<PathBuf>, replace_original: bool) -> FixParameters {
    FixParameters {
        preset: RepairPreset::Balanced,
        references: Vec::new(),
        output_dir,
        replace_original,
        time_budget: std::time::Duration::from_secs(20),
    }
}

fn use_temporary_cache(directory: &TempDir) {
    set_config_cache_path_test(directory.path().join("cache"), directory.path().join("config"));
}

#[test]
fn a_truncated_png_needs_the_balanced_preset() {
    let directory = tempfile::tempdir().unwrap();
    use_temporary_cache(&directory);
    let path = write_truncated_png(directory.path());
    let size = std::fs::metadata(&path).unwrap().len();

    let result = assess_file(&path, size, &AssessParameters::default(), &not_stopped());

    assert_eq!(result, Some(Repairability::Repairable(RepairPreset::Balanced)));
}

#[test]
fn a_healthy_png_is_not_reported_as_repairable() {
    let directory = tempfile::tempdir().unwrap();
    use_temporary_cache(&directory);
    let path = directory.path().join("healthy.png");
    std::fs::write(&path, noisy_png()).unwrap();

    assert_eq!(assess_file(&path, 1, &AssessParameters::default(), &not_stopped()), Some(Repairability::NotRepairable));
}

#[test]
fn files_above_the_size_limit_are_skipped() {
    let directory = tempfile::tempdir().unwrap();
    use_temporary_cache(&directory);
    let path = write_truncated_png(directory.path());
    let params = AssessParameters {
        max_file_size: 10,
        ..AssessParameters::default()
    };

    assert_eq!(assess_file(&path, 1000, &params, &not_stopped()), Some(Repairability::TooLarge));
}

#[test]
fn an_unknown_extension_is_not_repairable() {
    assert_eq!(
        assess_file(Path::new("notes.txt"), 1, &AssessParameters::default(), &not_stopped()),
        Some(Repairability::NotRepairable)
    );
}

#[test]
fn fixing_writes_a_copy_and_leaves_the_original_untouched() {
    let directory = tempfile::tempdir().unwrap();
    use_temporary_cache(&directory);
    let path = write_truncated_png(directory.path());
    let original = std::fs::read(&path).unwrap();

    let outcome = fix_file(&path, &fix_parameters(None, false), &not_stopped());

    let FixOutcome::Fixed { output, replaced_original, .. } = outcome else {
        panic!("expected a repair, got {outcome:?}");
    };
    assert!(!replaced_original);
    assert_ne!(output, path);
    assert!(output.exists());
    assert_eq!(std::fs::read(&path).unwrap(), original);
}

#[test]
fn replacing_the_original_gives_the_repaired_file_its_name() {
    let directory = tempfile::tempdir().unwrap();
    let original = directory.path().join("clip.mp4");
    let repaired = directory.path().join("clip.fixedffile.mp4");
    std::fs::write(&original, b"damaged").unwrap();
    std::fs::write(&repaired, b"repaired").unwrap();

    let result = fix::replace_original(&original, &repaired, |path| std::fs::remove_file(path).map_err(|e| e.to_string()));

    assert_eq!(result, Ok(()));
    assert_eq!(std::fs::read(&original).unwrap(), b"repaired");
    assert!(!repaired.exists());
}

#[test]
fn a_failed_removal_leaves_both_files_in_place() {
    let directory = tempfile::tempdir().unwrap();
    let original = directory.path().join("clip.mp4");
    let repaired = directory.path().join("clip.fixedffile.mp4");
    std::fs::write(&original, b"damaged").unwrap();
    std::fs::write(&repaired, b"repaired").unwrap();

    let result = fix::replace_original(&original, &repaired, |_| Err("no trash".to_string()));

    assert_eq!(result, Err("no trash".to_string()));
    assert_eq!(std::fs::read(&original).unwrap(), b"damaged");
    assert_eq!(std::fs::read(&repaired).unwrap(), b"repaired");
}

#[test]
fn a_set_stop_flag_cancels_without_writing_anything() {
    let directory = tempfile::tempdir().unwrap();
    use_temporary_cache(&directory);
    let path = write_truncated_png(directory.path());
    let stop = Arc::new(AtomicBool::new(true));

    assert_eq!(fix_file(&path, &fix_parameters(None, false), &stop), FixOutcome::Cancelled);
    assert_eq!(assess_file(&path, 1, &AssessParameters::default(), &stop), None);
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().filter(|e| e.as_ref().unwrap().path().is_file()).count(), 1);
}

#[test]
fn fixing_a_healthy_file_reports_why_nothing_was_done() {
    let directory = tempfile::tempdir().unwrap();
    use_temporary_cache(&directory);
    let path = directory.path().join("healthy.png");
    std::fs::write(&path, noisy_png()).unwrap();

    let outcome = fix_file(&path, &fix_parameters(None, false), &not_stopped());

    assert!(matches!(outcome, FixOutcome::Failed(_)), "{outcome:?}");
}

#[test]
fn collecting_references_from_a_folder_without_videos_stores_nothing() {
    let directory = tempfile::tempdir().unwrap();
    use_temporary_cache(&directory);
    let stored = collect_references(&[directory.path().to_path_buf()], &not_stopped(), None);

    assert!(matches!(stored, Ok(Some(_))), "{stored:?}");
}
