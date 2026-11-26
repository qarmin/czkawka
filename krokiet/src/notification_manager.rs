use std::sync::Mutex;
use std::thread::JoinHandle;

use log::error;

static PENDING_NOTIFICATIONS: Mutex<Vec<JoinHandle<()>>> = Mutex::new(Vec::new());

/// Must run before the first notification. Otherwise `mac-notification-sys` looks the sender up with AppleScript for
/// a non-existent app, which opens a "Where is use_default?" dialog and blocks until the user answers it.
/// Inside `Krokiet.app` its own bundle id is used - macOS 27 rejects notifications posted as Finder.
#[cfg(target_os = "macos")]
pub fn set_macos_notification_sender() {
    let sender = objc2_foundation::NSBundle::mainBundle()
        .bundleIdentifier()
        .map_or_else(|| "com.apple.Finder".to_string(), |id| id.to_string());
    match notify_rust::set_application(&sender) {
        Ok(()) => log::info!("Desktop notifications are sent as {sender}"),
        Err(e) => error!("Failed to set {sender} as the application that sends desktop notifications: {e}"),
    }
}

/// Sends from a separate thread - on macOS sending from the main thread spins a nested run loop while waiting
/// for delivery, which re-enters winit and aborts the app.
pub fn send_scan_completed_notification(tool: &str, body: &str) {
    let tool = tool.to_string();
    let body = body.to_string();
    match std::thread::Builder::new()
        .name("scan_notification".to_string())
        .spawn(move || send_notification_blocking(&tool, &body))
    {
        Ok(handle) => {
            let mut pending = PENDING_NOTIFICATIONS.lock().expect("notification thread list mutex poisoned");
            pending.retain(|handle| !handle.is_finished());
            pending.push(handle);
        }
        Err(e) => error!("Failed to spawn thread for desktop notification: {e}"),
    }
}

/// Lets notifications sent just before exit (e.g. `--scan --exit`) reach the system instead of dying with the process.
pub fn wait_for_pending_notifications() {
    let pending = std::mem::take(&mut *PENDING_NOTIFICATIONS.lock().expect("notification thread list mutex poisoned"));
    for handle in pending {
        if handle.join().is_err() {
            error!("Desktop notification thread panicked");
        }
    }
}

fn send_notification_blocking(tool: &str, body: &str) {
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
        Ok(_handle) => log::info!("Desktop notification sent"),
        Err(e) => error!("Failed to send desktop notification: {e}"),
    }
}

#[cfg(target_os = "linux")]
// TODO - linux on error - https://github.com/hoodie/notify-rust/issues/218
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
