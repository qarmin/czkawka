use std::fs;
use std::time::Duration;

use crate::krokiet::{Krokiet, Launch};
use crate::report::{Abort, Scenario, Totals};
use crate::scenarios::Context;
use crate::workspace::Workspace;
use crate::xvfb::Xvfb;

const SCAN_TIMEOUT: Duration = Duration::from_secs(60);
const DUPLICATE_CONTENT: &[u8] = &[7; 64 * 1024];
const UNIQUE_CONTENT: &[u8] = &[9; 64 * 1024];

/// Scan started from the command line finishes, closes the app and leaves the scanned files untouched
pub fn run(context: &Context) -> Totals {
    let mut scenario = Scenario::new("cli scan: duplicates".to_string(), context.artifacts_dir.clone());
    execute(&mut scenario, context).ok();
    scenario.finish()
}

fn execute(sc: &mut Scenario, context: &Context) -> Result<(), Abort> {
    let workspace = sc.step("create temporary HOME", Workspace::new())?;
    let files = [("dup_a.bin", DUPLICATE_CONTENT), ("dup_b.bin", DUPLICATE_CONTENT), ("unique.bin", UNIQUE_CONTENT)];
    for (name, content) in files {
        sc.step(&format!("create {name}"), workspace.write_file(name, content))?;
    }
    let xvfb = sc.step("start X server", Xvfb::start())?;
    sc.set_display(&xvfb.display);

    let args = ["-t", "duplicates", "-s", "-x", &workspace.data_dir().to_string_lossy()].map(String::from).to_vec();
    let launch = Launch {
        binary: &context.krokiet,
        display: &xvfb.display,
        workspace: &workspace,
        backend: "winit-software",
        scale: 1,
        args,
        extra_env: Vec::new(),
    };
    let mut app = sc.step("start app with scan arguments", Krokiet::start(&launch))?;
    let status = sc.step("app exits after the scan", app.wait_for_exit(SCAN_TIMEOUT))?;
    sc.check("app exit code is 0", status.success().then_some(()).ok_or(format!("exited with {status}")));

    for (name, content) in files {
        let path = workspace.data_dir().join(name);
        let unchanged = fs::read(&path)
            .map_err(|e| e.to_string())
            .and_then(|actual| (actual == content).then_some(()).ok_or("content changed".to_string()));
        sc.check(&format!("{name} is untouched"), unchanged);
    }
    sc.check("settings are saved", workspace.config_file().is_file().then_some(()).ok_or("no config_general.json"));
    Ok(())
}
