use crate::settings::{collect_settings, SettingsCustom};
use crate::{CurrentTab, GuiState, MainListModel, MainWindow, ProgressToSend};
use chrono::NaiveDateTime;
use crossbeam_channel::{Receiver, Sender};
use czkawka_core::common::{split_path, DEFAULT_THREAD_SIZE};
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
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = SimilarImages::new();
            set_common_settings(&mut finder, &custom_settings);
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
                    insert_data_to_model(&items, ModelRc::new(VecModel::default()), true);
                    for fe in vec_fe {
                        let (directory, file) = split_path(fe.get_path());
                        let data_model = VecModel::from_slice(&[
                            similar_images::get_string_from_similarity(&fe.similarity, hash_size).into(),
                            format_size(fe.size, BINARY).into(),
                            fe.dimensions.clone().into(),
                            file.into(),
                            directory.into(),
                            NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string().into(),
                        ]);

                        insert_data_to_model(&items, data_model, false);
                    }
                }
                app.set_similar_images_model(items.into());
                app.invoke_scan_ended(format!("Found {} similar images files", number_of_empty_files).into());
                app.global::<GuiState>().set_info_text(messages.into());
            })
        })
        .unwrap();
}

fn scan_empty_files(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = EmptyFiles::new();
            set_common_settings(&mut finder, &custom_settings);
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
                        file.into(),
                        directory.into(),
                        NaiveDateTime::from_timestamp_opt(fe.get_modified_date() as i64, 0).unwrap().to_string().into(),
                    ]);

                    insert_data_to_model(&items, data_model, false);
                }
                app.set_empty_files_model(items.into());
                app.invoke_scan_ended(format!("Found {} empty files", number_of_empty_files).into());
                app.global::<GuiState>().set_info_text(messages.into());
            })
        })
        .unwrap();
}

fn scan_empty_folders(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>, stop_receiver: Receiver<()>, custom_settings: SettingsCustom) {
    thread::Builder::new()
        .stack_size(DEFAULT_THREAD_SIZE)
        .spawn(move || {
            let mut finder = EmptyFolder::new();
            set_common_settings(&mut finder, &custom_settings);
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
                        file.into(),
                        directory.into(),
                        NaiveDateTime::from_timestamp_opt(folder_map[&path].modified_date as i64, 0).unwrap().to_string().into(),
                    ]);

                    insert_data_to_model(&items, data_model, false);
                }
                app.set_empty_folder_model(items.into());
                app.invoke_scan_ended(format!("Found {} empty folders", folder_map.len()).into());
                app.global::<GuiState>().set_info_text(messages.into());
            })
        })
        .unwrap();
}

fn insert_data_to_model(items: &Rc<VecModel<MainListModel>>, data_model: ModelRc<SharedString>, header_row: bool) {
    let main = MainListModel {
        checked: false,
        header_row,
        selected_row: false,
        val: ModelRc::new(data_model),
    };
    items.push(main);
}

fn set_common_settings<T>(component: &mut T, custom_settings: &SettingsCustom)
where
    T: CommonData,
{
    component.set_included_directory(custom_settings.included_directories.clone());
    component.set_excluded_directory(custom_settings.excluded_directories.clone());
    component.set_recursive_search(custom_settings.recursive_search);
    component.set_minimal_file_size(custom_settings.minimum_file_size as u64 * 1024);
    component.set_maximal_file_size(custom_settings.maximum_file_size as u64 * 1024);
    component.set_allowed_extensions(custom_settings.allowed_extensions.clone());
    component.set_excluded_items(custom_settings.excluded_items.split(',').map(str::to_string).collect());
    component.set_exclude_other_filesystems(custom_settings.ignore_other_file_systems);
    component.set_use_cache(custom_settings.use_cache);
    component.set_save_also_as_json(custom_settings.save_also_as_json);
}
