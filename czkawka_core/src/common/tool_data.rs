use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;

use crate::common::deletion;
pub use crate::common::deletion::{DeleteItemType, DeleteResult};
use crate::common::directories::Directories;
use crate::common::extensions::Extensions;
use crate::common::items::ExcludedItems;
use crate::common::model::{CheckingMethod, ToolType, WorkContinueStatus};
use crate::common::progress_data::ProgressData;
use crate::common::traits::ResultEntry;
use crate::helpers::messages::Messages;

#[derive(Debug, Clone, Default)]
pub struct CommonToolData {
    pub(crate) tool_type: ToolType,
    pub(crate) text_messages: Messages,
    pub(crate) directories: Directories,
    pub(crate) extensions: Extensions,
    pub(crate) excluded_items: ExcludedItems,
    pub(crate) recursive_search: bool,
    pub(crate) delete_method: DeleteMethod,
    pub(crate) maximal_file_size: u64,
    pub(crate) minimal_file_size: u64,
    pub(crate) stopped_search: bool,
    pub(crate) use_cache: bool,
    pub(crate) delete_outdated_cache: bool,
    pub(crate) save_also_as_json: bool,
    pub(crate) use_reference_folders: bool,
    pub(crate) dry_run: bool,
    pub(crate) move_to_trash: bool,
    pub(crate) hide_hard_links: bool,
}

#[derive(Eq, PartialEq, Clone, Debug, Copy, Default)]
pub enum DeleteMethod {
    #[default]
    None,
    Delete, // Just delete items
    AllExceptNewest,
    AllExceptOldest,
    OneOldest,
    OneNewest,
    HardLink,
    AllExceptBiggest,
    AllExceptSmallest,
    OneBiggest,
    OneSmallest,
}

impl CommonToolData {
    pub fn new(tool_type: ToolType) -> Self {
        Self {
            tool_type,
            text_messages: Messages::new(),
            directories: Directories::new(),
            extensions: Extensions::new(),
            excluded_items: ExcludedItems::new(),
            recursive_search: true,
            delete_method: DeleteMethod::None,
            maximal_file_size: u64::MAX,
            minimal_file_size: 0,
            stopped_search: false,
            use_cache: true,
            delete_outdated_cache: true,
            save_also_as_json: false,
            use_reference_folders: false,
            dry_run: false,
            move_to_trash: false,
            hide_hard_links: false,
        }
    }
}

pub trait CommonData {
    type Info;
    type Parameters;

    fn get_information(&self) -> Self::Info;
    fn get_params(&self) -> Self::Parameters;

    fn get_cd(&self) -> &CommonToolData;
    fn get_cd_mut(&mut self) -> &mut CommonToolData;
    fn get_check_method(&self) -> CheckingMethod {
        CheckingMethod::None
    }
    fn get_test_type(&self) -> (ToolType, CheckingMethod) {
        (self.get_cd().tool_type, self.get_check_method())
    }
    fn found_any_items(&self) -> bool;

    fn get_tool_type(&self) -> ToolType {
        self.get_cd().tool_type
    }

    fn set_hide_hard_links(&mut self, hide_hard_links: bool) {
        self.get_cd_mut().hide_hard_links = hide_hard_links;
    }
    fn get_hide_hard_links(&self) -> bool {
        self.get_cd().hide_hard_links
    }

    fn set_dry_run(&mut self, dry_run: bool) {
        self.get_cd_mut().dry_run = dry_run;
    }
    fn get_dry_run(&self) -> bool {
        self.get_cd().dry_run
    }

    fn set_use_cache(&mut self, use_cache: bool) {
        self.get_cd_mut().use_cache = use_cache;
    }
    fn get_use_cache(&self) -> bool {
        self.get_cd().use_cache
    }

    fn set_delete_outdated_cache(&mut self, delete_outdated_cache: bool) {
        self.get_cd_mut().delete_outdated_cache = delete_outdated_cache;
    }
    fn get_delete_outdated_cache(&self) -> bool {
        self.get_cd().delete_outdated_cache
    }

    fn get_stopped_search(&self) -> bool {
        self.get_cd().stopped_search
    }
    fn set_stopped_search(&mut self, stopped_search: bool) {
        self.get_cd_mut().stopped_search = stopped_search;
    }

    fn set_maximal_file_size(&mut self, maximal_file_size: u64) {
        self.get_cd_mut().maximal_file_size = match maximal_file_size {
            0 => 1,
            t => t,
        };
    }
    fn get_maximal_file_size(&self) -> u64 {
        self.get_cd().maximal_file_size
    }

    fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.get_cd_mut().minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }
    fn get_minimal_file_size(&self) -> u64 {
        self.get_cd().minimal_file_size
    }

    #[cfg(target_family = "unix")]
    fn set_exclude_other_filesystems(&mut self, exclude_other_filesystems: bool) {
        self.get_cd_mut().directories.set_exclude_other_filesystems(exclude_other_filesystems);
    }
    #[cfg(not(target_family = "unix"))]
    fn set_exclude_other_filesystems(&mut self, _exclude_other_filesystems: bool) {}

    fn get_text_messages(&self) -> &Messages {
        &self.get_cd().text_messages
    }
    fn get_text_messages_mut(&mut self) -> &mut Messages {
        &mut self.get_cd_mut().text_messages
    }

    fn set_save_also_as_json(&mut self, save_also_as_json: bool) {
        self.get_cd_mut().save_also_as_json = save_also_as_json;
    }
    fn get_save_also_as_json(&self) -> bool {
        self.get_cd().save_also_as_json
    }

    fn set_recursive_search(&mut self, recursive_search: bool) {
        self.get_cd_mut().recursive_search = recursive_search;
    }
    fn get_recursive_search(&self) -> bool {
        self.get_cd().recursive_search
    }

    fn set_use_reference_folders(&mut self, use_reference_folders: bool) {
        self.get_cd_mut().use_reference_folders = use_reference_folders;
    }
    fn get_use_reference_folders(&self) -> bool {
        self.get_cd().use_reference_folders
    }

    fn set_delete_method(&mut self, delete_method: DeleteMethod) {
        self.get_cd_mut().delete_method = delete_method;
    }
    fn get_delete_method(&self) -> DeleteMethod {
        self.get_cd().delete_method
    }

    // Only used for internal deleting - probably only useful in CLI, but not in GUI which probably uses its own delete method selection
    fn set_move_to_trash(&mut self, move_to_trash: bool) {
        self.get_cd_mut().move_to_trash = move_to_trash;
    }
    fn get_move_to_trash(&self) -> bool {
        self.get_cd().move_to_trash
    }

    fn set_included_paths(&mut self, included_paths: Vec<PathBuf>) {
        let messages = self.get_cd_mut().directories.set_included_paths(included_paths);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_excluded_paths(&mut self, excluded_paths: Vec<PathBuf>) {
        let messages = self.get_cd_mut().directories.set_excluded_paths(excluded_paths);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_reference_paths(&mut self, reference_paths: Vec<PathBuf>) {
        let messages = self.get_cd_mut().directories.set_reference_paths(reference_paths);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_allowed_extensions(&mut self, allowed_extensions: Vec<String>) {
        let messages = self.get_cd_mut().extensions.set_allowed_extensions(allowed_extensions);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_excluded_extensions(&mut self, excluded_extensions: Vec<String>) {
        let messages = self.get_cd_mut().extensions.set_excluded_extensions(excluded_extensions);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        let messages = self.get_cd_mut().excluded_items.set_excluded_items(excluded_items);
        self.get_cd_mut().text_messages.extend_with_another_messages(messages);
    }

    fn get_extensions_mut(&mut self) -> &mut Extensions {
        &mut self.get_cd_mut().extensions
    }

    #[expect(clippy::result_unit_err)]
    fn prepare_items(&mut self, tool_extensions: Option<&[&str]>) -> Result<(), ()> {
        let recursive_search = self.get_cd().recursive_search;
        // Optimizes directories and removes recursive calls
        match self.get_cd_mut().directories.optimize_directories(recursive_search, false) {
            Ok(messages) => {
                self.get_cd_mut().text_messages.extend_with_another_messages(messages);
            }
            Err(messages) => {
                self.get_cd_mut().text_messages.extend_with_another_messages(messages);
                return Err(());
            }
        }

        if let Err(e) = self.get_extensions_mut().set_and_validate_extensions(tool_extensions) {
            self.get_cd_mut().text_messages.critical = Some(e);
            return Err(());
        }

        Ok(())
    }

    fn delete_simple_elements_and_add_to_messages<T: ResultEntry + Sized + Send + Sync>(
        &mut self,
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
        delete_item_type: DeleteItemType<T>,
    ) -> WorkContinueStatus {
        deletion::delete_simple_elements_and_add_to_messages(self.get_cd_mut(), stop_flag, progress_sender, delete_item_type)
    }

    fn delete_advanced_elements_and_add_to_messages<T: ResultEntry + Sized + Send + Sync + Clone>(
        &mut self,
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
        files_to_process: Vec<Vec<T>>,
    ) -> WorkContinueStatus {
        deletion::delete_advanced_elements_and_add_to_messages(self.get_cd_mut(), stop_flag, progress_sender, files_to_process)
    }

    fn delete_elements<T: ResultEntry + Sized + Send + Sync>(
        &self,
        stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
        delete_item_type: DeleteItemType<T>,
    ) -> DeleteResult {
        deletion::delete_elements(self.get_cd(), stop_flag, progress_sender, delete_item_type)
    }

    #[expect(clippy::print_stdout)]
    fn debug_print_common(&self) {
        println!("---------------DEBUG PRINT COMMON---------------");
        println!("Included paths(before optimization) - {:?}", self.get_cd().directories.original_included_paths);
        println!("Excluded paths(before optimization) - {:?}", self.get_cd().directories.original_excluded_paths);
        println!("Reference paths(before optimization) - {:?}", self.get_cd().directories.original_reference_paths);
        println!("Included directories(optimized) - {:?}", self.get_cd().directories.included_directories);
        println!("Included files(optimized) - {:?}", self.get_cd().directories.included_files);
        println!("Excluded directories(optimized) - {:?}", self.get_cd().directories.excluded_directories);
        println!("Excluded files(optimized) - {:?}", self.get_cd().directories.excluded_files);
        println!("Reference directories(optimized) - {:?}", self.get_cd().directories.reference_directories);
        println!("Reference files(optimized) - {:?}", self.get_cd().directories.reference_files);
        println!("Tool type: {:?}", self.get_cd().tool_type);
        println!("Directories: {:?}", self.get_cd().directories);
        println!("Extensions: {:?}", self.get_cd().extensions);
        println!("Excluded items: {:?}", self.get_cd().excluded_items);
        println!("Recursive search: {}", self.get_cd().recursive_search);
        println!("Maximal file size: {}", self.get_cd().maximal_file_size);
        println!("Minimal file size: {}", self.get_cd().minimal_file_size);
        println!("Stopped search: {}", self.get_cd().stopped_search);
        println!("Use cache: {}", self.get_cd().use_cache);
        println!("Delete outdated cache: {}", self.get_cd().delete_outdated_cache);
        println!("Save also as json: {}", self.get_cd().save_also_as_json);
        println!("Delete method: {:?}", self.get_cd().delete_method);
        println!("Use reference folders: {}", self.get_cd().use_reference_folders);
        println!("Dry run: {}", self.get_cd().dry_run);
        println!("Hide hard links: {}", self.get_cd().hide_hard_links);

        println!("---------------DEBUG PRINT MESSAGES---------------");
        println!("Errors size - {}", self.get_cd().text_messages.errors.len());
        println!("Warnings size - {}", self.get_cd().text_messages.warnings.len());
        println!("Messages size - {}", self.get_cd().text_messages.messages.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_tool_data_new() {
        let tool_data = CommonToolData::new(ToolType::Duplicate);
        assert_eq!(tool_data.tool_type, ToolType::Duplicate);
        assert_eq!(tool_data.delete_method, DeleteMethod::None);
        assert_eq!(tool_data.maximal_file_size, u64::MAX);
        assert_eq!(tool_data.minimal_file_size, 0);
        assert!(tool_data.recursive_search);
        assert!(!tool_data.stopped_search);
        assert!(tool_data.use_cache);
        assert!(tool_data.delete_outdated_cache);
        assert!(!tool_data.save_also_as_json);
        assert!(!tool_data.use_reference_folders);
        assert!(!tool_data.dry_run);
    }
}
