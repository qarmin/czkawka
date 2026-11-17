use czkawka_core::tools::disk_usage::layout::{Layout, Shape};
use czkawka_core::tools::disk_usage::tree::DiskTree;
use slint::{ModelRc, SharedString, VecModel};

use crate::DiskUsageHeaderLabel;
use crate::disk_usage::describe::{format_total, node_name};
use crate::disk_usage::view::absolute_address;

const LABEL_PADDING_PX: f32 = 4.0;
/// Enough for any chart that is readable; keeps the Slint scene small when thousands of tiny folders are open.
const MAX_LABELS: usize = 1500;

pub(super) struct HeaderLabel {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
}

/// Name and size of every folder that has a header bar, placed over the bar.
pub(super) fn collect(tree: &DiskTree, layout: &Layout, zoom: &[u32], (chart_width, chart_height): (u32, u32)) -> Vec<HeaderLabel> {
    layout
        .items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.header_height > 0.0)
        .filter_map(|(index, item)| {
            let Shape::Rect { x, y, width, height: _ } = item.shape else {
                return None;
            };
            let relative = layout.address(index)?;
            let node = tree.node(&absolute_address(zoom, &relative))?;
            let (left, top) = ((x * chart_width as f32).round(), (y * chart_height as f32).round());
            let label_width = width * chart_width as f32 - 2.0 * LABEL_PADDING_PX;
            (label_width > 0.0).then(|| HeaderLabel {
                x: left + LABEL_PADDING_PX,
                y: top,
                width: label_width,
                height: (item.header_height * chart_height as f32).round(),
                text: format!("{}  {}", node_name(node), format_total(item.size)),
            })
        })
        .take(MAX_LABELS)
        .collect()
}

pub(super) fn to_model(labels: &[HeaderLabel]) -> ModelRc<DiskUsageHeaderLabel> {
    let labels = labels
        .iter()
        .map(|label| DiskUsageHeaderLabel {
            x: label.x,
            y: label.y,
            width: label.width,
            height: label.height,
            text: SharedString::from(label.text.as_str()),
        })
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(labels))
}
