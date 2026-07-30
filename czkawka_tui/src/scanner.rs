use crossbeam_channel::{unbounded, Receiver};
use czkawka_core::common::model::{CheckingMethod, HashType};
use czkawka_core::common::traits::Search;
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::tools::duplicate::{DuplicateFinder, DuplicateFinderParameters};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::path::PathBuf;
use std::thread;

pub struct Scanner {
    pub receiver: Receiver<Vec<String>>,
}

impl Scanner {
    pub fn new(dir: &str) -> Self {
        let (sender, receiver) = unbounded();

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

        thread::spawn(move || {
            let stop_flag = Arc::new(AtomicBool::new(false));
            df.search(&stop_flag, None);

            // Format results as strings
            let results = df.get_files_sorted_by_hash();
            let mut string_results = Vec::new();
            for (_hash, groups) in results {
                for group in groups {
                    for entry in group {
                        string_results.push(entry.path.to_string_lossy().to_string());
                    }
                }
            }

            let _ = sender.send(string_results);
        });

        Scanner { receiver }
    }
}
