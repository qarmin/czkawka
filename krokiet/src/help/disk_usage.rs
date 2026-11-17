use super::{bullet, heading, paragraph, tab, tool_section};
use crate::{ActiveTab, HelpSection, flk};

pub(super) fn section() -> HelpSection {
    tool_section(flk!("tool_disk_usage"), ActiveTab::DiskUsage, vec![overview(), usage(), settings(), limitations()])
}

fn overview() -> crate::HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_disk_usage_overview_heading")),
            paragraph(flk!("help_disk_usage_overview_what")),
            paragraph(flk!("help_disk_usage_overview_views")),
            paragraph(flk!("help_disk_usage_overview_colors")),
        ],
    )
}

fn usage() -> crate::HelpTab {
    tab(
        flk!("help_tab_usage"),
        vec![
            heading(flk!("help_disk_usage_usage_heading")),
            paragraph(flk!("help_disk_usage_usage_start")),
            bullet(flk!("help_disk_usage_usage_hover")),
            bullet(flk!("help_disk_usage_usage_select")),
            bullet(flk!("help_disk_usage_usage_open")),
            bullet(flk!("help_disk_usage_usage_scroll")),
            bullet(flk!("help_disk_usage_usage_rings")),
            bullet(flk!("help_disk_usage_usage_menu")),
            bullet(flk!("help_disk_usage_usage_filters")),
        ],
    )
}

fn settings() -> crate::HelpTab {
    tab(
        flk!("help_tab_settings"),
        vec![
            heading(flk!("help_disk_usage_settings_heading")),
            paragraph(flk!("help_disk_usage_settings_where")),
            bullet(flk!("help_disk_usage_settings_excluded_items")),
            bullet(flk!("help_disk_usage_settings_excluded_folders")),
            bullet(flk!("help_disk_usage_settings_detection")),
            bullet(flk!("help_disk_usage_settings_detail")),
            paragraph(flk!("help_disk_usage_settings_not_saved")),
        ],
    )
}

fn limitations() -> crate::HelpTab {
    tab(
        flk!("help_tab_limitations"),
        vec![
            heading(flk!("help_disk_usage_limits_heading")),
            bullet(flk!("help_disk_usage_limits_symlinks")),
            bullet(flk!("help_disk_usage_limits_small")),
            bullet(flk!("help_disk_usage_limits_total")),
            bullet(flk!("help_disk_usage_limits_changes")),
        ],
    )
}
