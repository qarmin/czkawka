use rfd::FileDialog;
use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::{Callabler, ExcludedDirectoriesModel, IncludedDirectoriesModel, MainWindow, Settings};

pub fn connect_add_remove_directories(app: &MainWindow) {
    connect_add_directories(app);
    connect_remove_directories(app);
    connect_add_manual_directories(app);
}

fn connect_add_manual_directories(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_added_manual_directories(move |included_directories, list_of_files_to_add| {
        let folders = list_of_files_to_add.lines().filter(|x| !x.is_empty()).map(str::to_string).collect::<Vec<_>>();
        if folders.is_empty() {
            return;
        }
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let settings = app.global::<Settings>();

        if included_directories {
            add_included_directories(&settings, &folders);
        } else {
            add_excluded_directories(&settings, &folders);
        }
    });
}

fn connect_remove_directories(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_remove_item_directories(move |included_directories| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let settings = app.global::<Settings>();

        let current_index = if included_directories {
            settings.get_included_directories_model_selected_idx()
        } else {
            settings.get_excluded_directories_model_selected_idx()
        };

        // Nothing selected
        if current_index == -1 {
            return;
        }

        if included_directories {
            let included_model = settings.get_included_directories_model();
            let model_count = included_model.iter().count();

            if model_count > current_index as usize {
                let mut included_model = included_model.iter().collect::<Vec<_>>();
                included_model.remove(current_index as usize);
                if current_index as usize != model_count - 1 {
                    included_model[current_index as usize].selected_row = true;
                    settings.set_included_directories_model_selected_idx(current_index);
                } else {
                    settings.set_included_directories_model_selected_idx(-1);
                }
                settings.set_included_directories_model(ModelRc::new(VecModel::from(included_model)));
            }
        } else {
            let excluded_model = settings.get_excluded_directories_model();
            let model_count = excluded_model.iter().count();

            if model_count > current_index as usize {
                let mut excluded_model = excluded_model.iter().collect::<Vec<_>>();
                excluded_model.remove(current_index as usize);
                if current_index as usize != model_count - 1 {
                    excluded_model[current_index as usize].selected_row = true;
                    settings.set_excluded_directories_model_selected_idx(current_index);
                } else {
                    settings.set_excluded_directories_model_selected_idx(-1);
                }
                settings.set_excluded_directories_model(ModelRc::new(VecModel::from(excluded_model)));
            }
        }
    });
}

fn connect_add_directories(app: &MainWindow) {
    let a = app.as_weak();
    app.on_folder_choose_requested(move |included_directories| {
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let directory = std::env::current_dir().unwrap_or(std::path::PathBuf::from("/"));

        let file_dialog = FileDialog::new().set_directory(directory);

        let Some(folders) = file_dialog.pick_folders() else {
            return;
        };
        let folders = folders.iter().map(|x| x.to_string_lossy().to_string()).collect::<Vec<_>>();

        let settings = app.global::<Settings>();
        if included_directories {
            add_included_directories(&settings, &folders);
        } else {
            add_excluded_directories(&settings, &folders);
        }
    });
}

fn add_included_directories(settings: &Settings, folders: &[String]) {
    let old_folders = settings.get_included_directories_model();
    let old_folders_path = old_folders.iter().map(|x| x.path.to_string()).collect::<Vec<_>>();
    let mut new_folders = old_folders.iter().collect::<Vec<_>>();

    let filtered_folders = folders.iter().filter(|x| !old_folders_path.contains(x)).collect::<Vec<_>>();

    new_folders.iter_mut().for_each(|x| x.selected_row = false);

    new_folders.extend(filtered_folders.iter().map(|path| IncludedDirectoriesModel {
        path: (*path).into(),
        referenced_folder: false,
        selected_row: false,
    }));

    new_folders.sort_by_key(|x| x.path.clone());

    let new_folders_model = ModelRc::new(VecModel::from(new_folders));
    settings.set_included_directories_model(new_folders_model);
}

fn add_excluded_directories(settings: &Settings, folders: &[String]) {
    let old_folders = settings.get_excluded_directories_model();
    let old_folders_path = old_folders.iter().map(|x| x.path.to_string()).collect::<Vec<_>>();
    let mut new_folders = old_folders.iter().collect::<Vec<_>>();

    let filtered_folders = folders.iter().filter(|x| !old_folders_path.contains(x)).collect::<Vec<_>>();

    new_folders.iter_mut().for_each(|x| x.selected_row = false);

    new_folders.extend(filtered_folders.iter().map(|path| ExcludedDirectoriesModel {
        path: (*path).into(),
        selected_row: false,
    }));

    new_folders.sort_by_key(|x| x.path.clone());

    let new_folders_model = ModelRc::new(VecModel::from(new_folders));
    settings.set_excluded_directories_model(new_folders_model);
}
