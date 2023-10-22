use crate::{split_path, CurrentTab, MainWindow, ProgressToSend};
use chrono::NaiveDateTime;
use crossbeam_channel::Sender;
use czkawka_core::common_dir_traversal::ProgressData;
use czkawka_core::common_tool::CommonData;
use czkawka_core::empty_folder::EmptyFolder;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};
use std::path::PathBuf;
use std::rc::Rc;
use std::thread;

pub fn connect_scan_button(app: &MainWindow, progress_sender: Sender<ProgressData>) {
    let a = app.as_weak();
    app.on_scanned(move |active_tab| {
        let progress_sender = progress_sender.clone();
        let app = a.upgrade().unwrap();
        app.set_scanning(true);
        app.set_progress_datas(ProgressToSend {
            all_progress: 0,
            current_progress: 0,
            step_name: SharedString::from(""),
        });

        let a = app.as_weak();
        match active_tab {
            CurrentTab::EmptyFolders => {
                scan_empty_folders(a, progress_sender);
            }
            _ => panic!(),
        }
    });
}

fn scan_empty_folders(a: Weak<MainWindow>, progress_sender: Sender<ProgressData>) {
    thread::spawn(move || {
        let mut ef = EmptyFolder::new();
        ef.set_included_directory(vec![PathBuf::from("/home/rafal/Desktop")]);
        ef.find_empty_folders(None, Some(&progress_sender));

        ef.get_empty_folder_list();

        let mut vector = ef.get_empty_folder_list().keys().cloned().collect::<Vec<PathBuf>>();

        vector.sort_unstable_by_key(|e| {
            let t = split_path(e.as_path());
            (t.0, t.1)
        });

        a.upgrade_in_event_loop(move |app| {
            let folder_map = ef.get_empty_folder_list();
            let items = Rc::new(VecModel::default());
            for path in vector {
                let (directory, file) = split_path(&path);
                let data_model = VecModel::from_slice(&[
                    SharedString::from(file),
                    SharedString::from(directory),
                    SharedString::from(NaiveDateTime::from_timestamp_opt(folder_map[&path].modified_date as i64, 0).unwrap().to_string()),
                ]);

                items.push((false, false, false, ModelRc::new(data_model)));
            }
            app.set_empty_folder_model(items.into());
            app.set_scanning(false);
        })
    });
}
