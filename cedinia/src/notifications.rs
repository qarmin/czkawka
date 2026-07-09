pub fn send_scan_completed(file_count: usize, only_when_background: bool) {
    if only_when_background && is_app_in_foreground() {
        return;
    }
    let title = "Cedinia";
    let body = crate::flc!("scan_completed_notification", file_count = file_count);
    send_notification(title, &body);
}

fn is_app_in_foreground() -> bool {
    #[cfg(target_os = "android")]
    {
        jni_high::android::activity::is_in_foreground().unwrap_or(false)
    }
    #[cfg(not(target_os = "android"))]
    {
        false
    }
}

pub fn are_system_notifications_enabled() -> bool {
    #[cfg(target_os = "android")]
    {
        jni_high::android::notifications::are_enabled().unwrap_or(true)
    }
    #[cfg(not(target_os = "android"))]
    {
        true
    }
}

pub fn open_system_notification_settings() {
    #[cfg(target_os = "android")]
    {
        if let Err(e) = jni_high::android::notifications::open_settings() {
            log::warn!("open_system_notification_settings: {e:?}");
        }
    }
}

#[cfg(target_os = "android")]
fn send_notification(title: &str, body: &str) {
    jni_high::android::notifications::send(title, body, "cedinia_scan", "Scan notifications", 1);
}

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
            log::error!("Failed to send desktop notification: {e}");
        }
    });
}

#[cfg(target_os = "linux")]
fn try_notify_send(summary: &str, body: &str) -> bool {
    match std::process::Command::new("notify-send").arg("--app-name=cedinia").arg(summary).arg(body).status() {
        Ok(s) if s.success() => true,
        Err(e) => {
            log::error!("Failed to execute notify-send: {e}");
            false
        }
        Ok(failed) => {
            log::error!("notify-send exited with non-zero status: {failed}");
            false
        }
    }
}
