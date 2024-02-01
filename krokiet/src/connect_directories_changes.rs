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
        let non_empty_lines = list_of_files_to_add.lines().filter(|x| !x.is_empty()).collect::<Vec<_>>();
        if non_empty_lines.is_empty() {
            return;
        }
        let app = a.upgrade().unwrap();
        let settings = app.global::<Settings>();

        if included_directories {
            let included_model = settings.get_included_directories_model();
            let mut included_model = included_model.iter().collect::<Vec<_>>();
            included_model.extend(non_empty_lines.iter().map(|x| IncludedDirectoriesModel {
                path: x.to_string_lossy().to_string().into(),
                referenced_folder: false,
                selected_row: false,
            }));
            included_model.sort_by_cached_key(|x| x.text.to_string());
            included_model.dedup();
            settings.set_included_directories_model(ModelRc::new(VecModel::from(included_model)));
        } else {
            let excluded_model = settings.get_excluded_directories_model();
            let mut excluded_model = excluded_model.iter().collect::<Vec<_>>();
            excluded_model.extend(non_empty_lines.iter().map(|x| ExcludedDirectoriesModel {
                path: x.to_string_lossy().to_string().into(),
                selected_row: false,
            }));
            excluded_model.sort_by_cached_key(|x| x.path.to_string());
            excluded_model.dedup();
            settings.set_excluded_directories_model(ModelRc::new(VecModel::from(excluded_model)));
        }
    });
}

fn connect_remove_directories(app: &MainWindow) {
    let a = app.as_weak();
    app.global::<Callabler>().on_remove_item_directories(move |included_directories, current_index| {
        // Nothing selected
        if current_index == -1 {
            return;
        }
        let app = a.upgrade().unwrap();
        let settings = app.global::<Settings>();

        if included_directories {
            let included_model = settings.get_included_directories();
            let model_count = included_model.iter().count();

            if model_count > current_index as usize {
                let mut included_model = included_model.iter().collect::<Vec<_>>();
                included_model.remove(current_index as usize);
                settings.set_included_directories(ModelRc::new(VecModel::from(included_model)));
            }
        } else {
            let excluded_model = settings.get_excluded_directories();
            let model_count = excluded_model.iter().count();

            if model_count > current_index as usize {
                let mut excluded_model = excluded_model.iter().collect::<Vec<_>>();
                excluded_model.remove(current_index as usize);
                settings.set_excluded_directories(ModelRc::new(VecModel::from(excluded_model)));
            }
        }
    });
}

fn connect_add_directories(app: &MainWindow) {
    let a = app.as_weak();
    app.on_folder_choose_requested(move |included_directories| {
        let app = a.upgrade().unwrap();

        let directory = std::env::current_dir().unwrap_or(std::path::PathBuf::from("/"));

        let file_dialog = FileDialog::new().set_directory(directory);

        let Some(folders) = file_dialog.pick_folders() else {
            return;
        };

        let settings = app.global::<Settings>();
        let old_folders = if included_directories {
            settings.get_included_directories_model()
        } else {
            settings.get_excluded_directories_model()
        };

        let mut new_folders = old_folders.iter().map(|x| x.path.to_string()).collect::<Vec<_>>();
        new_folders.extend(folders.iter().map(|x| x.to_string_lossy().to_string()));
        new_folders.sort();
        new_folders.dedup();

        let new_folders_standard_list_view = new_folders
            .iter()
            .map(|x| {
                let mut element = slint::StandardListViewItem::default();
                element.text = x.into();
                element
            })
            .collect::<Vec<_>>();
        let new_folders_model = ModelRc::new(VecModel::from(new_folders_standard_list_view));

        if included_directories {
            settings.set_included_directories_model(new_folders_model);
        } else {
            settings.set_excluded_directories_model(new_folders_model);
        }
    });
}
