use czkawka_core::common::build_runtime_info::BuildRuntimeInfo;
use czkawka_core::common::consts::{HEIC_EXTENSIONS, RAW_IMAGE_EXTENSIONS};
use czkawka_core::common::image::get_dynamic_image_from_path;
use rfd::FileDialog;
use slint::ComponentHandle;

use crate::connect_rfd::{hide_file_dialog_overlay, show_file_dialog_overlay};
use crate::{Callabler, GuiState, MainWindow};

pub(crate) fn apply_build_info(app: &MainWindow) {
    set_all_build_info_properties(&app.global::<GuiState>(), &BuildRuntimeInfo::get());
}

pub(crate) fn connect_build_info(app: &MainWindow) {
    connect_refresh_probes(app);
    connect_test_image_file(app);
}

/// Spawns the background probe thread and updates all build-info properties in the UI
/// once detection finishes.  Call this after `apply_build_info` so defaults are visible
/// immediately while the slower probes run in the background.
pub(crate) fn start_build_info_background_probes(app: &MainWindow) {
    let weak = app.as_weak();
    BuildRuntimeInfo::start_background_probes(move || {
        let info = BuildRuntimeInfo::get();
        info.log_runtime_summary();
        weak.upgrade_in_event_loop(move |app| {
            set_all_build_info_properties(&app.global::<GuiState>(), &info);
        })
        .expect("MainWindow dropped while build info probes were running");
    });
}

fn connect_refresh_probes(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<Callabler>().on_refresh_process_probes(move || {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        let gs = app.global::<GuiState>();
        if gs.get_build_info_refresh_running() {
            return;
        }
        gs.set_build_info_refresh_running(true);

        let weak2 = weak.clone();
        std::thread::spawn(move || {
            BuildRuntimeInfo::refresh_process_probes();
            let info = BuildRuntimeInfo::get();
            weak2
                .upgrade_in_event_loop(move |app| {
                    let gs = app.global::<GuiState>();
                    // Only process probes can change at runtime; update just those two.
                    gs.set_build_info_ffmpeg_runtime(info.ffmpeg_runtime);
                    gs.set_build_info_ffprobe_runtime(info.ffprobe_runtime);
                    gs.set_build_info_diagnostic_text(info.format_diagnostic_text("Krokiet").into());
                    gs.set_build_info_refresh_running(false);
                })
                .expect("MainWindow dropped while callback is still live");
        });
    });
}

fn connect_test_image_file(app: &MainWindow) {
    let weak = app.as_weak();
    app.global::<Callabler>().on_test_image_file(move || {
        let app = weak.upgrade().expect("MainWindow dropped while callback is still live");
        let gs = app.global::<GuiState>();
        if gs.get_build_info_test_file_running() {
            return;
        }
        gs.set_build_info_test_file_running(true);

        show_file_dialog_overlay(&app);

        let weak2 = weak.clone();
        std::thread::spawn(move || {
            let mut dialog = FileDialog::new();
            if let Ok(dir) = std::env::current_dir() {
                dialog = dialog.set_directory(dir);
            }
            let picked = dialog.pick_file();

            hide_file_dialog_overlay(&weak2);

            let (result, is_ok) = match picked {
                None => {
                    // User cancelled - restore previous state without overwriting result
                    weak2
                        .upgrade_in_event_loop(move |app| {
                            app.global::<GuiState>().set_build_info_test_file_running(false);
                        })
                        .expect("MainWindow dropped while callback is still live");
                    return;
                }
                Some(path) => {
                    let file_name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                    let ext = path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
                    let decoder = if RAW_IMAGE_EXTENSIONS.contains(&ext.as_str()) {
                        if cfg!(feature = "libraw") { "LibRAW" } else { "rawler" }
                    } else if HEIC_EXTENSIONS.contains(&ext.as_str()) {
                        "libheif"
                    } else if ext == "avif" {
                        if cfg!(feature = "libavif") { "libavif" } else { "image-rs" }
                    } else {
                        "image-rs"
                    };

                    match get_dynamic_image_from_path(&path.to_string_lossy(), None) {
                        Ok(loaded) => {
                            let text = format!(
                                "OK - {file_name} - {}x{} {:?} - {decoder}",
                                loaded.original_width,
                                loaded.original_height,
                                loaded.image.color(),
                            );
                            (text, true)
                        }
                        Err(e) => (format!("Error - {file_name} - {e} - decoder: {decoder}"), false),
                    }
                }
            };

            weak2
                .upgrade_in_event_loop(move |app| {
                    let gs = app.global::<GuiState>();
                    gs.set_build_info_test_file_result(result.into());
                    gs.set_build_info_test_file_ok(is_ok);
                    gs.set_build_info_test_file_running(false);
                })
                .expect("MainWindow dropped while callback is still live");
        });
    });
}

fn set_all_build_info_properties(gs: &GuiState, info: &BuildRuntimeInfo) {
    gs.set_build_info_heif_build(info.heif_build);
    gs.set_build_info_libraw_build(info.libraw_build);
    gs.set_build_info_libavif_build(info.libavif_build);
    gs.set_build_info_heif_runtime_hevc(info.heif_runtime_hevc);
    gs.set_build_info_heif_runtime_av1(info.heif_runtime_av1);
    gs.set_build_info_libraw_runtime(info.libraw_runtime);
    gs.set_build_info_libavif_runtime(info.libavif_runtime);
    gs.set_build_info_ffmpeg_runtime(info.ffmpeg_runtime);
    gs.set_build_info_ffprobe_runtime(info.ffprobe_runtime);
    gs.set_build_info_probes_complete(info.probes_complete);
    gs.set_build_info_diagnostic_text(info.format_diagnostic_text("Krokiet").into());
}
