use std::time::Duration;

use crate::krokiet::{Krokiet, Launch};
use crate::report::{Abort, Scenario};
use crate::workspace::Workspace;
use crate::x11::X11;
use crate::{screenshot, wait};

const WINDOW_TIMEOUT: Duration = Duration::from_secs(30);
const EXIT_TIMEOUT: Duration = Duration::from_secs(15);
const RENDER_TIMEOUT: Duration = Duration::from_secs(10);
const MIN_UNIQUE_COLORS: usize = 10;

pub fn start_and_find_window(sc: &mut Scenario, launch: &Launch, x11: &X11) -> Result<(Krokiet, u32), Abort> {
    let app = sc.step("start app", Krokiet::start(launch))?;
    let window = sc.step(
        "window appears",
        wait::until(WINDOW_TIMEOUT, "the app window", || x11.find_window_by_pid(app.pid()).map_err(|e| e.to_string())),
    )?;
    Ok((app, window))
}

pub fn close_cleanly(sc: &mut Scenario, app: &mut Krokiet, x11: &X11, window: u32) -> Result<(), Abort> {
    sc.step("request window close", x11.request_close(window))?;
    let status = sc.step("app exits", app.wait_for_exit(EXIT_TIMEOUT))?;
    sc.check("app exit code is 0", status.success().then_some(()).ok_or(format!("exited with {status}")));
    Ok(())
}

/// The first frame is drawn some time after the window is mapped, so poll instead of taking one screenshot
pub fn is_rendered(display: &str, window: u32, workspace: &Workspace) -> Result<(), String> {
    let path = workspace.data_dir().join("window.png");
    let mut colors = 0;
    wait::until(RENDER_TIMEOUT, "the window content", || {
        screenshot::save(display, Some(window), &path)?;
        colors = screenshot::unique_colors(&path)?;
        Ok((colors >= MIN_UNIQUE_COLORS).then_some(()))
    })
    .map_err(|e| format!("{e}, last screenshot had only {colors} unique colors"))
}
