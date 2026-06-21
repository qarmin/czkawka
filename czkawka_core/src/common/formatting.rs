use std::time::Duration;

pub fn format_time(duration: Duration) -> String {
    let hours = duration.as_secs() / 3600;
    let minutes = duration.as_secs() % 3600 / 60;
    let secs = duration.as_secs() % 60;
    let millis = duration.subsec_millis();
    if hours == 0 && minutes == 0 && secs == 0 {
        format!("{millis}ms")
    } else if hours == 0 && minutes == 0 {
        if millis / 10 == 0 { format!("{secs}s") } else { format!("{secs}.{:02}s", millis / 10) }
    } else if hours == 0 {
        if secs == 0 { format!("{minutes}m") } else { format!("{minutes}m {secs}s") }
    } else {
        if secs == 0 && minutes == 0 {
            format!("{hours}h")
        } else if secs == 0 {
            format!("{hours}h {minutes}m")
        } else {
            format!("{hours}h {minutes}m {secs}s")
        }
    }
}

pub(crate) fn create_crash_message(library_name: &str, file_path: &str, home_library_url: &str) -> String {
    format!(
        "{library_name} library crashed when opening \"{file_path}\", please check if this is fixed with the latest version of {library_name} and if it is not fixed, please report bug here - {home_library_url}"
    )
}

pub(crate) fn create_crash_message_generic(file_path: &str) -> String {
    format!("A panic/crash occurred while processing \"{file_path}\"")
}

pub(crate) fn normalize_error_string(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
