use super::{bullet, heading, paragraph, tab, tool_section};
use crate::{ActiveTab, HelpSection, HelpTab, flk};

fn duplicate_files_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_duplicate_files_overview_h1")),
            paragraph(flk!("help_duplicate_files_overview_p2")),
            paragraph(flk!("help_duplicate_files_overview_p3")),
            paragraph(flk!("help_duplicate_files_overview_p4")),
        ],
    )
}

fn duplicate_files_options() -> HelpTab {
    tab(
        flk!("help_tab_options"),
        vec![
            heading(flk!("help_duplicate_files_options_h1")),
            bullet(flk!("help_duplicate_files_options_b2")),
            bullet(flk!("help_duplicate_files_options_b3")),
            bullet(flk!("help_duplicate_files_options_b4")),
            bullet(flk!("help_duplicate_files_options_b5")),
            bullet(flk!("help_duplicate_files_options_b6")),
        ],
    )
}

fn duplicate_files_tips() -> HelpTab {
    tab(
        flk!("help_tab_tips"),
        vec![
            heading(flk!("help_duplicate_files_tips_h1")),
            bullet(flk!("help_duplicate_files_tips_b2")),
            bullet(flk!("help_duplicate_files_tips_b3")),
            bullet(flk!("help_duplicate_files_tips_b4")),
            bullet(flk!("help_duplicate_files_tips_b5")),
            bullet(flk!("help_duplicate_files_tips_b6")),
            bullet(flk!("help_duplicate_files_tips_b7")),
        ],
    )
}

pub(super) fn duplicate_files() -> HelpSection {
    tool_section(
        flk!("tool_duplicate_files"),
        ActiveTab::DuplicateFiles,
        vec![duplicate_files_overview(), duplicate_files_options(), duplicate_files_tips()],
    )
}

fn empty_folders_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_empty_folders_overview_h1")),
            paragraph(flk!("help_empty_folders_overview_p2")),
            paragraph(flk!("help_empty_folders_overview_p3")),
            paragraph(flk!("help_empty_folders_overview_p4")),
        ],
    )
}

pub(super) fn empty_folders() -> HelpSection {
    tool_section(flk!("tool_empty_folders"), ActiveTab::EmptyFolders, vec![empty_folders_overview()])
}

fn big_files_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_big_files_overview_h1")),
            paragraph(flk!("help_big_files_overview_p2")),
            bullet(flk!("help_big_files_overview_b3")),
            bullet(flk!("help_big_files_overview_b4")),
            paragraph(flk!("help_big_files_overview_p5")),
            paragraph(flk!("help_big_files_overview_p6")),
        ],
    )
}

pub(super) fn big_files() -> HelpSection {
    tool_section(flk!("tool_big_files"), ActiveTab::BigFiles, vec![big_files_overview()])
}

fn empty_files_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_empty_files_overview_h1")),
            paragraph(flk!("help_empty_files_overview_p2")),
            bullet(flk!("help_empty_files_overview_b3")),
            bullet(flk!("help_empty_files_overview_b4")),
            paragraph(flk!("help_empty_files_overview_p5")),
            paragraph(flk!("help_empty_files_overview_p6")),
        ],
    )
}

pub(super) fn empty_files() -> HelpSection {
    tool_section(flk!("tool_empty_files"), ActiveTab::EmptyFiles, vec![empty_files_overview()])
}

fn temporary_files_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_temporary_files_overview_h1")),
            paragraph(flk!("help_temporary_files_overview_p2")),
            paragraph(flk!("help_temporary_files_overview_p3")),
            paragraph(flk!("help_temporary_files_overview_p4")),
        ],
    )
}

pub(super) fn temporary_files() -> HelpSection {
    tool_section(flk!("tool_temporary_files"), ActiveTab::TemporaryFiles, vec![temporary_files_overview()])
}
