use czkawka_core::tools::disk_usage::category::FileCategory;
use czkawka_core::tools::disk_usage::tree::{DiskNode, DiskTree};
use humansize::{BINARY, format_size};
use slint::ComponentHandle;

use crate::{DiskUsageState, MainWindow, flk};

/// What the tooltip, the line under the chart and the context menu say about one item of the chart.
pub(super) struct ItemInfo {
    pub title: String,
    /// What it is ("File - Audio", "Folder"); empty for a block of small entries.
    pub kind: String,
    pub size: String,
    pub share: String,
    pub path: String,
    pub color: slint::Color,
    pub is_directory: bool,
}

impl ItemInfo {
    /// Kind, size and share in one line, for places that have room for a single one.
    pub(super) fn meta(&self) -> String {
        if self.kind.is_empty() {
            format!("{} ({})", self.size, self.share)
        } else {
            format!("{} - {} ({})", self.kind, self.size, self.share)
        }
    }
}

pub(super) fn node_info(tree: &DiskTree, address: &[u32], size: u64, total: u64) -> Option<ItemInfo> {
    let node = tree.node(address)?;
    let path = tree.path(address)?;
    let kind = if node.is_directory() {
        flk!("disk_usage_kind_folder")
    } else {
        format!("{} - {}", flk!("disk_usage_kind_file"), category_name(node.category))
    };
    Some(ItemInfo {
        title: display_name(node, &path),
        kind,
        size: format_size(size, BINARY),
        share: share(size, total),
        path: path.to_string_lossy().into_owned(),
        color: category_color(node.category),
        is_directory: node.is_directory(),
    })
}

/// The block that stands for entries too small to be drawn one by one; `parent_address` is the directory holding them.
pub(super) fn omitted_info(tree: &DiskTree, parent_address: &[u32], count: u32, size: u64, total: u64) -> Option<ItemInfo> {
    let path = tree.path(parent_address)?;
    Some(ItemInfo {
        title: flk!("disk_usage_small_items", count = count),
        kind: String::new(),
        size: format_size(size, BINARY),
        share: share(size, total),
        path: path.to_string_lossy().into_owned(),
        color: category_color(FileCategory::Other),
        is_directory: false,
    })
}

/// Name of the directory a chart zoomed into `zoom` shows; of the scan itself when `zoom` is empty.
pub(super) fn root_label(tree: &DiskTree, zoom: &[u32]) -> String {
    let address = if zoom.is_empty() && tree.roots.len() == 1 { &[0][..] } else { zoom };
    match tree.node(address).zip(tree.path(address)) {
        Some((node, path)) => display_name(node, &path),
        None => flk!("disk_usage_whole_scan"),
    }
}

pub(super) fn format_total(size: u64) -> String {
    format_size(size, BINARY)
}

fn display_name(node: &DiskNode, path: &std::path::Path) -> String {
    path.file_name().map_or_else(|| node.name.to_string(), |name| name.to_string_lossy().into_owned())
}

fn share(size: u64, total: u64) -> String {
    let percent = if total == 0 { 0.0 } else { size as f64 * 100.0 / total as f64 };
    if percent > 0.0 && percent < 0.1 {
        "<0.1%".to_string()
    } else {
        format!("{percent:.1}%")
    }
}

fn category_color(category: FileCategory) -> slint::Color {
    let [red, green, blue] = category.color();
    slint::Color::from_rgb_u8(red, green, blue)
}

pub(super) fn category_name(category: FileCategory) -> String {
    match category {
        FileCategory::Folder => flk!("disk_usage_category_folder"),
        FileCategory::Audio => flk!("disk_usage_category_audio"),
        FileCategory::Video => flk!("disk_usage_category_video"),
        FileCategory::Image => flk!("disk_usage_category_image"),
        FileCategory::Archive => flk!("disk_usage_category_archive"),
        FileCategory::Document => flk!("disk_usage_category_document"),
        FileCategory::Code => flk!("disk_usage_category_code"),
        FileCategory::DiskImage => flk!("disk_usage_category_disk_image"),
        FileCategory::Binary => flk!("disk_usage_category_binary"),
        FileCategory::Other => flk!("disk_usage_category_other"),
    }
}

pub(super) fn show_hover(app: &MainWindow, info: Option<&ItemInfo>) {
    let state = app.global::<DiskUsageState>();
    state.set_hover_title(info.map_or(String::new(), |info| info.title.clone()).into());
    state.set_hover_kind(info.map_or(String::new(), |info| info.kind.clone()).into());
    state.set_hover_size(info.map_or(String::new(), |info| info.size.clone()).into());
    state.set_hover_share(info.map_or(String::new(), |info| info.share.clone()).into());
    state.set_hover_path(info.map_or(String::new(), |info| info.path.clone()).into());
    if let Some(info) = info {
        state.set_hover_color(info.color);
    }
}

pub(super) fn show_selection(app: &MainWindow, info: Option<&ItemInfo>) {
    let state = app.global::<DiskUsageState>();
    state.set_has_selection(info.is_some());
    state.set_selected_is_directory(info.is_some_and(|info| info.is_directory));
    state.set_selected_title(info.map_or(String::new(), |info| info.title.clone()).into());
    state.set_selected_meta(info.map_or(String::new(), ItemInfo::meta).into());
    state.set_selected_text(info.map_or(String::new(), |info| format!("{}  -  {}", info.path, info.meta())).into());
    if let Some(info) = info {
        state.set_selected_color(info.color);
    }
}

#[cfg(test)]
mod tests {
    use super::share;

    #[test]
    fn share_is_shown_with_one_decimal() {
        assert_eq!(share(1, 8), "12.5%");
        assert_eq!(share(0, 100), "0.0%");
        assert_eq!(share(5, 0), "0.0%");
    }

    #[test]
    fn share_of_something_tiny_is_not_shown_as_zero() {
        assert_eq!(share(1, 10_000), "<0.1%");
        assert_eq!(share(1, 1000), "0.1%");
    }
}
