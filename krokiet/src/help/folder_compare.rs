use super::{bullet, heading, paragraph, tab, tool_section};
use crate::{ActiveTab, HelpSection, flk};

pub(super) fn section() -> HelpSection {
    tool_section(flk!("tool_folder_compare"), ActiveTab::FolderCompare, vec![overview(), usage(), settings()])
}

fn overview() -> crate::HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_folder_compare_overview_heading")),
            paragraph(flk!("help_folder_compare_overview_what")),
            paragraph(flk!("help_folder_compare_overview_modes")),
        ],
    )
}

fn usage() -> crate::HelpTab {
    tab(
        flk!("help_tab_usage"),
        vec![
            heading(flk!("help_folder_compare_usage_heading")),
            bullet(flk!("help_folder_compare_usage_duplicates")),
            bullet(flk!("help_folder_compare_usage_compare")),
            bullet(flk!("help_folder_compare_usage_delete")),
        ],
    )
}

fn settings() -> crate::HelpTab {
    tab(
        flk!("help_tab_settings"),
        vec![
            heading(flk!("help_folder_compare_settings_heading")),
            bullet(flk!("help_folder_compare_settings_names")),
            bullet(flk!("help_folder_compare_settings_subsets")),
            bullet(flk!("help_folder_compare_settings_min_files")),
            paragraph(flk!("help_folder_compare_settings_sizes")),
            paragraph(flk!("help_folder_compare_settings_not_saved")),
        ],
    )
}
