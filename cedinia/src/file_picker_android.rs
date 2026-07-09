use jni_high::{BridgeResultExt, android_bridge};

android_bridge! {
    dex = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex")),

    class FilePicker {
        java_name = "CediniaFilePicker",

        static fn pick_include_directory(start_path: &str);
        static fn pick_exclude_directory(start_path: &str);
        static fn open_url(url: &str);
        static fn open_file(path: &str);
        static fn open_folder(path: &str);
        static fn setup_nav_bar();
        static fn apply_theme_to_system_bars(dark_theme: bool);
        static fn acquire_wake_lock();
        static fn release_wake_lock();
        static fn has_storage_permission() -> bool;
        static fn request_storage_permission();

        callback fn on_directory_picked(path: String, is_include: bool);
    }
}

pub fn init() {
    FilePicker::set_on_directory_picked(|path, is_include| {
        slint::invoke_from_event_loop(move || {
            crate::app::on_directory_picked(path, is_include);
        })
        .expect("invoke_from_event_loop failed in on_directory_picked");
    });
}

pub fn launch_pick_directory(is_include: bool, start_path: &str) {
    if is_include {
        FilePicker::pick_include_directory(start_path).log_err("pick_include_directory");
    } else {
        FilePicker::pick_exclude_directory(start_path).log_err("pick_exclude_directory");
    }
}

pub fn open_url(url: &str) {
    FilePicker::open_url(url).log_err("open_url");
}

pub fn open_file(path: &str) {
    FilePicker::open_file(path).log_err("open_file");
}

pub fn open_folder(path: &str) {
    FilePicker::open_folder(path).log_err("open_folder");
}

pub fn setup_nav_bar() {
    FilePicker::setup_nav_bar().log_err("setup_nav_bar");
}

pub fn apply_theme_to_system_bars(dark_theme: bool) {
    FilePicker::apply_theme_to_system_bars(dark_theme).log_err("apply_theme_to_system_bars");
}

pub fn acquire_wakelock() {
    FilePicker::acquire_wake_lock().log_err("acquire_wakelock");
}

pub(crate) fn get_android_language_tag() -> Option<String> {
    jni_high::android::locale::system_locale_tag()
        .map_err(|e| {
            log::warn!("get_android_language_tag: JNI error: {:?}", e);
            e
        })
        .ok()
}

pub fn release_wakelock() {
    FilePicker::release_wake_lock().log_err("release_wakelock");
}

pub fn check_storage_permission() -> bool {
    FilePicker::has_storage_permission().unwrap_or(false)
}

pub fn request_storage_permission() {
    FilePicker::request_storage_permission().log_err("request_storage_permission");
}
