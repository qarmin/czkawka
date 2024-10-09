use crate::CurrentTab;
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
use std::cell::RefCell;
use std::rc::Rc;

pub type SharedState<T> = Rc<RefCell<Option<T>>>;

pub struct SharedModels {
    pub shared_duplication_state: SharedState<DuplicateFinder>,
    pub shared_empty_folders_state: SharedState<EmptyFolder>,
    pub shared_empty_files_state: SharedState<EmptyFiles>,
    pub shared_temporary_files_state: SharedState<Temporary>,
    pub shared_big_files_state: SharedState<BigFile>,
    pub shared_similar_images_state: SharedState<SimilarImages>,
    pub shared_similar_videos_state: SharedState<SimilarVideos>,
    pub shared_same_music_state: SharedState<SameMusic>,
    pub shared_same_invalid_symlinks: SharedState<InvalidSymlinks>,
    pub shared_broken_files_state: SharedState<BrokenFiles>,
    pub shared_bad_extensions_state: SharedState<BadExtensions>,
}

impl SharedModels {
    pub fn new() -> Self {
        Self {
            shared_duplication_state: Rc::new(RefCell::new(None)),
            shared_empty_folders_state: Rc::new(RefCell::new(None)),
            shared_empty_files_state: Rc::new(RefCell::new(None)),
            shared_temporary_files_state: Rc::new(RefCell::new(None)),
            shared_big_files_state: Rc::new(RefCell::new(None)),
            shared_similar_images_state: Rc::new(RefCell::new(None)),
            shared_similar_videos_state: Rc::new(RefCell::new(None)),
            shared_same_music_state: Rc::new(RefCell::new(None)),
            shared_same_invalid_symlinks: Rc::new(RefCell::new(None)),
            shared_broken_files_state: Rc::new(RefCell::new(None)),
            shared_bad_extensions_state: Rc::new(RefCell::new(None)),
        }
    }

    pub fn new_shared() -> Rc<Self> {
        Rc::new(Self::new())
    }

    pub fn save_results(&self, active_tab: CurrentTab, choosen_dir: &str) -> Result<(), String> {
        let cd = choosen_dir;
        let result = match active_tab {
            CurrentTab::DuplicateFiles => self.shared_duplication_state.borrow().as_ref().map(|x| x.save_all_in_one(cd, "results_duplicates")),
            CurrentTab::EmptyFolders => self
                .shared_empty_folders_state
                .borrow()
                .as_ref()
                .map(|x| x.save_all_in_one(cd, "results_empty_directories")),
            CurrentTab::EmptyFiles => self.shared_empty_files_state.borrow().as_ref().map(|x| x.save_all_in_one(cd, "results_empty_files")),
            CurrentTab::TemporaryFiles => self
                .shared_temporary_files_state
                .borrow()
                .as_ref()
                .map(|x| x.save_all_in_one(cd, "results_temporary_files")),
            CurrentTab::BigFiles => self.shared_big_files_state.borrow().as_ref().map(|x| x.save_all_in_one(cd, "results_big_files")),
            CurrentTab::SimilarImages => self.shared_similar_images_state.borrow().as_ref().map(|x| x.save_all_in_one(cd, "results_similar_images")),
            CurrentTab::SimilarVideos => self.shared_similar_videos_state.borrow().as_ref().map(|x| x.save_all_in_one(cd, "results_similar_videos")),
            CurrentTab::SimilarMusic => self.shared_same_music_state.borrow().as_ref().map(|x| x.save_all_in_one(cd, "results_same_music")),
            CurrentTab::InvalidSymlinks => self
                .shared_same_invalid_symlinks
                .borrow()
                .as_ref()
                .map(|x| x.save_all_in_one(cd, "results_invalid_symlinks")),
            CurrentTab::BrokenFiles => self.shared_broken_files_state.borrow().as_ref().map(|x| x.save_all_in_one(cd, "results_broken_files")),
            CurrentTab::BadExtensions => self.shared_bad_extensions_state.borrow().as_ref().map(|x| x.save_all_in_one(cd, "results_bad_extensions")),
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
