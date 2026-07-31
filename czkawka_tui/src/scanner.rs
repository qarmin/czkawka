use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use crossbeam_channel::{Receiver, Sender, unbounded};
use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::bad_extensions::{BadExtensions, BadExtensionsParameters};
use czkawka_core::tools::bad_names::{BadNames, BadNamesParameters, NameIssues};
use czkawka_core::tools::big_file::{BigFile, BigFileParameters, SearchMode};
use czkawka_core::tools::broken_files::{BrokenFiles, BrokenFilesParameters, CheckedTypes};
use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};
use czkawka_core::tools::empty_files::{EmptyFiles, EmptyFilesParameters};
use czkawka_core::tools::empty_folder::EmptyFolder;
use czkawka_core::tools::invalid_symlinks::InvalidSymlinks;
use czkawka_core::tools::same_music::{MusicSimilarity, SameMusic, SameMusicParameters};
use czkawka_core::tools::similar_images::{GeometricInvariance, SimilarImages, SimilarImagesParameters};
use czkawka_core::tools::similar_videos::{SimilarVideos, SimilarVideosParameters};
use czkawka_core::tools::temporary::{Temporary, TemporaryParameters};
use czkawka_core::tools::video_optimizer::{VideoOptimizer, VideoOptimizerParameters, VideoTranscodeParams};
use image_hasher::{FilterType as IHFilterType, HashAlg as IHHashAlg};

use crate::app::{ToolType, TuiGroup, TuiItem};

pub struct Scanner {
    pub receiver: Receiver<(ToolType, Vec<TuiGroup>)>,
    sender: Sender<(ToolType, Vec<TuiGroup>)>,
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

    pub fn start_scan(&mut self, tool_type: ToolType, included_dirs: Vec<String>, reference_dirs: Vec<String>) {
        self.stop_flag.store(false, Ordering::Relaxed);
        let stop_flag = self.stop_flag.clone();
        let sender = self.sender.clone();

        let included_paths: Vec<std::path::PathBuf> = included_dirs.into_iter().map(std::path::PathBuf::from).collect();
        let reference_paths: Vec<std::path::PathBuf> = reference_dirs.into_iter().map(std::path::PathBuf::from).collect();

        thread::spawn(move || {
            let _dir = ".".to_string(); // Temporary fallback
            let results = match tool_type {
                ToolType::Duplicate => {
                    let params = DuplicateFinderParameters::new(CheckingMethod::Hash, HashType::Blake3, false, 0, 0, false);
                    let mut df = DuplicateFinder::new(params);
                    df.set_included_paths(included_paths.clone());
                    df.set_reference_paths(reference_paths.clone());
                    df.search(&stop_flag, None);

                    let mut tui_groups = Vec::new();
                    for groups in df.get_files_sorted_by_hash().values() {
                        for group in groups {
                            let mut tui_group = TuiGroup { items: Vec::new() };
                            for entry in group {
                                tui_group.items.push(TuiItem {
                                    path: entry.path.to_string_lossy().to_string(),
                                    size: entry.size,
                                    modified_date: entry.modified_date,
                                });
                            }
                            tui_groups.push(tui_group);
                        }
                    }
                    tui_groups
                }
                ToolType::EmptyFolders => {
                    let mut ef = EmptyFolder::new();
                    ef.set_included_paths(included_paths.clone());
                    ef.search(&stop_flag, None);

                    let mut items = Vec::new();
                    for p in ef.get_empty_folder_list().values() {
                        items.push(TuiItem {
                            path: p.path.to_string_lossy().to_string(),
                            size: 0,
                            modified_date: p.modified_date,
                        });
                    }
                    vec![TuiGroup { items }]
                }
                ToolType::BigFiles => {
                    let params = BigFileParameters::new(50, SearchMode::BiggestFiles);
                    let mut bf = BigFile::new(params);
                    bf.set_included_paths(included_paths.clone());
                    bf.search(&stop_flag, None);

                    let mut items = Vec::new();
                    for f in bf.get_big_files() {
                        items.push(TuiItem {
                            path: f.path.to_string_lossy().to_string(),
                            size: f.size,
                            modified_date: f.modified_date,
                        });
                    }
                    vec![TuiGroup { items }]
                }
                ToolType::EmptyFiles => {
                    let mut ef = EmptyFiles::new(EmptyFilesParameters::default());
                    ef.set_included_paths(included_paths.clone());
                    ef.search(&stop_flag, None);

                    let mut items = Vec::new();
                    for f in ef.get_empty_files() {
                        items.push(TuiItem {
                            path: f.path.to_string_lossy().to_string(),
                            size: f.size,
                            modified_date: f.modified_date,
                        });
                    }
                    vec![TuiGroup { items }]
                }
                ToolType::Temporary => {
                    let mut params = TemporaryParameters::default();
                    if params.extensions.is_empty() {
                        params.extensions = vec!["tmp".to_string(), "temp".to_string()];
                    }
                    let mut tf = Temporary::new(params);
                    tf.set_included_paths(included_paths.clone());
                    tf.search(&stop_flag, None);

                    let mut items = Vec::new();
                    for f in tf.get_temporary_files() {
                        items.push(TuiItem {
                            path: f.path.to_string_lossy().to_string(),
                            size: f.size,
                            modified_date: f.modified_date,
                        });
                    }
                    vec![TuiGroup { items }]
                }
                ToolType::SimilarImages => {
                    let params = SimilarImagesParameters::new(10, 8, IHHashAlg::Gradient, IHFilterType::Lanczos3, false, false, GeometricInvariance::Off);
                    let mut si = SimilarImages::new(params);
                    si.set_included_paths(included_paths.clone());
                    si.set_reference_paths(reference_paths.clone());
                    si.search(&stop_flag, None);

                    let mut res = Vec::new();
                    for groups in si.get_similar_images() {
                        let mut tui_group = TuiGroup { items: Vec::new() };
                        for entry in groups {
                            tui_group.items.push(TuiItem {
                                path: entry.path.to_string_lossy().to_string(),
                                size: entry.size,
                                modified_date: entry.modified_date,
                            });
                        }
                        res.push(tui_group);
                    }
                    res
                }
                ToolType::SameMusic => {
                    let params = SameMusicParameters::new(MusicSimilarity::TRACK_TITLE, false, CheckingMethod::AudioTags, 10.0, 0.2, false);
                    let mut sm = SameMusic::new(params);
                    sm.set_included_paths(included_paths.clone());
                    sm.set_reference_paths(reference_paths.clone());
                    sm.search(&stop_flag, None);

                    let mut res = Vec::new();
                    for groups in sm.get_duplicated_music_entries() {
                        let mut tui_group = TuiGroup { items: Vec::new() };
                        for entry in groups {
                            tui_group.items.push(TuiItem {
                                path: entry.path.to_string_lossy().to_string(),
                                size: entry.size,
                                modified_date: entry.modified_date,
                            });
                        }
                        res.push(tui_group);
                    }
                    res
                }
                ToolType::InvalidSymlinks => {
                    let mut is = InvalidSymlinks::new();
                    is.set_included_paths(included_paths.clone());
                    is.search(&stop_flag, None);

                    let mut items = Vec::new();
                    for s in is.get_invalid_symlinks() {
                        items.push(TuiItem {
                            path: s.path.to_string_lossy().to_string(),
                            size: 0, // symlinks don't have direct size equivalent in the same way
                            modified_date: s.modified_date,
                        });
                    }
                    vec![TuiGroup { items }]
                }
                ToolType::BrokenFiles => {
                    let mut bf = BrokenFiles::new(BrokenFilesParameters::new(CheckedTypes::AUDIO)); // Just a sensible default for testing
                    bf.set_included_paths(included_paths.clone());
                    bf.search(&stop_flag, None);

                    let mut items = Vec::new();
                    for f in bf.get_broken_files() {
                        items.push(TuiItem {
                            path: f.path.to_string_lossy().to_string(),
                            size: f.size,
                            modified_date: f.modified_date,
                        });
                    }
                    vec![TuiGroup { items }]
                }
                ToolType::SimilarVideos => {
                    let params = SimilarVideosParameters::new(10, false, false, 0, 0, false, 0, 0.0, 0.0, 0.0, false, 0, false, 0, false, 0.0, 0.0, 0.0, 0); // Defaults from czkawka GUI
                    let mut sv = SimilarVideos::new(params);
                    sv.set_included_paths(included_paths.clone());
                    sv.set_reference_paths(reference_paths.clone());
                    sv.search(&stop_flag, None);

                    let mut res = Vec::new();
                    for group in sv.get_similar_videos() {
                        let mut tui_group = TuiGroup { items: Vec::new() };
                        for entry in group {
                            tui_group.items.push(TuiItem {
                                path: entry.path.to_string_lossy().to_string(),
                                size: entry.size,
                                modified_date: entry.modified_date,
                            });
                        }
                        res.push(tui_group);
                    }
                    res
                }
                ToolType::BadExtensions => {
                    let params = BadExtensionsParameters::new();
                    let mut be = BadExtensions::new(params);
                    be.set_included_paths(included_paths.clone());
                    be.search(&stop_flag, None);

                    let mut items = Vec::new();
                    for f in be.get_bad_extensions_files() {
                        items.push(TuiItem {
                            path: f.path.to_string_lossy().to_string(),
                            size: f.size,
                            modified_date: f.modified_date,
                        });
                    }
                    vec![TuiGroup { items }]
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
                    bn.set_included_paths(included_paths.clone());
                    bn.search(&stop_flag, None);

                    let mut items = Vec::new();
                    for f in bn.get_bad_names_files() {
                        items.push(TuiItem {
                            path: f.path.to_string_lossy().to_string(),
                            size: f.size,
                            modified_date: f.modified_date,
                        });
                    }
                    vec![TuiGroup { items }]
                }
                ToolType::VideoOptimizer => {
                    let params = VideoOptimizerParameters::VideoTranscode(VideoTranscodeParams::new(
                        vec!["hevc".to_string(), "h265".to_string(), "av1".to_string(), "vp9".to_string()],
                        false,
                        0,
                        false,
                        2,
                    ));
                    let mut vo = VideoOptimizer::new(params);
                    vo.set_included_paths(included_paths.clone());
                    vo.search(&stop_flag, None);

                    let mut items = Vec::new();
                    for f in vo.get_video_transcode_entries() {
                        items.push(TuiItem {
                            path: f.path.to_string_lossy().to_string(),
                            size: f.size,
                            modified_date: f.modified_date,
                        });
                    }
                    vec![TuiGroup { items }]
                }
            };
            let _ = sender.send((tool_type, results));
        });
    }
}

#[cfg(test)]
mod tests {
    use czkawka_core::common::config_cache_path::set_config_cache_path;

    use super::*;

    #[test]
    fn test_duplicate_scanner_no_panic() {
        set_config_cache_path("czkawka_tui_test", "czkawka_tui_test");

        let mut scanner = Scanner::new();
        scanner.start_scan(ToolType::Duplicate, vec![".".to_string()], vec![]);

        let mut results_received = false;

        // Wait for results
        for _ in 0..100 {
            if let Ok((tool_type, results)) = scanner.receiver.try_recv() {
                assert_eq!(tool_type, ToolType::Duplicate);

                // If it found duplicates, verify groups exist.
                // There might not be duplicates in ".", but if there are,
                // they should have group headers.
                if !results.is_empty() {
                    assert!(!results[0].items.is_empty(), "Expected at least one item if results exist");
                }

                results_received = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        assert!(results_received, "Scanner did not return results in a reasonable time");
    }
}
