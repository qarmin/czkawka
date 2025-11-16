use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use czkawka_core::common::model::CheckingMethod;
use czkawka_core::localizer_core::{fnc_get_similarity_minimal, fnc_get_similarity_very_high};
use czkawka_core::tools::big_file::SearchMode;
use czkawka_core::tools::similar_images::SIMILAR_VALUES;
use czkawka_core::tools::similar_images::core::get_string_from_similarity;
use gtk4::prelude::*;
use gtk4::{Builder, CheckButton, ComboBoxText, Entry, Label, Notebook, Picture, Scale, Widget};

use crate::flg;
use crate::gtk_traits::get_all_direct_children;
use crate::gui_structs::common_tree_view::{CommonTreeViews, SharedModelEnum, SubView};
use crate::gui_structs::gui_data::GuiData;
use crate::gui_structs::gui_settings::GuiSettings;
use crate::help_combo_box::{AUDIO_TYPE_CHECK_METHOD_COMBO_BOX, BIG_FILES_CHECK_METHOD_COMBO_BOX, DUPLICATES_CHECK_METHOD_COMBO_BOX, IMAGES_HASH_SIZE_COMBO_BOX};
use crate::notebook_enums::NotebookMainEnum;

#[derive(Clone)]
pub struct GuiMainNotebook {
    pub notebook_main: Notebook,

    // General

    // Duplicate
    pub combo_box_duplicate_check_method: ComboBoxText,
    pub combo_box_duplicate_hash_type: ComboBoxText,
    pub label_duplicate_check_method: Label,
    pub label_duplicate_hash_type: Label,
    pub check_button_duplicate_case_sensitive_name: CheckButton,

    pub image_preview_duplicates: Picture,

    // Big file
    pub label_big_shown_files: Label,
    pub entry_big_files_number: Entry,
    pub label_big_files_mode: Label,
    pub combo_box_big_files_mode: ComboBoxText,

    // Similar Images
    pub scale_similarity_similar_images: Scale,

    pub label_image_resize_algorithm: Label,
    pub label_image_hash_type: Label,
    pub label_image_hash_size: Label,

    pub combo_box_image_resize_algorithm: ComboBoxText,
    pub combo_box_image_hash_algorithm: ComboBoxText,
    pub combo_box_image_hash_size: ComboBoxText,

    pub check_button_image_ignore_same_size: CheckButton,
    pub check_button_video_ignore_same_size: CheckButton,

    pub label_image_similarity: Label,
    pub label_image_similarity_max: Label,

    pub image_preview_similar_images: Picture,
    pub label_similar_images_minimal_similarity: Label,

    // Video
    pub label_video_similarity: Label,
    pub label_video_similarity_min: Label,
    pub label_video_similarity_max: Label,

    pub scale_similarity_similar_videos: Scale,

    // Broken Files
    pub check_button_broken_files_audio: CheckButton,
    pub check_button_broken_files_pdf: CheckButton,
    pub check_button_broken_files_archive: CheckButton,
    pub check_button_broken_files_image: CheckButton,

    // Music
    pub check_button_music_title: CheckButton,
    pub check_button_music_artist: CheckButton,
    pub check_button_music_year: CheckButton,
    pub check_button_music_bitrate: CheckButton,
    pub check_button_music_genre: CheckButton,
    pub check_button_music_length: CheckButton,
    pub check_button_music_approximate_comparison: CheckButton,
    pub check_button_music_compare_only_in_title_group: CheckButton,
    #[expect(unused)]
    pub label_audio_check_type: Label,
    pub combo_box_audio_check_type: ComboBoxText,
    pub label_same_music_seconds: Label,
    pub label_same_music_similarity: Label,
    pub scale_seconds_same_music: Scale,
    pub scale_similarity_same_music: Scale,

    pub common_tree_views: CommonTreeViews,
}

impl GuiMainNotebook {
    pub(crate) fn create_from_builder(builder: &Builder, settings: &GuiSettings) -> Self {
        let notebook_main: Notebook = builder.object("notebook_main").expect("Cambalache");

        let combo_box_duplicate_check_method: ComboBoxText = builder.object("combo_box_duplicate_check_method").expect("Cambalache");
        let combo_box_duplicate_hash_type: ComboBoxText = builder.object("combo_box_duplicate_hash_type").expect("Cambalache");

        let entry_big_files_number: Entry = builder.object("entry_big_files_number").expect("Cambalache");

        //// Check Buttons
        let check_button_duplicate_case_sensitive_name: CheckButton = builder.object("check_button_duplicate_case_sensitive_name").expect("Cambalache");
        let check_button_music_title: CheckButton = builder.object("check_button_music_title").expect("Cambalache");
        let check_button_music_artist: CheckButton = builder.object("check_button_music_artist").expect("Cambalache");
        let check_button_music_year: CheckButton = builder.object("check_button_music_year").expect("Cambalache");
        let check_button_music_bitrate: CheckButton = builder.object("check_button_music_bitrate").expect("Cambalache");
        let check_button_music_genre: CheckButton = builder.object("check_button_music_genre").expect("Cambalache");
        let check_button_music_length: CheckButton = builder.object("check_button_music_length").expect("Cambalache");
        let check_button_music_approximate_comparison: CheckButton = builder.object("check_button_music_approximate_comparison").expect("Cambalache");
        let check_button_music_compare_only_in_title_group: CheckButton = builder.object("check_button_music_compare_only_in_title_group").expect("Cambalache");

        let check_button_broken_files_audio: CheckButton = builder.object("check_button_broken_files_audio").expect("Cambalache");
        let check_button_broken_files_pdf: CheckButton = builder.object("check_button_broken_files_pdf").expect("Cambalache");
        let check_button_broken_files_archive: CheckButton = builder.object("check_button_broken_files_archive").expect("Cambalache");
        let check_button_broken_files_image: CheckButton = builder.object("check_button_broken_files_image").expect("Cambalache");

        let scale_similarity_similar_images: Scale = builder.object("scale_similarity_similar_images").expect("Cambalache");
        let scale_similarity_similar_videos: Scale = builder.object("scale_similarity_similar_videos").expect("Cambalache");

        let combo_box_image_resize_algorithm: ComboBoxText = builder.object("combo_box_image_resize_algorithm").expect("Cambalache");
        let combo_box_image_hash_algorithm: ComboBoxText = builder.object("combo_box_image_hash_algorithm").expect("Cambalache");
        let combo_box_image_hash_size: ComboBoxText = builder.object("combo_box_image_hash_size").expect("Cambalache");
        let combo_box_big_files_mode: ComboBoxText = builder.object("combo_box_big_files_mode").expect("Cambalache");

        let check_button_image_ignore_same_size: CheckButton = builder.object("check_button_image_ignore_same_size").expect("Cambalache");
        let check_button_video_ignore_same_size: CheckButton = builder.object("check_button_video_ignore_same_size").expect("Cambalache");

        let label_similar_images_minimal_similarity: Label = builder.object("label_similar_images_minimal_similarity").expect("Cambalache");

        let label_duplicate_check_method: Label = builder.object("label_duplicate_check_method").expect("Cambalache");
        let label_duplicate_hash_type: Label = builder.object("label_duplicate_hash_type").expect("Cambalache");
        let label_big_shown_files: Label = builder.object("label_big_shown_files").expect("Cambalache");
        let label_image_resize_algorithm: Label = builder.object("label_image_resize_algorithm").expect("Cambalache");
        let label_image_hash_type: Label = builder.object("label_image_hash_type").expect("Cambalache");
        let label_image_hash_size: Label = builder.object("label_image_hash_size").expect("Cambalache");
        let label_image_similarity: Label = builder.object("label_image_similarity").expect("Cambalache");
        let label_image_similarity_max: Label = builder.object("label_image_similarity_max").expect("Cambalache");
        let label_video_similarity: Label = builder.object("label_video_similarity").expect("Cambalache");
        let label_video_similarity_min: Label = builder.object("label_video_similarity_min").expect("Cambalache");
        let label_video_similarity_max: Label = builder.object("label_video_similarity_max").expect("Cambalache");
        let label_big_files_mode: Label = builder.object("label_big_files_mode").expect("Cambalache");

        let image_preview_similar_images: Picture = builder.object("image_preview_similar_images").expect("Cambalache");
        let image_preview_duplicates: Picture = builder.object("image_preview_duplicates").expect("Cambalache");

        let label_audio_check_type: Label = builder.object("label_audio_check_type").expect("Cambalache");
        let combo_box_audio_check_type: ComboBoxText = builder.object("combo_box_audio_check_type").expect("Cambalache");
        let label_same_music_seconds: Label = builder.object("label_same_music_seconds").expect("Cambalache");
        let label_same_music_similarity: Label = builder.object("label_same_music_similarity").expect("Cambalache");
        let scale_seconds_same_music: Scale = builder.object("scale_seconds_same_music").expect("Cambalache");
        let scale_similarity_same_music: Scale = builder.object("scale_similarity_same_music").expect("Cambalache");

        #[rustfmt::skip]
        let subviews: Vec<_> = [
            SubView::new(builder, "scrolled_window_duplicate_finder", NotebookMainEnum::Duplicate, Some("image_preview_duplicates"), Some(settings.check_button_settings_show_preview_duplicates.clone()), SharedModelEnum::Duplicates(Rc::default())),
            SubView::new(builder, "scrolled_window_empty_folder_finder", NotebookMainEnum::EmptyDirectories, None, None, SharedModelEnum::EmptyFolder(Rc::default())),
            SubView::new(builder, "scrolled_window_empty_files_finder", NotebookMainEnum::EmptyFiles, None, None, SharedModelEnum::EmptyFiles(Rc::default())),
            SubView::new(builder, "scrolled_window_temporary_files_finder", NotebookMainEnum::Temporary, None, None, SharedModelEnum::Temporary(Rc::default())),
            SubView::new(builder, "scrolled_window_big_files_finder", NotebookMainEnum::BigFiles, None, None, SharedModelEnum::BigFile(Rc::default())),
            SubView::new(builder, "scrolled_window_similar_images_finder", NotebookMainEnum::SimilarImages, Some("image_preview_similar_images"), Some(settings.check_button_settings_show_preview_similar_images.clone()), SharedModelEnum::SimilarImages(Rc::default())),
            SubView::new(builder, "scrolled_window_similar_videos_finder", NotebookMainEnum::SimilarVideos, None, None, SharedModelEnum::SimilarVideos(Rc::default())),
            SubView::new(builder, "scrolled_window_same_music_finder", NotebookMainEnum::SameMusic, None, None, SharedModelEnum::SameMusic(Rc::default())),
            SubView::new(builder, "scrolled_window_invalid_symlinks", NotebookMainEnum::Symlinks, None, None, SharedModelEnum::Symlinks(Rc::default())),
            SubView::new(builder, "scrolled_window_broken_files", NotebookMainEnum::BrokenFiles, None, None, SharedModelEnum::BrokenFiles(Rc::default())),
            SubView::new(builder, "scrolled_window_bad_extensions", NotebookMainEnum::BadExtensions, None, None, SharedModelEnum::BadExtensions(Rc::default())),
        ]
        .into_iter()
        .collect();

        let common_tree_views = CommonTreeViews {
            notebook_main: notebook_main.clone(),
            subviews,
            preview_path: Rc::new(RefCell::new(String::new())),
        };

        Self {
            notebook_main,
            combo_box_duplicate_check_method,
            combo_box_duplicate_hash_type,
            label_duplicate_check_method,
            label_duplicate_hash_type,
            check_button_duplicate_case_sensitive_name,
            image_preview_duplicates,
            label_big_shown_files,
            entry_big_files_number,
            label_big_files_mode,
            combo_box_big_files_mode,
            scale_similarity_similar_images,
            label_image_resize_algorithm,
            label_image_hash_type,
            label_image_hash_size,
            combo_box_image_resize_algorithm,
            combo_box_image_hash_algorithm,
            combo_box_image_hash_size,
            check_button_image_ignore_same_size,
            check_button_video_ignore_same_size,
            label_image_similarity,
            label_image_similarity_max,
            image_preview_similar_images,
            label_similar_images_minimal_similarity,
            label_video_similarity,
            label_video_similarity_min,
            label_video_similarity_max,
            scale_similarity_similar_videos,
            check_button_broken_files_audio,
            check_button_broken_files_pdf,
            check_button_broken_files_archive,
            check_button_broken_files_image,
            check_button_music_title,
            check_button_music_artist,
            check_button_music_year,
            check_button_music_bitrate,
            check_button_music_genre,
            check_button_music_length,
            check_button_music_approximate_comparison,
            check_button_music_compare_only_in_title_group,
            label_audio_check_type,
            combo_box_audio_check_type,
            label_same_music_seconds,
            label_same_music_similarity,
            scale_seconds_same_music,
            scale_similarity_same_music,
            common_tree_views,
        }
    }

    pub(crate) fn setup(&self, gui_data: &GuiData) {
        self.common_tree_views.setup(gui_data);
    }

    pub(crate) fn update_language(&self) {
        self.check_button_duplicate_case_sensitive_name.set_label(Some(&flg!("duplicate_case_sensitive_name")));
        self.check_button_music_title.set_label(Some(&flg!("music_title_checkbox")));
        self.check_button_music_artist.set_label(Some(&flg!("music_artist_checkbox")));
        self.check_button_music_year.set_label(Some(&flg!("music_year_checkbox")));
        self.check_button_music_bitrate.set_label(Some(&flg!("music_bitrate_checkbox")));
        self.check_button_music_genre.set_label(Some(&flg!("music_genre_checkbox")));
        self.check_button_music_length.set_label(Some(&flg!("music_length_checkbox")));
        self.check_button_music_approximate_comparison.set_label(Some(&flg!("music_comparison_checkbox")));
        self.check_button_music_compare_only_in_title_group
            .set_label(Some(&flg!("music_compare_only_in_title_group")));

        self.check_button_music_approximate_comparison
            .set_tooltip_text(Some(&flg!("music_comparison_checkbox_tooltip")));

        self.label_duplicate_check_method.set_label(&flg!("main_label_check_method"));
        self.label_duplicate_hash_type.set_label(&flg!("main_label_hash_type"));
        self.label_big_shown_files.set_label(&flg!("main_label_shown_files"));
        self.label_image_resize_algorithm.set_label(&flg!("main_label_resize_algorithm"));
        self.label_image_hash_type.set_label(&flg!("main_label_hash_type"));
        self.label_image_hash_size.set_label(&flg!("main_label_hash_size"));
        self.label_image_similarity.set_label(&flg!("main_label_similarity"));
        self.label_image_similarity_max.set_label(&fnc_get_similarity_very_high());
        self.label_video_similarity.set_label(&flg!("main_label_similarity"));
        self.label_video_similarity_min.set_label(&fnc_get_similarity_minimal());
        self.label_video_similarity_max.set_label(&fnc_get_similarity_very_high());

        self.label_duplicate_check_method.set_tooltip_text(Some(&flg!("duplicate_check_method_tooltip")));
        self.combo_box_duplicate_check_method.set_tooltip_text(Some(&flg!("duplicate_check_method_tooltip")));
        self.label_duplicate_hash_type.set_tooltip_text(Some(&flg!("duplicate_hash_type_tooltip")));
        self.combo_box_duplicate_hash_type.set_tooltip_text(Some(&flg!("duplicate_hash_type_tooltip")));
        self.check_button_duplicate_case_sensitive_name
            .set_tooltip_text(Some(&flg!("duplicate_case_sensitive_name_tooltip")));
        self.check_button_music_compare_only_in_title_group
            .set_tooltip_text(Some(&flg!("music_compare_only_in_title_group_tooltip")));

        self.combo_box_image_hash_size.set_tooltip_text(Some(&flg!("image_hash_size_tooltip")));
        self.label_image_hash_size.set_tooltip_text(Some(&flg!("image_hash_size_tooltip")));

        self.combo_box_image_resize_algorithm.set_tooltip_text(Some(&flg!("image_resize_filter_tooltip")));
        self.label_image_resize_algorithm.set_tooltip_text(Some(&flg!("image_resize_filter_tooltip")));

        self.combo_box_image_hash_algorithm.set_tooltip_text(Some(&flg!("image_hash_alg_tooltip")));
        self.label_image_hash_type.set_tooltip_text(Some(&flg!("image_hash_alg_tooltip")));

        self.combo_box_big_files_mode.set_tooltip_text(Some(&flg!("big_files_mode_combobox_tooltip")));
        self.label_big_files_mode.set_tooltip_text(Some(&flg!("big_files_mode_combobox_tooltip")));
        self.label_big_files_mode.set_label(&flg!("big_files_mode_label"));

        self.check_button_image_ignore_same_size
            .set_tooltip_text(Some(&flg!("check_button_general_same_size_tooltip")));
        self.check_button_video_ignore_same_size
            .set_tooltip_text(Some(&flg!("check_button_general_same_size_tooltip")));
        self.check_button_image_ignore_same_size.set_label(Some(&flg!("check_button_general_same_size")));
        self.check_button_video_ignore_same_size.set_label(Some(&flg!("check_button_general_same_size")));

        self.check_button_broken_files_audio.set_label(Some(&flg!("main_check_box_broken_files_audio")));
        self.check_button_broken_files_archive.set_label(Some(&flg!("main_check_box_broken_files_archive")));
        self.check_button_broken_files_image.set_label(Some(&flg!("main_check_box_broken_files_image")));
        self.check_button_broken_files_pdf.set_label(Some(&flg!("main_check_box_broken_files_pdf")));

        self.label_same_music_seconds.set_label(&flg!("same_music_seconds_label"));
        self.label_same_music_similarity.set_label(&flg!("same_music_similarity_label"));
        self.label_same_music_seconds.set_tooltip_text(Some(&flg!("same_music_tooltip")));
        self.label_same_music_similarity.set_tooltip_text(Some(&flg!("same_music_tooltip")));
        self.scale_seconds_same_music.set_tooltip_text(Some(&flg!("same_music_tooltip")));
        self.scale_similarity_similar_videos.set_tooltip_text(Some(&flg!("same_music_tooltip")));

        {
            let hash_size_index = self.combo_box_image_hash_size.active().expect("Some hash size must be active") as usize;
            let hash_size = IMAGES_HASH_SIZE_COMBO_BOX[hash_size_index];
            match hash_size {
                8 => {
                    self.label_similar_images_minimal_similarity.set_text(&get_string_from_similarity(&SIMILAR_VALUES[0][5], 8));
                }
                16 => {
                    self.label_similar_images_minimal_similarity
                        .set_text(&get_string_from_similarity(&SIMILAR_VALUES[1][5], 16));
                }
                32 => {
                    self.label_similar_images_minimal_similarity
                        .set_text(&get_string_from_similarity(&SIMILAR_VALUES[2][5], 32));
                }
                64 => {
                    self.label_similar_images_minimal_similarity
                        .set_text(&get_string_from_similarity(&SIMILAR_VALUES[3][5], 64));
                }
                _ => panic!(),
            }
        }

        let vec_children: Vec<Widget> = get_all_direct_children(&self.notebook_main);
        let vec_children: Vec<Widget> = get_all_direct_children(&vec_children[1]);

        // Change name of main notebook tabs
        for (main_enum, fl_thing) in [
            (NotebookMainEnum::Duplicate as usize, flg!("main_notebook_duplicates")),
            (NotebookMainEnum::EmptyDirectories as usize, flg!("main_notebook_empty_directories")),
            (NotebookMainEnum::BigFiles as usize, flg!("main_notebook_big_files")),
            (NotebookMainEnum::EmptyFiles as usize, flg!("main_notebook_empty_files")),
            (NotebookMainEnum::Temporary as usize, flg!("main_notebook_temporary")),
            (NotebookMainEnum::SimilarImages as usize, flg!("main_notebook_similar_images")),
            (NotebookMainEnum::SimilarVideos as usize, flg!("main_notebook_similar_videos")),
            (NotebookMainEnum::SameMusic as usize, flg!("main_notebook_same_music")),
            (NotebookMainEnum::Symlinks as usize, flg!("main_notebook_symlinks")),
            (NotebookMainEnum::BrokenFiles as usize, flg!("main_notebook_broken_files")),
            (NotebookMainEnum::BadExtensions as usize, flg!("main_notebook_bad_extensions")),
        ] {
            self.notebook_main
                .tab_label(&vec_children[main_enum])
                .expect("Tab label must be set")
                .downcast::<Label>()
                .expect("Tab label must be a label")
                .set_text(&fl_thing);
        }

        // Change names of columns
        let mut names_of_columns: HashMap<NotebookMainEnum, Vec<String>> = HashMap::new();

        names_of_columns.insert(
            NotebookMainEnum::Duplicate,
            vec![
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::EmptyDirectories,
            vec![
                flg!("main_tree_view_column_folder_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::BigFiles,
            vec![
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::EmptyFiles,
            vec![
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::Temporary,
            vec![
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::SimilarImages,
            vec![
                flg!("main_tree_view_column_similarity"),
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_dimensions"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::SimilarVideos,
            vec![
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::SameMusic,
            vec![
                flg!("main_tree_view_column_size"),
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_title"),
                flg!("main_tree_view_column_artist"),
                flg!("main_tree_view_column_year"),
                flg!("main_tree_view_column_bitrate"),
                flg!("main_tree_view_column_length"),
                flg!("main_tree_view_column_genre"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::Symlinks,
            vec![
                flg!("main_tree_view_column_symlink_file_name"),
                flg!("main_tree_view_column_symlink_folder"),
                flg!("main_tree_view_column_destination_path"),
                flg!("main_tree_view_column_type_of_error"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::BrokenFiles,
            vec![
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_type_of_error"),
                flg!("main_tree_view_column_modification"),
            ],
        );
        names_of_columns.insert(
            NotebookMainEnum::BadExtensions,
            vec![
                flg!("main_tree_view_column_file_name"),
                flg!("main_tree_view_column_path"),
                flg!("main_tree_view_column_current_extension"),
                flg!("main_tree_view_column_proper_extensions"),
                // flg!("main_tree_view_column_modification"), // TODO - too much data?
            ],
        );

        for (key_enum, columns_names) in names_of_columns {
            let s = &self.common_tree_views.get_subview(key_enum);

            // Skipping first column because it is selection button
            assert_eq!(
                columns_names.len() + 1,
                s.tree_view.columns().len(),
                "Number of columns in tree view and names do not match for {:?}, tree_view - {:?}",
                key_enum,
                s.tree_view.widget_name()
            );
            for (column, name) in s.tree_view.columns().iter().skip(1).zip(columns_names.iter()) {
                column.set_title(name);
            }
        }

        {
            let active = self.combo_box_audio_check_type.active().unwrap_or(0);
            self.combo_box_audio_check_type.remove_all();
            for i in &AUDIO_TYPE_CHECK_METHOD_COMBO_BOX {
                let text = match i.check_method {
                    CheckingMethod::AudioTags => flg!("music_checking_by_tags"),
                    CheckingMethod::AudioContent => flg!("music_checking_by_content"),
                    _ => panic!(),
                };
                self.combo_box_audio_check_type.append_text(&text);
            }
            self.combo_box_audio_check_type.set_active(Some(active));
        }
        {
            let active = self.combo_box_duplicate_check_method.active().unwrap_or(0);
            self.combo_box_duplicate_check_method.remove_all();
            for i in &DUPLICATES_CHECK_METHOD_COMBO_BOX {
                let text = match i.check_method {
                    CheckingMethod::Hash => flg!("duplicate_mode_hash_combo_box"),
                    CheckingMethod::Size => flg!("duplicate_mode_size_combo_box"),
                    CheckingMethod::Name => flg!("duplicate_mode_name_combo_box"),
                    CheckingMethod::SizeName => flg!("duplicate_mode_size_name_combo_box"),
                    _ => panic!(),
                };
                self.combo_box_duplicate_check_method.append_text(&text);
            }
            self.combo_box_duplicate_check_method.set_active(Some(active));
        }
        {
            let active = self.combo_box_big_files_mode.active().unwrap_or(0);
            self.combo_box_big_files_mode.remove_all();
            for i in &BIG_FILES_CHECK_METHOD_COMBO_BOX {
                let text = match i.check_method {
                    SearchMode::SmallestFiles => flg!("big_files_mode_smallest_combo_box"),
                    SearchMode::BiggestFiles => flg!("big_files_mode_biggest_combo_box"),
                };
                self.combo_box_big_files_mode.append_text(&text);
            }
            self.combo_box_big_files_mode.set_active(Some(active));
        }
    }
}
