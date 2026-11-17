use std::fmt::Display;
use std::io::IsTerminal;
use std::path::PathBuf;

use crate::screenshot;

/// Returned by a failed fatal step, so `?` ends the scenario
pub struct Abort;

#[derive(Default, Clone, Copy)]
pub struct Totals {
    pub ok: usize,
    pub failed: usize,
    pub expected_fail: usize,
    pub unexpected_ok: usize,
}

impl Totals {
    pub fn add(&mut self, other: Totals) {
        self.ok += other.ok;
        self.failed += other.failed;
        self.expected_fail += other.expected_fail;
        self.unexpected_ok += other.unexpected_ok;
    }

    pub fn is_success(&self) -> bool {
        self.failed == 0 && self.unexpected_ok == 0
    }
}

pub struct Scenario {
    name: String,
    totals: Totals,
    display: Option<String>,
    artifacts_dir: PathBuf,
}

impl Scenario {
    pub fn new(name: String, artifacts_dir: PathBuf) -> Self {
        println!("\n=== {name} ===");
        Self {
            name,
            totals: Totals::default(),
            display: None,
            artifacts_dir,
        }
    }

    /// Enables screenshots of the X display on failed steps
    pub fn set_display(&mut self, display: &str) {
        self.display = Some(display.to_string());
    }

    pub fn finish(self) -> Totals {
        self.totals
    }

    /// Step the rest of the scenario depends on - a failure aborts the scenario
    pub fn step<T, E: Display>(&mut self, name: &str, result: Result<T, E>) -> Result<T, Abort> {
        match result {
            Ok(value) => {
                self.record_ok(name);
                Ok(value)
            }
            Err(e) => {
                self.record_fail(name, &e.to_string());
                Err(Abort)
            }
        }
    }

    /// Independent assertion - a failure is recorded and the scenario continues
    pub fn check<E: Display>(&mut self, name: &str, result: Result<(), E>) {
        match result {
            Ok(()) => self.record_ok(name),
            Err(e) => self.record_fail(name, &e.to_string()),
        }
    }

    /// Assertion that is known to fail until the linked problem is fixed
    #[allow(dead_code)] // no known failing step at the moment
    pub fn check_known_issue<E: Display>(&mut self, name: &str, issue: &str, result: Result<(), E>) {
        match result {
            Ok(()) => {
                println!("{} {name} - passes now, remove the known issue marker ({issue})", paint("[UNEXPECTED OK]", "35"));
                self.totals.unexpected_ok += 1;
            }
            Err(e) => {
                println!("{} {name} ({issue}): {e}", paint("[EXPECTED FAIL]", "33"));
                self.totals.expected_fail += 1;
            }
        }
    }

    fn record_ok(&mut self, name: &str) {
        println!("{} {name}", paint("[OK]", "32"));
        self.totals.ok += 1;
    }

    fn record_fail(&mut self, name: &str, reason: &str) {
        println!("{} {name}: {reason}", paint("[FAIL]", "31"));
        self.totals.failed += 1;
        self.save_screenshot();
    }

    fn save_screenshot(&self) {
        let Some(display) = &self.display else { return };
        let file_name = format!("{}-{}.png", self.name.replace([' ', ',', ':'], "_"), self.totals.failed);
        let path = self.artifacts_dir.join(file_name);
        match screenshot::save(display, None, &path) {
            Ok(()) => println!("      screenshot: {}", path.display()),
            Err(e) => println!("      screenshot failed: {e}"),
        }
    }
}

pub fn print_summary(totals: &Totals) {
    println!(
        "\n{} OK, {} EXPECTED FAIL, {} UNEXPECTED OK, {} FAIL",
        totals.ok, totals.expected_fail, totals.unexpected_ok, totals.failed
    );
}

fn paint(text: &str, color_code: &str) -> String {
    if std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
        format!("\x1b[{color_code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}
