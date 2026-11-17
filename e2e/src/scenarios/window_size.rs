use std::fs;
use std::path::Path;
use std::time::Duration;

use crate::krokiet::Launch;
use crate::report::{Abort, Scenario, Totals};
use crate::scenarios::Context;
use crate::scenarios::common::{close_cleanly, is_rendered, start_and_find_window};
use crate::wait;
use crate::workspace::Workspace;
use crate::x11::X11;
use crate::xvfb::Xvfb;

const LOGICAL_SIZE: (u32, u32) = (1000, 700);
const RESTARTS: usize = 2;

/// Window resized by the user keeps the same size after every restart, on any scale factor
pub fn run(context: &Context, backend: &str, scale: u32) -> Totals {
    let mut scenario = Scenario::new(format!("window size: {backend}, scale {scale}"), context.artifacts_dir.clone());
    execute(&mut scenario, context, backend, scale).ok();
    scenario.finish()
}

fn execute(sc: &mut Scenario, context: &Context, backend: &str, scale: u32) -> Result<(), Abort> {
    let workspace = sc.step("create temporary HOME", Workspace::new())?;
    let xvfb = sc.step("start X server", Xvfb::start())?;
    sc.set_display(&xvfb.display);
    let x11 = sc.step("connect to X server", X11::connect(&xvfb.display))?;
    let launch = Launch {
        binary: &context.krokiet,
        display: &xvfb.display,
        workspace: &workspace,
        backend,
        scale,
        args: Vec::new(),
        extra_env: Vec::new(),
    };
    let expected = (LOGICAL_SIZE.0 * scale, LOGICAL_SIZE.1 * scale);

    let (mut app, window) = start_and_find_window(sc, &launch, &x11)?;
    sc.check("window is rendered", is_rendered(&xvfb.display, window, &workspace));
    sc.step("resize window", x11.resize(window, expected.0, expected.1))?;
    sc.check("window has the requested size", wait_for_size(&x11, window, expected));
    close_cleanly(sc, &mut app, &x11, window)?;
    sc.check("settings store logical size", check_saved_logical_size(&workspace.config_file()));

    for restart in 1..=RESTARTS {
        let (mut app, window) = start_and_find_window(sc, &launch, &x11)?;
        sc.check(&format!("size after restart {restart} is unchanged"), wait_for_size(&x11, window, expected));
        close_cleanly(sc, &mut app, &x11, window)?;
    }
    Ok(())
}

/// Size changes arrive asynchronously, so poll instead of reading once
fn wait_for_size(x11: &X11, window: u32, expected: (u32, u32)) -> Result<(), String> {
    let mut last = (0, 0);
    let result = wait::until(Duration::from_secs(5), "the expected window size", || {
        last = x11.size(window).map_err(|e| e.to_string())?;
        Ok((last == expected).then_some(()))
    });
    result.map_err(|_| format!("expected {}x{}, got {}x{}", expected.0, expected.1, last.0, last.1))
}

fn check_saved_logical_size(config_file: &Path) -> Result<(), String> {
    let config = fs::read_to_string(config_file).map_err(|e| format!("cannot read {}: {e}", config_file.display()))?;
    let width = json_number(&config, "window_width")?;
    let height = json_number(&config, "window_height")?;
    ((width, height) == LOGICAL_SIZE)
        .then_some(())
        .ok_or(format!("expected {}x{}, saved {width}x{height}", LOGICAL_SIZE.0, LOGICAL_SIZE.1))
}

fn json_number(json: &str, key: &str) -> Result<u32, String> {
    let after_key = json.split_once(&format!("\"{key}\"")).ok_or(format!("no \"{key}\" in settings"))?.1;
    let digits: String = after_key.chars().skip_while(|c| !c.is_ascii_digit()).take_while(char::is_ascii_digit).collect();
    digits.parse().map_err(|e| format!("cannot parse \"{key}\": {e}"))
}
