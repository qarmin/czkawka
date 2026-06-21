use log::warn;

pub(super) fn set_clipboard(text: String) {
    // X11/Wayland clipboard ownership must be held until another app reads the content.
    // SetExtLinux::wait() blocks the thread until that happens, so we run it in the background.
    std::thread::spawn(move || {
        use arboard::Clipboard;
        #[cfg(target_os = "linux")]
        use arboard::SetExtLinux as _;
        match Clipboard::new() {
            Ok(mut ctx) => {
                #[cfg(target_os = "linux")]
                let result = ctx.set().wait().text(text);
                #[cfg(not(target_os = "linux"))]
                let result = ctx.set_text(text);
                if let Err(e) = result {
                    warn!("Failed to set clipboard contents: {e}");
                }
            }
            Err(e) => warn!("Failed to create clipboard context: {e}"),
        }
    });
}
