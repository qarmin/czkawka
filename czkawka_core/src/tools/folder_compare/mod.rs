mod cache;
pub mod core;
mod diff;
mod duplicates;
mod hashing;
mod scan;
#[cfg(test)]
mod tests;
pub mod traits;

use std::path::PathBuf;
use std::time::Duration;

use serde::Serialize;

use crate::common::model::HashType;
use crate::common::tool_data::CommonToolData;

/// Which question the tool answers; derived from whether a second side (reference paths) was given.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum FolderCompareMode {
    /// Finds identical and contained folders among the included paths.
    Duplicates,
    /// Compares the included paths (left side) with the reference paths (right side).
    Diff,
}

#[derive(Debug, Clone, Copy)]
pub struct FolderCompareParameters {
    pub hash_type: HashType,
    /// Folders count as equal when they hold the same file contents, regardless of file and folder names.
    pub ignore_names: bool,
    /// Duplicate mode skips folders with fewer files, which keeps trivial folders out of the results.
    pub min_files: u64,
    /// Duplicate mode skips folders smaller than this many bytes in total.
    pub min_size: u64,
    /// Duplicate mode also reports folders whose files all exist in another, bigger folder.
    pub find_subsets: bool,
}

impl FolderCompareParameters {
    pub const DEFAULT_MIN_FILES: u64 = 2;

    pub fn new(hash_type: HashType) -> Self {
        Self {
            hash_type,
            ignore_names: false,
            min_files: Self::DEFAULT_MIN_FILES,
            min_size: 0,
            find_subsets: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MatchRole {
    Identical,
    /// Every file of this folder exists in the containers of the same group.
    Contained,
    /// Holds all files of the contained folders of the same group, plus `extra_files` more.
    Container,
}

#[derive(Debug, Clone, Serialize)]
pub struct FolderMatch {
    pub path: PathBuf,
    pub role: MatchRole,
    pub file_count: u64,
    pub size: u64,
    /// Files that the container has on top of the contained folder; zero for other roles.
    pub extra_files: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum GroupRelation {
    Identical,
    /// Contained folders come first in [`FolderGroup::folders`], then the containers.
    Subset,
}

#[derive(Debug, Clone, Serialize)]
pub struct FolderGroup {
    pub relation: GroupRelation,
    pub folders: Vec<FolderMatch>,
}

impl FolderGroup {
    /// Bytes that could be freed by removing every folder of the group except the biggest one.
    pub fn wasted_size(&self) -> u64 {
        match self.relation {
            GroupRelation::Identical => {
                let total: u64 = self.folders.iter().map(|folder| folder.size).sum();
                total.saturating_sub(self.folders.iter().map(|folder| folder.size).max().unwrap_or(0))
            }
            GroupRelation::Subset => self.folders.iter().filter(|folder| folder.role == MatchRole::Contained).map(|folder| folder.size).sum(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DiffStatus {
    Identical,
    /// A file with different content, or a folder holding differences on both sides.
    Different,
    OnlyLeft,
    OnlyRight,
    /// A folder that has everything of its right counterpart, plus more.
    LeftHasMore,
    /// A folder that has everything of its left counterpart, plus more.
    RightHasMore,
}

/// A node of the diff tree; children of identical or one-sided folders are left out because they all share the folder's status.
#[derive(Debug, Clone, Serialize)]
pub struct DiffNode {
    pub name: String,
    pub kind: EntryKind,
    pub status: DiffStatus,
    pub left_path: Option<PathBuf>,
    pub right_path: Option<PathBuf>,
    pub left_size: u64,
    pub right_size: u64,
    pub left_files: u64,
    pub right_files: u64,
    pub children: Vec<Self>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DiffResult {
    pub left_roots: Vec<PathBuf>,
    pub right_roots: Vec<PathBuf>,
    /// Entries of all left and right roots merged by name.
    pub entries: Vec<DiffNode>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Info {
    pub number_of_groups: usize,
    pub number_of_folders: usize,
    pub number_of_different_entries: usize,
    /// Bytes freed by keeping one folder of every identical group; contained folders are not counted, they overlap with identical ones.
    pub wasted_size: u64,
    pub scanning_time: Duration,
}

pub struct FolderCompare {
    common_data: CommonToolData,
    information: Info,
    params: FolderCompareParameters,
    mode: FolderCompareMode,
    groups: Vec<FolderGroup>,
    diff: DiffResult,
}

impl FolderCompare {
    pub const fn get_mode(&self) -> FolderCompareMode {
        self.mode
    }

    pub fn get_groups(&self) -> &[FolderGroup] {
        &self.groups
    }

    pub const fn get_diff(&self) -> &DiffResult {
        &self.diff
    }
}
