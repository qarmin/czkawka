use crate::common::Common;
use crate::common_directory::Directories;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::{DebugPrint, PrintResults, SaveResults};
use crossbeam_channel::Receiver;
use std::collections::BTreeMap;
use std::fs::{File, Metadata};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{fs, thread};

#[derive(Debug)]
pub struct ProgressData {
    pub current_stage: u8,
    pub max_stage: u8,
    pub folders_checked: usize,
}

/// Enum with values which show if folder is empty.
/// In function "optimize_folders" automatically "Maybe" is changed to "Yes", so it is not necessary to put it here
#[derive(Eq, PartialEq, Copy, Clone)]
enum FolderEmptiness {
    No,
    Maybe,
}

/// Struct assigned to each checked folder with parent path(used to ignore parent if children are not empty) and flag which shows if folder is empty
#[derive(Clone)]
pub struct FolderEntry {
    parent_path: Option<PathBuf>, // Usable only when finding
    is_empty: FolderEmptiness,
    pub modified_date: u64,
}

/// Struct to store most basics info about all folder
pub struct EmptyFolder {
    information: Info,
    delete_folders: bool,
    text_messages: Messages,
    excluded_items: ExcludedItems,
    empty_folder_list: BTreeMap<PathBuf, FolderEntry>, // Path, FolderEntry
    directories: Directories,
    stopped_search: bool,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_empty_folders: usize,
}
impl Info {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Method implementation for EmptyFolder
impl EmptyFolder {
    /// New function providing basics values
    pub fn new() -> Self {
        Self {
            information: Default::default(),
            delete_folders: false,
            text_messages: Messages::new(),
            excluded_items: Default::default(),
            empty_folder_list: Default::default(),
            directories: Directories::new(),
            stopped_search: false,
        }
    }

    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    pub const fn get_empty_folder_list(&self) -> &BTreeMap<PathBuf, FolderEntry> {
        &self.empty_folder_list
    }

    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }
    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }
    /// Public function used by CLI to search for empty folders
    pub fn find_empty_folders(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(true, &mut self.text_messages);
        if !self.check_for_empty_folders(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        self.optimize_folders();
        if self.delete_folders {
            self.delete_empty_folders();
        }
        self.debug_print();
    }

    pub fn set_delete_folder(&mut self, delete_folder: bool) {
        self.delete_folders = delete_folder;
    }

    /// Clean directory tree
    /// If directory contains only 2 empty folders, then this directory should be removed instead two empty folders inside because it will produce another empty folder.
    fn optimize_folders(&mut self) {
        let mut new_directory_folders: BTreeMap<PathBuf, FolderEntry> = Default::default();

        for (name, folder_entry) in &self.empty_folder_list {
            match &folder_entry.parent_path {
                Some(t) => {
                    if !self.empty_folder_list.contains_key(t) {
                        new_directory_folders.insert(name.clone(), folder_entry.clone());
                    }
                }
                None => {
                    new_directory_folders.insert(name.clone(), folder_entry.clone());
                }
            }
        }
        self.empty_folder_list = new_directory_folders;
        self.information.number_of_empty_folders = self.empty_folder_list.len();
    }

    /// Function to check if folder are empty.
    /// Parameter initial_checking for second check before deleting to be sure that checked folder is still empty
    fn check_for_empty_folders(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&futures::channel::mpsc::UnboundedSender<ProgressData>>) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector
        let mut folders_checked: BTreeMap<PathBuf, FolderEntry> = Default::default();

        //// PROGRESS THREAD START
        const LOOP_DURATION: u32 = 200; //in ms
        let progress_thread_run = Arc::new(AtomicBool::new(true));

        let atomic_folder_counter = Arc::new(AtomicUsize::new(0));

        let progress_thread_handle;
        if let Some(progress_sender) = progress_sender {
            let progress_send = progress_sender.clone();
            let progress_thread_run = progress_thread_run.clone();
            let atomic_folder_counter = atomic_folder_counter.clone();
            progress_thread_handle = thread::spawn(move || loop {
                progress_send
                    .unbounded_send(ProgressData {
                        current_stage: 0,
                        max_stage: 0,
                        folders_checked: atomic_folder_counter.load(Ordering::Relaxed) as usize,
                    })
                    .unwrap();
                if !progress_thread_run.load(Ordering::Relaxed) {
                    break;
                }
                sleep(Duration::from_millis(LOOP_DURATION as u64));
            });
        } else {
            progress_thread_handle = thread::spawn(|| {});
        }
        //// PROGRESS THREAD END

        // Add root folders for finding
        for id in &self.directories.included_directories {
            folders_checked.insert(
                id.clone(),
                FolderEntry {
                    parent_path: None,
                    is_empty: FolderEmptiness::Maybe,
                    modified_date: 0,
                },
            );
            folders_to_check.push(id.clone());
        }

        while !folders_to_check.is_empty() {
            if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                // End thread which send info to gui
                progress_thread_run.store(false, Ordering::Relaxed);
                progress_thread_handle.join().unwrap();
                return false;
            }
            let current_folder = folders_to_check.pop().unwrap();
            // Checked folder may be deleted or we may not have permissions to open it so we assume that this folder is not be empty
            let read_dir = match fs::read_dir(&current_folder) {
                Ok(t) => t,
                Err(_inspected) => {
                    folders_checked.get_mut(&current_folder).unwrap().is_empty = FolderEmptiness::No;
                    continue;
                }
            };

            'dir: for entry in read_dir {
                let entry_data = match entry {
                    Ok(t) => t,
                    Err(_inspected) => {
                        set_as_not_empty_folder(&mut folders_checked, &current_folder);
                        continue 'dir;
                    } //Permissions denied
                };
                let metadata: Metadata = match entry_data.metadata() {
                    Ok(t) => t,
                    Err(_inspected) => {
                        set_as_not_empty_folder(&mut folders_checked, &current_folder);
                        continue 'dir;
                    } //Permissions denied
                };
                // If child is dir, still folder may be considered as empty if all children are only directories.
                if metadata.is_dir() {
                    atomic_folder_counter.fetch_add(1, Ordering::Relaxed);
                    let next_folder = current_folder.join(entry_data.file_name());
                    if self.excluded_items.is_excluded(&next_folder) || self.directories.is_excluded(&next_folder) {
                        set_as_not_empty_folder(&mut folders_checked, &current_folder);
                        continue 'dir;
                    }
                    folders_to_check.push(next_folder.clone());
                    folders_checked.insert(
                        next_folder.clone(),
                        FolderEntry {
                            parent_path: Some(current_folder.clone()),
                            is_empty: FolderEmptiness::Maybe,
                            modified_date: match metadata.modified() {
                                Ok(t) => match t.duration_since(UNIX_EPOCH) {
                                    Ok(d) => d.as_secs(),
                                    Err(_inspected) => {
                                        self.text_messages.warnings.push(format!("Folder {} seems to be modified before Unix Epoch.", current_folder.display()));
                                        0
                                    }
                                },
                                Err(e) => {
                                    self.text_messages.warnings.push(format!("Failed to read modification date of folder {}, reason {}", current_folder.display(), e));
                                    0
                                }
                            },
                        },
                    );
                } else {
                    set_as_not_empty_folder(&mut folders_checked, &current_folder)
                }
            }
        }
        // End thread which send info to gui
        progress_thread_run.store(false, Ordering::Relaxed);
        progress_thread_handle.join().unwrap();

        // We need to set empty folder list
        #[allow(unused_mut)] // Used is later by Windows build
        for (mut name, folder_entry) in folders_checked {
            if folder_entry.is_empty != FolderEmptiness::No {
                self.empty_folder_list.insert(name, folder_entry);
            }
        }

        Common::print_time(start_time, SystemTime::now(), "check_for_empty_folder".to_string());
        true
    }

    /// Deletes earlier found empty folders
    fn delete_empty_folders(&mut self) {
        let start_time: SystemTime = SystemTime::now();
        // Folders may be deleted or require too big privileges
        for name in self.empty_folder_list.keys() {
            match fs::remove_dir_all(name) {
                Ok(_) => (),
                Err(e) => self.text_messages.warnings.push(format!("Failed to remove folder {}, reason {}", name.display(), e)),
            };
        }

        Common::print_time(start_time, SystemTime::now(), "delete_files".to_string());
    }

    /// Set included dir which needs to be relative, exists etc.
    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) {
        self.directories.set_included_directory(included_directory, &mut self.text_messages);
    }
}

fn set_as_not_empty_folder(folders_checked: &mut BTreeMap<PathBuf, FolderEntry>, current_folder: &Path) {
    // Not folder so it may be a file or symbolic link so it isn't empty
    folders_checked.get_mut(current_folder).unwrap().is_empty = FolderEmptiness::No;
    let mut d = folders_checked.get_mut(current_folder).unwrap();
    // Loop to recursively set as non empty this and all his parent folders
    loop {
        d.is_empty = FolderEmptiness::No;
        if d.parent_path != None {
            let cf = d.parent_path.clone().unwrap();
            d = folders_checked.get_mut(&cf).unwrap();
        } else {
            break;
        }
    }
}

impl Default for EmptyFolder {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for EmptyFolder {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        println!("Number of empty folders - {}", self.information.number_of_empty_folders);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("-----------------------------------------");
    }
}
impl SaveResults for EmptyFolder {
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
        let start_time: SystemTime = SystemTime::now();
        let file_name: String = match file_name {
            "" => "results.txt".to_string(),
            k => k.to_string(),
        };

        let file_handler = match File::create(&file_name) {
            Ok(t) => t,
            Err(e) => {
                self.text_messages.errors.push(format!("Failed to create file {}, reason {}", file_name, e));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(writer, "Results of searching {:?} with excluded directories {:?}", self.directories.included_directories, self.directories.excluded_directories) {
            self.text_messages.errors.push(format!("Failed to save results to file {}, reason {}", file_name, e));
            return false;
        }

        if !self.empty_folder_list.is_empty() {
            writeln!(writer, "-------------------------------------------------Empty folder list-------------------------------------------------").unwrap();
            writeln!(writer, "Found {} empty folders", self.information.number_of_empty_folders).unwrap();
            for name in self.empty_folder_list.keys() {
                writeln!(writer, "{}", name.display()).unwrap();
            }
        } else {
            write!(writer, "Not found any empty folders.").unwrap();
        }
        Common::print_time(start_time, SystemTime::now(), "save_results_to_file".to_string());
        true
    }
}
impl PrintResults for EmptyFolder {
    /// Prints basic info about empty folders // TODO print better
    fn print_results(&self) {
        if !self.empty_folder_list.is_empty() {
            println!("Found {} empty folders", self.empty_folder_list.len());
        }
        for name in self.empty_folder_list.keys() {
            println!("{}", name.display());
        }
    }
}
