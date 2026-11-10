use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::disk_usage::tree::DiskTree;
use czkawka_core::tools::disk_usage::{ContentDetection, DiskUsage, DiskUsageParameters, Info};

use crate::scan_runner::{CommonFilters, ScanResultHandler, apply_filters, spawn_progress_forwarder};

pub(crate) fn scan_disk_usage<H: ScanResultHandler>(
    dirs: Vec<PathBuf>,
    accurate_categories: bool,
    filters: &CommonFilters,
    stop: &Arc<AtomicBool>,
    handler: &Arc<H>,
    scan_id: u32,
) -> (DiskTree, Info) {
    let (ptx, fwd) = spawn_progress_forwarder(Arc::clone(handler), scan_id);
    let content_detection = if accurate_categories {
        ContentDetection::LargeFiles
    } else {
        ContentDetection::ExtensionOnly
    };
    let params = DiskUsageParameters {
        ignore_excluded_items: false,
        ..DiskUsageParameters::new(content_detection)
    };
    let mut tool = DiskUsage::new(params);
    tool.set_included_paths(dirs);
    apply_filters(&mut tool, filters);
    tool.search(stop, Some(&ptx));
    drop(ptx);
    fwd.join().expect("Failed to join progress forwarder thread");

    let info = tool.get_information();
    (std::mem::take(tool.get_tree_mut()), info)
}
