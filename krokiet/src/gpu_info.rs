use std::ffi::{CStr, c_char, c_void};

use log::{debug, info};
use slint::{GraphicsAPI, RenderingState, Window};

#[cfg(any(feature = "vello", feature = "femtovg_wgpu"))]
macro_rules! log_wgpu_adapter {
    ($api_name:literal, $device:expr) => {{
        let adapter = $device.adapter_info();
        info!(
            "Graphics: {} {:?}, {}, driver {} {}",
            $api_name, adapter.backend, adapter.name, adapter.driver, adapter.driver_info
        );
    }};
}

const GL_RENDERER: u32 = 0x1F01;
const GL_VERSION: u32 = 0x1F02;

pub(crate) fn log_session_info() {
    let vars: Vec<String> = ["SLINT_BACKEND", "XDG_SESSION_TYPE", "XDG_CURRENT_DESKTOP", "WAYLAND_DISPLAY", "DISPLAY"]
        .into_iter()
        .filter_map(|var| Some(format!("{var}={}", std::env::var_os(var)?.to_string_lossy())))
        .collect();
    info!("Environment: {}", vars.join(", "));
}

/// Logs the graphics driver and device once the renderer has created its context
pub(crate) fn log_graphics_info_on_renderer_setup(window: &Window) {
    let result = window.set_rendering_notifier(|state, api| {
        if matches!(state, RenderingState::RenderingSetup) {
            log_graphics_api(api);
        }
    });
    if let Err(e) = result {
        info!("Graphics: unavailable for this renderer ({e})");
    }
}

fn log_graphics_api(api: &GraphicsAPI) {
    match api {
        GraphicsAPI::NativeOpenGL { get_proc_address } => log_opengl_strings(get_proc_address),
        #[cfg(feature = "vello")]
        GraphicsAPI::WGPU29 { device, .. } => log_wgpu_adapter!("wgpu 29", device),
        #[cfg(feature = "femtovg_wgpu")]
        GraphicsAPI::WGPU30 { device, .. } => log_wgpu_adapter!("wgpu 30", device),
        _ => debug!("Unsupported graphics API, GPU info is unavailable"),
    }
}

fn log_opengl_strings(get_proc_address: &dyn Fn(&CStr) -> *const c_void) {
    let get_string = get_proc_address(c"glGetString");
    if get_string.is_null() {
        debug!("glGetString is not available, OpenGL info is unavailable");
        return;
    }
    // SAFETY: glGetString has this signature in every OpenGL/GLES version and the context is current during RenderingSetup
    let get_string: unsafe extern "system" fn(u32) -> *const c_char = unsafe { std::mem::transmute(get_string) };
    let read = |name| {
        // SAFETY: a non-null result is a static NUL-terminated string owned by the driver
        let ptr = unsafe { get_string(name) };
        if ptr.is_null() {
            "<unknown>".into()
        } else {
            // SAFETY: same non-null NUL-terminated driver string as above
            unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
        }
    };
    info!("Graphics: OpenGL, {}, {}", read(GL_RENDERER), read(GL_VERSION));
}
