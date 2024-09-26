use gtk4::prelude::*;
use gtk4::{CheckButton, Widget};

use czkawka_core::common_dir_traversal::CheckingMethod;

use crate::gui_structs::gui_data::GuiData;
use crate::help_combo_box::AUDIO_TYPE_CHECK_METHOD_COMBO_BOX;
use crate::help_functions::scale_set_min_max_values;

const MINIMUM_SECONDS: f64 = 0.5;
const MAXIMUM_SECONDS: f64 = 180.0;
const DEFAULT_SECONDS: f64 = 15.0;
const MINIMUM_SIMILARITY: f64 = 0.0;
const MAXIMUM_SIMILARITY: f64 = 10.0;
const DEFAULT_SIMILARITY: f64 = 5.0;

pub fn connect_same_music_change_mode(gui_data: &GuiData) {
    let check_button_music_title = gui_data.main_notebook.check_button_music_title.clone();
    let check_button_music_approximate_comparison = gui_data.main_notebook.check_button_music_approximate_comparison.clone();
    let check_button_music_bitrate = gui_data.main_notebook.check_button_music_bitrate.clone();
    let check_button_music_artist = gui_data.main_notebook.check_button_music_artist.clone();
    let check_button_music_genre = gui_data.main_notebook.check_button_music_genre.clone();
    let check_button_music_length = gui_data.main_notebook.check_button_music_length.clone();
    let check_button_music_year = gui_data.main_notebook.check_button_music_year.clone();
    let buttons = [
        check_button_music_title,
        check_button_music_approximate_comparison,
        check_button_music_bitrate,
        check_button_music_artist,
        check_button_music_genre,
        check_button_music_year,
        check_button_music_length,
    ];

    let check_button_music_compare_only_in_title_group = gui_data.main_notebook.check_button_music_compare_only_in_title_group.clone();
    let reversed_buttons = [check_button_music_compare_only_in_title_group];

    let scale_seconds_same_music = gui_data.main_notebook.scale_seconds_same_music.clone();
    let scale_similarity_same_music = gui_data.main_notebook.scale_similarity_same_music.clone();
    let label_same_music_similarity = gui_data.main_notebook.label_same_music_similarity.clone();
    let label_same_music_seconds = gui_data.main_notebook.label_same_music_seconds.clone();

    scale_set_min_max_values(&scale_seconds_same_music, MINIMUM_SECONDS, MAXIMUM_SECONDS, DEFAULT_SECONDS, None);
    scale_set_min_max_values(&scale_similarity_same_music, MINIMUM_SIMILARITY, MAXIMUM_SIMILARITY, DEFAULT_SIMILARITY, None);

    let scales_and_labels = [
        scale_seconds_same_music.into(),
        scale_similarity_same_music.into(),
        label_same_music_similarity.into(),
        label_same_music_seconds.into(),
    ];

    let combo_box_audio_check_type = gui_data.main_notebook.combo_box_audio_check_type.clone();

    let check_method_index = combo_box_audio_check_type.active().expect("Failed to get active item") as usize;
    let check_method = AUDIO_TYPE_CHECK_METHOD_COMBO_BOX[check_method_index].check_method;

    disable_enable_buttons(&buttons, &reversed_buttons, &scales_and_labels, check_method);
    combo_box_audio_check_type.connect_changed(move |combo_box_text| {
        if let Some(active) = combo_box_text.active() {
            let check_method = AUDIO_TYPE_CHECK_METHOD_COMBO_BOX[active as usize].check_method;

            disable_enable_buttons(&buttons, &reversed_buttons, &scales_and_labels, check_method);
        }
    });
}

fn disable_enable_buttons(buttons: &[CheckButton; 7], reverse_buttons: &[CheckButton; 1], scales: &[Widget; 4], current_mode: CheckingMethod) {
    match current_mode {
        CheckingMethod::AudioTags => {
            buttons.iter().for_each(WidgetExt::show);
            reverse_buttons.iter().for_each(WidgetExt::hide);
            scales.iter().for_each(WidgetExt::hide);
        }
        CheckingMethod::AudioContent => {
            buttons.iter().for_each(WidgetExt::hide);
            reverse_buttons.iter().for_each(WidgetExt::show);
            scales.iter().for_each(WidgetExt::show);
        }
        _ => panic!(),
    }
}
