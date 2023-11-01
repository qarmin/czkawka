use crate::{MainWindow, Settings};
use rfd::FileDialog;
use slint::{ComponentHandle, Model, ModelRc, VecModel};

pub fn connect_add_directories(app: &MainWindow) {
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
            settings.get_included_directories()
        } else {
            settings.get_excluded_directories()
        };

        let mut new_folders = old_folders.iter().map(|x| x.text.to_string()).collect::<Vec<_>>();
        new_folders.extend(folders.iter().map(|x| x.to_string_lossy().to_string()));
        new_folders.sort();
        new_folders.dedup();

        let new_folders_standard_list_view = new_folders
            .iter()
            .map(|x| {
                let mut element = slint::StandardListViewItem::default();
                element.text = slint::SharedString::from(x.to_string());
                element
            })
            .collect::<Vec<_>>();
        let new_folders_model = ModelRc::new(VecModel::from(new_folders_standard_list_view));

        if included_directories {
            settings.set_included_directories(new_folders_model);
        } else {
            settings.set_excluded_directories(new_folders_model);
        }
    });
}
