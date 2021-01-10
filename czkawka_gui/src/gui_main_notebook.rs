use gtk::prelude::*;
use gtk::TreeView;

#[derive(Clone)]
pub struct GUIMainNotebook {
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
}

impl GUIMainNotebook {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
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

        Self {
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
        }
    }
}
