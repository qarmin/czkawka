use std::ffi::c_void;
use std::sync::{Arc, Mutex};

use android_activity::AndroidApp;
use jni::objects::{Global, JClass, JObject, JString, JValue};
use jni::sys::jboolean;
use jni::{EnvUnowned, jni_sig, jni_str};
use slint::invoke_from_event_loop;

static PENDING_PICK: std::sync::OnceLock<Arc<Mutex<Option<(String, bool)>>>> = std::sync::OnceLock::new();

fn pending_pick() -> &'static Arc<Mutex<Option<(String, bool)>>> {
    PENDING_PICK.get_or_init(|| Arc::new(Mutex::new(None)))
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
unsafe extern "system" fn Java_CediniaFilePicker_onDirectoryPicked(mut unowned: EnvUnowned<'_>, _class: JClass<'_>, path: JString<'_>, is_include: jboolean) {
    log::info!("Java_CediniaFilePicker_onDirectoryPicked: entered, is_include={}", is_include);

    let outcome = unowned.with_env_no_catch(|env| path.try_to_string(env));
    let path_str: String = match outcome.into_outcome() {
        jni::Outcome::Ok(s) => s,
        jni::Outcome::Err(e) => {
            log::error!("onDirectoryPicked: path read error: {:?}", e);
            return;
        }
        jni::Outcome::Panic(_) => {
            log::error!("onDirectoryPicked: panic reading path");
            return;
        }
    };

    if path_str.is_empty() {
        log::error!("onDirectoryPicked: empty path, aborting");
        return;
    }
    log::info!("onDirectoryPicked: path='{}' include={}", path_str, is_include);

    if let Ok(mut guard) = pending_pick().lock() {
        *guard = Some((path_str.clone(), is_include));
    }
    let pending = pending_pick().clone();
    let dispatch_result = invoke_from_event_loop(move || match pending.lock().ok().and_then(|mut g| g.take()) {
        Some((p, inc)) => {
            log::info!("invoke_from_event_loop: on_directory_picked path='{}' inc={}", p, inc);
            crate::app::on_directory_picked(p, inc);
        }
        None => log::warn!("invoke_from_event_loop: PENDING_PICK was empty"),
    });
    if let Err(e) = dispatch_result {
        log::error!("onDirectoryPicked: invoke_from_event_loop failed: {:?}", e);
    }
}

pub fn init(app: &AndroidApp) {
    log::info!("file_picker_android::init: starting");
    // Always update APP_HANDLE so that after Activity recreation the new
    // AndroidApp (with a valid vm_as_ptr) replaces the stale one.
    *APP_HANDLE.lock().expect("APP_HANDLE mutex poisoned") = Some(app.clone());

    let vm = try_jvm(app).expect("init: vm_as_ptr is null at app startup");
    let dex_data: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"));
    log::info!("file_picker_android::init: DEX size={}", dex_data.len());

    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        log::info!("file_picker_android::init: JVM attached");

        let dex_buffer = unsafe { env.new_direct_byte_buffer(dex_data.as_ptr() as *mut _, dex_data.len()) }?;
        let native_activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

        let parent_class_loader = env
            .call_method(&native_activity, jni_str!("getClassLoader"), jni_sig!(() -> java.lang.ClassLoader), &[])?
            .l()?;

        let dex_loader = match env.new_object(
            jni_str!("dalvik/system/InMemoryDexClassLoader"),
            jni_sig!((buf: java.nio.ByteBuffer, loader: java.lang.ClassLoader) -> void),
            &[JValue::Object(&dex_buffer), JValue::Object(&parent_class_loader)],
        ) {
            Ok(obj) => {
                log::info!("file_picker_android::init: using InMemoryDexClassLoader");
                env.exception_clear();
                obj
            }
            Err(e) => {
                log::info!("file_picker_android::init: InMemoryDexClassLoader failed ({:?}), trying DexClassLoader", e);
                env.exception_clear();
                let code_cache_dir = env.call_method(&native_activity, jni_str!("getCodeCacheDir"), jni_sig!(() -> java.io.File), &[])?.l()?;
                let path_obj = env.call_method(&code_cache_dir, jni_str!("getAbsolutePath"), jni_sig!(() -> java.lang.String), &[])?.l()?;
                let j_path = unsafe { JString::from_raw(env, path_obj.as_raw()) };
                let path_str = j_path.try_to_string(env)?;
                let dex_path = format!("{}/cedinia_picker.dex", path_str);
                std::fs::write(&dex_path, dex_data).expect("write dex");
                let oats_path = format!("{}/oats", path_str);
                let _ = std::fs::create_dir(&oats_path);
                let j_dex = env.new_string(&dex_path)?;
                let j_oats = env.new_string(&oats_path)?;
                env.new_object(
                    jni_str!("dalvik/system/DexClassLoader"),
                    jni_sig!((dexPath: java.lang.String, optimizedDirectory: java.lang.String,
                               librarySearchPath: java.lang.String, parent: java.lang.ClassLoader) -> void),
                    &[
                        JValue::Object(&j_dex),
                        JValue::Object(&j_oats),
                        JValue::Object(&JObject::null()),
                        JValue::Object(&parent_class_loader),
                    ],
                )?
            }
        };

        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                &dex_loader,
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };

        let native_methods = [unsafe {
            jni::NativeMethod::from_raw_parts(
                jni_str!("onDirectoryPicked"),
                jni_str!("(Ljava/lang/String;Z)V"),
                Java_CediniaFilePicker_onDirectoryPicked as *mut c_void,
            )
        }];
        unsafe { env.register_native_methods(&picker_class, &native_methods) }?;
        log::info!("file_picker_android::init: native method registered");

        let loader_global: Global<JObject<'static>> = env.new_global_ref(dex_loader)?;
        *DEX_LOADER_REF.lock().expect("DEX_LOADER_REF mutex poisoned") = Some(Arc::new(loader_global));

        let activity_global: Global<JObject<'static>> = env.new_global_ref(&native_activity)?;
        *ACTIVITY_GLOBAL_REF.lock().expect("ACTIVITY_GLOBAL_REF mutex poisoned") = Some(Arc::new(activity_global));

        log::info!("file_picker_android::init: complete");
        Ok(())
    })
    .expect("init JNI attachment failed");
}

// These are behind Mutex<Option<...>> so they can be updated each time the
// Android Activity is recreated (process is reused but Activity restarts).
static APP_HANDLE: Mutex<Option<AndroidApp>> = Mutex::new(None);
static DEX_LOADER_REF: Mutex<Option<Arc<Global<JObject<'static>>>>> = Mutex::new(None);
static ACTIVITY_GLOBAL_REF: Mutex<Option<Arc<Global<JObject<'static>>>>> = Mutex::new(None);

pub fn get_android_app() -> Option<AndroidApp> {
    APP_HANDLE.lock().ok()?.clone()
}

pub(crate) fn get_activity_global_ref() -> Option<Arc<Global<JObject<'static>>>> {
    ACTIVITY_GLOBAL_REF.lock().ok()?.clone()
}

fn get_loader() -> Option<Arc<Global<JObject<'static>>>> {
    DEX_LOADER_REF.lock().ok()?.clone()
}

/// Returns `Some(JavaVM)` if the VM pointer is still valid, or `None` when the
/// Android activity is paused / stopped and `vm_as_ptr()` has become null.
///
/// All JNI calls that may run from a Slint timer (rather than a direct user
/// interaction) must use this guard to avoid the
/// `assertion failed: !ptr.is_null()` panic inside `jni::JavaVM::from_raw`.
pub fn try_jvm(app: &AndroidApp) -> Option<jni::JavaVM> {
    let ptr = app.vm_as_ptr();
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { jni::JavaVM::from_raw(ptr as *mut _) })
}

pub fn launch_pick_directory(is_include: bool, start_path: &str) {
    log::info!("launch_pick_directory: is_include={} start_path='{}'", is_include, start_path);
    let Some(app) = get_android_app() else {
        log::error!("launch_pick_directory: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = get_loader() else {
        log::error!("launch_pick_directory: DEX loader not initialised");
        return;
    };
    let Some(activity_ref) = get_activity_global_ref() else {
        log::error!("launch_pick_directory: activity global ref not initialised");
        return;
    };

    let Some(vm) = try_jvm(&app) else {
        log::debug!("launch_pick_directory: vm_as_ptr is null, skipping");
        return;
    };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = activity_ref.as_obj();
        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                loader_ref.as_obj(),
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };

        let method = if is_include {
            jni_str!("pickIncludeDirectory")
        } else {
            jni_str!("pickExcludeDirectory")
        };
        let j_start = env.new_string(start_path)?;
        log::info!("launch_pick_directory: calling Java CediniaFilePicker method with start_path");
        env.call_static_method(
            &picker_class,
            method,
            jni_sig!((activity: android.app.Activity, startPath: java.lang.String) -> void),
            &[JValue::Object(&native_activity), JValue::Object(&j_start)],
        )?;
        log::info!("launch_pick_directory: Java call succeeded");
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("launch_pick_directory: JNI failed: {:?}", e));
}

pub fn open_url(url: &str) {
    let Some(app) = get_android_app() else {
        log::error!("open_url: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = get_loader() else {
        log::error!("open_url: DEX loader not initialised");
        return;
    };
    let Some(activity_ref) = get_activity_global_ref() else {
        log::error!("open_url: activity global ref not initialised");
        return;
    };

    let url = url.to_string();
    let Some(vm) = try_jvm(&app) else {
        return;
    };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = activity_ref.as_obj();
        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                loader_ref.as_obj(),
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };
        let j_url = env.new_string(&url)?;
        env.call_static_method(
            &picker_class,
            jni_str!("openUrl"),
            jni_sig!((activity: android.app.Activity, url: java.lang.String) -> void),
            &[JValue::Object(&native_activity), JValue::Object(&j_url)],
        )?;
        log::info!("open_url: Java call succeeded for {}", url);
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("open_url: JNI failed: {:?}", e));
}

pub fn open_file(path: &str) {
    let Some(app) = get_android_app() else {
        log::error!("open_file: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = get_loader() else {
        log::error!("open_file: DEX loader not initialised");
        return;
    };
    let Some(activity_ref) = get_activity_global_ref() else {
        log::error!("open_file: activity global ref not initialised");
        return;
    };

    let path = path.to_string();
    let Some(vm) = try_jvm(&app) else {
        return;
    };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = activity_ref.as_obj();
        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                loader_ref.as_obj(),
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };
        let j_path = env.new_string(&path)?;
        env.call_static_method(
            &picker_class,
            jni_str!("openFile"),
            jni_sig!((activity: android.app.Activity, path: java.lang.String) -> void),
            &[JValue::Object(&native_activity), JValue::Object(&j_path)],
        )?;
        log::info!("open_file: Java call succeeded for {}", path);
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("open_file: JNI failed: {:?}", e));
}

pub fn open_folder(path: &str) {
    let Some(app) = get_android_app() else {
        log::error!("open_folder: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = get_loader() else {
        log::error!("open_folder: DEX loader not initialised");
        return;
    };
    let Some(activity_ref) = get_activity_global_ref() else {
        log::error!("open_folder: activity global ref not initialised");
        return;
    };

    let path = path.to_string();
    let Some(vm) = try_jvm(&app) else {
        return;
    };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = activity_ref.as_obj();
        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                loader_ref.as_obj(),
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };
        let j_path = env.new_string(&path)?;
        env.call_static_method(
            &picker_class,
            jni_str!("openFolder"),
            jni_sig!((activity: android.app.Activity, path: java.lang.String) -> void),
            &[JValue::Object(&native_activity), JValue::Object(&j_path)],
        )?;
        log::info!("open_folder: Java call succeeded for {}", path);
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("open_folder: JNI failed: {:?}", e));
}

pub fn setup_nav_bar() {
    let Some(app) = get_android_app() else {
        log::error!("setup_nav_bar: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = get_loader() else {
        log::error!("setup_nav_bar: DEX loader not initialised");
        return;
    };
    let Some(activity_ref) = get_activity_global_ref() else {
        log::error!("setup_nav_bar: activity global ref not initialised");
        return;
    };

    let Some(vm) = try_jvm(&app) else {
        return;
    };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = activity_ref.as_obj();
        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                loader_ref.as_obj(),
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };
        env.call_static_method(
            &picker_class,
            jni_str!("setupNavBar"),
            jni_sig!((activity: android.app.Activity) -> void),
            &[JValue::Object(&native_activity)],
        )?;
        log::info!("setup_nav_bar: Java call succeeded");
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("setup_nav_bar: JNI failed: {:?}", e));
}

pub fn acquire_wakelock() {
    let Some(app) = get_android_app() else {
        log::error!("acquire_wakelock: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = get_loader() else {
        log::error!("acquire_wakelock: DEX loader not initialised");
        return;
    };
    let Some(activity_ref) = get_activity_global_ref() else {
        log::error!("acquire_wakelock: activity global ref not initialised");
        return;
    };

    let Some(vm) = try_jvm(&app) else {
        return;
    };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = activity_ref.as_obj();
        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                loader_ref.as_obj(),
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };
        env.call_static_method(
            &picker_class,
            jni_str!("acquireWakeLock"),
            jni_sig!((activity: android.app.Activity) -> void),
            &[JValue::Object(&native_activity)],
        )?;
        log::info!("acquire_wakelock: Java call succeeded");
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("acquire_wakelock: JNI failed: {:?}", e));
}

pub fn release_wakelock() {
    let Some(app) = get_android_app() else {
        log::error!("release_wakelock: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = get_loader() else {
        log::error!("release_wakelock: DEX loader not initialised");
        return;
    };
    let Some(activity_ref) = get_activity_global_ref() else {
        log::error!("release_wakelock: activity global ref not initialised");
        return;
    };

    let Some(vm) = try_jvm(&app) else {
        return;
    };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = activity_ref.as_obj();
        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                loader_ref.as_obj(),
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };
        env.call_static_method(
            &picker_class,
            jni_str!("releaseWakeLock"),
            jni_sig!((activity: android.app.Activity) -> void),
            &[JValue::Object(&native_activity)],
        )?;
        log::info!("release_wakelock: Java call succeeded");
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("release_wakelock: JNI failed: {:?}", e));
}

pub fn check_storage_permission() -> bool {
    let Some(app) = get_android_app() else { return false };
    let Some(loader_ref) = get_loader() else { return false };
    let Some(activity_ref) = get_activity_global_ref() else { return false };

    let Some(vm) = try_jvm(&app) else {
        log::debug!("check_storage_permission: vm_as_ptr is null (app paused?), returning false");
        return false;
    };
    let mut result = false;
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = activity_ref.as_obj();
        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                loader_ref.as_obj(),
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };
        let val = env
            .call_static_method(
                &picker_class,
                jni_str!("hasStoragePermission"),
                jni_sig!((activity: android.app.Activity) -> boolean),
                &[JValue::Object(&native_activity)],
            )?
            .z()?;
        result = val;
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("check_storage_permission: JNI failed: {:?}", e));
    result
}

pub fn request_storage_permission() {
    let Some(app) = get_android_app() else { return };
    let Some(loader_ref) = get_loader() else { return };
    let Some(activity_ref) = get_activity_global_ref() else { return };

    let Some(vm) = try_jvm(&app) else {
        log::debug!("request_storage_permission: vm_as_ptr is null (app paused?), skipping");
        return;
    };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = activity_ref.as_obj();
        let class_name = env.new_string("CediniaFilePicker")?;
        let picker_class_obj = env
            .call_method(
                loader_ref.as_obj(),
                jni_str!("findClass"),
                jni_sig!((name: java.lang.String) -> java.lang.Class),
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let picker_class: JClass = unsafe { JClass::from_raw(env, picker_class_obj.as_raw()) };
        env.call_static_method(
            &picker_class,
            jni_str!("requestStoragePermission"),
            jni_sig!((activity: android.app.Activity) -> void),
            &[JValue::Object(&native_activity)],
        )?;
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("request_storage_permission: JNI failed: {:?}", e));
}
