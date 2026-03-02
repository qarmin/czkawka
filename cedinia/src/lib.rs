#![allow(clippy::unwrap_used)]

mod app;
mod callbacks;
mod model;
mod scan_runner;
mod scanners;
pub mod settings;
mod thumbnail_loader;
mod volumes;

#[cfg(target_os = "android")]
mod file_picker_android;

slint::include_modules!();

pub use app::run_app;

#[cfg(target_os = "android")]
fn setup_android_paths(android_app: &slint::android::AndroidApp) {
    use jni::objects::{JObject, JString};
    use jni::{jni_sig, jni_str};

    let vm = unsafe { jni::JavaVM::from_raw(android_app.vm_as_ptr() as *mut _) };

    let _ = vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let activity = unsafe { JObject::from_raw(env, android_app.activity_as_ptr() as *mut _) };

        let files_dir: JObject = env.call_method(&activity, jni_str!("getFilesDir"), jni_sig!(() -> java.io.File), &[])?.l()?;
        let files_path_raw = env
            .call_method(&files_dir, jni_str!("getAbsolutePath"), jni_sig!(() -> java.lang.String), &[])?
            .l()?
            .into_raw();
        let files_path: JString = unsafe { JString::from_raw(env, files_path_raw) };
        let files_str: String = files_path.try_to_string(env)?;

        let cache_dir: JObject = env.call_method(&activity, jni_str!("getCacheDir"), jni_sig!(() -> java.io.File), &[])?.l()?;
        let cache_path_raw = env
            .call_method(&cache_dir, jni_str!("getAbsolutePath"), jni_sig!(() -> java.lang.String), &[])?
            .l()?
            .into_raw();
        let cache_path: JString = unsafe { JString::from_raw(env, cache_path_raw) };
        let cache_str: String = cache_path.try_to_string(env)?;

        unsafe {
            std::env::set_var("CZKAWKA_CONFIG_PATH", &files_str);
            std::env::set_var("CZKAWKA_CACHE_PATH", &cache_str);
        }
        eprintln!("setup_android_paths: config='{}' cache='{}'", files_str, cache_str);
        Ok(())
    });
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: slint::android::AndroidApp) {
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

    let rect = android_app.content_rect();
    let inset_bottom_px = if rect.bottom > 0 { 0.0f32 } else { 48.0 * scale };
    log::info!("android_main: content_rect={:?} inset_bottom={}", rect, inset_bottom_px);

    log::info!("android_main: launching app UI");
    app::run_app_with_insets(inset_bottom_px, scale, android_app);
    log::info!("android_main: app UI returned (exiting)");
}
