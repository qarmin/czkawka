use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

const HOMEBREW_BIN_DIRS: &[&str] = &["/opt/homebrew/bin", "/usr/local/bin"];

/// Apps started from Finder get only `/usr/bin:/bin:/usr/sbin:/sbin` (`LSEnvironment` cannot override `PATH`),
/// so ffmpeg/ffprobe installed by Homebrew would not be found.
/// Must be called before any other thread is spawned.
pub(crate) fn add_homebrew_to_path() {
    if let Some(path) = path_with_dirs_appended(env::var_os("PATH"), HOMEBREW_BIN_DIRS) {
        // SAFETY: called at the start of main, before any other thread exists
        unsafe {
            env::set_var("PATH", path);
        }
    }
}

fn path_with_dirs_appended(current: Option<OsString>, dirs: &[&str]) -> Option<OsString> {
    let mut entries: Vec<PathBuf> = current.map(|path| env::split_paths(&path).collect()).unwrap_or_default();
    let missing: Vec<PathBuf> = dirs.iter().map(PathBuf::from).filter(|dir| !entries.contains(dir)).collect();
    if missing.is_empty() {
        return None;
    }
    entries.extend(missing);
    Some(env::join_paths(entries).expect("entries come from split_paths or constants, so none contains the separator"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_only_missing_dirs() {
        let result = path_with_dirs_appended(Some("/usr/bin:/usr/local/bin".into()), HOMEBREW_BIN_DIRS);
        assert_eq!(result, Some("/usr/bin:/usr/local/bin:/opt/homebrew/bin".into()));
    }

    #[test]
    fn nothing_to_do_when_all_present() {
        let result = path_with_dirs_appended(Some("/opt/homebrew/bin:/usr/bin:/usr/local/bin".into()), HOMEBREW_BIN_DIRS);
        assert_eq!(result, None);
    }

    #[test]
    fn missing_path_variable() {
        let result = path_with_dirs_appended(None, HOMEBREW_BIN_DIRS);
        assert_eq!(result, Some("/opt/homebrew/bin:/usr/local/bin".into()));
    }
}
