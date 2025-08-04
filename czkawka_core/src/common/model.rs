use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::Xxh3;

use crate::common_traits::ResultEntry;
use crate::tools::duplicate::MyHasher;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum ToolType {
    Duplicate,
    EmptyFolders,
    EmptyFiles,
    InvalidSymlinks,
    BrokenFiles,
    BadExtensions,
    BigFile,
    SameMusic,
    SimilarImages,
    SimilarVideos,
    TemporaryFiles,
    #[default]
    None,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Default, Deserialize, Serialize)]
pub enum CheckingMethod {
    #[default]
    None,
    Name,
    SizeName,
    Size,
    Hash,
    AudioTags,
    AudioContent,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
}

impl ResultEntry for FileEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
    fn get_size(&self) -> u64 {
        self.size
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Default)]
pub enum HashType {
    #[default]
    Blake3,
    Crc32,
    Xxh3,
}

impl HashType {
    pub(crate) fn hasher(self) -> Box<dyn MyHasher> {
        match self {
            Self::Blake3 => Box::new(blake3::Hasher::new()),
            Self::Crc32 => Box::new(crc32fast::Hasher::new()),
            Self::Xxh3 => Box::new(Xxh3::new()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum WorkContinueStatus {
    Continue,
    Stop,
}
