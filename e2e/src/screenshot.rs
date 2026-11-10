use std::path::Path;
use std::process::Command;

/// Saves the whole screen, or a single window when its X id is given
pub fn save(display: &str, window: Option<u32>, path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
    }
    let target = window.map_or("root".to_string(), |id| format!("0x{id:x}"));
    let status = Command::new("import")
        .args(["-display", display, "-window", &target])
        .arg(path)
        .status()
        .map_err(|e| format!("cannot run import (ImageMagick): {e}"))?;
    status.success().then_some(()).ok_or_else(|| format!("import exited with {status}"))
}

pub fn unique_colors(path: &Path) -> Result<usize, String> {
    let output = Command::new("identify")
        .args(["-format", "%k"])
        .arg(path)
        .output()
        .map_err(|e| format!("cannot run identify (ImageMagick): {e}"))?;
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .map_err(|e| format!("unexpected identify output: {e}"))
}
