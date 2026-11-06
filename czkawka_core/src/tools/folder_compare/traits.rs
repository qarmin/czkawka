use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::tools::folder_compare::{DiffNode, DiffStatus, FolderCompare, FolderCompareMode, FolderCompareParameters, FolderGroup, GroupRelation, Info, MatchRole};

impl AllTraits for FolderCompare {}

impl DeletingItems for FolderCompare {
    fn delete_files(&mut self, _stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        WorkContinueStatus::Continue
    }
}

impl DebugPrint for FolderCompare {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }

        println!("### INDIVIDUAL DEBUG PRINT ###");
        println!("Mode: {:?}", self.mode);
        println!("Info: {:?}", self.information);
        println!("Params: {:?}", self.params);
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl Search for FolderCompare {
    #[fun_time(message = "find_folder_compare", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            let Ok((left, right)) = self.prepare_sides() else { return };
            if self.compare_folders(&left, &right, stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
            }
        })();

        self.information.scanning_time = start_time.elapsed();

        if !self.common_data.stopped_search {
            self.debug_print();
        }
    }
}

impl CommonData for FolderCompare {
    type Info = Info;
    type Parameters = FolderCompareParameters;

    fn get_information(&self) -> Self::Info {
        self.information
    }
    fn get_params(&self) -> Self::Parameters {
        self.params
    }
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn found_any_items(&self) -> bool {
        self.information.number_of_groups > 0 || self.information.number_of_different_entries > 0
    }
}

impl PrintResults for FolderCompare {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.write_base_search_paths(writer)?;
        if !self.found_any_items() {
            return match self.mode {
                FolderCompareMode::Duplicates => writeln!(writer, "Not found any duplicated folders."),
                FolderCompareMode::Diff => writeln!(writer, "The compared folders are identical."),
            };
        }
        match self.mode {
            FolderCompareMode::Duplicates => self.groups.iter().try_for_each(|group| write_group(writer, group)),
            FolderCompareMode::Diff => write_diff(writer, &self.diff.entries, 0),
        }
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        match self.mode {
            FolderCompareMode::Duplicates => self.save_results_to_file_as_json_internal(file_name, &self.groups, pretty_print),
            FolderCompareMode::Diff => self.save_results_to_file_as_json_internal(file_name, &self.diff, pretty_print),
        }
    }
}

fn write_group<T: Write>(writer: &mut T, group: &FolderGroup) -> std::io::Result<()> {
    let title = match group.relation {
        GroupRelation::Identical => "Identical folders",
        GroupRelation::Subset => "Folders contained in bigger ones",
    };
    writeln!(writer, "\n-------- {title}, {} could be freed --------", format_size(group.wasted_size(), BINARY))?;
    for folder in &group.folders {
        let role = match folder.role {
            MatchRole::Identical => String::new(),
            MatchRole::Contained => " (contained)".to_string(),
            MatchRole::Container => format!(" (contains it, plus {} extra files)", folder.extra_files),
        };
        writeln!(
            writer,
            "{} - {} files, {}{role}",
            folder.path.to_string_lossy(),
            folder.file_count,
            format_size(folder.size, BINARY)
        )?;
    }
    Ok(())
}

const fn status_marker(status: DiffStatus) -> &'static str {
    match status {
        DiffStatus::Identical => "=",
        DiffStatus::Different => "~",
        DiffStatus::OnlyLeft => "<",
        DiffStatus::OnlyRight => ">",
        DiffStatus::LeftHasMore => "<+",
        DiffStatus::RightHasMore => "+>",
    }
}

fn write_diff<T: Write>(writer: &mut T, nodes: &[DiffNode], depth: usize) -> std::io::Result<()> {
    for node in nodes {
        let suffix = if node.kind == crate::tools::folder_compare::EntryKind::Directory { "/" } else { "" };
        writeln!(writer, "{:indent$}{:<2} {}{suffix}", "", status_marker(node.status), node.name, indent = depth * 2)?;
        write_diff(writer, &node.children, depth + 1)?;
    }
    Ok(())
}
