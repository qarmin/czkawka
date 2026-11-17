use super::{bullet, heading, paragraph, tab, tool_section};
use crate::{ActiveTab, HelpSection, HelpTab, flk};

fn similar_images_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_similar_images_overview_h1")),
            paragraph(flk!("help_similar_images_overview_p2")),
            paragraph(flk!("help_similar_images_overview_p3")),
            paragraph(flk!("help_similar_images_overview_p4")),
            paragraph(flk!("help_similar_images_overview_p5")),
        ],
    )
}

fn similar_images_options() -> HelpTab {
    tab(
        flk!("help_tab_options"),
        vec![
            heading(flk!("help_similar_images_options_h1")),
            bullet(flk!("help_similar_images_options_b2")),
            bullet(flk!("help_similar_images_options_b3")),
            bullet(flk!("help_similar_images_options_b4")),
            bullet(flk!("help_similar_images_options_b5")),
            bullet(flk!("help_similar_images_options_b6")),
            bullet(flk!("help_similar_images_options_b7")),
        ],
    )
}

fn similar_images_tips() -> HelpTab {
    tab(
        flk!("help_tab_tips"),
        vec![
            heading(flk!("help_similar_images_tips_h1")),
            bullet(flk!("help_similar_images_tips_b2")),
            bullet(flk!("help_similar_images_tips_b3")),
            bullet(flk!("help_similar_images_tips_b4")),
            bullet(flk!("help_similar_images_tips_b5")),
            bullet(flk!("help_similar_images_tips_b6")),
            bullet(flk!("help_similar_images_tips_b7")),
        ],
    )
}

pub(super) fn similar_images() -> HelpSection {
    tool_section(
        flk!("tool_similar_images"),
        ActiveTab::SimilarImages,
        vec![similar_images_overview(), similar_images_options(), similar_images_tips()],
    )
}

fn similar_videos_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_similar_videos_overview_h1")),
            paragraph(flk!("help_similar_videos_overview_p2")),
            paragraph(flk!("help_similar_videos_overview_p3")),
            bullet(flk!("help_similar_videos_overview_b4")),
            bullet(flk!("help_similar_videos_overview_b5")),
            bullet(flk!("help_similar_videos_overview_b6")),
            paragraph(flk!("help_similar_videos_overview_p7")),
        ],
    )
}

fn similar_videos_options() -> HelpTab {
    tab(
        flk!("help_tab_options"),
        vec![
            heading(flk!("help_similar_videos_options_h1")),
            paragraph(flk!("help_similar_videos_options_p2")),
            bullet(flk!("help_similar_videos_options_b3")),
            bullet(flk!("help_similar_videos_options_b4")),
            bullet(flk!("help_similar_videos_options_b5")),
            bullet(flk!("help_similar_videos_options_b6")),
            bullet(flk!("help_similar_videos_options_b7")),
            bullet(flk!("help_similar_videos_options_b8")),
            bullet(flk!("help_similar_videos_options_b9")),
            bullet(flk!("help_similar_videos_options_b10")),
            heading(flk!("help_similar_videos_options_h11")),
            bullet(flk!("help_similar_videos_options_b12")),
        ],
    )
}

fn similar_videos_tips() -> HelpTab {
    tab(
        flk!("help_tab_tips"),
        vec![
            heading(flk!("help_similar_videos_tips_h1")),
            bullet(flk!("help_similar_videos_tips_b2")),
            bullet(flk!("help_similar_videos_tips_b3")),
            bullet(flk!("help_similar_videos_tips_b4")),
            bullet(flk!("help_similar_videos_tips_b5")),
        ],
    )
}

pub(super) fn similar_videos() -> HelpSection {
    tool_section(
        flk!("tool_similar_videos"),
        ActiveTab::SimilarVideos,
        vec![similar_videos_overview(), similar_videos_options(), similar_videos_tips()],
    )
}

fn music_duplicates_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_music_duplicates_overview_h1")),
            paragraph(flk!("help_music_duplicates_overview_p2")),
            bullet(flk!("help_music_duplicates_overview_b3")),
            bullet(flk!("help_music_duplicates_overview_b4")),
            paragraph(flk!("help_music_duplicates_overview_p5")),
        ],
    )
}

pub(super) fn music_duplicates() -> HelpSection {
    tool_section(flk!("tool_music_duplicates"), ActiveTab::SimilarMusic, vec![music_duplicates_overview()])
}

fn broken_files_overview() -> HelpTab {
    tab(
        flk!("help_tab_overview"),
        vec![
            heading(flk!("help_broken_files_overview_h1")),
            paragraph(flk!("help_broken_files_overview_p2")),
            paragraph(flk!("help_broken_files_overview_p3")),
            bullet(flk!("help_broken_files_overview_b4")),
            bullet(flk!("help_broken_files_overview_b5")),
            bullet(flk!("help_broken_files_overview_b6")),
            paragraph(flk!("help_broken_files_overview_p7")),
        ],
    )
}

pub(super) fn broken_files() -> HelpSection {
    tool_section(flk!("tool_broken_files"), ActiveTab::BrokenFiles, vec![broken_files_overview()])
}
