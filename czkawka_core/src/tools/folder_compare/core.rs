use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::model::{CheckingMethod, ToolType, WorkContinueStatus};
use crate::common::progress_data::{ProgressData, ToolStage};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::flc;
use crate::tools::folder_compare::cache::CacheSettings;
use crate::tools::folder_compare::diff::{DiffBuilder, collect_pair_files, count_differences};
use crate::tools::folder_compare::duplicates::{DirIndexBuilder, collect_candidate_files, find_groups};
use crate::tools::folder_compare::hashing::{HashingOutcome, hash_matching_files};
use crate::tools::folder_compare::scan::{FsNode, ScanContext, scan_roots, without_nested_roots};
use crate::tools::folder_compare::{DiffResult, FolderCompare, FolderCompareMode, FolderCompareParameters, FolderGroup, GroupRelation, Info};

type ScannedRoots = Vec<(PathBuf, FsNode)>;

impl FolderCompare {
    pub fn new(params: FolderCompareParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::FolderCompare),
            information: Info::default(),
            params,
            mode: FolderCompareMode::Duplicates,
            groups: Vec::new(),
            diff: DiffResult::default(),
        }
    }

    /// Unlike other tools the reference paths are not required to lie inside the included ones, they are the second side; included paths that are also reference paths are not part of the left side.
    pub(crate) fn prepare_sides(&mut self) -> Result<(Vec<PathBuf>, Vec<PathBuf>), ()> {
        if self.get_extensions_mut().set_and_validate_extensions(None).is_err() {
            return Err(());
        }
        let directories = &self.common_data.directories;
        let not_excluded = |root: &&PathBuf| !directories.excluded_directories.iter().any(|excluded| root.starts_with(excluded));
        let is_reference = |root: &&PathBuf| directories.reference_directories.contains(root);
        let left = without_nested_roots(
            &directories
                .included_directories
                .iter()
                .filter(not_excluded)
                .filter(|root| !is_reference(root))
                .cloned()
                .collect::<Vec<_>>(),
        );
        let right = without_nested_roots(&directories.reference_directories.iter().filter(not_excluded).cloned().collect::<Vec<_>>());

        if left.is_empty() {
            self.common_data.text_messages.critical = Some(flc!("core_cannot_start_scan_no_included_paths"));
            return Err(());
        }
        let overlap = left.iter().find_map(|l| right.iter().find(|r| l.starts_with(r) || r.starts_with(l)).map(|r| (l, r)));
        if let Some((left_path, right_path)) = overlap {
            self.common_data.text_messages.critical = Some(flc!(
                "core_folder_compare_sides_overlap",
                left = left_path.to_string_lossy().to_string(),
                right = right_path.to_string_lossy().to_string()
            ));
            return Err(());
        }
        Ok((left, right))
    }

    #[fun_time(message = "compare_folders", level = "debug")]
    pub(crate) fn compare_folders(
        &mut self,
        left: &[PathBuf],
        right: &[PathBuf],
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
    ) -> WorkContinueStatus {
        self.mode = if right.is_empty() { FolderCompareMode::Duplicates } else { FolderCompareMode::Diff };

        let (left_trees, right_trees) = self.scan_sides(left, right, stop_flag, progress_sender);
        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        let candidates = match self.mode {
            FolderCompareMode::Duplicates => collect_candidate_files(&left_trees),
            FolderCompareMode::Diff => collect_pair_files(&left_trees, &right_trees),
        };
        let cache_settings = CacheSettings {
            enabled: self.get_use_cache(),
            delete_outdated: self.get_delete_outdated_cache(),
            save_also_as_json: self.get_save_also_as_json(),
        };
        let outcome = hash_matching_files(candidates, self.params.hash_type, cache_settings, stop_flag, progress_sender);
        if outcome.stopped {
            return WorkContinueStatus::Stop;
        }
        self.build_result(&left_trees, &right_trees, left, right, &outcome);
        WorkContinueStatus::Continue
    }

    /// Both sides share one progress counter, so it keeps growing instead of restarting for the second side.
    fn scan_sides(&self, left: &[PathBuf], right: &[PathBuf], stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> (ScannedRoots, ScannedRoots) {
        let progress_handler = prepare_thread_handler_common(progress_sender, ToolStage::CollectingFiles(CheckingMethod::None), 0, 0);
        let context = ScanContext {
            directories: &self.common_data.directories,
            excluded_items: &self.common_data.excluded_items,
            extensions: &self.common_data.extensions,
            stop_flag,
            entries_counter: progress_handler.items_counter(),
        };
        let scan = |roots: &[PathBuf]| -> ScannedRoots { roots.iter().cloned().zip(scan_roots(roots, &context)).collect() };
        let trees = (scan(left), scan(right));
        progress_handler.join_thread();
        trees
    }

    fn build_result(&mut self, left_trees: &ScannedRoots, right_trees: &ScannedRoots, left: &[PathBuf], right: &[PathBuf], outcome: &HashingOutcome) {
        self.common_data.text_messages.warnings.extend(outcome.warnings.iter().cloned());
        self.common_data.text_messages.extend_with_another_messages(outcome.messages.clone());
        match self.mode {
            FolderCompareMode::Duplicates => {
                let dirs = DirIndexBuilder::new(&outcome.shared_hashes, self.params.ignore_names).build(left_trees);
                self.groups = find_groups(&dirs, &self.params);
                self.information.number_of_groups = self.groups.len();
                self.information.number_of_folders = self.groups.iter().map(|group| group.folders.len()).sum();
                self.information.wasted_size = self
                    .groups
                    .iter()
                    .filter(|group| group.relation == GroupRelation::Identical)
                    .map(FolderGroup::wasted_size)
                    .sum();
            }
            FolderCompareMode::Diff => {
                let entries = DiffBuilder {
                    shared_hashes: &outcome.shared_hashes,
                }
                .build(left_trees, right_trees);
                self.information.number_of_different_entries = count_differences(&entries);
                self.diff = DiffResult {
                    left_roots: left.to_vec(),
                    right_roots: right.to_vec(),
                    entries,
                };
            }
        }
    }
}
