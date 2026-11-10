use std::fs::OpenOptions;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::Duration;

use crate::wait;
use crate::workspace::Workspace;

pub struct Launch<'a> {
    pub binary: &'a Path,
    pub display: &'a str,
    pub workspace: &'a Workspace,
    pub backend: &'a str,
    pub scale: u32,
    pub args: Vec<String>,
    pub extra_env: Vec<(&'a str, &'a str)>,
}

pub struct Krokiet {
    child: Child,
}

impl Krokiet {
    pub fn start(launch: &Launch) -> Result<Self, String> {
        let home = launch.workspace.home();
        let log_path = launch.workspace.log_file();
        let log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| format!("cannot open {}: {e}", log_path.display()))?;
        let log_for_stdout = log.try_clone().map_err(|e| format!("cannot clone log handle: {e}"))?;

        let child = Command::new(launch.binary)
            .args(&launch.args)
            .env("DISPLAY", launch.display)
            .env_remove("WAYLAND_DISPLAY")
            .env("HOME", &home)
            .env("XDG_CONFIG_HOME", home.join(".config"))
            .env("XDG_CACHE_HOME", home.join(".cache"))
            .env("SLINT_BACKEND", launch.backend)
            .env("SLINT_SCALE_FACTOR", launch.scale.to_string())
            .env("RUST_LOG", "info")
            .envs(launch.extra_env.iter().copied())
            .stdout(Stdio::from(log_for_stdout))
            .stderr(Stdio::from(log))
            .spawn()
            .map_err(|e| format!("cannot run {}: {e}", launch.binary.display()))?;
        Ok(Self { child })
    }

    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    pub fn wait_for_exit(&mut self, timeout: Duration) -> Result<ExitStatus, String> {
        wait::until(timeout, "the app to exit", || self.child.try_wait().map_err(|e| format!("cannot check app status: {e}")))
    }
}

impl Drop for Krokiet {
    fn drop(&mut self) {
        self.child.kill().ok();
        self.child.wait().ok();
    }
}
