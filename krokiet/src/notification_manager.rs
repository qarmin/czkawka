use log::error;

pub fn send_scan_completed_notification(tool: &str, body: &str) {
    #[cfg(feature = "notifications")]
    {
        #[cfg(target_os = "linux")]
        if try_notify_send(tool, body) {
            return;
        }

        // Fallback: notify-rust (covers macOS, Windows and Linux without notify-send)
        let mut notif = notify_rust::Notification::new();
        notif.summary(tool).body(body);
        #[cfg(all(unix, not(target_os = "macos")))]
        notif.urgency(notify_rust::Urgency::Normal);
        match notif.show() {
            Ok(_) => log::info!("Desktop notification sent"),
            Err(e) => error!("Failed to send desktop notification: {e}"),
        }
    }
}

#[cfg(target_os = "linux")]
fn try_notify_send(summary: &str, body: &str) -> bool {
    match std::process::Command::new("notify-send").arg("--app-name=krokiet").arg(summary).arg(body).status() {
        Ok(s) if s.success() => {
            log::info!("Desktop notification sent via notify-send");
            true
        }
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
