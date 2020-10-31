extern crate gtk;
use crate::gui_data::GuiData;
use czkawka_core::common_traits::SaveResults;
use gtk::prelude::*;

pub fn connect_button_save(gui_data: &GuiData) {
    let shared_buttons = gui_data.shared_buttons.clone();
    let buttons_save_clone = gui_data.buttons_save.clone();
    let buttons_save = gui_data.buttons_save.clone();
    let entry_info = gui_data.entry_info.clone();
    let shared_duplication_state = gui_data.shared_duplication_state.clone();
    let shared_empty_folders_state = gui_data.shared_empty_folders_state.clone();
    let shared_big_files_state = gui_data.shared_big_files_state.clone();
    let shared_temporary_files_state = gui_data.shared_temporary_files_state.clone();
    let shared_empty_files_state = gui_data.shared_empty_files_state.clone();
    let shared_similar_images_state = gui_data.shared_similar_images_state.clone();
    let shared_zeroed_files_state = gui_data.shared_zeroed_files_state.clone();
    let notebook_main_children_names = gui_data.notebook_main_children_names.clone();
    let notebook_main = gui_data.notebook_main.clone();
    buttons_save_clone.connect_clicked(move |_| match notebook_main_children_names.get(notebook_main.get_current_page().unwrap() as usize).unwrap().as_str() {
        "notebook_main_duplicate_finder_label" => {
            let file_name = "results_duplicates.txt";

            let mut df = shared_duplication_state.borrow_mut();
            df.save_results_to_file(file_name);

            entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
            // Set state
            {
                buttons_save.hide();
                *shared_buttons.borrow_mut().get_mut("duplicate").unwrap().get_mut("save").unwrap() = false;
            }
        }
        "scrolled_window_main_empty_folder_finder" => {
            let file_name = "results_empty_folder.txt";

            let mut ef = shared_empty_folders_state.borrow_mut();
            ef.save_results_to_file(file_name);

            entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
            // Set state
            {
                buttons_save.hide();
                *shared_buttons.borrow_mut().get_mut("empty_folder").unwrap().get_mut("save").unwrap() = false;
            }
        }
        "scrolled_window_main_empty_files_finder" => {
            let file_name = "results_empty_files.txt";

            let mut df = shared_empty_files_state.borrow_mut();
            df.save_results_to_file(file_name);

            entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
            // Set state
            {
                buttons_save.hide();
                *shared_buttons.borrow_mut().get_mut("empty_file").unwrap().get_mut("save").unwrap() = false;
            }
        }
        "scrolled_window_main_temporary_files_finder" => {
            let file_name = "results_temporary_files.txt";

            let mut df = shared_temporary_files_state.borrow_mut();
            df.save_results_to_file(file_name);

            entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
            // Set state
            {
                buttons_save.hide();
                *shared_buttons.borrow_mut().get_mut("temporary_file").unwrap().get_mut("save").unwrap() = false;
            }
        }
        "notebook_big_main_file_finder" => {
            let file_name = "results_big_files.txt";

            let mut df = shared_big_files_state.borrow_mut();
            df.save_results_to_file(file_name);

            entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
            // Set state
            {
                buttons_save.hide();
                *shared_buttons.borrow_mut().get_mut("big_file").unwrap().get_mut("save").unwrap() = false;
            }
        }
        "notebook_main_similar_images_finder_label" => {
            let file_name = "results_similar_images.txt";

            let mut df = shared_similar_images_state.borrow_mut();
            df.save_results_to_file(file_name);

            entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
            // Set state
            {
                buttons_save.hide();
                *shared_buttons.borrow_mut().get_mut("similar_images").unwrap().get_mut("save").unwrap() = false;
            }
        }
        "notebook_main_zeroed_files_finder_label" => {
            let file_name = "results_zeroed_files.txt";

            let mut zf = shared_zeroed_files_state.borrow_mut();
            zf.save_results_to_file(file_name);

            entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
            // Set state
            {
                buttons_save.hide();
                *shared_buttons.borrow_mut().get_mut("zeroed_files").unwrap().get_mut("save").unwrap() = false;
            }
        }
        e => panic!("Not existent {}", e),
    });
}
