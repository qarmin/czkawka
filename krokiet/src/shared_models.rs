use std::sync::{Arc, Mutex};

use czkawka_core::bad_extensions::BadExtensions;
use czkawka_core::big_file::BigFile;
use czkawka_core::broken_files::BrokenFiles;
use czkawka_core::common_traits::PrintResults;
use czkawka_core::duplicate::DuplicateFinder;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::invalid_symlinks::InvalidSymlinks;
use czkawka_core::same_music::SameMusic;
use czkawka_core::similar_images::SimilarImages;
use czkawka_core::similar_videos::SimilarVideos;
use czkawka_core::temporary::Temporary;

use crate::CurrentTab;

pub struct SharedModels {
    pub shared_duplication_state: Option<DuplicateFinder>,
    pub shared_empty_folders_state: Option<EmptyFolder>,
    pub shared_empty_files_state: Option<EmptyFiles>,
    pub shared_temporary_files_state: Option<Temporary>,
    pub shared_big_files_state: Option<BigFile>,
    pub shared_similar_images_state: Option<SimilarImages>,
    pub shared_similar_videos_state: Option<SimilarVideos>,
    pub shared_same_music_state: Option<SameMusic>,
    pub shared_same_invalid_symlinks: Option<InvalidSymlinks>,
    pub shared_broken_files_state: Option<BrokenFiles>,
    pub shared_bad_extensions_state: Option<BadExtensions>,
}

impl SharedModels {
    pub fn new() -> Self {
        Self {
            shared_duplication_state: None,
            shared_empty_folders_state: None,
            shared_empty_files_state: None,
            shared_temporary_files_state: None,
            shared_big_files_state: None,
            shared_similar_images_state: None,
            shared_similar_videos_state: None,
            shared_same_music_state: None,
            shared_same_invalid_symlinks: None,
            shared_broken_files_state: None,
            shared_bad_extensions_state: None,
        }
    }

    pub fn new_shared() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::new()))
    }

    pub fn save_results(&self, active_tab: CurrentTab, choosen_dir: &str) -> Result<(), String> {
        let cd = choosen_dir;
        let result = match active_tab {
            CurrentTab::DuplicateFiles => self.shared_duplication_state.as_ref().map(|x| x.save_all_in_one(cd, "results_duplicates")),
            CurrentTab::EmptyFolders => self.shared_empty_folders_state.as_ref().map(|x| x.save_all_in_one(cd, "results_empty_directories")),
            CurrentTab::EmptyFiles => self.shared_empty_files_state.as_ref().map(|x| x.save_all_in_one(cd, "results_empty_files")),
            CurrentTab::TemporaryFiles => self.shared_temporary_files_state.as_ref().map(|x| x.save_all_in_one(cd, "results_temporary_files")),
            CurrentTab::BigFiles => self.shared_big_files_state.as_ref().map(|x| x.save_all_in_one(cd, "results_big_files")),
            CurrentTab::SimilarImages => self.shared_similar_images_state.as_ref().map(|x| x.save_all_in_one(cd, "results_similar_images")),
            CurrentTab::SimilarVideos => self.shared_similar_videos_state.as_ref().map(|x| x.save_all_in_one(cd, "results_similar_videos")),
            CurrentTab::SimilarMusic => self.shared_same_music_state.as_ref().map(|x| x.save_all_in_one(cd, "results_same_music")),
            CurrentTab::InvalidSymlinks => self.shared_same_invalid_symlinks.as_ref().map(|x| x.save_all_in_one(cd, "results_invalid_symlinks")),
            CurrentTab::BrokenFiles => self.shared_broken_files_state.as_ref().map(|x| x.save_all_in_one(cd, "results_broken_files")),
            CurrentTab::BadExtensions => self.shared_bad_extensions_state.as_ref().map(|x| x.save_all_in_one(cd, "results_bad_extensions")),
            CurrentTab::Settings | CurrentTab::About => panic!("Cannot save results for settings or about tab"),
        };

        let current_path = match std::env::current_dir() {
            Ok(t) => t.to_string_lossy().to_string(),
            Err(_) => "<unknown>".to_string(),
        };

        match result.expect("Tried to save results, without running scan(bug which needs to be fixed)") {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Failed to save results to folder {current_path}, reason {e}")),
        }
    }
}
