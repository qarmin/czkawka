//! Deciding where a repaired file goes and putting it there.

use std::path::{Path, PathBuf};

use crate::file_fixer::candidate::Payload;
use crate::file_fixer::engine::{RepairOptions, RepairResult};
use crate::file_fixer::error::{FixerError, Result};

/// Marks a written output, and a source name a batch run should skip re-processing.
///
/// Spelled oddly on purpose: a plain "fixed" is common enough in real filenames (camera
/// firmware, other tools) that using it as a sentinel would misidentify unrelated files as
/// this tool's own output.
pub const FIXED_MARKER: &str = "fixedffile";

/// True when `path`'s name already carries [`FIXED_MARKER`], meaning it is itself a previous
/// output rather than a fresh input to repair.
pub fn is_fixed_output_name(path: &Path) -> bool {
    path.file_name().is_some_and(|name| name.to_string_lossy().contains(FIXED_MARKER))
}

/// Where the fixed copy of `input` lands (or already lives, from an earlier run).
pub fn fixed_output_path(input: &Path, output_dir: Option<&Path>) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default();
    let extension = input.extension();
    let mut name = stem.to_os_string();
    name.push(".");
    name.push(FIXED_MARKER);
    if let Some(extension) = extension {
        name.push(".");
        name.push(extension);
    }

    match output_dir {
        Some(dir) => dir.join(name),
        None => input.with_file_name(name),
    }
}

/// Writes every candidate, winner included, named after the strategy that produced it. Useful
/// when the automatic choice looks wrong and the alternatives need to be compared by eye.
pub fn keep_candidates(input: &Path, result: &RepairResult, directory: &Path) {
    if let Err(error) = std::fs::create_dir_all(directory) {
        log::warn!("cannot create {}: {error}", directory.display());
        return;
    }

    let stem = input.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let extension = input.extension().map(|e| e.to_string_lossy().to_string());

    for candidate in result.winner.iter().chain(&result.rejected) {
        let mut name = format!("{stem}__{}", candidate.strategy);
        if let Some(extension) = &extension {
            name.push('.');
            name.push_str(extension);
        }
        let target = directory.join(name);
        if let Err(error) = candidate.data.write_to(&target) {
            log::warn!("cannot write {}: {error}", target.display());
        }
    }
}

pub fn strategy_was_passthrough(result: &RepairResult) -> bool {
    result.winner.as_ref().is_some_and(|w| w.strategy.ends_with("_verbatim"))
}

pub fn write_output(input: &Path, data: &Payload, options: &RepairOptions) -> Result<PathBuf> {
    let output = output_path_for(input, options);
    if output.exists() && !options.force {
        return Err(FixerError::OutputExists { path: output });
    }
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).map_err(|e| FixerError::write(parent, e))?;
    }
    data.write_to(&output).map_err(|e| FixerError::write(&output, e))?;
    Ok(output)
}

fn output_path_for(input: &Path, options: &RepairOptions) -> PathBuf {
    fixed_output_path(input, options.output_dir.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_lands_next_to_the_input_by_default() {
        let options = RepairOptions::default();
        let path = output_path_for(Path::new("/photos/holiday.jpg"), &options);
        assert_eq!(path, Path::new("/photos/holiday.fixedffile.jpg"));
    }

    #[test]
    fn output_dir_redirects_the_result() {
        let options = RepairOptions {
            output_dir: Some(PathBuf::from("/out")),
            ..RepairOptions::default()
        };
        let path = output_path_for(Path::new("/photos/holiday.jpg"), &options);
        assert_eq!(path, Path::new("/out/holiday.fixedffile.jpg"));
    }

    #[test]
    fn a_file_without_an_extension_still_gets_a_name() {
        let options = RepairOptions::default();
        let path = output_path_for(Path::new("/recovered/f00123"), &options);
        assert_eq!(path, Path::new("/recovered/f00123.fixedffile"));
    }

    #[test]
    fn an_existing_output_is_not_overwritten_without_force() {
        let directory = tempfile::tempdir().expect("temp dir");
        let input = directory.path().join("clip.png");
        let existing = directory.path().join("clip.fixedffile.png");
        std::fs::write(&existing, b"old").expect("write");

        let options = RepairOptions {
            output_dir: Some(directory.path().to_path_buf()),
            ..RepairOptions::default()
        };
        let error = write_output(&input, &Payload::Bytes(b"new".to_vec()), &options).unwrap_err();
        assert!(matches!(error, FixerError::OutputExists { .. }));
        assert_eq!(std::fs::read(&existing).expect("read"), b"old");
    }

    #[test]
    fn force_replaces_an_existing_output() {
        let directory = tempfile::tempdir().expect("temp dir");
        let input = directory.path().join("clip.png");
        let existing = directory.path().join("clip.fixedffile.png");
        std::fs::write(&existing, b"old").expect("write");

        let options = RepairOptions {
            output_dir: Some(directory.path().to_path_buf()),
            force: true,
            ..RepairOptions::default()
        };
        write_output(&input, &Payload::Bytes(b"new".to_vec()), &options).expect("write");
        assert_eq!(std::fs::read(&existing).expect("read"), b"new");
    }

    #[test]
    fn a_name_carrying_the_marker_is_recognized_as_already_fixed() {
        assert!(is_fixed_output_name(Path::new("/x/holiday.fixedffile.jpg")));
        assert!(!is_fixed_output_name(Path::new("/x/holiday.jpg")));
    }

    #[test]
    fn the_fixed_path_is_found_next_to_the_input_without_an_output_dir() {
        let path = fixed_output_path(Path::new("/photos/holiday.jpg"), None);
        assert_eq!(path, Path::new("/photos/holiday.fixedffile.jpg"));
    }
}
