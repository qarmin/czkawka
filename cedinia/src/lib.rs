#![allow(clippy::unwrap_used)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::todo)]
mod app;
mod callbacks;
pub mod common;
mod compare;
mod file_actions;
#[cfg(target_os = "android")]
mod file_picker_android;
pub mod localizer_cedinia;
mod model;
mod notifications;
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
    match jni_high::android::activity::app_dirs(android_app) {
        Ok(dirs) => {
            let _ = ANDROID_FILES_PATH.set(dirs.files_dir.clone());
            let _ = ANDROID_CACHE_PATH.set(dirs.cache_dir);
            unsafe { std::env::set_var("DATA_DIR", &dirs.files_dir) };
            eprintln!("setup_android_paths: config='{}'", dirs.files_dir);
        }
        Err(e) => eprintln!("setup_android_paths: {e:?}"),
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
    // DATA_DIR must be set before setup_logger_cache, which resolves the cache folder from it.
    setup_android_paths(&android_app);
    crate::app::setup_logger_cache();
    log::info!("android_main: started");
    asan_smoketest_if_requested();
    let scale = android_app.config().density().unwrap_or(160) as f32 / 160.0;
    log::info!("android_main: display scale={:.2}", scale);
    log::info!("android_main: initialising jni_high context");
    jni_high::AndroidContext::init(android_app.clone());
    file_picker_android::init();
    log::info!("android_main: jni_high context ready");
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

// Triggers a deliberate heap-buffer-overflow so ASan's abort proves it's active in the build.
// Only fires when CEDINIA_ASAN_SMOKETEST is set (asan_wrap.sh sets it for `just android_asan smoke`).
#[cfg(target_os = "android")]
fn asan_smoketest_if_requested() {
    if std::env::var_os("CEDINIA_ASAN_SMOKETEST").is_none() {
        return;
    }
    log::error!("ASAN SMOKETEST: triggering a deliberate heap-buffer-overflow now");
    let v: Vec<u8> = vec![0xAB; 4];
    let ptr = v.as_ptr();
    let offset = std::hint::black_box(64usize);
    let byte = unsafe { std::ptr::read_volatile(ptr.add(offset)) };
    std::hint::black_box(byte);
    log::error!("ASAN SMOKETEST: still alive at byte={byte:#x} - ASan is NOT active in this build");
}
