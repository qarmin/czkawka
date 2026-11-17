use super::{bullet, heading, paragraph, tab, tool_section};
use crate::{ActiveTab, HelpSection, HelpTab, flk};

fn invalid_symlinks_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_invalid_symlinks_overview_h1")),
            paragraph(flk!("help_invalid_symlinks_overview_p2")),
            paragraph(flk!("help_invalid_symlinks_overview_p3")),
        ],
    )
}

pub(super) fn invalid_symlinks() -> HelpSection {
    tool_section(flk!("tool_invalid_symlinks"), ActiveTab::InvalidSymlinks, vec![invalid_symlinks_overview()])
}

fn bad_extensions_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_bad_extensions_overview_h1")),
            paragraph(flk!("help_bad_extensions_overview_p2")),
            paragraph(flk!("help_bad_extensions_overview_p3")),
            paragraph(flk!("help_bad_extensions_overview_p4")),
        ],
    )
}

pub(super) fn bad_extensions() -> HelpSection {
    tool_section(flk!("tool_bad_extensions"), ActiveTab::BadExtensions, vec![bad_extensions_overview()])
}

fn bad_names_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_bad_names_overview_h1")),
            paragraph(flk!("help_bad_names_overview_p2")),
            bullet(flk!("help_bad_names_overview_b3")),
            bullet(flk!("help_bad_names_overview_b4")),
            bullet(flk!("help_bad_names_overview_b5")),
            bullet(flk!("help_bad_names_overview_b6")),
            bullet(flk!("help_bad_names_overview_b7")),
            bullet(flk!("help_bad_names_overview_b8")),
        ],
    )
}

pub(super) fn bad_names() -> HelpSection {
    tool_section(flk!("tool_bad_names"), ActiveTab::BadNames, vec![bad_names_overview()])
}

fn exif_remover_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_exif_remover_overview_h1")),
            paragraph(flk!("help_exif_remover_overview_p2")),
            paragraph(flk!("help_exif_remover_overview_p3")),
            paragraph(flk!("help_exif_remover_overview_p4")),
            paragraph(flk!("help_exif_remover_overview_p5")),
        ],
    )
}

pub(super) fn exif_remover() -> HelpSection {
    tool_section(flk!("tool_exif_remover"), ActiveTab::ExifRemover, vec![exif_remover_overview()])
}

fn video_optimizer_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_video_optimizer_overview_h1")),
            paragraph(flk!("help_video_optimizer_overview_p2")),
            bullet(flk!("help_video_optimizer_overview_b3")),
            bullet(flk!("help_video_optimizer_overview_b4")),
            paragraph(flk!("help_video_optimizer_overview_p5")),
        ],
    )
}

pub(super) fn video_optimizer() -> HelpSection {
    tool_section(flk!("tool_video_optimizer"), ActiveTab::VideoOptimizer, vec![video_optimizer_overview()])
}
