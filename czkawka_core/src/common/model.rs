use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::Xxh3;

use crate::common::traits::ResultEntry;
use crate::tools::duplicate::MyHasher;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum ToolType {
    Duplicate,
    EmptyFolders,
    EmptyFiles,
    InvalidSymlinks,
    BrokenFiles,
    BadExtensions,
    BadNames,
    BigFile,
    SameMusic,
    SimilarImages,
    SimilarVideos,
    TemporaryFiles,
    ExifRemover,
    VideoOptimizer,
    #[default]
    None,
}

impl ToolType {
    pub fn may_use_reference_paths(self) -> bool {
        matches!(self, Self::Duplicate | Self::SameMusic | Self::SimilarImages | Self::SimilarVideos)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_entry_basic_operations() {
        let entry = FileEntry {
            path: PathBuf::from("/test/file.txt"),
            size: 1024,
            modified_date: 123456,
        };

        assert_eq!(entry.get_path(), Path::new("/test/file.txt"));
        assert_eq!(entry.get_size(), 1024);
        assert_eq!(entry.get_modified_date(), 123456);

        let entry2 = entry.clone();
        assert_eq!(entry, entry2);
    }

    #[test]
    fn test_hash_type_creates_hashers() {
        let blake3_hasher = HashType::Blake3.hasher();
        let crc32_hasher = HashType::Crc32.hasher();
        let xxh3_hasher = HashType::Xxh3.hasher();

        // Just verify they can be created
        assert!(std::mem::size_of_val(&blake3_hasher) > 0);
        assert!(std::mem::size_of_val(&crc32_hasher) > 0);
        assert!(std::mem::size_of_val(&xxh3_hasher) > 0);
    }

    #[test]
    fn test_checking_method_default() {
        assert_eq!(CheckingMethod::default(), CheckingMethod::None);
    }

    #[test]
    fn test_tool_type_default() {
        assert_eq!(ToolType::default(), ToolType::None);
    }

    #[test]
    fn test_delete_method_default() {
        use crate::common::tool_data::DeleteMethod;
        assert_eq!(DeleteMethod::default(), DeleteMethod::None);
    }
}
