use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crossbeam_channel::Receiver;
use fun_time::fun_time;
use futures::channel::mpsc::UnboundedSender;
use log::debug;

use crate::common_dir_traversal::{Collect, DirTraversalBuilder, DirTraversalResult, FolderEmptiness, FolderEntry, ProgressData, ToolType};
use crate::common_tool::{CommonData, CommonToolData};
use crate::common_traits::{DebugPrint, PrintResults};

pub struct EmptyFolder {
    common_data: CommonToolData,
    information: Info,
    delete_folders: bool,
    empty_folder_list: BTreeMap<PathBuf, FolderEntry>, // Path, FolderEntry
}

#[derive(Default)]
pub struct Info {
    pub number_of_empty_folders: usize,
}

impl EmptyFolder {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::EmptyFolders),
            information: Default::default(),
            delete_folders: false,
            empty_folder_list: Default::default(),
        }
    }

    pub const fn get_empty_folder_list(&self) -> &BTreeMap<PathBuf, FolderEntry> {
        &self.empty_folder_list
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    #[fun_time(message = "find_empty_folders")]
    pub fn find_empty_folders(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.optimize_dirs_before_start();
        if !self.check_for_empty_folders(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.optimize_folders();
        if self.delete_folders {
            self.delete_empty_folders();
        }
        self.debug_print();
    }

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

    #[fun_time(message = "check_for_empty_folders")]
    fn check_for_empty_folders(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let result = DirTraversalBuilder::new()
            .root_dirs(self.common_data.directories.included_directories.clone())
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .directories(self.common_data.directories.clone())
            .excluded_items(self.common_data.excluded_items.clone())
            .collect(Collect::EmptyFolders)
            .max_stage(0)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { .. } => {
                unreachable!()
            }
            DirTraversalResult::SuccessFolders { folder_entries, warnings } => {
                for (name, folder_entry) in folder_entries {
                    if folder_entry.is_empty != FolderEmptiness::No {
                        self.empty_folder_list.insert(name, folder_entry);
                    }
                }

                self.common_data.text_messages.warnings.extend(warnings);
                debug!("Found {} empty folders.", self.empty_folder_list.len());
                true
            }
            DirTraversalResult::Stopped => false,
        }
    }

    #[fun_time(message = "delete_empty_folders")]
    fn delete_empty_folders(&mut self) {
        // Folders may be deleted or require too big privileges
        for name in self.empty_folder_list.keys() {
            match fs::remove_dir_all(name) {
                Ok(()) => (),
                Err(e) => self
                    .common_data
                    .text_messages
                    .warnings
                    .push(format!("Failed to remove folder {}, reason {}", name.display(), e)),
            };
        }
    }
}

impl Default for EmptyFolder {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for EmptyFolder {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }

        println!("---------------DEBUG PRINT---------------");
        println!("Number of empty folders - {}", self.information.number_of_empty_folders);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for EmptyFolder {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.empty_folder_list.is_empty() {
            writeln!(writer, "--------------------------Empty folder list--------------------------")?;
            writeln!(writer, "Found {} empty folders", self.information.number_of_empty_folders)?;
            for name in self.empty_folder_list.keys() {
                writeln!(writer, "{}", name.display())?;
            }
        } else {
            write!(writer, "Not found any empty folders.")?;
        }

        Ok(())
    }
}

impl CommonData for EmptyFolder {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
}
impl EmptyFolder {
    pub fn set_delete_folder(&mut self, delete_folder: bool) {
        self.delete_folders = delete_folder;
    }
}
