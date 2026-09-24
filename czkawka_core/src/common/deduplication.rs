use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

#[derive(Debug, thiserror::Error)]
pub enum DeduplicationError {
    #[error("Cannot deduplicate {path}: {source}")]
    Io { path: PathBuf, source: io::Error },
    #[error("CoW deduplication is only available on Linux")]
    UnsupportedPlatform,
    #[error("Only regular files can be deduplicated")]
    NotRegularFile,
    #[error("The files have different sizes")]
    DifferentSizes,
    #[error("The files already refer to the same inode; deduplication cannot separate hardlinks")]
    SameFile,
    #[error("The kernel found different contents; no differing data was replaced")]
    DifferentContents,
    #[error("Deduplication was stopped; completed ranges remain safely shared")]
    Cancelled,
    #[error("The filesystem did not deduplicate the requested range")]
    InvalidProgress,
    #[error("The filesystem returned an unknown deduplication status: {0}")]
    UnknownStatus(i32),
}

/// Shares identical extents without replacing either file or its metadata.
/// The kernel verifies each range. Errors or cancellation can leave earlier ranges shared,
/// but never overwrite differing contents. Requires a supporting Linux filesystem.
pub fn deduplicate_files(source: &Path, destination: &Path, stop: &AtomicBool) -> Result<(), DeduplicationError> {
    #[cfg(target_os = "linux")]
    return linux::deduplicate_files(source, destination, stop);

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (source, destination, stop);
        Err(DeduplicationError::UnsupportedPlatform)
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::fs::{File, OpenOptions};
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    use std::sync::atomic::Ordering;

    use super::*;

    const CHUNK_SIZE: u64 = 16 * 1024 * 1024;

    #[repr(C)]
    #[derive(Default)]
    struct DedupeHeader {
        source_offset: u64,
        source_length: u64,
        destination_count: u16,
        reserved1: u16,
        reserved2: u32,
    }

    #[repr(C)]
    #[derive(Default)]
    struct DedupeInfo {
        destination_fd: i64,
        destination_offset: u64,
        bytes_deduped: u64,
        status: i32,
        reserved: u32,
    }

    #[repr(C)]
    struct DedupeRequest {
        header: DedupeHeader,
        info: DedupeInfo,
    }

    fn file_error(path: &Path, source: io::Error) -> DeduplicationError {
        DeduplicationError::Io { path: path.to_owned(), source }
    }

    fn open_file(path: &Path, writable: bool) -> Result<File, DeduplicationError> {
        OpenOptions::new()
            .read(true)
            .write(writable)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(path)
            .map_err(|e| file_error(path, e))
    }

    pub(super) fn deduplicate_files(source: &Path, destination: &Path, stop: &AtomicBool) -> Result<(), DeduplicationError> {
        let source_file = open_file(source, false)?;
        let destination_file = open_file(destination, true)?;
        let source_meta = source_file.metadata().map_err(|e| file_error(source, e))?;
        let destination_meta = destination_file.metadata().map_err(|e| file_error(destination, e))?;
        if !source_meta.is_file() || !destination_meta.is_file() {
            return Err(DeduplicationError::NotRegularFile);
        }
        if source_meta.dev() == destination_meta.dev() && source_meta.ino() == destination_meta.ino() {
            return Err(DeduplicationError::SameFile);
        }
        if source_meta.len() != destination_meta.len() {
            return Err(DeduplicationError::DifferentSizes);
        }

        let mut offset = 0;
        while offset < source_meta.len() {
            if stop.load(Ordering::Relaxed) {
                return Err(DeduplicationError::Cancelled);
            }
            let length = (source_meta.len() - offset).min(CHUNK_SIZE);
            offset += deduplicate_range(&source_file, &destination_file, destination, offset, length)?;
        }
        Ok(())
    }

    fn deduplicate_range(source: &File, destination: &File, destination_path: &Path, offset: u64, length: u64) -> Result<u64, DeduplicationError> {
        let mut request = DedupeRequest {
            header: DedupeHeader {
                source_offset: offset,
                source_length: length,
                destination_count: 1,
                ..DedupeHeader::default()
            },
            info: DedupeInfo {
                destination_fd: i64::from(destination.as_raw_fd()),
                destination_offset: offset,
                ..DedupeInfo::default()
            },
        };
        // Linux encodes only the fixed header size, excluding the trailing destination array.
        let operation = libc::_IOWR::<DedupeHeader>(0x94, 54);
        // SAFETY: Both descriptors stay open. The C-layout buffer contains the header and
        // exactly one writable destination entry, matching destination_count; reserved fields are zero.
        let result = unsafe { libc::ioctl(source.as_raw_fd(), operation, &mut request) };
        if result < 0 {
            return Err(file_error(destination_path, io::Error::last_os_error()));
        }
        match request.info.status {
            0 if request.info.bytes_deduped > 0 && request.info.bytes_deduped <= length => Ok(request.info.bytes_deduped),
            0 => Err(DeduplicationError::InvalidProgress),
            1 => Err(DeduplicationError::DifferentContents),
            status if status < 0 => Err(file_error(destination_path, io::Error::from_raw_os_error(status.saturating_neg()))),
            status => Err(DeduplicationError::UnknownStatus(status)),
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests;
