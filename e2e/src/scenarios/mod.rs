use std::path::PathBuf;

use crate::report::Totals;

mod cli_scan;
mod common;
mod renderers;
mod window_size;

pub struct Context {
    pub krokiet: PathBuf,
    pub artifacts_dir: PathBuf,
}

const BACKENDS: [&str; 2] = ["winit-software", "winit-femtovg"];
const RENDERERS: [renderers::Renderer; 3] = [
    renderers::Renderer {
        backend: "winit-femtovg",
        log_marker: "OpenGL",
    },
    renderers::Renderer {
        backend: "femtovg-wgpu",
        log_marker: "wgpu",
    },
    renderers::Renderer {
        backend: "vello",
        log_marker: "unavailable",
    },
];
const SCALES: [u32; 2] = [1, 2];

pub fn run_all(context: &Context, filter: Option<&str>) -> Totals {
    let mut totals = Totals::default();
    let selected = |name: &str| filter.is_none_or(|f| name.contains(f));

    if selected("window_size") {
        for backend in BACKENDS {
            for scale in SCALES {
                totals.add(window_size::run(context, backend, scale));
            }
        }
    }
    if selected("renderers") {
        for renderer in &RENDERERS {
            totals.add(renderers::run(context, renderer));
        }
    }
    if selected("cli_scan") {
        totals.add(cli_scan::run(context));
    }
    totals
}
