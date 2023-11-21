use crate::common::create_vec_model_from_vec_string;
use crate::settings::{collect_settings, SettingsCustom};
use crate::{CurrentTab, GuiState, MainListModel, MainWindow, ProgressToSend};
use chrono::NaiveDateTime;
use crossbeam_channel::{Receiver, Sender};
use czkawka_core::common::split_path;
use czkawka_core::common_dir_traversal::ProgressData;
use czkawka_core::common_tool::CommonData;
use czkawka_core::common_traits::ResultEntry;
use czkawka_core::empty_files::EmptyFiles;
use czkawka_core::empty_folder::EmptyFolder;
use czkawka_core::similar_images;
use czkawka_core::similar_images::SimilarImages;
use humansize::{format_size, BINARY};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};
use std::path::PathBuf;
use std::rc::Rc;
use std::thread;

pub fn connect_scan_button(app: &MainWindow, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>) {
    let a = app.as_weak();
    app.on_scan_starting(move |active_tab| {
        let progress_sender = progress_sender.clone();
        let stop_receiver = stop_receiver.clone();
        let app = a.upgrade().unwrap();

        app.set_progress_datas(ProgressToSend {
            all_progress: 0,
            current_progress: -1,
            step_name: SharedString::from(""),
        });

        let custom_settings = collect_settings(&app);

        let a = app.as_weak();
        match active_tab {
            CurrentTab::EmptyFolders => {
                scan_empty_folders(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::EmptyFiles => {
                scan_empty_files(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::SimilarImages => {
                scan_similar_images(a, progress_sender, stop_receiver, custom_settings);
            }
            CurrentTab::Settings => panic!("Button should be disabled"),
        }
    });
}

// TODO handle referenced folders
fn scan_similar_images(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::spawn(move || {
        let mut finder = SimilarImages::new();
        finder.set_included_directory(custom_settings.included_directories.clone());
        finder.set_excluded_directory(custom_settings.excluded_directories.clone());
        finder.find_similar_images(Some(&stop_receiver), Some(&progress_sender));

        let mut vector = finder.get_similar_images().clone();
        let messages = finder.get_text_messages().create_messages_text();

        for vec_fe in &mut vector {
            vec_fe.sort_unstable_by_key(|e| e.similarity);
        }

        let hash_size = finder.hash_size;

        a.upgrade_in_event_loop(move |app| {
            let number_of_empty_files = vector.len();
            let items = Rc::new(VecModel::default());
            for vec_fe in vector {
                items.push(MainListModel {
                    checked: false,
                    header_row: true,
                    selected_row: false,
                    val: ModelRc::new(VecModel::default()),
                });
                for fe in vec_fe {
                    let (directory, file) = split_path(fe.get_path());
                    let data_model = create_vec_model_from_vec_string(vec![
                        similar_images::get_string_from_similarity(&fe.similarity, hash_size),
                        format_size(fe.size, BINARY),
                        fe.dimensions.clone(),
                        file,
                        directory,
                        NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string(),
                    ]);

                    let main = MainListModel {
                        checked: false,
                        header_row: false,
                        selected_row: false,
                        val: ModelRc::new(data_model),
                    };
                    items.push(main);
                }
            }
            app.set_similar_images_model(items.into());
            app.invoke_scan_ended(format!("Found {} similar images files", number_of_empty_files).into());
            app.global::<GuiState>().set_info_text(messages.into());
        })
    });
}

fn scan_empty_files(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::spawn(move || {
        let mut finder = EmptyFiles::new();
        finder.set_included_directory(custom_settings.included_directories.clone());
        finder.set_excluded_directory(custom_settings.excluded_directories.clone());
        finder.find_empty_files(Some(&stop_receiver), Some(&progress_sender));

        let mut vector = finder.get_empty_files().clone();
        let messages = finder.get_text_messages().create_messages_text();

        vector.sort_unstable_by_key(|e| {
            let t = split_path(e.get_path());
            (t.0, t.1)
        });

        a.upgrade_in_event_loop(move |app| {
            let number_of_empty_files = vector.len();
            let items = Rc::new(VecModel::default());
            for fe in vector {
                let (directory, file) = split_path(fe.get_path());
                let data_model = VecModel::from_slice(&[
                    SharedString::from(file),
                    SharedString::from(directory),
                    SharedString::from(NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string()),
                ]);

                let main = MainListModel {
                    checked: false,
                    header_row: false,
                    selected_row: false,
                    val: ModelRc::new(data_model),
                };
                items.push(main);
            }
            app.set_empty_files_model(items.into());
            app.invoke_scan_ended(format!("Found {} empty files", number_of_empty_files).into());
            app.global::<GuiState>().set_info_text(messages.into());
        })
    });
}

fn scan_empty_folders(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::spawn(move || {
        let mut finder = EmptyFolder::new();
        finder.set_included_directory(custom_settings.included_directories.clone());
        finder.set_excluded_directory(custom_settings.excluded_directories.clone());
        finder.find_empty_folders(Some(&stop_receiver), Some(&progress_sender));

        let mut vector = finder.get_empty_folder_list().keys().cloned().collect::<Vec<PathBuf>>();
        let messages = finder.get_text_messages().create_messages_text();

        vector.sort_unstable_by_key(|e| {
            let t = split_path(e.as_path());
            (t.0, t.1)
        });

        a.upgrade_in_event_loop(move |app| {
            let folder_map = finder.get_empty_folder_list();
            let items = Rc::new(VecModel::default());
            for path in vector {
                let (directory, file) = split_path(&path);
                let data_model = VecModel::from_slice(&[
                    SharedString::from(file),
                    SharedString::from(directory),
                    SharedString::from(NaiveDateTime::from_timestamp_opt(folder_map[&path].modified_date as i64, 0).unwrap().to_string()),
                ]);

                let main = MainListModel {
                    checked: false,
                    header_row: false,
                    selected_row: false,
                    val: ModelRc::new(data_model),
                };
                items.push(main);
            }
            app.set_empty_folder_model(items.into());
            app.invoke_scan_ended(format!("Found {} empty folders", folder_map.len()).into());
            app.global::<GuiState>().set_info_text(messages.into());
        })
    });
}
