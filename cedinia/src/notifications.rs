use log::error;

/// Send a system notification that the scan has completed.
///
/// On desktop, uses `notify-rust`. On Android, uses JNI to call
/// `NotificationManager.notify()` (silent no-op if the permission is not
/// granted on API 33+).
///
/// If `only_when_background` is true, the notification is skipped when the
/// app is currently in the foreground (Android: checked via ActivityManager;
/// desktop: never skipped, since focus can't be reliably detected here).
pub fn send_scan_completed(file_count: usize, only_when_background: bool) {
    if only_when_background && is_app_in_foreground() {
        return;
    }
    let title = "Cedinia";
    let body = format!("Scan completed – {file_count} items found");
    send_notification(title, &body);
}

// ── Foreground detection ──────────────────────────────────────────────────────

/// Returns `true` when the app is currently visible to the user.
/// On Android this queries `ActivityManager.getRunningAppProcesses()` and
/// checks that our process has `IMPORTANCE_FOREGROUND` (100).
/// On other platforms it always returns `false` so notifications are never
/// suppressed (there is no reliable cross-platform window-focus API here).
fn is_app_in_foreground() -> bool {
    #[cfg(target_os = "android")]
    {
        is_app_in_foreground_android()
    }
    #[cfg(not(target_os = "android"))]
    {
        false
    }
}

#[cfg(target_os = "android")]
fn is_app_in_foreground_android() -> bool {
    let Some(app) = crate::file_picker_android::get_android_app() else {
        return false;
    };

    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    let result = vm.attach_current_thread(|env| -> jni::errors::Result<bool> {
        use jni::objects::{JObject, JValue};
        use jni::signature::{FieldSignature, RuntimeFieldSignature};
        use jni::{jni_sig, jni_str};

        let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

        // Get ActivityManager service ("activity" is the well-known service name)
        let svc_name: JObject = env.new_string("activity")?.into();
        let am: JObject = env
            .call_method(
                &activity,
                jni_str!("getSystemService"),
                jni_sig!((name: java.lang.String) -> java.lang.Object),
                &[JValue::Object(&svc_name)],
            )?
            .l()?;

        if am.is_null() {
            return Ok(false);
        }

        // List<RunningAppProcessInfo> getRunningAppProcesses()
        let processes: JObject = env.call_method(&am, jni_str!("getRunningAppProcesses"), jni_sig!(() -> java.util.List), &[])?.l()?;

        if processes.is_null() {
            return Ok(false);
        }

        let my_pid = std::process::id() as i32;
        let size: i32 = env.call_method(&processes, jni_str!("size"), jni_sig!(() -> int), &[])?.i()?;

        let int_rfs = RuntimeFieldSignature::from_str("I").unwrap();
        let int_sig = FieldSignature::from(&int_rfs);
        for i in 0..size {
            let item: JObject = env
                .call_method(&processes, jni_str!("get"), jni_sig!((index: int) -> java.lang.Object), &[JValue::Int(i)])?
                .l()?;

            let pid: i32 = env.get_field(&item, jni_str!("pid"), &int_sig)?.i()?;
            if pid == my_pid {
                let importance: i32 = env.get_field(&item, jni_str!("importance"), &int_sig)?.i()?;
                // IMPORTANCE_FOREGROUND = 100
                return Ok(importance == 100);
            }
        }

        Ok(false)
    });

    result.unwrap_or(false)
}

// ── System notification permission ───────────────────────────────────────────

/// Returns `true` when the OS has **not** blocked notifications for this app.
/// On Android this calls `NotificationManager.areNotificationsEnabled()`.
/// On other platforms it always returns `true`.
pub fn are_system_notifications_enabled() -> bool {
    #[cfg(target_os = "android")]
    {
        are_system_notifications_enabled_android()
    }
    #[cfg(not(target_os = "android"))]
    {
        true
    }
}

/// Opens the system notification settings page for this app so the user can
/// unblock notifications. No-op on non-Android platforms.
pub fn open_system_notification_settings() {
    #[cfg(target_os = "android")]
    open_system_notification_settings_android();
}

#[cfg(target_os = "android")]
fn are_system_notifications_enabled_android() -> bool {
    let Some(app) = crate::file_picker_android::get_android_app() else {
        return true;
    };
    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    let result = vm.attach_current_thread(|env| -> jni::errors::Result<bool> {
        use jni::objects::{JObject, JValue};
        use jni::{jni_sig, jni_str};

        let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

        let svc_name: JObject = env.new_string("notification")?.into();
        let nm: JObject = env
            .call_method(
                &activity,
                jni_str!("getSystemService"),
                jni_sig!((name: java.lang.String) -> java.lang.Object),
                &[JValue::Object(&svc_name)],
            )?
            .l()?;

        if nm.is_null() {
            return Ok(true);
        }

        let enabled: bool = env.call_method(&nm, jni_str!("areNotificationsEnabled"), jni_sig!(() -> boolean), &[])?.z()?;
        Ok(enabled)
    });
    result.unwrap_or(true)
}

#[cfg(target_os = "android")]
fn open_system_notification_settings_android() {
    let Some(app) = crate::file_picker_android::get_android_app() else {
        return;
    };
    std::thread::spawn(move || {
        use jni::objects::{JObject, JValue};
        use jni::{jni_sig, jni_str};

        let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
        let _ = vm.attach_current_thread(|env| -> jni::errors::Result<()> {
            let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

            let pkg: JObject = env.call_method(&activity, jni_str!("getPackageName"), jni_sig!(() -> java.lang.String), &[])?.l()?;

            let action: JObject = env.new_string("android.settings.APP_NOTIFICATION_SETTINGS")?.into();
            let intent = env.new_object(jni_str!("android/content/Intent"), jni_sig!((action: java.lang.String) -> void), &[JValue::Object(&action)])?;

            let key: JObject = env.new_string("android.provider.extra.APP_PACKAGE")?.into();
            env.call_method(
                &intent,
                jni_str!("putExtra"),
                jni_sig!((name: java.lang.String, value: java.lang.String) -> android.content.Intent),
                &[JValue::Object(&key), JValue::Object(&pkg)],
            )?;

            env.call_method(
                &activity,
                jni_str!("startActivity"),
                jni_sig!((intent: android.content.Intent) -> void),
                &[JValue::Object(&intent)],
            )?;
            Ok(())
        });
    });
}

// ── Desktop implementation ────────────────────────────────────────────────────

#[cfg(not(target_os = "android"))]
fn send_notification(title: &str, body: &str) {
    let title = title.to_string();
    let body = body.to_string();
    std::thread::spawn(move || {
        #[cfg(target_os = "linux")]
        if try_notify_send(&title, &body) {
            return;
        }

        let mut notif = notify_rust::Notification::new();
        notif.summary(&title).body(&body);
        #[cfg(all(unix, not(target_os = "macos")))]
        notif.urgency(notify_rust::Urgency::Normal);
        if let Err(e) = notif.show() {
            error!("Failed to send desktop notification: {e}");
        }
    });
}

#[cfg(target_os = "linux")]
fn try_notify_send(summary: &str, body: &str) -> bool {
    match std::process::Command::new("notify-send").arg("--app-name=cedinia").arg(summary).arg(body).status() {
        Ok(s) if s.success() => true,
        Err(e) => {
            error!("Failed to execute notify-send: {e}");
            false
        }
        Ok(failed) => {
            error!("notify-send exited with non-zero status: {failed}");
            false
        }
    }
}

// ── Android implementation ────────────────────────────────────────────────────

#[cfg(target_os = "android")]
fn send_notification(title: &str, body: &str) {
    let Some(app) = crate::file_picker_android::get_android_app() else {
        log::warn!("send_notification: AndroidApp not initialised");
        return;
    };

    let title = title.to_string();
    let body = body.to_string();

    // Spawn a background thread so we don't block the Slint event loop.
    std::thread::spawn(move || {
        use jni::objects::{JObject, JValue};
        use jni::signature::RuntimeMethodSignature;
        use jni::{jni_sig, jni_str};

        let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
        let result = vm.attach_current_thread(|env| -> jni::errors::Result<()> {
            let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

            // ── Resolve app icon via Resources.getIdentifier ──────────────
            // Avoids hardcoding a system resource ID that may differ across
            // Android versions. Falls back to android.R.drawable.ic_dialog_info
            // (0x01080011 = 17301521) if the mipmap is not found.
            let icon_id = {
                let resources: JObject = env
                    .call_method(&activity, jni_str!("getResources"), jni_sig!(() -> android.content.res.Resources), &[])?
                    .l()?;
                let pkg: JObject = env.call_method(&activity, jni_str!("getPackageName"), jni_sig!(() -> java.lang.String), &[])?.l()?;
                let icon_name = env.new_string("ic_launcher")?;
                let icon_type = env.new_string("mipmap")?;
                let id = env
                    .call_method(
                        &resources,
                        jni_str!("getIdentifier"),
                        jni_sig!((name: java.lang.String, defType: java.lang.String, defPackage: java.lang.String) -> int),
                        &[JValue::Object(&icon_name), JValue::Object(&icon_type), JValue::Object(&pkg)],
                    )?
                    .i()?;
                if id != 0 { id } else { 17301521 } // 0x01080011 = ic_dialog_info
            };

            // ── Notification channel (required for API 26+) ───────────────
            let channel_id_str = env.new_string("cedinia_scan")?;
            let channel_name_str = env.new_string("Scan notifications")?;
            // IMPORTANCE_DEFAULT = 3
            let channel = env.new_object(
                jni_str!("android/app/NotificationChannel"),
                jni_sig!((id: java.lang.String, name: java.lang.CharSequence, importance: int) -> void),
                &[JValue::Object(&channel_id_str), JValue::Object(&channel_name_str), JValue::Int(3)],
            )?;

            let svc_name = env.new_string("notification")?.into();
            let nm: JObject = env
                .call_method(
                    &activity,
                    jni_str!("getSystemService"),
                    jni_sig!((name: java.lang.String) -> java.lang.Object),
                    &[JValue::Object(&svc_name)],
                )?
                .l()?;

            if nm.is_null() {
                log::warn!("send_notification: getSystemService(notification) returned null");
                return Ok(());
            }

            env.call_method(
                &nm,
                jni_str!("createNotificationChannel"),
                jni_sig!((channel: android.app.NotificationChannel) -> void),
                &[JValue::Object(&channel)],
            )?;

            // ── Build notification ────────────────────────────────────────
            // jni_sig! cannot express Notification$Builder (inner class with '$'),
            // so we use RuntimeMethodSignature for those descriptors.
            let sig_ctor = RuntimeMethodSignature::from_str("(Landroid/content/Context;Ljava/lang/String;)V").unwrap();
            let builder = env.new_object(
                jni_str!("android/app/Notification$Builder"),
                &sig_ctor.method_signature(),
                &[JValue::Object(&activity), JValue::Object(&channel_id_str)],
            )?;

            let sig_set_cs = RuntimeMethodSignature::from_str("(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;").unwrap();
            let title_jstr = env.new_string(&title)?.into();
            env.call_method(&builder, jni_str!("setContentTitle"), &sig_set_cs.method_signature(), &[JValue::Object(&title_jstr)])?;
            let body_jstr = env.new_string(&body)?.into();
            env.call_method(&builder, jni_str!("setContentText"), &sig_set_cs.method_signature(), &[JValue::Object(&body_jstr)])?;

            let sig_icon = RuntimeMethodSignature::from_str("(I)Landroid/app/Notification$Builder;").unwrap();
            env.call_method(&builder, jni_str!("setSmallIcon"), &sig_icon.method_signature(), &[JValue::Int(icon_id)])?;

            let sig_bool = RuntimeMethodSignature::from_str("(Z)Landroid/app/Notification$Builder;").unwrap();
            env.call_method(&builder, jni_str!("setAutoCancel"), &sig_bool.method_signature(), &[JValue::Bool(true)])?;

            let sig_build = RuntimeMethodSignature::from_str("()Landroid/app/Notification;").unwrap();
            let notification: JObject = env.call_method(&builder, jni_str!("build"), &sig_build.method_signature(), &[])?.l()?;

            env.call_method(
                &nm,
                jni_str!("notify"),
                jni_sig!((id: int, notification: android.app.Notification) -> void),
                &[JValue::Int(1), JValue::Object(&notification)],
            )?;

            Ok(())
        });

        if let Err(e) = result {
            log::warn!("send_notification: JNI error: {e:?}");
        }
    });
}
