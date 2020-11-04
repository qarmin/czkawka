extern crate gtk;
use crate::gui_data::GuiData;
use czkawka_core::common_traits::SaveResults;
use gtk::prelude::*;

pub fn connect_button_save(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let buttons_save = gui_data.buttons_save.clone();
    let shared_duplication_state = gui_data.shared_duplication_state.clone();
    let shared_empty_folders_state = gui_data.shared_empty_folders_state.clone();
    let shared_big_files_state = gui_data.shared_big_files_state.clone();
    let shared_temporary_files_state = gui_data.shared_temporary_files_state.clone();
    let shared_empty_files_state = gui_data.shared_empty_files_state.clone();
    let shared_similar_images_state = gui_data.shared_similar_images_state.clone();
    let shared_same_music_state = gui_data.shared_same_music_state.clone();
    let shared_zeroed_files_state = gui_data.shared_zeroed_files_state.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    buttons_save.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            let file_name = "results_duplicates.txt";

            shared_duplication_state.borrow_mut().save_results_to_file(file_name);

            post_save_things(file_name, "duplicate", &gui_data);
        }
        "scrolled_window_main_empty_folder_finder" => {
            let file_name = "results_empty_folder.txt";

            shared_empty_folders_state.borrow_mut().save_results_to_file(file_name);

            post_save_things(file_name, "empty_folder", &gui_data);
        }
        "scrolled_window_main_empty_files_finder" => {
            let file_name = "results_empty_files.txt";

            shared_empty_files_state.borrow_mut().save_results_to_file(file_name);

            post_save_things(file_name, "empty_file", &gui_data);
        }
        "scrolled_window_main_temporary_files_finder" => {
            let file_name = "results_temporary_files.txt";

            shared_temporary_files_state.borrow_mut().save_results_to_file(file_name);

            post_save_things(file_name, "temporary_file", &gui_data);
        }
        "notebook_big_main_file_finder" => {
            let file_name = "results_big_files.txt";

            shared_big_files_state.borrow_mut().save_results_to_file(file_name);

            post_save_things(file_name, "big_file", &gui_data);
        }
        "notebook_main_similar_images_finder_label" => {
            let file_name = "results_similar_images.txt";

            shared_similar_images_state.borrow_mut().save_results_to_file(file_name);

            post_save_things(file_name, "similar_images", &gui_data);
        }
        "notebook_main_zeroed_files_finder" => {
            let file_name = "results_zeroed_files.txt";

            shared_zeroed_files_state.borrow_mut().save_results_to_file(file_name);

            post_save_things(file_name, "zeroed_files", &gui_data);
        }
        "notebook_main_same_music_finder" => {
            let file_name = "results_same_music.txt";

            shared_same_music_state.borrow_mut().save_results_to_file(file_name);

            post_save_things(file_name, "same_music", &gui_data);
        }
        e => panic!("Not existent {}", e),
    });
}
fn post_save_things(file_name: &str, type_of_tab: &str, gui_data: &GuiData) {
    let entry_info = gui_data.entry_info.clone();
    let buttons_save = gui_data.buttons_save.clone();
    let shared_buttons = gui_data.shared_buttons.clone();

    entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
    // Set state
    {
        buttons_save.hide();
        *shared_buttons.borrow_mut().get_mut(type_of_tab).unwrap().get_mut("save").unwrap() = false;
    }
}
