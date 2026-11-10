use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::model::{CheckingMethod, ToolType, WorkContinueStatus};
use crate::common::progress_data::{ProgressData, ToolStage};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::CommonToolData;
use crate::tools::disk_usage::scan::portable::PortableSource;
use crate::tools::disk_usage::scan::{ScanContext, TreeSource};
use crate::tools::disk_usage::tree::{DiskNode, DiskTree, NodeKind};
use crate::tools::disk_usage::{DiskUsage, DiskUsageParameters, Info};

impl DiskUsage {
    pub fn new(params: DiskUsageParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::DiskUsage),
            information: Info::default(),
            tree: DiskTree::default(),
            params,
        }
    }

    #[fun_time(message = "scan_disk_usage", level = "debug")]
    pub(crate) fn scan_disk_usage(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        let directories = &self.common_data.directories;
        let roots = directories.included_directories.iter().chain(&directories.included_files).cloned().collect::<Vec<_>>();

        let progress_handler = prepare_thread_handler_common(progress_sender, ToolStage::CollectingFiles(CheckingMethod::None), 0, 0);
        let context = ScanContext {
            directories,
            stop_flag,
            entries_counter: progress_handler.items_counter(),
            content_detection: self.params.content_detection,
            excluded_items: &self.common_data.excluded_items,
            ignore_excluded_items: self.params.ignore_excluded_items,
            ignore_excluded_directories: self.params.ignore_excluded_directories,
        };
        let tree = PortableSource.scan(&roots, &context);
        progress_handler.join_thread();

        self.information = count_entries(&tree);
        self.tree = tree;

        if check_if_stop_received(stop_flag) {
            WorkContinueStatus::Stop
        } else {
            WorkContinueStatus::Continue
        }
    }
}

fn count_entries(tree: &DiskTree) -> Info {
    let mut info = Info {
        total_size: tree.total_size(),
        ..Info::default()
    };
    tree.roots.iter().for_each(|root| count_node(root, &mut info));
    info
}

fn count_node(node: &DiskNode, info: &mut Info) {
    match node.kind {
        NodeKind::File => info.number_of_files += 1,
        NodeKind::Directory => info.number_of_directories += 1,
        NodeKind::Inaccessible => info.number_of_inaccessible_entries += 1,
        NodeKind::Symlink => {}
    }
    node.children.iter().for_each(|child| count_node(child, info));
}
