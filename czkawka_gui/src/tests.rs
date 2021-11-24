use crate::help_functions::{get_notebook_enum_from_tree_view, NOTEBOOKS_INFOS};
use crate::notebook_enums::{to_notebook_main_enum, NUMBER_OF_NOTEBOOK_MAIN_TABS};
use crate::GuiData;

pub fn validate_notebook_data(gui_data: &GuiData) {
    // Test treeviews names, each treeview should have set name same as variable name
    let tree_view_arr: [&gtk::TreeView; NUMBER_OF_NOTEBOOK_MAIN_TABS] = [
        &gui_data.main_notebook.tree_view_duplicate_finder,
        &gui_data.main_notebook.tree_view_similar_videos_finder,
        &gui_data.main_notebook.tree_view_temporary_files_finder,
        &gui_data.main_notebook.tree_view_big_files_finder,
        &gui_data.main_notebook.tree_view_empty_files_finder,
        &gui_data.main_notebook.tree_view_broken_files,
        &gui_data.main_notebook.tree_view_empty_folder_finder,
        &gui_data.main_notebook.tree_view_same_music_finder,
        &gui_data.main_notebook.tree_view_similar_images_finder,
        &gui_data.main_notebook.tree_view_invalid_symlinks,
    ];
    for (_i, item) in tree_view_arr.iter().enumerate() {
        // println!("Checking {} element", i);

        get_notebook_enum_from_tree_view(item);
    }

    // This test main info about notebooks
    // Should have same order as notebook enum types
    for (i, item) in NOTEBOOKS_INFOS.iter().enumerate() {
        let en = to_notebook_main_enum(i as u32);
        assert_eq!(item.notebook_type, en);
    }

    // Tests if data returned from array get_notebook_enum_from_tree_view are in right
    for (i, item) in gui_data.main_notebook.get_main_tree_views().iter().enumerate() {
        let nb_en = get_notebook_enum_from_tree_view(item);
        assert_eq!(to_notebook_main_enum(i as u32), nb_en);
    }
}
