use crate::gui_data::GuiData;
use crate::notebook_enums::*;
use czkawka_core::common_traits::SaveResults;
use gtk::prelude::*;

pub fn connect_button_save(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let buttons_save = gui_data.bottom_buttons.buttons_save.clone();
    let shared_duplication_state = gui_data.shared_duplication_state.clone();
    let shared_empty_folders_state = gui_data.shared_empty_folders_state.clone();
    let shared_big_files_state = gui_data.shared_big_files_state.clone();
    let shared_temporary_files_state = gui_data.shared_temporary_files_state.clone();
    let shared_empty_files_state = gui_data.shared_empty_files_state.clone();
    let shared_similar_images_state = gui_data.shared_similar_images_state.clone();
    let shared_same_music_state = gui_data.shared_same_music_state.clone();
    let shared_zeroed_files_state = gui_data.shared_zeroed_files_state.clone();
    let shared_same_invalid_symlinks = gui_data.shared_same_invalid_symlinks.clone();
    let shared_broken_files_state = gui_data.shared_broken_files_state.clone();
    let notebook_main = gui_data.main_notebook.notebook_main.clone();
    buttons_save.connect_clicked(move |_| {
        let file_name;

        match to_notebook_main_enum(notebook_main.current_page().unwrap()) {
            NotebookMainEnum::Duplicate => {
                file_name = "results_duplicates.txt";

                shared_duplication_state.borrow_mut().save_results_to_file(file_name);
            }
            NotebookMainEnum::EmptyDirectories => {
                file_name = "results_empty_folder.txt";

                shared_empty_folders_state.borrow_mut().save_results_to_file(file_name);
            }
            NotebookMainEnum::EmptyFiles => {
                file_name = "results_empty_files.txt";

                shared_empty_files_state.borrow_mut().save_results_to_file(file_name);
            }
            NotebookMainEnum::Temporary => {
                file_name = "results_temporary_files.txt";

                shared_temporary_files_state.borrow_mut().save_results_to_file(file_name);
            }
            NotebookMainEnum::BigFiles => {
                file_name = "results_big_files.txt";

                shared_big_files_state.borrow_mut().save_results_to_file(file_name);
            }
            NotebookMainEnum::SimilarImages => {
                file_name = "results_similar_images.txt";

                shared_similar_images_state.borrow_mut().save_results_to_file(file_name);
            }
            NotebookMainEnum::Zeroed => {
                file_name = "results_zeroed_files.txt";

                shared_zeroed_files_state.borrow_mut().save_results_to_file(file_name);
            }
            NotebookMainEnum::SameMusic => {
                file_name = "results_same_music.txt";

                shared_same_music_state.borrow_mut().save_results_to_file(file_name);
            }
            NotebookMainEnum::Symlinks => {
                file_name = "results_invalid_symlinks.txt";

                shared_same_invalid_symlinks.borrow_mut().save_results_to_file(file_name);
            }
            NotebookMainEnum::BrokenFiles => {
                file_name = "results_broken_files.txt";

                shared_broken_files_state.borrow_mut().save_results_to_file(file_name);
            }
        }
        post_save_things(file_name, &to_notebook_main_enum(notebook_main.current_page().unwrap()), &gui_data);
    });
}
fn post_save_things(file_name: &str, type_of_tab: &NotebookMainEnum, gui_data: &GuiData) {
    let entry_info = gui_data.entry_info.clone();
    let buttons_save = gui_data.bottom_buttons.buttons_save.clone();
    let shared_buttons = gui_data.shared_buttons.clone();

    entry_info.set_text(format!("Saved results to file {}", file_name).as_str());
    // Set state
    {
        buttons_save.hide();
        *shared_buttons.borrow_mut().get_mut(type_of_tab).unwrap().get_mut("save").unwrap() = false;
    }
}
