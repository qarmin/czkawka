use std::fs::{self, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::fs::{MetadataExt, PermissionsExt, symlink};
use std::sync::atomic::Ordering;

use super::*;

#[test]
fn rejects_different_sizes_without_changing_files() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source");
    let destination = dir.path().join("destination");
    fs::write(&source, b"original").unwrap();
    fs::write(&destination, b"different").unwrap();
    assert!(matches!(
        deduplicate_files(&source, &destination, &AtomicBool::new(false)),
        Err(DeduplicationError::DifferentSizes)
    ));
    assert_eq!(fs::read(source).unwrap(), b"original");
    assert_eq!(fs::read(destination).unwrap(), b"different");
}

#[test]
fn rejects_symlinks_and_existing_hardlinks() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source");
    let destination = dir.path().join("destination");
    let link = dir.path().join("symlink");
    fs::write(&source, b"original").unwrap();
    fs::hard_link(&source, &destination).unwrap();
    symlink(&source, &link).unwrap();
    let stop = AtomicBool::new(false);
    assert!(matches!(deduplicate_files(&source, &destination, &stop), Err(DeduplicationError::SameFile)));
    assert!(matches!(deduplicate_files(&link, &destination, &stop), Err(DeduplicationError::Io { .. })));
    assert!(matches!(deduplicate_files(&source, &link, &stop), Err(DeduplicationError::Io { .. })));
    assert!(fs::symlink_metadata(link).unwrap().is_symlink());
    assert_eq!(fs::read(source).unwrap(), b"original");
}

#[test]
fn missing_destination_is_not_created() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source");
    let destination = dir.path().join("missing");
    fs::write(&source, b"original").unwrap();
    assert!(deduplicate_files(&source, &destination, &AtomicBool::new(false)).is_err());
    assert!(!destination.exists());
}

#[test]
fn cancellation_preserves_contents() {
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("source");
    let destination = dir.path().join("destination");
    fs::write(&source, b"original").unwrap();
    fs::write(&destination, b"original").unwrap();
    let stop = AtomicBool::new(false);
    stop.store(true, Ordering::Relaxed);
    assert!(matches!(deduplicate_files(&source, &destination, &stop), Err(DeduplicationError::Cancelled)));
    assert_eq!(fs::read(source).unwrap(), b"original");
    assert_eq!(fs::read(destination).unwrap(), b"original");
}

#[test]
#[ignore = "Requires CZKAWKA_COW_TEST_DIR on a writable Btrfs or reflink-enabled XFS filesystem"]
fn cow_preserves_identity_metadata_and_independent_writes() {
    let root = std::env::var_os("CZKAWKA_COW_TEST_DIR").expect("Set CZKAWKA_COW_TEST_DIR to a CoW filesystem");
    let dir = tempfile::tempdir_in(root).unwrap();
    let source = dir.path().join("source");
    let destination = dir.path().join("destination");
    // Cross the ioctl chunk boundary and include an unaligned EOF.
    let content = vec![42; 17 * 1024 * 1024 + 137];
    fs::write(&source, &content).unwrap();
    fs::write(&destination, &content).unwrap();
    fs::set_permissions(&source, fs::Permissions::from_mode(0o640)).unwrap();
    fs::set_permissions(&destination, fs::Permissions::from_mode(0o600)).unwrap();
    let before_source = fs::metadata(&source).unwrap();
    let before_destination = fs::metadata(&destination).unwrap();
    let stop = AtomicBool::new(false);

    deduplicate_files(&source, &destination, &stop).unwrap();
    deduplicate_files(&source, &destination, &stop).unwrap();
    let after_source = fs::metadata(&source).unwrap();
    let after_destination = fs::metadata(&destination).unwrap();
    assert_eq!(before_source.ino(), after_source.ino());
    assert_eq!(before_destination.ino(), after_destination.ino());
    assert_ne!(after_source.ino(), after_destination.ino());
    for (before, after) in [(&before_source, &after_source), (&before_destination, &after_destination)] {
        assert_eq!(before.mode(), after.mode());
        assert_eq!(before.uid(), after.uid());
        assert_eq!(before.gid(), after.gid());
        assert_eq!(before.modified().unwrap(), after.modified().unwrap());
    }

    let mut file = OpenOptions::new().write(true).open(&destination).unwrap();
    file.seek(SeekFrom::Start(16 * 1024 * 1024)).unwrap();
    file.write_all(b"changed").unwrap();
    let changed = fs::read(&destination).unwrap();
    assert_ne!(changed, content);
    assert_eq!(fs::read(&source).unwrap(), content);
    assert!(matches!(deduplicate_files(&source, &destination, &stop), Err(DeduplicationError::DifferentContents)));
    assert_eq!(fs::read(&destination).unwrap(), changed);
    assert_eq!(fs::read(&source).unwrap(), content);
}
