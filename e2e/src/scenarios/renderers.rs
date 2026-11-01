use crate::krokiet::Launch;
use crate::report::{Abort, Scenario, Totals};
use crate::scenarios::Context;
use crate::scenarios::common::{close_cleanly, is_rendered, start_and_find_window};
use crate::workspace::Workspace;
use crate::x11::X11;
use crate::xvfb::Xvfb;

/// Text of the startup graphics log line that proves the requested renderer is the one in use
pub struct Renderer {
    pub backend: &'static str,
    pub log_marker: &'static str,
}

/// Every renderer starts, draws the window and closes cleanly
pub fn run(context: &Context, renderer: &Renderer) -> Totals {
    let mut scenario = Scenario::new(format!("renderer: {}", renderer.backend), context.artifacts_dir.clone());
    execute(&mut scenario, context, renderer).ok();
    scenario.finish()
}

fn execute(sc: &mut Scenario, context: &Context, renderer: &Renderer) -> Result<(), Abort> {
    let workspace = sc.step("create temporary HOME", Workspace::new())?;
    let xvfb = sc.step("start X server", Xvfb::start())?;
    sc.set_display(&xvfb.display);
    let x11 = sc.step("connect to X server", X11::connect(&xvfb.display))?;
    let launch = Launch {
        binary: &context.krokiet,
        display: &xvfb.display,
        workspace: &workspace,
        backend: renderer.backend,
        scale: 1,
        args: Vec::new(),
        // Xvfb has only a software Vulkan device, which wgpu ignores without this
        extra_env: vec![("SLINT_WGPU_CPU", "1")],
    };

    let (mut app, window) = start_and_find_window(sc, &launch, &x11)?;
    sc.check("window is rendered", is_rendered(&xvfb.display, window, &workspace));
    sc.check("log shows the requested renderer", log_contains(&workspace, renderer.log_marker));
    close_cleanly(sc, &mut app, &x11, window)
}

fn log_contains(workspace: &Workspace, marker: &str) -> Result<(), String> {
    let log = workspace.read_log()?;
    let graphics_lines: Vec<&str> = log.lines().filter(|line| line.contains("Graphics:")).collect();
    graphics_lines
        .iter()
        .any(|line| line.contains(marker))
        .then_some(())
        .ok_or(format!("no \"Graphics:\" line with \"{marker}\", got {graphics_lines:?}"))
}
