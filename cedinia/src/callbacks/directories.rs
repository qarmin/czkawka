use std::path::PathBuf;
use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

use crate::volumes::{detect_storage_volumes, refresh_volumes_flags};
use crate::{AppState, DirectoryEntry, MainWindow, VolumeEntry};

pub(crate) fn wire_directories(window: &MainWindow, included_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>, excluded_dirs: Rc<std::cell::RefCell<Vec<PathBuf>>>) {
    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_pick_include_dir(move || {
            #[cfg(not(target_os = "android"))]
            {
                let win = weak.unwrap();
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    inc.borrow_mut().push(path);
                    win.set_directories_model(build_dir_model(&inc.borrow(), &exc.borrow()));
                }
            }
            #[cfg(target_os = "android")]
            {
                let _ = (&weak, &inc, &exc);
                crate::file_picker_android::launch_pick_directory(true);
            }
        });
    }
    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_pick_exclude_dir(move || {
            #[cfg(not(target_os = "android"))]
            {
                let win = weak.unwrap();
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    exc.borrow_mut().push(path);
                    win.set_directories_model(build_dir_model(&inc.borrow(), &exc.borrow()));
                }
            }
            #[cfg(target_os = "android")]
            {
                let _ = (&weak, &inc, &exc);
                crate::file_picker_android::launch_pick_directory(false);
            }
        });
    }
    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_add_include_dir(move |path| {
            let win = weak.unwrap();
            inc.borrow_mut().push(PathBuf::from(path.as_str()));
            win.set_directories_model(build_dir_model(&inc.borrow(), &exc.borrow()));
            refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
        });
    }
    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_remove_include_dir(move |path| {
            let win = weak.unwrap();
            inc.borrow_mut().retain(|p| p.to_string_lossy() != path.as_str());
            win.set_directories_model(build_dir_model(&inc.borrow(), &exc.borrow()));
            refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
        });
    }
    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_add_exclude_dir(move |path| {
            let win = weak.unwrap();
            exc.borrow_mut().push(PathBuf::from(path.as_str()));
            win.set_directories_model(build_dir_model(&inc.borrow(), &exc.borrow()));
            refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
        });
    }
    {
        let weak = window.as_weak();
        let inc = included_dirs.clone();
        let exc = excluded_dirs.clone();
        window.global::<AppState>().on_remove_exclude_dir(move |path| {
            let win = weak.unwrap();
            exc.borrow_mut().retain(|p| p.to_string_lossy() != path.as_str());
            win.set_directories_model(build_dir_model(&inc.borrow(), &exc.borrow()));
            refresh_volumes_flags(&win, &inc.borrow(), &exc.borrow());
        });
    }
    {
        let weak = window.as_weak();
        let inc = included_dirs;
        let exc = excluded_dirs;
        window.global::<AppState>().on_list_storage_volumes(move || {
            let raw = detect_storage_volumes();
            let inc_set: Vec<String> = inc.borrow().iter().map(|p| p.to_string_lossy().to_string()).collect();
            let exc_set: Vec<String> = exc.borrow().iter().map(|p| p.to_string_lossy().to_string()).collect();
            let volumes: Vec<VolumeEntry> = raw
                .into_iter()
                .map(|mut v| {
                    let path = v.path.to_string();
                    v.is_included = inc_set.contains(&path);
                    v.is_excluded = exc_set.contains(&path);
                    v
                })
                .collect();
            if let Some(win) = weak.upgrade() {
                win.global::<AppState>().set_storage_volumes(ModelRc::new(VecModel::from(volumes)));
            }
        });
    }
}

pub(crate) fn build_dir_model(included: &[PathBuf], excluded: &[PathBuf]) -> ModelRc<DirectoryEntry> {
    let mut entries: Vec<DirectoryEntry> = included
        .iter()
        .map(|p| DirectoryEntry {
            path: SharedString::from(p.to_string_lossy().to_string()),
            is_included: true,
        })
        .collect();
    for p in excluded {
        entries.push(DirectoryEntry {
            path: SharedString::from(p.to_string_lossy().to_string()),
            is_included: false,
        });
    }
    ModelRc::new(VecModel::from(entries))
}
