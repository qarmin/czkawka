use slint::{ComponentHandle, Model, SharedString};

use crate::settings::gui_settings_values::StringComboBoxItems;
use crate::{BigFilesSettings, DuplicateSettings, GeneralSettings, MainWindow, SameMusicSettings, SimilarImagesSettings};

pub(crate) fn set_initial_gui_infos(app: &MainWindow) {
    let items = StringComboBoxItems::new();

    fn display_names<T: std::fmt::Debug + Clone>(items: &[crate::settings::gui_settings_values::StringComboBoxItem<T>]) -> Vec<SharedString> {
        items.iter().map(|e| SharedString::from(e.display_name.as_str())).collect()
    }

    let general = app.global::<GeneralSettings>();
    let dup = app.global::<DuplicateSettings>();
    let si = app.global::<SimilarImagesSettings>();
    let bf = app.global::<BigFilesSettings>();
    let sm = app.global::<SameMusicSettings>();

    let slint_vec = |model: slint::ModelRc<SharedString>| model.iter().collect::<Vec<SharedString>>();

    assert_eq!(
        slint_vec(general.get_min_file_size_options()),
        display_names(&items.min_file_size),
        "GeneralSettings.min_file_size_options out of sync with Rust"
    );

    assert_eq!(
        slint_vec(dup.get_check_method_options()),
        display_names(&items.duplicates_check_method),
        "DuplicateSettings.check_method_options out of sync with Rust"
    );
    assert_eq!(
        slint_vec(dup.get_hash_type_options()),
        display_names(&items.duplicates_hash_type),
        "DuplicateSettings.hash_type_options out of sync with Rust"
    );

    assert_eq!(
        slint_vec(si.get_similarity_preset_options()),
        display_names(&items.similarity_preset),
        "SimilarImagesSettings.similarity_preset_options out of sync with Rust"
    );
    assert_eq!(
        slint_vec(si.get_hash_size_options()),
        display_names(&items.hash_size),
        "SimilarImagesSettings.hash_size_options out of sync with Rust"
    );
    assert_eq!(
        slint_vec(si.get_hash_alg_options()),
        display_names(&items.hash_alg),
        "SimilarImagesSettings.hash_alg_options out of sync with Rust"
    );
    assert_eq!(
        slint_vec(si.get_image_filter_options()),
        display_names(&items.image_filter),
        "SimilarImagesSettings.image_filter_options out of sync with Rust"
    );

    assert_eq!(
        slint_vec(bf.get_search_mode_options()),
        display_names(&items.biggest_files_method),
        "BigFilesSettings.search_mode_options out of sync with Rust"
    );
    assert_eq!(
        slint_vec(bf.get_count_options()),
        display_names(&items.big_files_count),
        "BigFilesSettings.count_options out of sync with Rust"
    );

    assert_eq!(
        slint_vec(sm.get_check_method_options()),
        display_names(&items.same_music_check_method),
        "SameMusicSettings.check_method_options out of sync with Rust"
    );
}
