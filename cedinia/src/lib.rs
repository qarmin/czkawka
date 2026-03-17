#![allow(clippy::unwrap_used)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::todo)]
mod app;
mod callbacks;
pub mod common;
#[cfg(target_os = "android")]
mod file_picker_android;
pub mod localizer_cedinia;
mod model;
mod scan_runner;
mod scanners;
mod set_initial_gui_infos;
pub mod settings;
mod thumbnail_loader;
pub mod translations;
mod volumes;
slint::include_modules!();
pub use app::run_app;

static ANDROID_FILES_PATH: std::sync::OnceLock<String> = std::sync::OnceLock::new();
static ANDROID_CACHE_PATH: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn android_files_path() -> Option<&'static str> {
    ANDROID_FILES_PATH.get().map(String::as_str)
}
pub fn android_cache_path() -> Option<&'static str> {
    ANDROID_CACHE_PATH.get().map(String::as_str)
}

#[cfg(target_os = "android")]
fn setup_android_paths(android_app: &slint::android::AndroidApp) {
    use jni::objects::{JObject, JString};
    use jni::{jni_sig, jni_str};

    let vm = unsafe { jni::JavaVM::from_raw(android_app.vm_as_ptr() as *mut _) };
    let _ = vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let activity_raw = unsafe { JObject::from_raw(env, android_app.activity_as_ptr() as *mut _) };

        let files_dir: JObject = env.call_method(&activity_raw, jni_str!("getFilesDir"), jni_sig!(() -> java.io.File), &[])?.l()?;
        let files_path_obj = env.call_method(&files_dir, jni_str!("getAbsolutePath"), jni_sig!(() -> java.lang.String), &[])?.l()?;
        let files_path: JString = env.cast_local::<JString>(files_path_obj)?;
        let files_str: String = files_path.try_to_string(env)?;

        let cache_dir: JObject = env.call_method(&activity_raw, jni_str!("getCacheDir"), jni_sig!(() -> java.io.File), &[])?.l()?;
        let cache_path_obj = env.call_method(&cache_dir, jni_str!("getAbsolutePath"), jni_sig!(() -> java.lang.String), &[])?.l()?;
        let cache_path: JString = env.cast_local::<JString>(cache_path_obj)?;
        let cache_str: String = cache_path.try_to_string(env)?;

        let _ = ANDROID_FILES_PATH.set(files_str.clone());
        let _ = ANDROID_CACHE_PATH.set(cache_str.clone());

        unsafe { std::env::set_var("DATA_DIR", &files_str) };

        eprintln!("setup_android_paths: config='{}' cache='{}'", files_str, cache_str);
        Ok(())
    });
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
    // Init logcat logging FIRST so every log::* call and every panic message
    // appears under `adb logcat -s cedinia`.  Without this all output goes to
    // stdout/stderr which Android silently discards for native code.
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Debug).with_tag("cedinia"));
    setup_android_paths(&android_app);
    crate::app::setup_logger_cache();
    log::info!("android_main: started");
    let scale = android_app.config().density().unwrap_or(160) as f32 / 160.0;
    log::info!("android_main: display scale={:.2}", scale);
    log::info!("android_main: initialising file picker (JNI + DEX)");
    file_picker_android::init(&android_app);
    log::info!("android_main: file picker initialised");
    slint::android::init(android_app.clone()).expect("Failed to initialise Slint Android backend");
    log::info!("android_main: Slint backend initialised");
    file_picker_android::setup_nav_bar();
    log::info!("android_main: nav bar pinned");
    let rect = android_app.content_rect();
    let inset_bottom_px = if rect.bottom > 0 { 0.0f32 } else { 48.0 * scale };
    log::info!("android_main: content_rect={:?} inset_bottom={}", rect, inset_bottom_px);
    log::info!("android_main: launching app UI");
    app::run_app_with_insets(inset_bottom_px, scale, android_app);
    log::info!("android_main: app UI returned (exiting)");
}
