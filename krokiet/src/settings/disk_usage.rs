use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Model};

use crate::{DiskUsageMode, DiskUsageState, MainWindow};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiskUsageSettings {
    #[serde(default)]
    pub sunburst: bool,
    #[serde(default = "default_depth")]
    pub depth: u32,
    #[serde(default = "default_content_detection_index")]
    pub content_detection_index: i32,
    #[serde(default = "ttrue")]
    pub ignore_excluded_items: bool,
    #[serde(default)]
    pub ignore_excluded_directories: bool,
    #[serde(default)]
    pub nested_borders: bool,
    #[serde(default)]
    pub folder_headers: bool,
    #[serde(default = "default_group_exponent")]
    pub group_exponent: u32,
    #[serde(default = "default_min_item_px")]
    pub min_item_px: u32,
    #[serde(default)]
    pub folder_block_px: u32,
    /// Indices of the file types hidden in the chart.
    #[serde(default)]
    pub disabled_categories: Vec<usize>,
}

impl Default for DiskUsageSettings {
    fn default() -> Self {
        serde_json::from_str("{}").expect("Cannot fail creating {} from string")
    }
}

const fn ttrue() -> bool {
    true
}

const fn default_depth() -> u32 {
    6
}

const fn default_content_detection_index() -> i32 {
    1
}

const fn default_group_exponent() -> u32 {
    6
}

const fn default_min_item_px() -> u32 {
    1
}

pub(crate) fn set_disk_usage_settings_to_gui(app: &MainWindow, settings: &DiskUsageSettings) {
    crate::disk_usage::ensure_categories(app);
    let state = app.global::<DiskUsageState>();
    state.set_mode(if settings.sunburst { DiskUsageMode::Sunburst } else { DiskUsageMode::Treemap });
    state.set_depth(settings.depth as f32);
    state.set_content_detection_index(settings.content_detection_index);
    state.set_ignore_excluded_items(settings.ignore_excluded_items);
    state.set_ignore_excluded_directories(settings.ignore_excluded_directories);
    state.set_nested_borders(settings.nested_borders);
    state.set_folder_headers(settings.folder_headers);
    state.set_group_exponent(settings.group_exponent as f32);
    state.set_min_item_px(settings.min_item_px as f32);
    state.set_folder_block_px(settings.folder_block_px as f32);

    let categories = state.get_categories();
    for index in 0..categories.row_count() {
        if let Some(mut category) = categories.row_data(index) {
            category.enabled = !settings.disabled_categories.contains(&index);
            categories.set_row_data(index, category);
        }
    }
    state.invoke_options_changed();
}

pub(crate) fn collect_disk_usage_settings(app: &MainWindow) -> DiskUsageSettings {
    let state = app.global::<DiskUsageState>();
    DiskUsageSettings {
        sunburst: state.get_mode() == DiskUsageMode::Sunburst,
        depth: state.get_depth().round().max(0.0) as u32,
        content_detection_index: state.get_content_detection_index(),
        ignore_excluded_items: state.get_ignore_excluded_items(),
        ignore_excluded_directories: state.get_ignore_excluded_directories(),
        nested_borders: state.get_nested_borders(),
        folder_headers: state.get_folder_headers(),
        group_exponent: state.get_group_exponent().round().max(0.0) as u32,
        min_item_px: state.get_min_item_px().round().max(0.0) as u32,
        folder_block_px: state.get_folder_block_px().round().max(0.0) as u32,
        disabled_categories: state
            .get_categories()
            .iter()
            .enumerate()
            .filter(|(_, category)| !category.enabled)
            .map(|(index, _)| index)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::DiskUsageSettings;

    #[test]
    fn missing_section_gives_the_defaults_of_the_ui() {
        let settings = DiskUsageSettings::default();

        assert!(!settings.sunburst);
        assert_eq!(settings.depth, 6);
        assert_eq!(settings.content_detection_index, 1);
        assert!(settings.ignore_excluded_items);
        assert!(!settings.folder_headers);
        assert!(settings.disabled_categories.is_empty());
    }

    #[test]
    fn settings_survive_a_round_trip_through_json() {
        let settings = DiskUsageSettings {
            sunburst: true,
            folder_headers: true,
            disabled_categories: vec![2, 5],
            ..DiskUsageSettings::default()
        };

        let json = serde_json::to_string(&settings).unwrap();

        assert_eq!(serde_json::from_str::<DiskUsageSettings>(&json).unwrap(), settings);
    }
}
