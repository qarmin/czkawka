use log::trace;
use slint::ComponentHandle;

use crate::common::{connect_i32_into_u64, split_u64_into_i32s};
use crate::{ActiveTab, Callabler, GuiState, MainWindow};

pub(crate) fn change_number_of_checked_items(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_change_number_of_checked_items(move |number_of_changed_items| {
        trace!("Changing number of checked items with {number_of_changed_items}");
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let active_tab = app.global::<GuiState>().get_active_tab();
        change_number_of_enabled_items(&app, active_tab, number_of_changed_items as i64);
    });
}

// TODO - sad day for code readability, because slint not supports i64 - https://github.com/slint-ui/slint/issues/6589
pub(crate) fn set_number_of_enabled_items(app: &MainWindow, active_tab: ActiveTab, items_number: u64) {
    let (it1, it2) = split_u64_into_i32s(items_number);
    match active_tab {
        ActiveTab::DuplicateFiles => {
            app.global::<GuiState>().set_selected_results_duplicates(it1);
            app.global::<GuiState>().set_selected_results_duplicates2(it2);
        }
        ActiveTab::EmptyFolders => {
            app.global::<GuiState>().set_selected_results_empty_folders(it1);
            app.global::<GuiState>().set_selected_results_empty_folders2(it2);
        }
        ActiveTab::BigFiles => {
            app.global::<GuiState>().set_selected_results_big_files(it1);
            app.global::<GuiState>().set_selected_results_big_files2(it2);
        }
        ActiveTab::EmptyFiles => {
            app.global::<GuiState>().set_selected_results_empty_files(it1);
            app.global::<GuiState>().set_selected_results_empty_files2(it2);
        }
        ActiveTab::TemporaryFiles => {
            app.global::<GuiState>().set_selected_results_temporary_files(it1);
            app.global::<GuiState>().set_selected_results_temporary_files2(it2);
        }
        ActiveTab::SimilarImages => {
            app.global::<GuiState>().set_selected_results_similar_images(it1);
            app.global::<GuiState>().set_selected_results_similar_images2(it2);
        }
        ActiveTab::SimilarVideos => {
            app.global::<GuiState>().set_selected_results_similar_videos(it1);
            app.global::<GuiState>().set_selected_results_similar_videos2(it2);
        }
        ActiveTab::SimilarMusic => {
            app.global::<GuiState>().set_selected_results_similar_music(it1);
            app.global::<GuiState>().set_selected_results_similar_music2(it2);
        }
        ActiveTab::InvalidSymlinks => {
            app.global::<GuiState>().set_selected_results_invalid_symlinks(it1);
            app.global::<GuiState>().set_selected_results_invalid_symlinks2(it2);
        }
        ActiveTab::BrokenFiles => {
            app.global::<GuiState>().set_selected_results_broken_files(it1);
            app.global::<GuiState>().set_selected_results_broken_files2(it2);
        }
        ActiveTab::BadExtensions => {
            app.global::<GuiState>().set_selected_results_bad_extensions(it1);
            app.global::<GuiState>().set_selected_results_bad_extensions2(it2);
        }
        ActiveTab::BadNames => {
            app.global::<GuiState>().set_selected_results_bad_names(it1);
            app.global::<GuiState>().set_selected_results_bad_names2(it2);
        }
        ActiveTab::ExifRemover => {
            app.global::<GuiState>().set_selected_results_exif_remover(it1);
            app.global::<GuiState>().set_selected_results_exif_remover2(it2);
        }
        ActiveTab::VideoOptimizer => {
            app.global::<GuiState>().set_selected_results_video_optimizer(it1);
            app.global::<GuiState>().set_selected_results_video_optimizer2(it2);
        }
        _ => unreachable!("Current tab is not a tool that has enabled items"),
    }
}

pub(crate) fn change_number_of_enabled_items(app: &MainWindow, active_tab: ActiveTab, additions: i64) {
    let before_number_of_items = get_number_of_enabled_items(app, active_tab);
    let after_number_of_items = before_number_of_items.checked_add_signed(additions).unwrap_or_else(|| {
        panic!("Counter desync: before_number_of_items = {before_number_of_items}, additions = {additions}, tab = {active_tab:?}");
    });
    set_number_of_enabled_items(app, active_tab, after_number_of_items);
}

pub(crate) fn get_number_of_enabled_items(app: &MainWindow, active_tab: ActiveTab) -> u64 {
    let (it1, it2) = match active_tab {
        ActiveTab::DuplicateFiles => (
            app.global::<GuiState>().get_selected_results_duplicates(),
            app.global::<GuiState>().get_selected_results_duplicates2(),
        ),
        ActiveTab::EmptyFolders => (
            app.global::<GuiState>().get_selected_results_empty_folders(),
            app.global::<GuiState>().get_selected_results_empty_folders2(),
        ),
        ActiveTab::BigFiles => (
            app.global::<GuiState>().get_selected_results_big_files(),
            app.global::<GuiState>().get_selected_results_big_files2(),
        ),
        ActiveTab::EmptyFiles => (
            app.global::<GuiState>().get_selected_results_empty_files(),
            app.global::<GuiState>().get_selected_results_empty_files2(),
        ),
        ActiveTab::TemporaryFiles => (
            app.global::<GuiState>().get_selected_results_temporary_files(),
            app.global::<GuiState>().get_selected_results_temporary_files2(),
        ),
        ActiveTab::SimilarImages => (
            app.global::<GuiState>().get_selected_results_similar_images(),
            app.global::<GuiState>().get_selected_results_similar_images2(),
        ),
        ActiveTab::SimilarVideos => (
            app.global::<GuiState>().get_selected_results_similar_videos(),
            app.global::<GuiState>().get_selected_results_similar_videos2(),
        ),
        ActiveTab::SimilarMusic => (
            app.global::<GuiState>().get_selected_results_similar_music(),
            app.global::<GuiState>().get_selected_results_similar_music2(),
        ),
        ActiveTab::InvalidSymlinks => (
            app.global::<GuiState>().get_selected_results_invalid_symlinks(),
            app.global::<GuiState>().get_selected_results_invalid_symlinks2(),
        ),
        ActiveTab::BrokenFiles => (
            app.global::<GuiState>().get_selected_results_broken_files(),
            app.global::<GuiState>().get_selected_results_broken_files2(),
        ),
        ActiveTab::BadExtensions => (
            app.global::<GuiState>().get_selected_results_bad_extensions(),
            app.global::<GuiState>().get_selected_results_bad_extensions2(),
        ),
        ActiveTab::BadNames => (
            app.global::<GuiState>().get_selected_results_bad_names(),
            app.global::<GuiState>().get_selected_results_bad_names2(),
        ),
        ActiveTab::ExifRemover => (
            app.global::<GuiState>().get_selected_results_exif_remover(),
            app.global::<GuiState>().get_selected_results_exif_remover2(),
        ),
        ActiveTab::VideoOptimizer => (
            app.global::<GuiState>().get_selected_results_video_optimizer(),
            app.global::<GuiState>().get_selected_results_video_optimizer2(),
        ),
        _ => unreachable!("Current tab is not a tool that has enabled items"),
    };
    connect_i32_into_u64(it1, it2)
}
