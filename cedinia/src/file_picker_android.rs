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
    APP_HANDLE.get_or_init(|| app.clone());

    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
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
        let _ = DEX_LOADER_REF.set(loader_global);
        log::info!("file_picker_android::init: complete");
        Ok(())
    })
    .expect("init JNI attachment failed");
}

static APP_HANDLE: std::sync::OnceLock<AndroidApp> = std::sync::OnceLock::new();
static DEX_LOADER_REF: std::sync::OnceLock<Global<JObject<'static>>> = std::sync::OnceLock::new();

pub fn get_android_app() -> Option<&'static AndroidApp> {
    APP_HANDLE.get()
}

pub fn launch_pick_directory(is_include: bool) {
    log::info!("launch_pick_directory: is_include={}", is_include);
    let Some(app) = APP_HANDLE.get() else {
        log::error!("launch_pick_directory: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = DEX_LOADER_REF.get() else {
        log::error!("launch_pick_directory: DEX loader not initialised");
        return;
    };

    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

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
        log::info!("launch_pick_directory: calling Java CediniaFilePicker method");
        env.call_static_method(
            &picker_class,
            method,
            jni_sig!((activity: android.app.Activity) -> void),
            &[JValue::Object(&native_activity)],
        )?;
        log::info!("launch_pick_directory: Java call succeeded");
        Ok(())
    })
    .unwrap_or_else(|e| log::error!("launch_pick_directory: JNI failed: {:?}", e));
}

pub fn open_url(url: &str) {
    let Some(app) = APP_HANDLE.get() else {
        log::error!("open_url: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = DEX_LOADER_REF.get() else {
        log::error!("open_url: DEX loader not initialised");
        return;
    };

    let url = url.to_string();
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };
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

pub fn setup_nav_bar() {
    let Some(app) = APP_HANDLE.get() else {
        log::error!("setup_nav_bar: AndroidApp not initialised");
        return;
    };
    let Some(loader_ref) = DEX_LOADER_REF.get() else {
        log::error!("setup_nav_bar: DEX loader not initialised");
        return;
    };

    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };
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

pub fn check_storage_permission() -> bool {
    let Some(app) = APP_HANDLE.get() else { return false };
    let Some(loader_ref) = DEX_LOADER_REF.get() else { return false };

    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    let mut result = false;
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };
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
    let Some(app) = APP_HANDLE.get() else { return };
    let Some(loader_ref) = DEX_LOADER_REF.get() else { return };

    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let native_activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };
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
