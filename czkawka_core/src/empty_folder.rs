use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crossbeam_channel::Receiver;
use fun_time::fun_time;
use futures::channel::mpsc::UnboundedSender;
use log::debug;
use rayon::prelude::*;

use crate::common_dir_traversal::{Collect, DirTraversalBuilder, DirTraversalResult, FolderEmptiness, FolderEntry, ProgressData, ToolType};
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::{DebugPrint, PrintResults};

pub struct EmptyFolder {
    common_data: CommonToolData,
    information: Info,
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
            empty_folder_list: Default::default(),
        }
    }

    pub const fn get_empty_folder_list(&self) -> &BTreeMap<PathBuf, FolderEntry> {
        &self.empty_folder_list
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    #[fun_time(message = "find_empty_folders", level = "info")]
    pub fn find_empty_folders(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.optimize_dirs_before_start();
        if !self.check_for_empty_folders(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.optimize_folders();

        self.delete_files();
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

    #[fun_time(message = "check_for_empty_folders", level = "debug")]
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

    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self) {
        if self.get_delete_method() == DeleteMethod::None {
            return;
        }
        let folders_to_remove = self.empty_folder_list.keys().collect::<Vec<_>>();

        let errors: Vec<_> = folders_to_remove
            .into_par_iter()
            .filter_map(|name| {
                if let Err(e) = fs::remove_dir_all(name) {
                    Some(format!("Failed to remove folder {name:?}, reason {e}"))
                } else {
                    None
                }
            })
            .collect();
        self.get_text_messages_mut().errors.extend(errors);
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

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.empty_folder_list.keys().collect::<Vec<_>>(), pretty_print)
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
