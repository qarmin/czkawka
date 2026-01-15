use rfd::FileDialog;
use slint::{ComponentHandle, Model, ModelRc, VecModel};

use crate::{Callabler, ExcludedPathsModel, IncludedPathsModel, MainWindow, Settings};

pub(crate) fn connect_add_remove_directories(app: &MainWindow) {
    connect_add_directories(app);
    connect_add_files(app);
    connect_remove_directories(app);
    connect_add_manual_directories(app);
}

fn connect_add_manual_directories(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_added_manual_paths(move |included_paths, list_of_files_to_add| {
        let folders = list_of_files_to_add.lines().filter(|x| !x.is_empty()).map(str::to_string).collect::<Vec<_>>();
        if folders.is_empty() {
            return;
        }
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let settings = app.global::<Settings>();

        if included_paths {
            add_included_paths(&settings, &folders);
        } else {
            add_excluded_paths(&settings, &folders);
        }
    });
}

fn filter_model<T: Clone>(model: &ModelRc<T>, index_to_remove: i32) -> Vec<T> {
    model.iter().enumerate().filter(|(idx, _)| *idx as i32 != index_to_remove).map(|(_, item)| item).collect()
}

fn connect_remove_directories(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_remove_item_paths(move |included_paths, index_to_remove| {
        let app = a.upgrade().expect("Failed to upgrade app :(");
        let settings = app.global::<Settings>();

        if included_paths {
            let included_model = settings.get_included_paths_model();
            let new_model = filter_model(&included_model, index_to_remove);

            assert_eq!(included_model.iter().count(), new_model.len() + 1, "Removing item should reduce model size by 1");
            settings.set_included_paths_model(ModelRc::new(VecModel::from(new_model)));
        } else {
            let excluded_model = settings.get_excluded_paths_model();
            let new_model = filter_model(&excluded_model, index_to_remove);

            assert_eq!(excluded_model.iter().count(), new_model.len() + 1, "Removing item should reduce model size by 1");
            settings.set_excluded_paths_model(ModelRc::new(VecModel::from(new_model)));
        }
    });
}

fn connect_add_directories(app: &MainWindow) {
    let a = app.as_weak();
    app.on_folder_choose_requested(move |included_paths| {
        println!("Folder choose requested");
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let directory = std::env::current_dir().unwrap_or(std::path::PathBuf::from("/"));

        let file_dialog = FileDialog::new().set_directory(directory);

        let Some(folders) = file_dialog.pick_folders() else {
            return;
        };
        let folders = folders.iter().map(|x| x.to_string_lossy().to_string()).collect::<Vec<_>>();

        let settings = app.global::<Settings>();
        if included_paths {
            add_included_paths(&settings, &folders);
        } else {
            add_excluded_paths(&settings, &folders);
        }
    });
}

fn connect_add_files(app: &MainWindow) {
    let a = app.as_weak();
    app.on_file_choose_requested(move |included_paths| {
        println!("File choose requested");
        let app = a.upgrade().expect("Failed to upgrade app :(");

        let directory = std::env::current_dir().unwrap_or(std::path::PathBuf::from("/"));

        let file_dialog = FileDialog::new().set_directory(directory);

        let Some(files) = file_dialog.pick_files() else {
            return;
        };
        let files = files.iter().map(|x| x.to_string_lossy().to_string()).collect::<Vec<_>>();

        let settings = app.global::<Settings>();
        if included_paths {
            add_included_paths(&settings, &files);
        } else {
            add_excluded_paths(&settings, &files);
        }
    });
}

fn add_included_paths(settings: &Settings, folders: &[String]) {
    let old_folders = settings.get_included_paths_model();
    let old_folders_path = old_folders.iter().map(|x| x.path.to_string()).collect::<Vec<_>>();
    let mut new_folders = old_folders.iter().collect::<Vec<_>>();

    let filtered_folders = folders.iter().filter(|x| !old_folders_path.contains(x)).collect::<Vec<_>>();

    for x in &mut new_folders {
        x.selected_row = false;
    }

    new_folders.extend(filtered_folders.iter().map(|path| IncludedPathsModel {
        path: (*path).into(),
        referenced_path: false,
        selected_row: false,
    }));

    new_folders.sort_by_key(|x| x.path.clone());

    let new_folders_model = ModelRc::new(VecModel::from(new_folders));
    settings.set_included_paths_model(new_folders_model);
}

fn add_excluded_paths(settings: &Settings, folders: &[String]) {
    let old_folders = settings.get_excluded_paths_model();
    let old_folders_path = old_folders.iter().map(|x| x.path.to_string()).collect::<Vec<_>>();
    let mut new_folders = old_folders.iter().collect::<Vec<_>>();

    let filtered_folders = folders.iter().filter(|x| !old_folders_path.contains(x)).collect::<Vec<_>>();

    for x in &mut new_folders {
        x.selected_row = false;
    }

    new_folders.extend(filtered_folders.iter().map(|path| ExcludedPathsModel {
        path: (*path).into(),
        selected_row: false,
    }));

    new_folders.sort_by_key(|x| x.path.clone());

    let new_folders_model = ModelRc::new(VecModel::from(new_folders));
    settings.set_excluded_paths_model(new_folders_model);
}
