use super::{bullet, general_section, heading, paragraph, tab};
use crate::{HelpSection, HelpTab, flk};

fn getting_started_quick_start() -> HelpTab {
    tab(
        flk!("help_tab_quick_start"),
        vec![
            heading(flk!("help_getting_started_quick_start_h1")),
            bullet(flk!("help_getting_started_quick_start_b2")),
            bullet(flk!("help_getting_started_quick_start_b3")),
            bullet(flk!("help_getting_started_quick_start_b4")),
            bullet(flk!("help_getting_started_quick_start_b5")),
            paragraph(flk!("help_getting_started_quick_start_p6")),
            paragraph(flk!("help_getting_started_quick_start_p7")),
            paragraph(flk!("help_getting_started_quick_start_p8")),
        ],
    )
}

fn getting_started_interface() -> HelpTab {
    tab(
        flk!("help_tab_interface"),
        vec![
            heading(flk!("help_getting_started_interface_h1")),
            bullet(flk!("help_getting_started_interface_b2")),
            bullet(flk!("help_getting_started_interface_b3")),
            bullet(flk!("help_getting_started_interface_b4")),
            bullet(flk!("help_getting_started_interface_b5")),
            bullet(flk!("help_getting_started_interface_b6")),
            bullet(flk!("help_getting_started_interface_b7")),
            heading(flk!("help_getting_started_interface_h8")),
            paragraph(flk!("help_getting_started_interface_p9")),
            paragraph(flk!("help_getting_started_interface_p10")),
            paragraph(flk!("help_getting_started_interface_p11")),
        ],
    )
}

fn getting_started_results() -> HelpTab {
    tab(
        flk!("help_tab_results"),
        vec![
            heading(flk!("help_getting_started_results_h1")),
            paragraph(flk!("help_getting_started_results_p2")),
            bullet(flk!("help_getting_started_results_b3")),
            bullet(flk!("help_getting_started_results_b4")),
            bullet(flk!("help_getting_started_results_b5")),
            heading(flk!("help_getting_started_results_h6")),
            paragraph(flk!("help_getting_started_results_p7")),
            bullet(flk!("help_getting_started_results_b8")),
            bullet(flk!("help_getting_started_results_b9")),
            bullet(flk!("help_getting_started_results_b10")),
            bullet(flk!("help_getting_started_results_b11")),
            bullet(flk!("help_getting_started_results_b12")),
            paragraph(flk!("help_getting_started_results_p13")),
            heading(flk!("help_getting_started_results_h14")),
            bullet(flk!("help_getting_started_results_b15")),
            bullet(flk!("help_getting_started_results_b16")),
            bullet(flk!("help_getting_started_results_b17")),
        ],
    )
}

pub(super) fn getting_started() -> HelpSection {
    general_section(
        flk!("help_section_getting_started"),
        vec![getting_started_quick_start(), getting_started_interface(), getting_started_results()],
    )
}

fn selection_selecting() -> HelpTab {
    tab(
        flk!("help_tab_selecting"),
        vec![
            heading(flk!("help_selection_selecting_h1")),
            paragraph(flk!("help_selection_selecting_p2")),
            bullet(flk!("help_selection_selecting_b3")),
            bullet(flk!("help_selection_selecting_b4")),
            paragraph(flk!("help_selection_selecting_p5")),
            heading(flk!("help_selection_selecting_h6")),
            paragraph(flk!("help_selection_selecting_p7")),
            bullet(flk!("help_selection_selecting_b8")),
            bullet(flk!("help_selection_selecting_b9")),
            bullet(flk!("help_selection_selecting_b10")),
            bullet(flk!("help_selection_selecting_b11")),
            bullet(flk!("help_selection_selecting_b12")),
            bullet(flk!("help_selection_selecting_b13")),
            paragraph(flk!("help_selection_selecting_p14")),
        ],
    )
}

fn selection_actions() -> HelpTab {
    tab(
        flk!("help_tab_actions"),
        vec![
            heading(flk!("help_selection_actions_h1")),
            bullet(flk!("help_selection_actions_b2")),
            bullet(flk!("help_selection_actions_b3")),
            bullet(flk!("help_selection_actions_b4")),
            bullet(flk!("help_selection_actions_b5")),
            bullet(flk!("help_selection_actions_b6")),
            bullet(flk!("help_selection_actions_b7")),
            bullet(flk!("help_selection_actions_b8")),
            paragraph(flk!("help_selection_actions_p9")),
        ],
    )
}

fn selection_reference() -> HelpTab {
    tab(
        flk!("help_tab_reference"),
        vec![
            heading(flk!("help_selection_reference_h1")),
            paragraph(flk!("help_selection_reference_p2")),
            paragraph(flk!("help_selection_reference_p3")),
            paragraph(flk!("help_selection_reference_p4")),
            paragraph(flk!("help_selection_reference_p5")),
        ],
    )
}

pub(super) fn selection() -> HelpSection {
    general_section(flk!("help_section_selection"), vec![selection_selecting(), selection_actions(), selection_reference()])
}

fn settings_filters() -> HelpTab {
    tab(
        flk!("help_tab_filters"),
        vec![
            heading(flk!("help_settings_filters_h1")),
            paragraph(flk!("help_settings_filters_p2")),
            bullet(flk!("help_settings_filters_b3")),
            bullet(flk!("help_settings_filters_b4")),
            bullet(flk!("help_settings_filters_b5")),
            bullet(flk!("help_settings_filters_b6")),
            bullet(flk!("help_settings_filters_b7")),
            bullet(flk!("help_settings_filters_b8")),
            heading(flk!("help_settings_filters_h9")),
            paragraph(flk!("help_settings_filters_p10")),
        ],
    )
}

fn settings_general() -> HelpTab {
    tab(
        flk!("help_tab_general"),
        vec![
            heading(flk!("help_settings_general_h1")),
            bullet(flk!("help_settings_general_b2")),
            bullet(flk!("help_settings_general_b3")),
            bullet(flk!("help_settings_general_b4")),
            bullet(flk!("help_settings_general_b5")),
            bullet(flk!("help_settings_general_b6")),
            bullet(flk!("help_settings_general_b7")),
            bullet(flk!("help_settings_general_b8")),
            bullet(flk!("help_settings_general_b9")),
            bullet(flk!("help_settings_general_b10")),
            bullet(flk!("help_settings_general_b11")),
            paragraph(flk!("help_settings_general_p12")),
        ],
    )
}

fn settings_presets() -> HelpTab {
    tab(
        flk!("help_tab_presets"),
        vec![
            heading(flk!("help_settings_presets_h1")),
            paragraph(flk!("help_settings_presets_p2")),
            paragraph(flk!("help_settings_presets_p3")),
            paragraph(flk!("help_settings_presets_p4")),
        ],
    )
}

fn settings_files() -> HelpTab {
    tab(
        flk!("help_tab_files"),
        vec![
            heading(flk!("help_settings_files_h1")),
            bullet(flk!("help_settings_files_b2")),
            bullet(flk!("help_settings_files_b3")),
            bullet(flk!("help_settings_files_b4")),
            paragraph(flk!("help_settings_files_p5")),
            paragraph(flk!("help_settings_files_p6")),
            paragraph(flk!("help_settings_files_p7")),
            heading(flk!("help_settings_files_h8")),
            paragraph(flk!("help_settings_files_p9")),
            paragraph(flk!("help_settings_files_p10")),
        ],
    )
}

pub(super) fn settings() -> HelpSection {
    general_section(
        flk!("help_section_settings"),
        vec![settings_filters(), settings_general(), settings_presets(), settings_files()],
    )
}

fn shortcuts_shortcuts() -> HelpTab {
    tab(
        flk!("help_tab_shortcuts"),
        vec![
            heading(flk!("help_shortcuts_shortcuts_h1")),
            paragraph(flk!("help_shortcuts_shortcuts_p2")),
            bullet(flk!("help_shortcuts_shortcuts_b3")),
            bullet(flk!("help_shortcuts_shortcuts_b4")),
            bullet(flk!("help_shortcuts_shortcuts_b5")),
            bullet(flk!("help_shortcuts_shortcuts_b6")),
            bullet(flk!("help_shortcuts_shortcuts_b7")),
            bullet(flk!("help_shortcuts_shortcuts_b8")),
            heading(flk!("help_shortcuts_shortcuts_h9")),
            bullet(flk!("help_shortcuts_shortcuts_b10")),
            bullet(flk!("help_shortcuts_shortcuts_b11")),
            bullet(flk!("help_shortcuts_shortcuts_b12")),
            bullet(flk!("help_shortcuts_shortcuts_b13")),
        ],
    )
}

pub(super) fn shortcuts() -> HelpSection {
    general_section(flk!("help_section_shortcuts"), vec![shortcuts_shortcuts()])
}
