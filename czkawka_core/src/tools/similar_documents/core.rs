use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::debug;
use rayon::prelude::*;

use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::prepare_thread_handler_common;
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::tools::similar_documents::{DocumentEntry, Info, SimilarDocuments, SimilarDocumentsParameters};

/// Supported document extensions.
const DOCUMENT_EXTENSIONS: &[&str] = &[
    "txt", "md", "rst", "csv", "tsv", "log", "json", "xml", "yaml", "yml", "toml", "ini", "cfg", "html", "htm", "tex", "rtf", "py", "rs", "js", "ts", "c", "cpp", "h", "hpp",
    "java", "go", "rb", "php", "sh", "bat", "ps1", "sql",
];

impl SimilarDocuments {
    pub fn new(params: SimilarDocumentsParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::SimilarDocuments),
            information: Info::default(),
            document_entries: Vec::new(),
            similar_documents: Vec::new(),
            params,
        }
    }

    #[fun_time(message = "collect_documents", level = "debug")]
    pub(crate) fn collect_documents(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.prepare_items(Some(DOCUMENT_EXTENSIONS)).is_err() {
            return WorkContinueStatus::Continue;
        }

        let result = DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.document_entries = grouped_file_entries.into_values().flatten().map(|fe| fe.into_document_entry()).collect();
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("collect_documents - Found {} documents.", self.document_entries.len());
                WorkContinueStatus::Continue
            }
            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    #[fun_time(message = "compute_signatures", level = "debug")]
    pub(crate) fn compute_signatures(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let n = self.document_entries.len();
        let progress = prepare_thread_handler_common(progress_sender, CurrentStage::SimilarDocumentsHashing, n, self.get_test_type(), 0);

        let shingle_size = self.params.shingle_size;
        let num_hashes = self.params.num_hashes;

        let results: Vec<Option<Vec<u64>>> = self
            .document_entries
            .par_iter()
            .map(|entry| {
                if stop_flag.load(Ordering::Relaxed) {
                    return None;
                }
                progress.items_counter().fetch_add(1, Ordering::Relaxed);
                let text = match extract_text(&entry.path) {
                    Ok(t) if !t.is_empty() => t,
                    _ => return None,
                };
                Some(compute_minhash(&text, shingle_size, num_hashes))
            })
            .collect();

        progress.join_thread();

        if stop_flag.load(Ordering::Relaxed) {
            return WorkContinueStatus::Stop;
        }

        // Remove entries with no valid signature
        let mut valid_entries = Vec::new();
        for (entry, sig) in self.document_entries.drain(..).zip(results) {
            if let Some(sig) = sig {
                if !sig.is_empty() {
                    let mut entry = entry;
                    entry.minhash_signature = sig;
                    valid_entries.push(entry);
                }
            }
        }
        self.document_entries = valid_entries;
        debug!("Valid document entries with signatures: {}", self.document_entries.len());

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "compare_signatures", level = "debug")]
    pub(crate) fn compare_signatures(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let entries = &self.document_entries;
        let n = entries.len();
        let threshold = self.params.similarity_threshold;

        let progress = prepare_thread_handler_common(progress_sender, CurrentStage::SimilarDocumentsComparing, n, self.get_test_type(), 0);

        // Simple union-find for clustering
        let mut parent: Vec<usize> = (0..n).collect();
        // Store best similarity for each entry
        let mut best_sim: Vec<f64> = vec![1.0; n]; // self-similarity is 1.0

        fn find(parent: &mut [usize], mut i: usize) -> usize {
            while parent[i] != i {
                parent[i] = parent[parent[i]];
                i = parent[i];
            }
            i
        }

        for i in 0..n {
            if stop_flag.load(Ordering::Relaxed) {
                progress.join_thread();
                return WorkContinueStatus::Stop;
            }
            progress.items_counter().fetch_add(1, Ordering::Relaxed);

            for j in (i + 1)..n {
                let sim = jaccard_from_minhash(&entries[i].minhash_signature, &entries[j].minhash_signature);
                if sim >= threshold {
                    let ri = find(&mut parent, i);
                    let rj = find(&mut parent, j);
                    if ri != rj {
                        parent[ri] = rj;
                    }
                    if sim < best_sim[j] {
                        best_sim[j] = sim;
                    }
                    if sim < best_sim[i] {
                        best_sim[i] = sim;
                    }
                }
            }
        }

        progress.join_thread();

        // Collect clusters
        let mut clusters: HashMap<usize, Vec<(usize, DocumentEntry)>> = HashMap::new();
        for (i, entry) in self.document_entries.drain(..).enumerate() {
            let root = find(&mut parent, i);
            let mut entry = entry;
            entry.similarity = best_sim[i];
            clusters.entry(root).or_default().push((i, entry));
        }

        self.similar_documents = clusters
            .into_values()
            .filter(|group| group.len() > 1)
            .map(|mut group| {
                group.sort_by_key(|(idx, _)| *idx);
                group.into_iter().map(|(_, e)| e).collect()
            })
            .collect();

        self.information.number_of_groups = self.similar_documents.len();
        self.information.number_of_similar_documents = self.similar_documents.iter().map(|g| g.len()).sum();

        debug!(
            "Found {} groups with {} similar documents",
            self.information.number_of_groups, self.information.number_of_similar_documents
        );

        WorkContinueStatus::Continue
    }
}

/// Extract text content from a file. For plain text files, reads directly.
/// Limited to first 256KB to keep memory and time bounded.
fn extract_text(path: &std::path::Path) -> Result<String, String> {
    const MAX_BYTES: usize = 256 * 1024;

    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut buf = vec![0u8; MAX_BYTES];
    let n = file.read(&mut buf).map_err(|e| e.to_string())?;
    buf.truncate(n);

    // Try to interpret as UTF-8; fall back to lossy conversion
    let text = String::from_utf8(buf).unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned());

    Ok(text)
}

/// Compute a MinHash signature from text using word-level shingles.
fn compute_minhash(text: &str, shingle_size: usize, num_hashes: usize) -> Vec<u64> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() < shingle_size {
        return Vec::new();
    }

    // Pre-generate hash seeds (deterministic)
    let seeds: Vec<u64> = (0..num_hashes as u64)
        .map(|i| i.wrapping_mul(0x517cc1b727220a95).wrapping_add(0x6c62272e07bb0142))
        .collect();

    let mut signature = vec![u64::MAX; num_hashes];

    for window in words.windows(shingle_size) {
        let shingle: String = window.join(" ");
        let base_hash = xxhash_rust::xxh3::xxh3_64(shingle.as_bytes());

        for (i, seed) in seeds.iter().enumerate() {
            let h = base_hash.wrapping_mul(*seed).wrapping_add(seed.rotate_left(32));
            if h < signature[i] {
                signature[i] = h;
            }
        }
    }

    signature
}

/// Estimate Jaccard similarity from two MinHash signatures.
fn jaccard_from_minhash(sig_a: &[u64], sig_b: &[u64]) -> f64 {
    if sig_a.len() != sig_b.len() || sig_a.is_empty() {
        return 0.0;
    }
    let matches = sig_a.iter().zip(sig_b.iter()).filter(|(a, b)| a == b).count();
    matches as f64 / sig_a.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minhash_identical_texts() {
        let text = "the quick brown fox jumps over the lazy dog";
        let sig1 = compute_minhash(text, 3, 128);
        let sig2 = compute_minhash(text, 3, 128);
        assert_eq!(jaccard_from_minhash(&sig1, &sig2), 1.0);
    }

    #[test]
    fn test_minhash_similar_texts() {
        let text1 = "the quick brown fox jumps over the lazy dog near the river bank";
        let text2 = "the quick brown fox leaps over the lazy dog near the river bank";
        let sig1 = compute_minhash(text1, 3, 128);
        let sig2 = compute_minhash(text2, 3, 128);
        let sim = jaccard_from_minhash(&sig1, &sig2);
        assert!(sim > 0.5, "Similar texts should have similarity > 0.5, got {sim}");
    }

    #[test]
    fn test_minhash_different_texts() {
        let text1 = "the quick brown fox jumps over the lazy dog";
        let text2 = "a completely different document about quantum physics and mathematics";
        let sig1 = compute_minhash(text1, 3, 128);
        let sig2 = compute_minhash(text2, 3, 128);
        let sim = jaccard_from_minhash(&sig1, &sig2);
        assert!(sim < 0.3, "Different texts should have similarity < 0.3, got {sim}");
    }

    #[test]
    fn test_minhash_empty_text() {
        let sig = compute_minhash("", 3, 128);
        assert!(sig.is_empty());
    }

    #[test]
    fn test_minhash_short_text() {
        let sig = compute_minhash("hello world", 3, 128);
        assert!(sig.is_empty(), "Text with fewer words than shingle_size should produce empty signature");
    }
}
