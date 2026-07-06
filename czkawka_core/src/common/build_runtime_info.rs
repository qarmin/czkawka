use std::sync::{OnceLock, RwLock};

/// Information about which features were compiled in and which pass a runtime probe.
///
/// `*_build` reflects compile-time feature flags; it never changes.
/// `*_runtime` reflects an actual probe: decode a tiny embedded test image (or spawn a
/// subprocess) to confirm the feature works end-to-end on this machine.  Before background
/// probing completes, all `*_runtime` fields are `false` and `probes_complete` is `false`.
///
/// `refresh_process_probes()` re-runs the ffmpeg/ffprobe checks so the user can install
/// those tools and have the app notice without restarting.  Library-based probes (HEIF,
/// LibRAW, AVIF) always reflect startup state - the shared libraries are dlopen'd once and
/// cannot be reloaded mid-process.
#[derive(Debug, Clone)]
pub struct BuildRuntimeInfo {
    // --- compile-time ---
    pub heif_build: bool,
    pub libraw_build: bool,
    pub libavif_build: bool,

    // --- runtime-probed ---
    // HEVC decode requires libheif-plugin-libde265; AV1 decode requires libheif-plugin-aomdec or libheif-plugin-dav1d.
    pub heif_runtime_hevc: bool,
    pub heif_runtime_av1: bool,
    pub libraw_runtime: bool,
    pub libavif_runtime: bool,
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
    ///
    /// Must be called AFTER `czkawka_core::common::image::register_image_decoding_hooks()`
    /// so the HEIF decoder hook is in place before the HEIF probe runs.
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
    ///
    /// Must be called AFTER `register_image_decoding_hooks()`.
    pub fn detect_and_store() {
        let probed = Self::detect();
        let lock = INFO.get_or_init(|| RwLock::new(Self::defaults()));
        let mut info = lock.write().expect("BuildRuntimeInfo lock poisoned");
        *info = probed;
    }

    /// Re-probe ffmpeg and ffprobe availability and store the result.
    ///
    /// Library-based probes (HEIF, LibRAW, AVIF) are intentionally excluded: those shared
    /// libraries are loaded once at process startup and cannot be reloaded without a restart.
    pub fn refresh_process_probes() {
        let lock = INFO.get_or_init(|| RwLock::new(Self::defaults()));
        let mut info = lock.write().expect("BuildRuntimeInfo lock poisoned");
        info.ffmpeg_runtime = Self::probe_process("ffmpeg");
        info.ffprobe_runtime = Self::probe_process("ffprobe");
    }

    /// Logs a compact one-line summary of runtime probe results via `log::info!`.
    pub fn log_runtime_summary(&self) {
        let yn = |b: bool| if b { "yes" } else { "no" };
        log::info!(
            "Runtime probes: HEIF-HEVC={} HEIF-AV1={} LibRAW={} AVIF={} FFmpeg={} FFprobe={}",
            yn(self.heif_runtime_hevc),
            yn(self.heif_runtime_av1),
            yn(self.libraw_runtime),
            yn(self.libavif_runtime),
            yn(self.ffmpeg_runtime),
            yn(self.ffprobe_runtime),
        );
    }

    /// Returns a multi-line string with the full version line and runtime probe results,
    /// suitable for copying into a bug report.
    pub fn format_diagnostic_text(&self, app: &str) -> String {
        use crate::common::logger::format_version_string;
        let yn = |b: bool| if b { "yes" } else { "no" };
        let pending = if self.probes_complete { "" } else { " (pending...)" };
        let mut text = format!(
            "{}\n\nRuntime probes{}:\n  HEIF HEVC: build={}, runtime={}\n  HEIF AV1:  build={}, runtime={}\n  LibRAW:    build={}, runtime={}\n  AVIF:      build={}, runtime={}\n  FFmpeg:    runtime={}\n  FFprobe:   runtime={}",
            format_version_string(app),
            pending,
            yn(self.heif_build),
            yn(self.heif_runtime_hevc),
            yn(self.heif_build),
            yn(self.heif_runtime_av1),
            yn(self.libraw_build),
            yn(self.libraw_runtime),
            yn(self.libavif_build),
            yn(self.libavif_runtime),
            yn(self.ffmpeg_runtime),
            yn(self.ffprobe_runtime),
        );
        if self.probes_complete {
            if self.heif_build && !self.heif_runtime_hevc {
                text.push_str("\n\nNote: HEIF HEVC runtime probe failed - install libheif-plugin-libde265 to decode .heic files.");
            }
            if self.heif_build && !self.heif_runtime_av1 {
                text.push_str("\n\nNote: HEIF AV1 runtime probe failed - install libheif-plugin-dav1d or libheif-plugin-aomdec to decode AV1 HEIF/AVIF files.");
            }
        }
        text
    }

    fn defaults() -> Self {
        Self {
            heif_build: cfg!(feature = "heif"),
            libraw_build: cfg!(feature = "libraw"),
            libavif_build: cfg!(feature = "libavif"),
            heif_runtime_hevc: false,
            heif_runtime_av1: false,
            libraw_runtime: false,
            libavif_runtime: false,
            ffmpeg_runtime: false,
            ffprobe_runtime: false,
            probes_complete: false,
        }
    }

    fn detect() -> Self {
        Self {
            heif_build: cfg!(feature = "heif"),
            libraw_build: cfg!(feature = "libraw"),
            libavif_build: cfg!(feature = "libavif"),
            heif_runtime_hevc: Self::probe_heif_hevc(),
            heif_runtime_av1: Self::probe_heif_av1(),
            libraw_runtime: Self::probe_libraw(),
            libavif_runtime: Self::probe_libavif(),
            ffmpeg_runtime: Self::probe_process("ffmpeg"),
            ffprobe_runtime: Self::probe_process("ffprobe"),
            probes_complete: true,
        }
    }

    #[cfg(feature = "heif")]
    fn probe_heif_hevc() -> bool {
        use crate::common::image::{ImageType, get_dynamic_image_from_bytes};
        let bytes: &[u8] = include_bytes!("test_assets/test_3x3.heic");
        get_dynamic_image_from_bytes(bytes, ImageType::Normal).is_ok()
    }

    #[cfg(not(feature = "heif"))]
    fn probe_heif_hevc() -> bool {
        false
    }

    #[cfg(feature = "heif")]
    fn probe_heif_av1() -> bool {
        use crate::common::image::{ImageType, get_dynamic_image_from_bytes};
        let bytes: &[u8] = include_bytes!("test_assets/test_heif_av1.avif");
        get_dynamic_image_from_bytes(bytes, ImageType::Normal).is_ok()
    }

    #[cfg(not(feature = "heif"))]
    fn probe_heif_av1() -> bool {
        false
    }

    #[cfg(feature = "libraw")]
    fn probe_libraw() -> bool {
        use crate::common::image::{ImageType, get_dynamic_image_from_bytes};
        let bytes: &[u8] = include_bytes!("test_assets/test_raw.dng");
        get_dynamic_image_from_bytes(bytes, ImageType::Raw).is_ok()
    }

    #[cfg(not(feature = "libraw"))]
    fn probe_libraw() -> bool {
        false
    }

    #[cfg(feature = "libavif")]
    fn probe_libavif() -> bool {
        let bytes: &[u8] = include_bytes!("test_assets/test_3x3.avif");
        image::load_from_memory(bytes).is_ok()
    }

    #[cfg(not(feature = "libavif"))]
    fn probe_libavif() -> bool {
        false
    }

    fn probe_process(name: &str) -> bool {
        use std::process::Stdio;

        use crate::common::process_utils::disable_windows_console_window;

        let mut cmd = std::process::Command::new(name);
        disable_windows_console_window(&mut cmd);
        cmd.arg("-version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok_and(|s| s.success())
    }
}
