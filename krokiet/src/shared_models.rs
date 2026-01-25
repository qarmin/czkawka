use std::sync::{Arc, Mutex};

use czkawka_core::common::traits::PrintResults;
use czkawka_core::tools::bad_extensions::BadExtensions;
use czkawka_core::tools::bad_names::BadNames;
use czkawka_core::tools::big_file::BigFile;
use czkawka_core::tools::broken_files::BrokenFiles;
use czkawka_core::tools::duplicate::DuplicateFinder;
use czkawka_core::tools::empty_files::EmptyFiles;
use czkawka_core::tools::empty_folder::EmptyFolder;
use czkawka_core::tools::exif_remover::ExifRemover;
use czkawka_core::tools::invalid_symlinks::InvalidSymlinks;
use czkawka_core::tools::same_music::SameMusic;
use czkawka_core::tools::similar_images::SimilarImages;
use czkawka_core::tools::similar_videos::SimilarVideos;
use czkawka_core::tools::temporary::Temporary;
use czkawka_core::tools::video_optimizer::VideoOptimizer;

use crate::ActiveTab;

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
    pub shared_bad_names_state: Option<BadNames>,
    pub shared_exif_remover_state: Option<ExifRemover>,
    pub shared_video_optimizer_state: Option<VideoOptimizer>,
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
            shared_bad_names_state: None,
            shared_exif_remover_state: None,
            shared_video_optimizer_state: None,
        }
    }

    pub fn new_shared() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::new()))
    }

    pub(crate) fn save_results(&self, active_tab: ActiveTab, chosen_dir: &str) -> Result<(), String> {
        let cd = chosen_dir;
        let result = match active_tab {
            ActiveTab::DuplicateFiles => self.shared_duplication_state.as_ref().map(|x| x.save_all_in_one(cd, "results_duplicates")),
            ActiveTab::EmptyFolders => self.shared_empty_folders_state.as_ref().map(|x| x.save_all_in_one(cd, "results_empty_directories")),
            ActiveTab::EmptyFiles => self.shared_empty_files_state.as_ref().map(|x| x.save_all_in_one(cd, "results_empty_files")),
            ActiveTab::TemporaryFiles => self.shared_temporary_files_state.as_ref().map(|x| x.save_all_in_one(cd, "results_temporary_files")),
            ActiveTab::BigFiles => self.shared_big_files_state.as_ref().map(|x| x.save_all_in_one(cd, "results_big_files")),
            ActiveTab::SimilarImages => self.shared_similar_images_state.as_ref().map(|x| x.save_all_in_one(cd, "results_similar_images")),
            ActiveTab::SimilarVideos => self.shared_similar_videos_state.as_ref().map(|x| x.save_all_in_one(cd, "results_similar_videos")),
            ActiveTab::SimilarMusic => self.shared_same_music_state.as_ref().map(|x| x.save_all_in_one(cd, "results_same_music")),
            ActiveTab::InvalidSymlinks => self.shared_same_invalid_symlinks.as_ref().map(|x| x.save_all_in_one(cd, "results_invalid_symlinks")),
            ActiveTab::BrokenFiles => self.shared_broken_files_state.as_ref().map(|x| x.save_all_in_one(cd, "results_broken_files")),
            ActiveTab::BadExtensions => self.shared_bad_extensions_state.as_ref().map(|x| x.save_all_in_one(cd, "results_bad_extensions")),
            ActiveTab::BadNames => self.shared_bad_names_state.as_ref().map(|x| x.save_all_in_one(cd, "results_bad_names")),
            ActiveTab::ExifRemover => self.shared_exif_remover_state.as_ref().map(|x| x.save_all_in_one(cd, "results_exif_remover")),
            ActiveTab::VideoOptimizer => self.shared_video_optimizer_state.as_ref().map(|x| x.save_all_in_one(cd, "results_video_optimizer")),
            ActiveTab::Settings | ActiveTab::About => panic!("Cannot save results for settings or about tab"),
        };

        let current_path = match std::env::current_dir() {
            Ok(t) => t.to_string_lossy().to_string(),
            Err(_) => "<unknown>".to_string(),
        };

        match result.expect("Tried to save results, without running scan(bug which needs to be fixed)") {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Failed to save results to folder \"{current_path}\", reason {e}")),
        }
    }
}
