extern crate gtk;
use crate::gui_data::GuiData;
use crate::help_functions::*;
use crate::notebook_enums::*;
use gtk::prelude::*;
use gtk::{TreeIter, TreePath};
use std::fs;
use std::path::PathBuf;

pub fn connect_button_move(gui_data: &GuiData) {
    let gui_data = gui_data.clone();

    let buttons_move = gui_data.bottom_buttons.buttons_move.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();

    let tree_view_duplicate_finder = gui_data.main_notebook.tree_view_duplicate_finder.clone();
    let tree_view_same_music_finder = gui_data.main_notebook.tree_view_same_music_finder.clone();
    let tree_view_similar_images_finder = gui_data.main_notebook.tree_view_similar_images_finder.clone();

    let image_preview_similar_images = gui_data.main_notebook.image_preview_similar_images.clone();

    buttons_move.connect_clicked(move |_| match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
        NotebookMainEnum::Duplicate => {
            move_things(
                tree_view_duplicate_finder.clone(),
                ColumnsDuplicates::Name as i32,
                ColumnsDuplicates::Path as i32,
                ColumnsDuplicates::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::SameMusic => {
            move_things(
                tree_view_same_music_finder.clone(),
                ColumnsSameMusic::Name as i32,
                ColumnsSameMusic::Path as i32,
                ColumnsSameMusic::ActiveSelectButton as i32,
                &gui_data,
            );
        }
        NotebookMainEnum::SimilarImages => {
            move_things(
                tree_view_similar_images_finder.clone(),
                ColumnsSimilarImages::Name as i32,
                ColumnsSimilarImages::Path as i32,
                ColumnsSimilarImages::ActiveSelectButton as i32,
                &gui_data,
            );
            image_preview_similar_images.hide();
        }
        e => panic!("Not existent {:?}", e),
    });
}

// TODO create and show folder chooser where user can select path
fn move_things(tree_view: gtk::TreeView, column_file_name: i32, column_path: i32, column_selection: i32, gui_data: &GuiData) {
    let text_view_errors = gui_data.text_view_errors.clone();
    let window_main = gui_data.window_main.clone();

    reset_text_view(&text_view_errors);

    let chooser = gtk::FileChooserDialog::with_buttons(
        Some("Choose folder to which you want to move duplicated files"),
        Some(&window_main),
        gtk::FileChooserAction::SelectFolder,
        &[("Ok", gtk::ResponseType::Ok), ("Close", gtk::ResponseType::Cancel)],
    );
    chooser.set_select_multiple(true);
    chooser.show_all();
    let response_type = chooser.run();
    if response_type == gtk::ResponseType::Ok {
        let folders = chooser.filenames();
        if folders.len() != 1 {
            add_text_to_text_view(&text_view_errors, format!("Only 1 path must be selected to be able to copy there duplicated files, found {:?}", folders).as_str());
        } else {
            // TODO here add entire logic of copying
        }
    }
    chooser.close();
}
