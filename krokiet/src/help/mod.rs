mod disk_usage;
mod faq;
mod general;
mod tools_files;
mod tools_media;
mod tools_other;

use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::{ActiveTab, Callabler, GuiState, HelpBlock, HelpBlockKind, HelpSection, HelpState, HelpTab, MainWindow};

pub(crate) fn connect_help(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<Callabler>().on_prepare_help(move || {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        select_section_of_active_tool(&app);
    });
}

pub(crate) fn set_help_content(app: &MainWindow) {
    let sections = vec![
        general::getting_started(),
        general::selection(),
        general::settings(),
        tools_files::duplicate_files(),
        tools_files::empty_folders(),
        tools_files::big_files(),
        tools_files::empty_files(),
        tools_files::temporary_files(),
        tools_media::similar_images(),
        tools_media::similar_videos(),
        tools_media::music_duplicates(),
        tools_media::broken_files(),
        tools_other::invalid_symlinks(),
        tools_other::bad_extensions(),
        tools_other::bad_names(),
        tools_other::exif_remover(),
        tools_other::video_optimizer(),
        disk_usage::section(),
        general::shortcuts(),
        faq::faq(),
    ];
    app.global::<HelpState>().set_sections(ModelRc::new(VecModel::from(sections)));
}

fn select_section_of_active_tool(app: &MainWindow) {
    let active_tab = app.global::<GuiState>().get_active_tab();
    let state = app.global::<HelpState>();
    let section_idx = state.get_sections().iter().position(|section| section.is_tool && section.tool == active_tab);
    state.set_selected_section(section_idx.map_or(0, |idx| idx as i32));
    state.set_selected_tab(0);
}

fn block(kind: HelpBlockKind, text: String) -> HelpBlock {
    HelpBlock { kind, text: text.into() }
}

fn heading(text: String) -> HelpBlock {
    block(HelpBlockKind::Heading, text)
}

fn paragraph(text: String) -> HelpBlock {
    block(HelpBlockKind::Paragraph, text)
}

fn bullet(text: String) -> HelpBlock {
    block(HelpBlockKind::Bullet, text)
}

fn question(text: String) -> HelpBlock {
    block(HelpBlockKind::Question, text)
}

fn answer(text: String) -> HelpBlock {
    block(HelpBlockKind::Answer, text)
}

fn tab(title: String, blocks: Vec<HelpBlock>) -> HelpTab {
    HelpTab {
        title: title.into(),
        blocks: ModelRc::new(VecModel::from(blocks)),
    }
}

fn general_section(title: String, tabs: Vec<HelpTab>) -> HelpSection {
    HelpSection {
        title: title.into(),
        tool: ActiveTab::About,
        is_tool: false,
        tabs: ModelRc::new(VecModel::from(tabs)),
    }
}

fn tool_section(title: String, tool: ActiveTab, tabs: Vec<HelpTab>) -> HelpSection {
    HelpSection {
        title: title.into(),
        tool,
        is_tool: true,
        tabs: ModelRc::new(VecModel::from(tabs)),
    }
}
