use std::path::{Path, PathBuf};
use std::{fmt, io};

pub type Result<T> = std::result::Result<T, FixerError>;

/// Every variant carries enough context to locate the problem without consulting a log.
/// Bare [`io::Error`] is deliberately never surfaced: "Permission denied" without a path
/// is unactionable.
#[derive(Debug, thiserror::Error)]
pub enum FixerError {
    #[error("failed to read {path}: {source}")]
    Read { path: PathBuf, source: io::Error },

    #[error("failed to write {path}: {source}")]
    Write { path: PathBuf, source: io::Error },

    #[error("failed to memory-map {path} ({size} bytes): {source}")]
    Mmap { path: PathBuf, size: u64, source: io::Error },

    #[error("{path} is empty, nothing to repair")]
    EmptyFile { path: PathBuf },

    #[error("{path} would need {requested} bytes but only {available} are present")]
    TruncatedRead { path: PathBuf, requested: u64, available: u64 },

    #[error("could not determine the format of {path}")]
    UnknownFormat { path: PathBuf },

    #[error("{path}: refusing to allocate {requested} bytes, limit is {limit}")]
    AllocationTooLarge { path: PathBuf, requested: u64, limit: u64 },

    #[error("{path} already exists, pass --force to overwrite it")]
    OutputExists { path: PathBuf },

    #[error("ffmpeg is required for video repair but was not found in PATH")]
    FfmpegMissing,

    #[error("ffmpeg failed ({context}): {stderr}")]
    Ffmpeg { context: String, stderr: String },

    #[error("{path}: {message}")]
    Malformed { path: PathBuf, message: String },
}

impl FixerError {
    pub fn read(path: impl AsRef<Path>, source: io::Error) -> Self {
        Self::Read {
            path: path.as_ref().to_path_buf(),
            source,
        }
    }

    pub fn write(path: impl AsRef<Path>, source: io::Error) -> Self {
        Self::Write {
            path: path.as_ref().to_path_buf(),
            source,
        }
    }

    pub fn malformed(path: impl AsRef<Path>, message: impl fmt::Display) -> Self {
        Self::Malformed {
            path: path.as_ref().to_path_buf(),
            message: message.to_string(),
        }
    }
}
