use crate::connect_button_hardlink::hardlink_symlink;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use gtk::prelude::*;

pub fn connect_button_symlink(gui_data: &GuiData) {
    let gui_data = gui_data.clone();

    let buttons_symlink = gui_data.bottom_buttons.buttons_symlink.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();

    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();
    let image_preview_duplicates = gui_data.main_notebook.image_preview_duplicates.clone();

    buttons_symlink.connect_clicked(move |_| match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
        NotebookMainEnum::Duplicate => {
            hardlink_symlink(
                tree_view_duplicate_finder.clone(),
                ColumnsDuplicates::Name as i32,
                ColumnsDuplicates::Path as i32,
                ColumnsDuplicates::Color as i32,
                ColumnsDuplicates::ActiveSelectButton as i32,
                false,
                &gui_data,
            );
            image_preview_duplicates.hide();
        }
        NotebookMainEnum::SameMusic => {
            hardlink_symlink(
                tree_view_same_music_finder.clone(),
                ColumnsSameMusic::Name as i32,
                ColumnsSameMusic::Path as i32,
                ColumnsSameMusic::Color as i32,
                ColumnsSameMusic::ActiveSelectButton as i32,
                false,
                &gui_data,
            );
        }
        NotebookMainEnum::SimilarImages => {
            hardlink_symlink(
                tree_view_similar_images_finder.clone(),
                ColumnsSimilarImages::Name as i32,
                ColumnsSimilarImages::Path as i32,
                ColumnsSimilarImages::Color as i32,
                ColumnsSimilarImages::ActiveSelectButton as i32,
                false,
                &gui_data,
            );
            image_preview_similar_images.hide();
        }
        e => panic!("Not existent {:?}", e),
    });
}
