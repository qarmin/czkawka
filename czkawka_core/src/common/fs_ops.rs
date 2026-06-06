use std::io::Error;
use std::path::Path;
use std::{fs, io};

use crate::flc;

const MAX_SYMLINK_HARDLINK_ATTEMPTS: u8 = 5;

#[cfg(all(feature = "xdg_portal_trash", target_os = "linux"))]
thread_local! {
    static TOKIO_RT: std::cell::RefCell<Option<Result<tokio::runtime::Runtime, String>>> = const { std::cell::RefCell::new(None) };
}

#[cfg(all(feature = "xdg_portal_trash", target_os = "linux"))]
fn with_runtime<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce(&tokio::runtime::Runtime) -> Result<R, String>,
{
    TOKIO_RT.with(|cell| {
        let mut opt = cell.borrow_mut();

        if opt.is_none() {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| format!("Failed to build Tokio runtime: {e}"));

            *opt = Some(rt);
        }

        match opt.as_ref().expect("Tokio runtime is initialized before") {
            Ok(rt) => f(rt),
            Err(e) => Err(e.clone()),
        }
    })
}

pub fn check_if_folder_contains_only_empty_folders<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref();
    if !path.is_dir() {
        return Err(flc!("core_not_directory_remove", path = path.to_string_lossy()));
    }

    let mut entries_to_check = Vec::new();
    let Ok(initial_entry) = path.read_dir() else {
        return Err(flc!("core_cannot_read_directory", path = path.to_string_lossy()));
    };
    for entry in initial_entry {
        if let Ok(entry) = entry {
            entries_to_check.push(entry);
        } else {
            return Err(flc!("core_cannot_read_entry_from_directory", path = path.to_string_lossy()));
        }
    }
    loop {
        let Some(entry) = entries_to_check.pop() else {
            break;
        };
        let Some(file_type) = entry.file_type().ok() else {
            return Err(flc!(
                "core_unknown_directory_entry",
                entry = entry.path().to_string_lossy().to_string(),
                path = path.to_string_lossy()
            ));
        };

        if !file_type.is_dir() {
            return Err(flc!(
                "core_folder_contains_file_inside",
                entry = entry.path().to_string_lossy().to_string(),
                folder = path.to_string_lossy()
            ));
        }
        let Ok(internal_read_dir) = entry.path().read_dir() else {
            return Err(flc!("core_cannot_read_directory", path = path.to_string_lossy().to_string()));
        };
        for internal_elements in internal_read_dir {
            if let Ok(internal_element) = internal_elements {
                entries_to_check.push(internal_element);
            } else {
                return Err(flc!("core_cannot_read_entry_from_directory", path = path.to_string_lossy().to_string()));
            }
        }
    }

    Ok(())
}

/// A wrapper around `trash::delete`. Note that for platforms that do not have native trash support
/// (Android, iOS), this function will always return an [`Error`]. When the `xdg_portal_trash` feature is
/// enabled, the portal-based implementation will only be used on Linux; on other desktop OSes the
/// regular `trash::delete` fallback will be used instead.
fn trash_delete<P: AsRef<Path>>(path: P) -> Result<(), String> {
    let path = path.as_ref();

    #[cfg(not(any(target_os = "android", target_os = "ios", all(feature = "xdg_portal_trash", target_os = "linux"))))]
    {
        trash::delete(path).map_err(|err| err.to_string())
    }

    #[cfg(all(feature = "xdg_portal_trash", target_os = "linux"))]
    {
        use std::os::fd::AsFd;
        let file = std::fs::OpenOptions::new().write(true).read(true).open(path).map_err(|err| err.to_string())?;

        with_runtime(|rt| rt.block_on(async move { ashpd::desktop::trash::trash_file(&file.as_fd()).await.map_err(|e| e.to_string()) }))?;

        Ok(())
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let _path = path;
        Err("trash is not supported on this platform".to_string())
    }
}

/// Remove the folder if it only contains empty folders/is empty. If `remove_to_trash` is set, the folder
/// will instead be sent to the system's recycle bin/trash equivalent rather than being deleted.
///
/// Note: if used on Android or iOS platforms, ensure `remove_to_trash` is false, as trash is not supported
/// and will always return an [`Error`].
pub fn remove_folder_if_contains_only_empty_folders<P: AsRef<Path>>(path: P, remove_to_trash: bool) -> Result<(), String> {
    check_if_folder_contains_only_empty_folders(&path)?;

    let path = path.as_ref();

    if remove_to_trash {
        trash_delete(path).map_err(|e| format!("Cannot move folder \"{}\" to trash, reason {e}", path.to_string_lossy()))
    } else {
        fs::remove_dir_all(path).map_err(|e| format!("Cannot remove directory \"{}\", reason {e}", path.to_string_lossy()))
    }
}

/// Remove a single file. If `remove_to_trash` is set, the folder will instead be sent to the system's
/// recycle bin/trash equivalent rather than being deleted.
///
/// Note: if used on Android or iOS platforms, ensure `remove_to_trash` is false, as trash is not supported
/// and will always return an [`Error`].
pub fn remove_single_file<P: AsRef<Path>>(full_path: P, remove_to_trash: bool) -> Result<(), String> {
    if remove_to_trash {
        if let Err(e) = trash_delete(&full_path) {
            return Err(flc!("core_error_moving_to_trash", file = full_path.as_ref().to_string_lossy().to_string(), error = e));
        }
    } else {
        if let Err(e) = fs::remove_file(&full_path) {
            return Err(flc!("core_error_removing", file = full_path.as_ref().to_string_lossy().to_string(), error = e.to_string()));
        }
    }
    Ok(())
}

/// Remove a single folder recursively. If `remove_to_trash` is set, the folder will instead be sent to the system's
/// recycle bin/trash equivalent rather than being deleted.
///
/// Note: if used on Android or iOS platforms, ensure `remove_to_trash` is false, as trash is not supported
/// and will always return an [`Error`].
pub fn remove_single_folder(full_path: &str, remove_to_trash: bool) -> Result<(), String> {
    if remove_to_trash {
        if let Err(e) = trash_delete(full_path) {
            return Err(flc!("core_error_moving_to_trash", file = full_path, error = e));
        }
    } else {
        if let Err(e) = fs::remove_dir_all(full_path) {
            return Err(flc!("core_error_removing", file = full_path, error = e.to_string()));
        }
    }
    Ok(())
}

// Function to create hardlink, when destination exists
// This is always true in this app, because creating hardlink, to newly created file is pointless
pub fn make_hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let dst_dir = dst.parent().ok_or_else(|| Error::other("No parent"))?;
    let mut temp;
    let mut attempts = MAX_SYMLINK_HARDLINK_ATTEMPTS;
    loop {
        temp = dst_dir.join(format!("{}.czkawka_tmp", rand::random::<u128>()));
        if !temp.exists() {
            break;
        }
        attempts -= 1;
        if attempts == 0 {
            return Err(Error::other("Cannot choose temporary file for hardlink creation"));
        }
    }
    fs::rename(dst, temp.as_path())?;
    match fs::hard_link(src, dst) {
        Ok(()) => {
            fs::remove_file(&temp)?;
            Ok(())
        }
        Err(e) => {
            let _ = fs::rename(&temp, dst);
            Err(e)
        }
    }
}

#[cfg(any(target_family = "unix", target_family = "windows"))]
pub fn make_file_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let dst_dir = dst.parent().ok_or_else(|| Error::other("No parent"))?;
    let mut temp;
    let mut attempts = MAX_SYMLINK_HARDLINK_ATTEMPTS;
    loop {
        temp = dst_dir.join(format!("{}.czkawka_tmp", rand::random::<u128>()));
        if !temp.exists() {
            break;
        }
        attempts -= 1;
        if attempts == 0 {
            return Err(Error::other("Cannot choose temporary file for symlink creation"));
        }
    }
    fs::rename(dst, temp.as_path())?;
    let result: Result<_, _>;
    #[cfg(target_family = "unix")]
    {
        result = std::os::unix::fs::symlink(src, dst);
    }
    #[cfg(target_family = "windows")]
    {
        result = std::os::windows::fs::symlink_file(src, dst);
    }
    match result {
        Ok(()) => {
            fs::remove_file(&temp)?;
            Ok(())
        }
        Err(e) => {
            let _ = fs::rename(&temp, dst);
            Err(e)
        }
    }
}

#[cfg(not(any(target_family = "unix", target_family = "windows")))]
pub fn make_file_symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    Err(Error::new(io::ErrorKind::Other, "Soft links are not supported on this platform"))
}

pub fn debug_save_file(path: &str, data: &str) {
    use std::io::Write;
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = writeln!(f, "{data}");
    }
}
