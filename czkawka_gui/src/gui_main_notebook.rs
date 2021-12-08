use gtk::prelude::*;
use gtk::{EventControllerKey, TreeView};

use crate::notebook_enums::NUMBER_OF_NOTEBOOK_MAIN_TABS;

#[derive(Clone)]
pub struct GuiMainNotebook {
    pub notebook_main: gtk::Notebook,

    pub scrolled_window_duplicate_finder: gtk::ScrolledWindow,
    pub scrolled_window_empty_folder_finder: gtk::ScrolledWindow,
    pub scrolled_window_empty_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_temporary_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_big_files_finder: gtk::ScrolledWindow,
    pub scrolled_window_similar_images_finder: gtk::ScrolledWindow,
    pub scrolled_window_similar_videos_finder: gtk::ScrolledWindow,
    pub scrolled_window_same_music_finder: gtk::ScrolledWindow,
    pub scrolled_window_invalid_symlinks: gtk::ScrolledWindow,
    pub scrolled_window_broken_files: gtk::ScrolledWindow,

    pub tree_view_duplicate_finder: gtk::TreeView,
    pub tree_view_empty_folder_finder: gtk::TreeView,
    pub tree_view_empty_files_finder: gtk::TreeView,
    pub tree_view_temporary_files_finder: gtk::TreeView,
    pub tree_view_big_files_finder: gtk::TreeView,
    pub tree_view_similar_images_finder: gtk::TreeView,
    pub tree_view_similar_videos_finder: gtk::TreeView,
    pub tree_view_same_music_finder: gtk::TreeView,
    pub tree_view_invalid_symlinks: gtk::TreeView,
    pub tree_view_broken_files: gtk::TreeView,

    pub evk_tree_view_duplicate_finder: gtk::EventControllerKey, // TODO, in GTK4 this can be changed to e.g. add_controller which is not 100% compatible with this  - https://discourse.gnome.org/t/how-to-convert-code-to-use-eventcontrollerkey/8198/2
    pub evk_tree_view_empty_folder_finder: gtk::EventControllerKey,
    pub evk_tree_view_empty_files_finder: gtk::EventControllerKey,
    pub evk_tree_view_temporary_files_finder: gtk::EventControllerKey,
    pub evk_tree_view_big_files_finder: gtk::EventControllerKey,
    pub evk_tree_view_similar_images_finder: gtk::EventControllerKey,
    pub evk_tree_view_similar_videos_finder: gtk::EventControllerKey,
    pub evk_tree_view_same_music_finder: gtk::EventControllerKey,
    pub evk_tree_view_invalid_symlinks: gtk::EventControllerKey,
    pub evk_tree_view_broken_files: gtk::EventControllerKey,

    pub entry_similar_images_minimal_size: gtk::Entry,
    pub entry_similar_images_maximal_size: gtk::Entry,
    pub entry_similar_videos_minimal_size: gtk::Entry,
    pub entry_similar_videos_maximal_size: gtk::Entry,
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
    pub check_button_music_approximate_comparison: gtk::CheckButton,

    //// Radio Buttons
    // Duplicates
    pub radio_button_duplicates_name: gtk::RadioButton,
    pub radio_button_duplicates_size: gtk::RadioButton,
    pub radio_button_duplicates_hash: gtk::RadioButton,

    pub scale_similarity_similar_images: gtk::Scale,
    pub scale_similarity_similar_videos: gtk::Scale,

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

    pub radio_button_similar_hash_size_8: gtk::RadioButton,
    pub radio_button_similar_hash_size_16: gtk::RadioButton,
    pub radio_button_similar_hash_size_32: gtk::RadioButton,
    pub radio_button_similar_hash_size_64: gtk::RadioButton,

    pub check_button_image_ignore_same_size: gtk::CheckButton,

    pub label_similar_images_minimal_similarity: gtk::Label,

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
        let scrolled_window_similar_videos_finder: gtk::ScrolledWindow = builder.object("scrolled_window_similar_videos_finder").unwrap();
        let scrolled_window_same_music_finder: gtk::ScrolledWindow = builder.object("scrolled_window_same_music_finder").unwrap();
        let scrolled_window_invalid_symlinks: gtk::ScrolledWindow = builder.object("scrolled_window_invalid_symlinks").unwrap();
        let scrolled_window_broken_files: gtk::ScrolledWindow = builder.object("scrolled_window_broken_files").unwrap();

        let tree_view_duplicate_finder: gtk::TreeView = TreeView::new();
        tree_view_duplicate_finder.set_widget_name("PIERD");
        let tree_view_empty_folder_finder: gtk::TreeView = TreeView::new();
        let tree_view_empty_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_temporary_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_big_files_finder: gtk::TreeView = TreeView::new();
        let tree_view_similar_images_finder: gtk::TreeView = TreeView::new();
        let tree_view_similar_videos_finder: gtk::TreeView = TreeView::new();
        let tree_view_same_music_finder: gtk::TreeView = TreeView::new();
        let tree_view_invalid_symlinks: gtk::TreeView = TreeView::new();
        let tree_view_broken_files: gtk::TreeView = TreeView::new();

        let evk_tree_view_duplicate_finder: gtk::EventControllerKey = EventControllerKey::new(&tree_view_duplicate_finder);
        let evk_tree_view_empty_folder_finder: gtk::EventControllerKey = EventControllerKey::new(&tree_view_empty_folder_finder);
        let evk_tree_view_empty_files_finder: gtk::EventControllerKey = EventControllerKey::new(&tree_view_empty_files_finder);
        let evk_tree_view_temporary_files_finder: gtk::EventControllerKey = EventControllerKey::new(&tree_view_temporary_files_finder);
        let evk_tree_view_big_files_finder: gtk::EventControllerKey = EventControllerKey::new(&tree_view_big_files_finder);
        let evk_tree_view_similar_images_finder: gtk::EventControllerKey = EventControllerKey::new(&tree_view_similar_images_finder);
        let evk_tree_view_similar_videos_finder: gtk::EventControllerKey = EventControllerKey::new(&tree_view_similar_videos_finder);
        let evk_tree_view_same_music_finder: gtk::EventControllerKey = EventControllerKey::new(&tree_view_same_music_finder);
        let evk_tree_view_invalid_symlinks: gtk::EventControllerKey = EventControllerKey::new(&tree_view_invalid_symlinks);
        let evk_tree_view_broken_files: gtk::EventControllerKey = EventControllerKey::new(&tree_view_broken_files);

        // TODO GTK 4
        // let evk_tree_view_duplicate_finder: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_duplicate_finder.add_controller(&evk_tree_view_duplicate_finder);
        // let evk_tree_view_empty_folder_finder: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_empty_folder_finder.add_controller(&evk_tree_view_empty_folder_finder);
        // let evk_tree_view_empty_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_empty_files_finder.add_controller(&evk_tree_view_empty_files_finder);
        // let evk_tree_view_temporary_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_temporary_files_finder.add_controller(&evk_tree_view_temporary_files_finder);
        // let evk_tree_view_big_files_finder: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_big_files_finder.add_controller(&evk_tree_view_big_files_finder);
        // let evk_tree_view_similar_images_finder: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_similar_images_finder.add_controller(&evk_tree_view_similar_images_finder);
        // let evk_tree_view_similar_videos_finder: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_similar_videos_finder.add_controller(&evk_tree_view_similar_videos_finder);
        // let evk_tree_view_same_music_finder: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_same_music_finder.add_controller(&evk_tree_view_same_music_finder);
        // let evk_tree_view_invalid_symlinks: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_invalid_symlinks.add_controller(&evk_tree_view_invalid_symlinks);
        // let evk_tree_view_broken_files: gtk4::EventControllerKey = EventControllerKey::new();
        // tree_view_broken_files.add_controller(&evk_tree_view_broken_files);

        let entry_similar_images_minimal_size: gtk::Entry = builder.object("entry_similar_images_minimal_size").unwrap();
        let entry_similar_images_maximal_size: gtk::Entry = builder.object("entry_similar_images_maximal_size").unwrap();
        let entry_similar_videos_minimal_size: gtk::Entry = builder.object("entry_similar_videos_minimal_size").unwrap();
        let entry_similar_videos_maximal_size: gtk::Entry = builder.object("entry_similar_videos_maximal_size").unwrap();
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
        let check_button_music_approximate_comparison: gtk::CheckButton = builder.object("check_button_music_approximate_comparison").unwrap();

        //// Radio Buttons
        let radio_button_duplicates_name: gtk::RadioButton = builder.object("radio_button_duplicates_name").unwrap();
        let radio_button_duplicates_size: gtk::RadioButton = builder.object("radio_button_duplicates_size").unwrap();
        let radio_button_duplicates_hash: gtk::RadioButton = builder.object("radio_button_duplicates_hash").unwrap();

        radio_button_duplicates_name.set_tooltip_text(Some("Finds files which have same name.\n\nThis mode not checking what file contain inside, so be carefully when using it."));
        radio_button_duplicates_size.set_tooltip_text(Some("Finds files which have same size.\n\nThis mode not checking what file contain inside, so be carefully when using it."));
        radio_button_duplicates_hash.set_tooltip_text(Some(
            "Finds files which have the same content.\n\nThis mode hashes file and later compare this hashes to find duplicates.\n\nTool heavily uses cache, so second and further scans of same data should be a lot of faster that first.",
        ));

        let scale_similarity_similar_images: gtk::Scale = builder.object("scale_similarity_similar_images").unwrap();
        let scale_similarity_similar_videos: gtk::Scale = builder.object("scale_similarity_similar_videos").unwrap();

        let radio_button_hash_type_blake3: gtk::RadioButton = builder.object("radio_button_hash_type_blake3").unwrap();
        let radio_button_hash_type_crc32: gtk::RadioButton = builder.object("radio_button_hash_type_crc32").unwrap();
        let radio_button_hash_type_xxh3: gtk::RadioButton = builder.object("radio_button_hash_type_xxh3").unwrap();

        radio_button_hash_type_blake3.set_tooltip_text(Some("Blake3 is cryptographic hash function. It is used as default hash algorithm, because it is very fast."));
        radio_button_hash_type_crc32.set_tooltip_text(Some("CRC32 is simple hash function. It should be faster than Blake3, but probably may have very rarely some collisions."));
        radio_button_hash_type_xxh3.set_tooltip_text(Some("XXH3 is very similar in case of performance and hash quality to Blake3, so such modes can be easily used ."));

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

        let radio_button_similar_hash_size_8: gtk::RadioButton = builder.object("radio_button_similar_hash_size_8").unwrap();
        let radio_button_similar_hash_size_16: gtk::RadioButton = builder.object("radio_button_similar_hash_size_16").unwrap();
        let radio_button_similar_hash_size_32: gtk::RadioButton = builder.object("radio_button_similar_hash_size_32").unwrap();
        let radio_button_similar_hash_size_64: gtk::RadioButton = builder.object("radio_button_similar_hash_size_64").unwrap();

        radio_button_similar_hash_size_8.set_tooltip_text(Some("Default hash size, with very high similarity it produce quite good results and don't save too much data too cache."));
        radio_button_similar_hash_size_16.set_tooltip_text(Some("More precise than 8, so can be used to find very similar pictures, but create bigger cache entries."));
        radio_button_similar_hash_size_32.set_tooltip_text(Some("Hash of this size provide very big similarity which is more than enough for most usages."));
        radio_button_similar_hash_size_64.set_tooltip_text(Some("Paranoid mode, such tool create really big cache files and will catch almost same images."));

        let check_button_image_ignore_same_size: gtk::CheckButton = builder.object("check_button_image_ignore_same_size").unwrap();

        let label_similar_images_minimal_similarity: gtk::Label = builder.object("label_similar_images_minimal_similarity").unwrap();

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
            scrolled_window_similar_videos_finder,
            scrolled_window_same_music_finder,
            scrolled_window_invalid_symlinks,
            scrolled_window_broken_files,
            tree_view_duplicate_finder,
            tree_view_empty_folder_finder,
            tree_view_empty_files_finder,
            tree_view_temporary_files_finder,
            tree_view_big_files_finder,
            tree_view_similar_images_finder,
            tree_view_similar_videos_finder,
            tree_view_same_music_finder,
            tree_view_invalid_symlinks,
            tree_view_broken_files,
            evk_tree_view_duplicate_finder,
            evk_tree_view_empty_folder_finder,
            evk_tree_view_empty_files_finder,
            evk_tree_view_temporary_files_finder,
            evk_tree_view_big_files_finder,
            evk_tree_view_similar_images_finder,
            evk_tree_view_similar_videos_finder,
            evk_tree_view_same_music_finder,
            evk_tree_view_invalid_symlinks,
            evk_tree_view_broken_files,
            entry_similar_images_minimal_size,
            entry_similar_images_maximal_size,
            entry_similar_videos_minimal_size,
            entry_similar_videos_maximal_size,
            entry_duplicate_minimal_size,
            entry_big_files_number,
            entry_same_music_minimal_size,
            check_button_music_title,
            check_button_music_artist,
            check_button_music_album_title,
            check_button_music_album_artist,
            check_button_music_year,
            check_button_music_approximate_comparison,
            radio_button_duplicates_name,
            radio_button_duplicates_size,
            radio_button_duplicates_hash,
            scale_similarity_similar_images,
            scale_similarity_similar_videos,
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
            radio_button_similar_hash_size_8,
            radio_button_similar_hash_size_16,
            radio_button_similar_hash_size_32,
            radio_button_similar_hash_size_64,
            check_button_image_ignore_same_size,
            label_similar_images_minimal_similarity,
            image_preview_similar_images,
            entry_duplicate_maximal_size,
            entry_same_music_maximal_size,
            image_preview_duplicates,
        }
    }

    pub fn get_main_tree_views(&self) -> [TreeView; NUMBER_OF_NOTEBOOK_MAIN_TABS] {
        [
            self.tree_view_duplicate_finder.clone(),
            self.tree_view_empty_folder_finder.clone(),
            self.tree_view_big_files_finder.clone(),
            self.tree_view_empty_files_finder.clone(),
            self.tree_view_temporary_files_finder.clone(),
            self.tree_view_similar_images_finder.clone(),
            self.tree_view_similar_videos_finder.clone(),
            self.tree_view_same_music_finder.clone(),
            self.tree_view_invalid_symlinks.clone(),
            self.tree_view_broken_files.clone(),
        ]
    }
}
