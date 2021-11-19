use gtk::prelude::*;
use gtk::TreeView;

#[derive(Clone)]
pub struct GuiMainNotebook {
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
    pub scrolled_window_broken_files: gtk::ScrolledWindow,

    pub tree_view_duplicate_finder: gtk::TreeView,
    pub tree_view_empty_folder_finder: gtk::TreeView,
    pub tree_view_empty_files_finder: gtk::TreeView,
    pub tree_view_temporary_files_finder: gtk::TreeView,
    pub tree_view_big_files_finder: gtk::TreeView,
    pub tree_view_similar_images_finder: gtk::TreeView,
    pub tree_view_zeroed_files_finder: gtk::TreeView,
    pub tree_view_same_music_finder: gtk::TreeView,
    pub tree_view_invalid_symlinks: gtk::TreeView,
    pub tree_view_broken_files: gtk::TreeView,

    pub entry_similar_images_minimal_size: gtk::Entry,
    pub entry_similar_images_maximal_size: gtk::Entry,
    pub entry_duplicate_minimal_size: gtk::Entry,
    pub entry_duplicate_maximal_size: gtk::Entry,
    pub entry_same_music_minimal_size: gtk::Entry,
    pub entry_same_music_maximal_size: gtk::Entry,

    pub entry_big_files_number: gtk::Entry,

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

    pub scale_similarity: gtk::Scale,

    pub radio_button_hash_type_blake3: gtk::RadioButton,
    pub radio_button_hash_type_crc32: gtk::RadioButton,
    pub radio_button_hash_type_xxh3: gtk::RadioButton,

    pub radio_button_resize_algorithm_lanczos3: gtk::RadioButton,
    pub radio_button_resize_algorithm_nearest: gtk::RadioButton,
    pub radio_button_resize_algorithm_triangle: gtk::RadioButton,
    pub radio_button_resize_algorithm_gaussian: gtk::RadioButton,
    pub radio_button_resize_algorithm_catmullrom: gtk::RadioButton,

    pub radio_button_similar_hash_algorithm_gradient: gtk::RadioButton,
    pub radio_button_similar_hash_algorithm_blockhash: gtk::RadioButton,
    pub radio_button_similar_hash_algorithm_mean: gtk::RadioButton,
    pub radio_button_similar_hash_algorithm_vertgradient: gtk::RadioButton,
    pub radio_button_similar_hash_algorithm_doublegradient: gtk::RadioButton,

    pub radio_button_similar_hash_size_4: gtk::RadioButton,
    pub radio_button_similar_hash_size_8: gtk::RadioButton,
    pub radio_button_similar_hash_size_16: gtk::RadioButton,

    pub image_preview_similar_images: gtk::Image,
    pub image_preview_duplicates: gtk::Image,
}

impl GuiMainNotebook {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let notebook_main: gtk::Notebook = builder.object("notebook_main").unwrap();

        let scrolled_window_duplicate_finder: gtk::ScrolledWindow = builder.object("scrolled_window_duplicate_finder").unwrap();
        let scrolled_window_empty_folder_finder: gtk::ScrolledWindow = builder.object("scrolled_window_empty_folder_finder").unwrap();
        let scrolled_window_empty_files_finder: gtk::ScrolledWindow = builder.object("scrolled_window_empty_files_finder").unwrap();
        let scrolled_window_temporary_files_finder: gtk::ScrolledWindow = builder.object("scrolled_window_temporary_files_finder").unwrap();
        let scrolled_window_big_files_finder: gtk::ScrolledWindow = builder.object("scrolled_window_big_files_finder").unwrap();
        let scrolled_window_similar_images_finder: gtk::ScrolledWindow = builder.object("scrolled_window_similar_images_finder").unwrap();
        let scrolled_window_zeroed_files_finder: gtk::ScrolledWindow = builder.object("scrolled_window_zeroed_files_finder").unwrap();
        let scrolled_window_same_music_finder: gtk::ScrolledWindow = builder.object("scrolled_window_same_music_finder").unwrap();
        let scrolled_window_invalid_symlinks: gtk::ScrolledWindow = builder.object("scrolled_window_invalid_symlinks").unwrap();
        let scrolled_window_broken_files: gtk::ScrolledWindow = builder.object("scrolled_window_broken_files").unwrap();

        let tree_view_duplicate_finder: gtk::TreeView = TreeView::new();
        let tree_view_empty_folder_finder: gtk::TreeView = TreeView::new();
        let tree_view_empty_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_temporary_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_big_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_similar_images_finder: gtk::TreeView = TreeView::new();
        let tree_view_zeroed_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_same_music_finder: gtk::TreeView = TreeView::new();
        let tree_view_invalid_symlinks: gtk::TreeView = TreeView::new();
        let tree_view_broken_files: gtk::TreeView = TreeView::new();

        let entry_similar_images_minimal_size: gtk::Entry = builder.object("entry_similar_images_minimal_size").unwrap();
        let entry_similar_images_maximal_size: gtk::Entry = builder.object("entry_similar_images_maximal_size").unwrap();
        let entry_duplicate_minimal_size: gtk::Entry = builder.object("entry_duplicate_minimal_size").unwrap();
        let entry_duplicate_maximal_size: gtk::Entry = builder.object("entry_duplicate_maximal_size").unwrap();
        let entry_same_music_minimal_size: gtk::Entry = builder.object("entry_same_music_minimal_size").unwrap();
        let entry_same_music_maximal_size: gtk::Entry = builder.object("entry_same_music_maximal_size").unwrap();

        let entry_big_files_number: gtk::Entry = builder.object("entry_big_files_number").unwrap();

        //// Check Buttons
        let check_button_music_title: gtk::CheckButton = builder.object("check_button_music_title").unwrap();
        let check_button_music_artist: gtk::CheckButton = builder.object("check_button_music_artist").unwrap();
        let check_button_music_album_title: gtk::CheckButton = builder.object("check_button_music_album_title").unwrap();
        let check_button_music_album_artist: gtk::CheckButton = builder.object("check_button_music_album_artist").unwrap();
        let check_button_music_year: gtk::CheckButton = builder.object("check_button_music_year").unwrap();

        //// Radio Buttons
        let radio_button_duplicates_name: gtk::RadioButton = builder.object("radio_button_duplicates_name").unwrap();
        let radio_button_duplicates_size: gtk::RadioButton = builder.object("radio_button_duplicates_size").unwrap();
        let radio_button_duplicates_hashmb: gtk::RadioButton = builder.object("radio_button_duplicates_hashmb").unwrap();
        let radio_button_duplicates_hash: gtk::RadioButton = builder.object("radio_button_duplicates_hash").unwrap();

        let scale_similarity: gtk::Scale = builder.object("scale_similarity").unwrap();

        let radio_button_hash_type_blake3: gtk::RadioButton = builder.object("radio_button_hash_type_blake3").unwrap();
        let radio_button_hash_type_crc32: gtk::RadioButton = builder.object("radio_button_hash_type_crc32").unwrap();
        let radio_button_hash_type_xxh3: gtk::RadioButton = builder.object("radio_button_hash_type_xxh3").unwrap();

        let radio_button_resize_algorithm_lanczos3: gtk::RadioButton = builder.object("radio_button_resize_algorithm_lanczos3").unwrap();
        let radio_button_resize_algorithm_nearest: gtk::RadioButton = builder.object("radio_button_resize_algorithm_nearest").unwrap();
        let radio_button_resize_algorithm_triangle: gtk::RadioButton = builder.object("radio_button_resize_algorithm_triangle").unwrap();
        let radio_button_resize_algorithm_gaussian: gtk::RadioButton = builder.object("radio_button_resize_algorithm_gaussian").unwrap();
        let radio_button_resize_algorithm_catmullrom: gtk::RadioButton = builder.object("radio_button_resize_algorithm_catmullrom").unwrap();

        let radio_button_similar_hash_algorithm_gradient: gtk::RadioButton = builder.object("radio_button_similar_hash_algorithm_gradient").unwrap();
        let radio_button_similar_hash_algorithm_blockhash: gtk::RadioButton = builder.object("radio_button_similar_hash_algorithm_blockhash").unwrap();
        let radio_button_similar_hash_algorithm_mean: gtk::RadioButton = builder.object("radio_button_similar_hash_algorithm_mean").unwrap();
        let radio_button_similar_hash_algorithm_vertgradient: gtk::RadioButton = builder.object("radio_button_similar_hash_algorithm_vertgradient").unwrap();
        let radio_button_similar_hash_algorithm_doublegradient: gtk::RadioButton = builder.object("radio_button_similar_hash_algorithm_doublegradient").unwrap();

        let radio_button_similar_hash_size_4: gtk::RadioButton = builder.object("radio_button_similar_hash_size_4").unwrap();
        let radio_button_similar_hash_size_8: gtk::RadioButton = builder.object("radio_button_similar_hash_size_8").unwrap();
        let radio_button_similar_hash_size_16: gtk::RadioButton = builder.object("radio_button_similar_hash_size_16").unwrap();

        let image_preview_similar_images: gtk::Image = builder.object("image_preview_similar_images").unwrap();
        let image_preview_duplicates: gtk::Image = builder.object("image_preview_duplicates").unwrap();

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
            scrolled_window_broken_files,
            tree_view_duplicate_finder,
            tree_view_empty_folder_finder,
            tree_view_empty_files_finder,
            tree_view_temporary_files_finder,
            tree_view_big_files_finder,
            tree_view_similar_images_finder,
            tree_view_zeroed_files_finder,
            tree_view_same_music_finder,
            tree_view_invalid_symlinks,
            tree_view_broken_files,
            entry_similar_images_minimal_size,
            entry_similar_images_maximal_size,
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
            scale_similarity,
            radio_button_hash_type_blake3,
            radio_button_hash_type_crc32,
            radio_button_hash_type_xxh3,
            radio_button_resize_algorithm_lanczos3,
            radio_button_resize_algorithm_nearest,
            radio_button_resize_algorithm_triangle,
            radio_button_resize_algorithm_gaussian,
            radio_button_resize_algorithm_catmullrom,
            radio_button_similar_hash_algorithm_gradient,
            radio_button_similar_hash_algorithm_blockhash,
            radio_button_similar_hash_algorithm_mean,
            radio_button_similar_hash_algorithm_vertgradient,
            radio_button_similar_hash_algorithm_doublegradient,
            radio_button_similar_hash_size_4,
            radio_button_similar_hash_size_8,
            radio_button_similar_hash_size_16,
            image_preview_similar_images,
            entry_duplicate_maximal_size,
            entry_same_music_maximal_size,
            image_preview_duplicates,
        }
    }
}
