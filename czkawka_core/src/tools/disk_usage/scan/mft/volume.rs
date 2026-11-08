/// Splits a Windows path into its drive letter and the names of the directories below the drive root.
pub(super) fn split_drive_path(path: &str) -> Option<(char, Vec<&str>)> {
    let mut characters = path.chars();
    let letter = characters.next().filter(char::is_ascii_alphabetic)?;
    let rest = path.get(1..)?.strip_prefix(':')?;
    if !rest.is_empty() && !rest.starts_with(['\\', '/']) {
        return None;
    }
    let components = rest.split(['\\', '/']).filter(|component| !component.is_empty() && *component != ".").collect();
    Some((letter.to_ascii_uppercase(), components))
}

#[cfg(test)]
mod tests {
    use super::split_drive_path;

    #[test]
    fn splits_drive_and_components() {
        assert_eq!(split_drive_path(r"c:\Users\me\"), Some(('C', vec!["Users", "me"])));
        assert_eq!(split_drive_path("D:/data"), Some(('D', vec!["data"])));
        assert_eq!(split_drive_path(r"E:\"), Some(('E', vec![])));
        assert_eq!(split_drive_path("F:"), Some(('F', vec![])));
    }

    #[test]
    fn rejects_paths_without_drive_letter() {
        assert_eq!(split_drive_path("/home/user"), None);
        assert_eq!(split_drive_path(r"\\server\share"), None);
        assert_eq!(split_drive_path("C:relative"), None);
        assert_eq!(split_drive_path(""), None);
    }
}
