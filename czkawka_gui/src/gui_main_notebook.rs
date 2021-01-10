use gtk::prelude::*;
use gtk::TreeView;

#[derive(Clone)]
pub struct GUIMainNotebook {
    pub notebook_main: gtk::Notebook,

    pub scrolled_window_duplicate_finder: gtk::ScrolledWindow,
    pub scrolled_window_empty_folder_finder: gtk::ScrolledWindow,
    pub scrolled_window_empty_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_temporary_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_big_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_similar_images_finder: gtk::ScrolledWindow,
    pub scrolled_window_zeroed_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_same_music_finder: gtk::ScrolledWindow,
    pub scrolled_window_invalid_symlinks: gtk::ScrolledWindow,

    pub tree_view_duplicate_finder: gtk::TreeView,
    pub tree_view_empty_folder_finder: gtk::TreeView,
    pub tree_view_empty_files_finder: gtk::TreeView,
    pub tree_view_temporary_files_finder: gtk::TreeView,
    pub tree_view_big_files_finder: gtk::TreeView,
    pub tree_view_similar_images_finder: gtk::TreeView,
    pub tree_view_zeroed_files_finder: gtk::TreeView,
    pub tree_view_same_music_finder: gtk::TreeView,
    pub tree_view_invalid_symlinks: gtk::TreeView,

    pub entry_similar_images_minimal_size: gtk::Entry,
    pub entry_duplicate_minimal_size: gtk::Entry,
    pub entry_big_files_number: gtk::Entry,
    pub entry_same_music_minimal_size: gtk::Entry,

    //// Check Buttons
    pub check_button_music_title: gtk::CheckButton,
    pub check_button_music_artist: gtk::CheckButton,
    pub check_button_music_album_title: gtk::CheckButton,
    pub check_button_music_album_artist: gtk::CheckButton,
    pub check_button_music_year: gtk::CheckButton,

    //// Radio Buttons
    // Duplicates
    pub radio_button_duplicates_name: gtk::RadioButton,
    pub radio_button_duplicates_size: gtk::RadioButton,
    pub radio_button_duplicates_hashmb: gtk::RadioButton,
    pub radio_button_duplicates_hash: gtk::RadioButton,

    pub radio_button_similar_images_minimal: gtk::RadioButton,
    pub radio_button_similar_images_very_small: gtk::RadioButton,
    pub radio_button_similar_images_small: gtk::RadioButton,
    pub radio_button_similar_images_medium: gtk::RadioButton,
    pub radio_button_similar_images_high: gtk::RadioButton,
    pub radio_button_similar_images_very_high: gtk::RadioButton,

    pub image_preview_similar_images: gtk::Image,
}

impl GUIMainNotebook {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let notebook_main: gtk::Notebook = builder.get_object("notebook_main").unwrap();

        let scrolled_window_duplicate_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_duplicate_finder").unwrap();
        let scrolled_window_empty_folder_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_empty_folder_finder").unwrap();
        let scrolled_window_empty_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_empty_files_finder").unwrap();
        let scrolled_window_temporary_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_temporary_files_finder").unwrap();
        let scrolled_window_big_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_big_files_finder").unwrap();
        let scrolled_window_similar_images_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_similar_images_finder").unwrap();
        let scrolled_window_zeroed_files_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_zeroed_files_finder").unwrap();
        let scrolled_window_same_music_finder: gtk::ScrolledWindow = builder.get_object("scrolled_window_same_music_finder").unwrap();
        let scrolled_window_invalid_symlinks: gtk::ScrolledWindow = builder.get_object("scrolled_window_invalid_symlinks").unwrap();

        let tree_view_duplicate_finder: gtk::TreeView = TreeView::new();
        let tree_view_empty_folder_finder: gtk::TreeView = TreeView::new();
        let tree_view_empty_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_temporary_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_big_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_similar_images_finder: gtk::TreeView = TreeView::new();
        let tree_view_zeroed_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_same_music_finder: gtk::TreeView = TreeView::new();
        let tree_view_invalid_symlinks: gtk::TreeView = TreeView::new();

        let entry_similar_images_minimal_size: gtk::Entry = builder.get_object("entry_similar_images_minimal_size").unwrap();
        let entry_duplicate_minimal_size: gtk::Entry = builder.get_object("entry_duplicate_minimal_size").unwrap();
        let entry_big_files_number: gtk::Entry = builder.get_object("entry_big_files_number").unwrap();
        let entry_same_music_minimal_size: gtk::Entry = builder.get_object("entry_same_music_minimal_size").unwrap();

        //// Check Buttons
        let check_button_music_title: gtk::CheckButton = builder.get_object("check_button_music_title").unwrap();
        let check_button_music_artist: gtk::CheckButton = builder.get_object("check_button_music_artist").unwrap();
        let check_button_music_album_title: gtk::CheckButton = builder.get_object("check_button_music_album_title").unwrap();
        let check_button_music_album_artist: gtk::CheckButton = builder.get_object("check_button_music_album_artist").unwrap();
        let check_button_music_year: gtk::CheckButton = builder.get_object("check_button_music_year").unwrap();

        //// Radio Buttons
        let radio_button_duplicates_name: gtk::RadioButton = builder.get_object("radio_button_duplicates_name").unwrap();
        let radio_button_duplicates_size: gtk::RadioButton = builder.get_object("radio_button_duplicates_size").unwrap();
        let radio_button_duplicates_hashmb: gtk::RadioButton = builder.get_object("radio_button_duplicates_hashmb").unwrap();
        let radio_button_duplicates_hash: gtk::RadioButton = builder.get_object("radio_button_duplicates_hash").unwrap();

        let radio_button_similar_images_minimal: gtk::RadioButton = builder.get_object("radio_button_similar_images_minimal").unwrap();
        let radio_button_similar_images_very_small: gtk::RadioButton = builder.get_object("radio_button_similar_images_very_small").unwrap();
        let radio_button_similar_images_small: gtk::RadioButton = builder.get_object("radio_button_similar_images_small").unwrap();
        let radio_button_similar_images_medium: gtk::RadioButton = builder.get_object("radio_button_similar_images_medium").unwrap();
        let radio_button_similar_images_high: gtk::RadioButton = builder.get_object("radio_button_similar_images_high").unwrap();
        let radio_button_similar_images_very_high: gtk::RadioButton = builder.get_object("radio_button_similar_images_very_high").unwrap();

        let image_preview_similar_images: gtk::Image = builder.get_object("image_preview_similar_images").unwrap();

        Self {
            notebook_main,
            scrolled_window_duplicate_finder,
            scrolled_window_empty_folder_finder,
            scrolled_window_empty_files_finder,
            scrolled_window_temporary_files_finder,
            scrolled_window_big_files_finder,
            scrolled_window_similar_images_finder,
            scrolled_window_zeroed_files_finder,
            scrolled_window_same_music_finder,
            scrolled_window_invalid_symlinks,
            tree_view_duplicate_finder,
            tree_view_empty_folder_finder,
            tree_view_empty_files_finder,
            tree_view_temporary_files_finder,
            tree_view_big_files_finder,
            tree_view_similar_images_finder,
            tree_view_zeroed_files_finder,
            tree_view_same_music_finder,
            tree_view_invalid_symlinks,
            entry_similar_images_minimal_size,
            entry_duplicate_minimal_size,
            entry_big_files_number,
            entry_same_music_minimal_size,
            check_button_music_title,
            check_button_music_artist,
            check_button_music_album_title,
            check_button_music_album_artist,
            check_button_music_year,
            radio_button_duplicates_name,
            radio_button_duplicates_size,
            radio_button_duplicates_hashmb,
            radio_button_duplicates_hash,
            radio_button_similar_images_minimal,
            radio_button_similar_images_very_small,
            radio_button_similar_images_small,
            radio_button_similar_images_medium,
            radio_button_similar_images_high,
            radio_button_similar_images_very_high,
            image_preview_similar_images,
        }
    }
}
