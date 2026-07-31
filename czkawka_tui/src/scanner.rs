use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::path::PathBuf;
use std::thread;

use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::traits::{Search};
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};
use czkawka_core::tools::empty_folder::{EmptyFolder};
use czkawka_core::tools::big_file::{BigFile, BigFileParameters, SearchMode};
use czkawka_core::tools::empty_files::{EmptyFiles, EmptyFilesParameters};
use czkawka_core::tools::temporary::{Temporary, TemporaryParameters};
use czkawka_core::tools::similar_images::{SimilarImages, SimilarImagesParameters, GeometricInvariance};
use czkawka_core::tools::same_music::{SameMusic, SameMusicParameters, MusicSimilarity};
use czkawka_core::tools::invalid_symlinks::{InvalidSymlinks};
use czkawka_core::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};
use czkawka_core::tools::similar_videos::{SimilarVideos, SimilarVideosParameters};
use czkawka_core::tools::bad_extensions::{BadExtensions, BadExtensionsParameters};
use czkawka_core::tools::bad_names::{BadNames, BadNamesParameters, NameIssues};
use czkawka_core::tools::video_optimizer::{VideoOptimizer, VideoOptimizerParameters, VideoTranscodeParams};
use image_hasher::{FilterType as IHFilterType, HashAlg as IHHashAlg};

use crate::app::ToolType;

pub struct Scanner {
    pub receiver: Receiver<(ToolType, Vec<String>)>,
    sender: Sender<(ToolType, Vec<String>)>,
    pub stop_flag: Arc<AtomicBool>,
}

impl Scanner {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        Scanner {
            receiver,
            sender,
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    pub fn start_scan(&mut self, tool_type: ToolType, dir: &str) {
        self.stop_flag.store(false, Ordering::Relaxed);
        let stop_flag = self.stop_flag.clone();
        let sender = self.sender.clone();
        let dir = dir.to_string();

        thread::spawn(move || {
            let results = match tool_type {
                ToolType::Duplicate => {
                    let params = DuplicateFinderParameters::new(
                        CheckingMethod::Hash,
                        HashType::Blake3,
                        false,
                        0,
                        0,
                        false
                    );
                    let mut df = DuplicateFinder::new(params);
                    df.set_included_paths(vec![PathBuf::from(dir)]);
                    df.search(&stop_flag, None);

                    let mut string_results = Vec::new();
                    for (_hash, groups) in df.get_files_sorted_by_hash() {
                        for group in groups {
                            string_results.push(format!("--- Group ({} files) ---", group.len()));
                            for entry in group {
                                string_results.push(entry.path.to_string_lossy().to_string());
                            }
                        }
                    }
                    string_results
                }
                ToolType::EmptyFolders => {
                    let mut ef = EmptyFolder::new();
                    ef.set_included_paths(vec![PathBuf::from(dir)]);
                    ef.search(&stop_flag, None);

                    ef.get_empty_folder_list().values().map(|p| p.path.to_string_lossy().to_string()).collect()
                }
                ToolType::BigFiles => {
                    let params = BigFileParameters::new(50, SearchMode::BiggestFiles);
                    let mut bf = BigFile::new(params);
                    bf.set_included_paths(vec![PathBuf::from(dir)]);
                    bf.search(&stop_flag, None);

                    bf.get_big_files().iter().map(|f| f.path.to_string_lossy().to_string()).collect()
                }
                ToolType::EmptyFiles => {
                    let mut ef = EmptyFiles::new(EmptyFilesParameters::default());
                    ef.set_included_paths(vec![PathBuf::from(dir)]);
                    ef.search(&stop_flag, None);

                    ef.get_empty_files().iter().map(|f| f.path.to_string_lossy().to_string()).collect()
                }
                ToolType::Temporary => {
                    let mut params = TemporaryParameters::default();
                    if params.extensions.is_empty() {
                         params.extensions = vec!["tmp".to_string(), "temp".to_string()];
                    }
                    let mut tf = Temporary::new(params);
                    tf.set_included_paths(vec![PathBuf::from(dir)]);
                    tf.search(&stop_flag, None);

                    tf.get_temporary_files().iter().map(|f| f.path.to_string_lossy().to_string()).collect()
                }
                ToolType::SimilarImages => {
                    let params = SimilarImagesParameters::new(10, 8, IHHashAlg::Gradient, IHFilterType::Lanczos3, false, false, GeometricInvariance::Off);
                    let mut si = SimilarImages::new(params);
                    si.set_included_paths(vec![PathBuf::from(dir)]);
                    si.search(&stop_flag, None);

                    let mut res = Vec::new();
                    for groups in si.get_similar_images() {
                        for entry in groups {
                            res.push(entry.path.to_string_lossy().to_string());
                        }
                    }
                    res
                }
                ToolType::SameMusic => {
                    let params = SameMusicParameters::new(MusicSimilarity::TRACK_TITLE, false, CheckingMethod::AudioTags, 10.0, 0.2, false);
                    let mut sm = SameMusic::new(params);
                    sm.set_included_paths(vec![PathBuf::from(dir)]);
                    sm.search(&stop_flag, None);

                    let mut res = Vec::new();
                    for groups in sm.get_duplicated_music_entries() {
                        for entry in groups {
                            res.push(entry.path.to_string_lossy().to_string());
                        }
                    }
                    res
                }
                ToolType::InvalidSymlinks => {
                    let mut is = InvalidSymlinks::new();
                    is.set_included_paths(vec![PathBuf::from(dir)]);
                    is.search(&stop_flag, None);

                    is.get_invalid_symlinks().iter().map(|s| s.path.to_string_lossy().to_string()).collect()
                }
                ToolType::BrokenFiles => {
                    let mut bf = BrokenFiles::new(BrokenFilesParameters::new(CheckedTypes::AUDIO)); // Just a sensible default for testing
                    bf.set_included_paths(vec![PathBuf::from(dir)]);
                    bf.search(&stop_flag, None);

                    bf.get_broken_files().iter().map(|f| f.path.to_string_lossy().to_string()).collect()
                }
                ToolType::SimilarVideos => {
                    let params = SimilarVideosParameters::new(
                        10, false, false, 0, 0, false, 0, 0.0, 0.0, 0.0, false, 0, false, 0, false, 0.0, 0.0, 0.0, 0
                    ); // Defaults from czkawka GUI
                    let mut sv = SimilarVideos::new(params);
                    sv.set_included_paths(vec![PathBuf::from(dir)]);
                    sv.search(&stop_flag, None);

                    let mut res = Vec::new();
                    for group in sv.get_similar_videos() {
                        for entry in group {
                            res.push(entry.path.to_string_lossy().to_string());
                        }
                    }
                    res
                }
                ToolType::BadExtensions => {
                    let params = BadExtensionsParameters::new();
                    let mut be = BadExtensions::new(params);
                    be.set_included_paths(vec![PathBuf::from(dir)]);
                    be.search(&stop_flag, None);

                    be.get_bad_extensions_files().iter().map(|f| f.path.to_string_lossy().to_string()).collect()
                }
                ToolType::BadNames => {
                    let name_issues = NameIssues {
                        uppercase_extension: true,
                        emoji_used: true,
                        space_at_start_or_end: true,
                        non_ascii_graphical: true,
                        restricted_charset_allowed: None,
                        remove_duplicated_non_alphanumeric: true,
                    };
                    let params = BadNamesParameters::new(name_issues);
                    let mut bn = BadNames::new(params);
                    bn.set_included_paths(vec![PathBuf::from(dir)]);
                    bn.search(&stop_flag, None);

                    bn.get_bad_names_files().iter().map(|f| f.path.to_string_lossy().to_string()).collect()
                }
                ToolType::VideoOptimizer => {
                    let params = VideoOptimizerParameters::VideoTranscode(VideoTranscodeParams::new(
                        vec!["hevc".to_string(), "h265".to_string(), "av1".to_string(), "vp9".to_string()],
                        false,
                        0,
                        false,
                        2
                    ));
                    let mut vo = VideoOptimizer::new(params);
                    vo.set_included_paths(vec![PathBuf::from(dir)]);
                    vo.search(&stop_flag, None);

                    vo.get_video_transcode_entries().iter().map(|f| f.path.to_string_lossy().to_string()).collect()
                }
            };
            let _ = sender.send((tool_type, results));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use czkawka_core::common::config_cache_path::set_config_cache_path;

    #[test]
    fn test_duplicate_scanner_no_panic() {
        set_config_cache_path("czkawka_tui_test", "czkawka_tui_test");

        let mut scanner = Scanner::new();
        scanner.start_scan(ToolType::Duplicate, ".");

        let mut results_received = false;

        // Wait for results
        for _ in 0..100 {
            if let Ok((tool_type, results)) = scanner.receiver.try_recv() {
                assert_eq!(tool_type, ToolType::Duplicate);

                // If it found duplicates, verify group headers exist.
                // There might not be duplicates in ".", but if there are,
                // they should have group headers.
                if !results.is_empty() {
                    let has_group_header = results.iter().any(|r| r.starts_with("--- Group"));
                    assert!(has_group_header, "Expected at least one group header if results exist");
                }

                results_received = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        assert!(results_received, "Scanner did not return results in a reasonable time");
    }
}
