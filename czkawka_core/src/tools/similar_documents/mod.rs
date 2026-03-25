pub mod core;
pub mod traits;

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::common::model::FileEntry;
use crate::common::tool_data::CommonToolData;
use crate::common::traits::ResultEntry;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified_date: u64,
    /// MinHash signature for the document's text content.
    #[serde(skip)]
    pub minhash_signature: Vec<u64>,
    /// Estimated Jaccard similarity to the group's reference document (0.0–1.0).
    #[serde(default)]
    pub similarity: f64,
}

impl ResultEntry for DocumentEntry {
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

impl FileEntry {
    fn into_document_entry(self) -> DocumentEntry {
        DocumentEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,
            minhash_signature: Vec::new(),
            similarity: 0.0,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Info {
    pub number_of_similar_documents: usize,
    pub number_of_groups: usize,
    pub scanning_time: Duration,
}

#[derive(Clone)]
pub struct SimilarDocumentsParameters {
    /// Minimum Jaccard similarity (0.0–1.0) to consider two documents similar.
    pub similarity_threshold: f64,
    /// Number of hash functions in the MinHash signature.
    pub num_hashes: usize,
    /// Shingle size (number of consecutive words per shingle).
    pub shingle_size: usize,
}

impl SimilarDocumentsParameters {
    pub fn new(similarity_threshold: f64, num_hashes: usize, shingle_size: usize) -> Self {
        Self {
            similarity_threshold: similarity_threshold.clamp(0.0, 1.0),
            num_hashes: num_hashes.max(16),
            shingle_size: shingle_size.max(1),
        }
    }
}

impl Default for SimilarDocumentsParameters {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.7,
            num_hashes: 128,
            shingle_size: 3,
        }
    }
}

pub struct SimilarDocuments {
    pub(crate) common_data: CommonToolData,
    pub(crate) information: Info,
    pub(crate) document_entries: Vec<DocumentEntry>,
    /// Groups of similar documents.
    pub(crate) similar_documents: Vec<Vec<DocumentEntry>>,
    pub(crate) params: SimilarDocumentsParameters,
}

impl SimilarDocuments {
    pub fn get_similar_documents(&self) -> &Vec<Vec<DocumentEntry>> {
        &self.similar_documents
    }

    pub fn get_information(&self) -> Info {
        self.information
    }

    pub(crate) fn get_params(&self) -> &SimilarDocumentsParameters {
        &self.params
    }
}
