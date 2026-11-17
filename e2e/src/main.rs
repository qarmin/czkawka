mod krokiet;
mod report;
mod scenarios;
mod screenshot;
mod wait;
mod workspace;
mod x11;
mod xvfb;

use std::path::PathBuf;
use std::process::ExitCode;

use scenarios::Context;

const REQUIRED_TOOLS: [&str; 3] = ["Xvfb", "import", "identify"];

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(krokiet) = args.next().map(PathBuf::from) else {
        eprintln!("Usage: e2e <path to krokiet binary> [scenario name filter: window_size | renderers | cli_scan]");
        return ExitCode::from(2);
    };
    let filter = args.next();

    if let Err(missing) = check_required_tools() {
        eprintln!("Missing programs: {missing}. On Debian/Ubuntu: sudo apt install xvfb imagemagick");
        return ExitCode::from(2);
    }
    if !krokiet.is_file() {
        eprintln!("Krokiet binary not found: {}", krokiet.display());
        return ExitCode::from(2);
    }

    let artifacts_dir = std::env::var_os("E2E_ARTIFACTS").map_or_else(|| PathBuf::from("artifacts"), PathBuf::from);
    let context = Context {
        krokiet: krokiet.canonicalize().unwrap_or(krokiet),
        artifacts_dir,
    };
    let totals = scenarios::run_all(&context, filter.as_deref());
    report::print_summary(&totals);
    if totals.is_success() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}

fn check_required_tools() -> Result<(), String> {
    let path = std::env::var_os("PATH").unwrap_or_default();
    let missing: Vec<&str> = REQUIRED_TOOLS
        .into_iter()
        .filter(|tool| !std::env::split_paths(&path).any(|dir| dir.join(tool).is_file()))
        .collect();
    if missing.is_empty() { Ok(()) } else { Err(missing.join(", ")) }
}
