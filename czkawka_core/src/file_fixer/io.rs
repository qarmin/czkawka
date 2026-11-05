//! Reading damaged files without trusting anything they say about themselves.

use std::fs::File;
use std::path::{Path, PathBuf};

use memmap2::Mmap;

use crate::file_fixer::error::{FixerError, Result};

/// Ceiling on any single allocation derived from a length field found inside a file.
/// A damaged `IHDR` can claim 65535x65535 and a damaged ISOBMFF box can claim 4 GB; neither
/// should be able to make the process ask the allocator for it.
pub const DEFAULT_ALLOCATION_LIMIT: u64 = 1 << 30;

/// A read-only view of a file on disk.
///
/// Mapped rather than read so that a 50 GB video costs the same as a 20 KB thumbnail, and so
/// strategies can hold overlapping sub-slices without copying. The mapping is never written to.
#[derive(Debug)]
pub struct SourceFile {
    path: PathBuf,
    map: Mmap,
}

impl SourceFile {
    /// # Caveat
    ///
    /// The mapping reflects the file as it exists on disk. If another process truncates the
    /// file while a repair is running, touching the vanished pages raises `SIGBUS` and kills
    /// the process. Repairing files that something else is actively writing is not supported.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path).map_err(|e| FixerError::read(path, e))?;
        let size = file.metadata().map_err(|e| FixerError::read(path, e))?.len();

        if size == 0 {
            return Err(FixerError::EmptyFile { path: path.to_path_buf() });
        }

        // SAFETY: the mapping is read-only and never aliased mutably. The only way its contents
        // can change underneath us is an external writer, which is documented as unsupported
        // above; no invariant of this crate depends on the bytes staying stable.
        let map = unsafe { Mmap::map(&file) }.map_err(|source| FixerError::Mmap {
            path: path.to_path_buf(),
            size,
            source,
        })?;

        Ok(Self { path: path.to_path_buf(), map })
    }

    pub fn data(&self) -> &[u8] {
        &self.map
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn len(&self) -> u64 {
        self.map.len() as u64
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// Clamps a length read out of a file to what the file can actually back.
///
/// Returns the usable length rather than an error: a truncated structure is the normal case
/// here, and refusing to look at it would defeat the point of the tool.
pub fn clamp_to_available(requested: u64, offset: u64, total: u64) -> u64 {
    requested.min(total.saturating_sub(offset))
}

/// Rejects an allocation whose size came from untrusted input and exceeds `limit`.
pub fn checked_allocation(path: &Path, requested: u64, limit: u64) -> Result<usize> {
    if requested > limit {
        return Err(FixerError::AllocationTooLarge {
            path: path.to_path_buf(),
            requested,
            limit,
        });
    }
    usize::try_from(requested).map_err(|_| FixerError::AllocationTooLarge {
        path: path.to_path_buf(),
        requested,
        limit,
    })
}

/// Reads a big-endian `u32` at `offset`, or `None` if the file ends first.
pub fn read_u32_be(data: &[u8], offset: usize) -> Option<u32> {
    let bytes: [u8; 4] = data.get(offset..offset + 4)?.try_into().ok()?;
    Some(u32::from_be_bytes(bytes))
}

/// Reads a big-endian `u64` at `offset`, or `None` if the file ends first.
pub fn read_u64_be(data: &[u8], offset: usize) -> Option<u64> {
    let bytes: [u8; 8] = data.get(offset..offset + 8)?.try_into().ok()?;
    Some(u64::from_be_bytes(bytes))
}

/// Reads a big-endian `u16` at `offset`, or `None` if the file ends first.
pub fn read_u16_be(data: &[u8], offset: usize) -> Option<u16> {
    let bytes: [u8; 2] = data.get(offset..offset + 2)?.try_into().ok()?;
    Some(u16::from_be_bytes(bytes))
}

/// Reads a little-endian `u32` at `offset`, or `None` if the file ends first. RIFF-based
/// containers (AVI, WAV) store their chunk sizes this way, unlike ISO-BMFF's big-endian boxes.
pub fn read_u32_le(data: &[u8], offset: usize) -> Option<u32> {
    let bytes: [u8; 4] = data.get(offset..offset + 4)?.try_into().ok()?;
    Some(u32::from_le_bytes(bytes))
}

/// Reads a little-endian `u16` at `offset`, or `None` if the file ends first.
pub fn read_u16_le(data: &[u8], offset: usize) -> Option<u16> {
    let bytes: [u8; 2] = data.get(offset..offset + 2)?.try_into().ok()?;
    Some(u16::from_le_bytes(bytes))
}

/// Reads a little-endian `i32` at `offset`, or `None` if the file ends first. BMP stores a
/// top-down image as a negative height, so the sign carries meaning and must survive the read.
pub fn read_i32_le(data: &[u8], offset: usize) -> Option<i32> {
    read_u32_le(data, offset).map(|value| value as i32)
}

/// Overwrites the little-endian `u32` at `offset`, doing nothing when it would not fit.
pub fn put_u32_le(data: &mut [u8], offset: usize, value: u32) {
    if let Some(slot) = data.get_mut(offset..offset + 4) {
        slot.copy_from_slice(&value.to_le_bytes());
    }
}

/// Overwrites the little-endian `u16` at `offset`, doing nothing when it would not fit.
pub fn put_u16_le(data: &mut [u8], offset: usize, value: u16) {
    if let Some(slot) = data.get_mut(offset..offset + 2) {
        slot.copy_from_slice(&value.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn mapping_exposes_the_file_contents() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(b"hello world").unwrap();
        file.flush().unwrap();

        let source = SourceFile::open(file.path()).unwrap();
        assert_eq!(source.data(), b"hello world");
        assert_eq!(source.len(), 11);
        assert!(!source.is_empty());
    }

    #[test]
    fn an_empty_file_is_rejected_with_its_path() {
        let file = NamedTempFile::new().unwrap();
        let error = SourceFile::open(file.path()).unwrap_err();

        assert!(matches!(error, FixerError::EmptyFile { .. }));
        assert!(error.to_string().contains(&file.path().display().to_string()));
    }

    #[test]
    fn a_missing_file_reports_its_path() {
        let error = SourceFile::open("/nonexistent/path/to/file.png").unwrap_err();
        assert!(error.to_string().contains("/nonexistent/path/to/file.png"));
    }

    #[test]
    fn lengths_are_clamped_to_what_the_file_can_back() {
        assert_eq!(clamp_to_available(4_000_000_000, 0, 1000), 1000);
        assert_eq!(clamp_to_available(100, 950, 1000), 50);
        assert_eq!(clamp_to_available(100, 2000, 1000), 0);
        assert_eq!(clamp_to_available(50, 0, 1000), 50);
    }

    #[test]
    fn an_oversized_allocation_is_refused() {
        let path = Path::new("hostile.png");
        assert_eq!(checked_allocation(path, 1024, DEFAULT_ALLOCATION_LIMIT).unwrap(), 1024);

        let error = checked_allocation(path, u64::MAX, DEFAULT_ALLOCATION_LIMIT).unwrap_err();
        assert!(matches!(error, FixerError::AllocationTooLarge { .. }));
    }

    #[test]
    fn integer_reads_stop_at_the_end_of_the_data() {
        let data = [0x00, 0x00, 0x01, 0x00, 0xFF];
        assert_eq!(read_u32_be(&data, 0), Some(256));
        assert_eq!(read_u32_be(&data, 2), None);
        assert_eq!(read_u64_be(&data, 0), None);
        assert_eq!(read_u32_le(&data, 0), Some(0x0001_0000));
        assert_eq!(read_u32_le(&data, 2), None);
        assert_eq!(read_u16_le(&data, 2), Some(1));
        assert_eq!(read_u16_be(&data, 2), Some(256));
        assert_eq!(read_u16_le(&data, 4), None);
    }

    /// A top-down BMP states its height as a negative number, so the sign has to survive.
    #[test]
    fn a_signed_read_keeps_the_sign() {
        let data = (-200i32).to_le_bytes();
        assert_eq!(read_i32_le(&data, 0), Some(-200));
        assert_eq!(read_i32_le(&data, 1), None);
    }

    #[test]
    fn writes_that_would_not_fit_are_dropped_rather_than_panicking() {
        let mut data = [0u8; 6];
        put_u32_le(&mut data, 0, 0x0102_0304);
        put_u16_le(&mut data, 4, 0x0506);
        assert_eq!(data, [0x04, 0x03, 0x02, 0x01, 0x06, 0x05]);

        put_u32_le(&mut data, 4, 0xFFFF_FFFF);
        put_u16_le(&mut data, 5, 0xFFFF);
        assert_eq!(data, [0x04, 0x03, 0x02, 0x01, 0x06, 0x05]);
    }
}
