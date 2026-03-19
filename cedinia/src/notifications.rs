/// Send a system notification that the scan has completed.
///
/// On desktop, uses `notify-rust`. On Android, uses JNI to call
/// `NotificationManager.notify()` (silent no-op if the permission is not
/// granted on API 33+).
pub fn send_scan_completed(file_count: usize) {
    let title = "Cedinia";
    let body = format!("Scan completed – {file_count} items found");
    send_notification(title, &body);
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

        if let Err(e) = notify_rust::Notification::new()
            .summary(&title)
            .body(&body)
            .urgency(notify_rust::Urgency::Normal)
            .show()
        {
            eprintln!("Failed to send desktop notification: {e}");
        }
    });
}

#[cfg(target_os = "linux")]
fn try_notify_send(summary: &str, body: &str) -> bool {
    match std::process::Command::new("notify-send")
        .arg("--app-name=cedinia")
        .arg(summary)
        .arg(body)
        .status()
    {
        Ok(s) if s.success() => true,
        _ => false,
    }
}

// ── Android implementation ────────────────────────────────────────────────────

#[cfg(target_os = "android")]
fn send_notification(title: &str, body: &str) {
    use jni::objects::{JObject, JValue};
    use jni::{jni_sig, jni_str};

    let Some(app) = crate::file_picker_android::get_android_app() else {
        log::warn!("send_notification: AndroidApp not initialised");
        return;
    };

    let title = title.to_string();
    let body = body.to_string();

    let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr() as *mut _) };
    let result = vm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let activity = unsafe { JObject::from_raw(env, app.activity_as_ptr() as *mut _) };

        // ── Notification channel (required for API 26+) ───────────────────
        let channel_id_str = env.new_string("cedinia_scan")?;
        let channel_name_str = env.new_string("Scan notifications")?;
        // IMPORTANCE_DEFAULT = 3
        let channel = env.new_object(
            jni_str!("android/app/NotificationChannel"),
            jni_sig!((id: java.lang.String, name: java.lang.CharSequence, importance: int) -> void),
            &[
                JValue::Object(&channel_id_str),
                JValue::Object(&channel_name_str),
                JValue::Int(3),
            ],
        )?;

        let nm_service_str = env.new_string("notification")?;
        let nm: JObject = env
            .call_method(
                &activity,
                jni_str!("getSystemService"),
                jni_sig!((name: java.lang.String) -> java.lang.Object),
                &[JValue::Object(&nm_service_str)],
            )?
            .l()?;

        env.call_method(
            &nm,
            jni_str!("createNotificationChannel"),
            jni_sig!((channel: android.app.NotificationChannel) -> void),
            &[JValue::Object(&channel)],
        )?;

        // ── Build notification ────────────────────────────────────────────
        // jni_sig! cannot express Notification$Builder (inner class with '$'),
        // so we parse the raw JNI descriptors via RuntimeMethodSignature.
        use jni::signature::RuntimeMethodSignature;

        let sig_ctor =
            RuntimeMethodSignature::from_str("(Landroid/content/Context;Ljava/lang/String;)V")
                .unwrap();
        let builder = env.new_object(
            jni_str!("android/app/Notification$Builder"),
            &sig_ctor.method_signature(),
            &[JValue::Object(&activity), JValue::Object(&channel_id_str)],
        )?;

        let title_jstr = env.new_string(&title)?;
        let sig_set_text = RuntimeMethodSignature::from_str(
            "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;",
        )
        .unwrap();
        env.call_method(
            &builder,
            jni_str!("setContentTitle"),
            &sig_set_text.method_signature(),
            &[JValue::Object(&title_jstr)],
        )?;

        let body_jstr = env.new_string(&body)?;
        env.call_method(
            &builder,
            jni_str!("setContentText"),
            &sig_set_text.method_signature(),
            &[JValue::Object(&body_jstr)],
        )?;

        // Use android.R.drawable.ic_dialog_info (17301624) as a fallback
        // small icon – every Android version ships it.
        let sig_set_icon = RuntimeMethodSignature::from_str(
            "(I)Landroid/app/Notification$Builder;",
        )
        .unwrap();
        env.call_method(
            &builder,
            jni_str!("setSmallIcon"),
            &sig_set_icon.method_signature(),
            &[JValue::Int(17301624)],
        )?;

        // AUTO_CANCEL = true so the notification dismisses itself on tap.
        let sig_set_bool = RuntimeMethodSignature::from_str(
            "(Z)Landroid/app/Notification$Builder;",
        )
        .unwrap();
        env.call_method(
            &builder,
            jni_str!("setAutoCancel"),
            &sig_set_bool.method_signature(),
            &[JValue::Bool(true)],
        )?;

        let sig_build =
            RuntimeMethodSignature::from_str("()Landroid/app/Notification;").unwrap();
        let notification: JObject = env
            .call_method(&builder, jni_str!("build"), &sig_build.method_signature(), &[])?
            .l()?;

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
}
