use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

pub struct Xvfb {
    child: Child,
    pub display: String,
}

impl Xvfb {
    pub fn start() -> Result<Self, String> {
        let mut child = Command::new("Xvfb")
            .args(["-displayfd", "1", "-screen", "0", "2560x1600x24", "-nolisten", "tcp"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("cannot run Xvfb: {e}"))?;
        let stdout = child.stdout.take().ok_or("Xvfb has no stdout")?;

        let mut display_number = String::new();
        BufReader::new(stdout)
            .read_line(&mut display_number)
            .map_err(|e| format!("cannot read display number: {e}"))?;
        let display_number = display_number.trim();
        if display_number.is_empty() {
            return Err("Xvfb exited before reporting its display".to_string());
        }
        Ok(Self {
            child,
            display: format!(":{display_number}"),
        })
    }
}

impl Drop for Xvfb {
    fn drop(&mut self) {
        self.child.kill().ok();
        self.child.wait().ok();
    }
}
