use std::sync::{OnceLock, RwLock};

/// Availability of the external tools (ffmpeg, ffprobe), checked by spawning them.
///
/// Before background probing completes, all `*_runtime` fields are `false` and `probes_complete` is `false`.
/// `refresh_process_probes()` re-runs the checks so the user can install those tools without restarting.
#[derive(Debug, Clone)]
pub struct BuildRuntimeInfo {
    pub ffmpeg_runtime: bool,
    pub ffprobe_runtime: bool,

    /// False until `start_background_probes` (or `detect_and_store`) completes all probes.
    pub probes_complete: bool,
}

static INFO: OnceLock<RwLock<BuildRuntimeInfo>> = OnceLock::new();

impl BuildRuntimeInfo {
    /// Returns a snapshot of the current info.
    ///
    /// Before `start_background_probes` or `detect_and_store` completes, all `*_runtime`
    /// fields are `false` and `probes_complete` is `false`.
    pub fn get() -> Self {
        INFO.get_or_init(|| RwLock::new(Self::defaults())).read().expect("BuildRuntimeInfo lock poisoned").clone()
    }

    /// Spawns a background thread that runs all probes and stores the result.
    /// `on_complete` is called from that thread once everything is stored.
    pub fn start_background_probes<F: FnOnce() + Send + 'static>(on_complete: F) {
        INFO.get_or_init(|| RwLock::new(Self::defaults()));
        std::thread::spawn(move || {
            let probed = Self::detect();
            {
                let lock = INFO.get().expect("initialized above");
                let mut info = lock.write().expect("BuildRuntimeInfo lock poisoned");
                *info = probed;
            }
            on_complete();
        });
    }

    /// Runs all probes synchronously and stores the result.  Suitable for CLI where
    /// blocking at startup is acceptable and `start_background_probes` is not needed.
    pub fn detect_and_store() {
        let probed = Self::detect();
        let lock = INFO.get_or_init(|| RwLock::new(Self::defaults()));
        let mut info = lock.write().expect("BuildRuntimeInfo lock poisoned");
        *info = probed;
    }

    /// Re-probe ffmpeg and ffprobe availability and store the result.
    pub fn refresh_process_probes() {
        let lock = INFO.get_or_init(|| RwLock::new(Self::defaults()));
        let mut info = lock.write().expect("BuildRuntimeInfo lock poisoned");
        info.ffmpeg_runtime = Self::probe_process("ffmpeg");
        info.ffprobe_runtime = Self::probe_process("ffprobe");
    }

    /// Logs a compact one-line summary of runtime probe results via `log::info!`.
    pub fn log_runtime_summary(&self) {
        let yn = |b: bool| if b { "yes" } else { "no" };
        log::info!("Runtime probes: FFmpeg={} FFprobe={}", yn(self.ffmpeg_runtime), yn(self.ffprobe_runtime),);
    }

    /// Returns a multi-line string with the full version line and runtime probe results,
    /// suitable for copying into a bug report.
    pub fn format_diagnostic_text(&self, app: &str) -> String {
        use crate::common::logger::format_version_string;
        let yn = |b: bool| if b { "yes" } else { "no" };
        let pending = if self.probes_complete { "" } else { " (pending...)" };
        format!(
            "{}\n\nRuntime probes{}:\n  FFmpeg:    runtime={}\n  FFprobe:   runtime={}",
            format_version_string(app),
            pending,
            yn(self.ffmpeg_runtime),
            yn(self.ffprobe_runtime),
        )
    }

    fn defaults() -> Self {
        Self {
            ffmpeg_runtime: false,
            ffprobe_runtime: false,
            probes_complete: false,
        }
    }

    fn detect() -> Self {
        Self {
            ffmpeg_runtime: Self::probe_process("ffmpeg"),
            ffprobe_runtime: Self::probe_process("ffprobe"),
            probes_complete: true,
        }
    }

    fn probe_process(name: &str) -> bool {
        use std::process::Stdio;

        use crate::common::process_utils::disable_windows_console_window;

        let mut cmd = std::process::Command::new(name);
        disable_windows_console_window(&mut cmd);
        cmd.arg("-version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok_and(|s| s.success())
    }
}
