use std::process::Command;

use log::{error, info, warn};

const SLINT_BACKEND_ENV: &str = "SLINT_BACKEND";
const ATTEMPT_ENV: &str = "KROKIET_RENDERER_ATTEMPT";

// Slint's own default order differs (wgpu before OpenGL, vello last), so the order has to be forced explicitly
const PREFERRED_RENDERERS: &[&str] = &[
    #[cfg(any(feature = "femtovg", feature = "winit_femtovg"))]
    "femtovg",
    #[cfg(feature = "femtovg_wgpu")]
    "femtovg-wgpu",
    #[cfg(feature = "vello")]
    "vello",
    #[cfg(any(feature = "software", feature = "winit_software"))]
    "software",
];

fn attempt_from_env() -> Option<usize> {
    std::env::var(ATTEMPT_ENV).ok()?.parse().ok()
}

/// Must be called before any other thread is spawned.
pub(crate) fn apply_preferred_renderer() {
    let attempt = match attempt_from_env() {
        Some(attempt) => attempt,
        None if std::env::var_os(SLINT_BACKEND_ENV).is_some() => return,
        None => 0,
    };
    let Some(renderer) = PREFERRED_RENDERERS.get(attempt) else {
        return;
    };
    // SAFETY: called at the start of main, before any other thread exists
    unsafe {
        std::env::set_var(SLINT_BACKEND_ENV, format!("winit-{renderer}"));
        std::env::set_var(ATTEMPT_ENV, attempt.to_string());
    }
}

/// Slint can select a platform only once per process, so a failed renderer is replaced by restarting the app.
/// Returns the exit code of the restarted process, or `None` when there is nothing left to try.
pub(crate) fn restart_with_next_renderer() -> Option<i32> {
    let attempt = attempt_from_env()?;
    let next_renderer = PREFERRED_RENDERERS.get(attempt + 1)?;
    warn!("Renderer \"{}\" failed to start, restarting with \"{next_renderer}\"", PREFERRED_RENDERERS[attempt]);

    let exe = std::env::current_exe()
        .inspect_err(|e| error!("Failed to get path of current executable, cannot try next renderer: {e}"))
        .ok()?;
    let status = Command::new(exe)
        .args(std::env::args_os().skip(1))
        .env(ATTEMPT_ENV, (attempt + 1).to_string())
        .env_remove(SLINT_BACKEND_ENV)
        .status()
        .inspect_err(|e| error!("Failed to restart with renderer \"{next_renderer}\": {e}"))
        .ok()?;
    info!("Restarted process with renderer \"{next_renderer}\" finished with {status}");
    Some(status.code().unwrap_or(1))
}
