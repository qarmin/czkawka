//! Live progress for batch runs.
//!
//! A repair is slow by nature - a single clip can spend its whole budget inside ffmpeg - so a
//! run over a directory is measured in minutes or hours. Collecting every report before
//! printing anything made that indistinguishable from a hang, which is the one failure mode a
//! long-running tool must never have.
//!
//! Two things fix it, and both matter. Results are printed the moment each file finishes, so
//! output starts immediately and never stops moving. Above them sits a bar that answers the
//! questions the result lines cannot: how many files are left, and how long that will take.
//!
//! The bar is drawn on stderr and the results on stdout, so `file_fixer . > log.txt` keeps a
//! clean log while the terminal still shows progress.

use std::io::{IsTerminal, Write};
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Redraw interval. Fast enough to feel live, slow enough that a run over thousands of small
/// files does not spend its time formatting escape sequences.
const REDRAW_EVERY: Duration = Duration::from_millis(100);
const BAR_WIDTH: usize = 24;
/// Leaves room for the counter, percentage, bar, rate and ETA within a narrow terminal.
const NAME_WIDTH: usize = 30;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    Auto,
    Always,
    Never,
}

impl Visibility {
    fn resolve(self) -> bool {
        match self {
            Self::Auto => std::io::stderr().is_terminal(),
            Self::Always => true,
            Self::Never => false,
        }
    }
}

/// Guarded state, kept behind one lock so the bar can never interleave with a result line.
struct Render {
    last_draw: Instant,
    /// Non-empty only while the bar is on screen and needs erasing before anything else prints.
    drawn: bool,
    current: String,
}

pub struct Progress {
    enabled: bool,
    total: usize,
    total_bytes: u64,
    done: AtomicUsize,
    done_bytes: AtomicU64,
    started: Instant,
    render: Mutex<Render>,
}

impl Progress {
    pub fn new(total: usize, total_bytes: u64, visibility: Visibility) -> Self {
        Self {
            enabled: visibility.resolve() && total > 0,
            total,
            total_bytes,
            done: AtomicUsize::new(0),
            done_bytes: AtomicU64::new(0),
            started: Instant::now(),
            render: Mutex::new(Render {
                last_draw: Instant::now(),
                drawn: false,
                current: String::new(),
            }),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }

    /// Names the file in the bar. With several workers the name shown is simply the most
    /// recent one to start, which is what a reader wants: evidence that work is moving.
    pub fn file_started(&self, path: &Path) {
        if !self.enabled {
            return;
        }
        let name = path
            .file_name()
            .map_or_else(|| path.display().to_string(), |n| n.to_string_lossy().into_owned());
        if let Ok(mut render) = self.render.lock() {
            render.current = name;
            self.draw(&mut render, false);
        }
    }

    /// Prints one finished file's line and advances the counters.
    ///
    /// Output order follows completion, not the input list, because a bar that only moves
    /// when the alphabetically-next file happens to finish would stall for minutes on one
    /// slow video while dozens of others complete behind it.
    pub fn file_finished(&self, bytes: u64, line: &str) {
        let done = self.done.fetch_add(1, Ordering::Relaxed) + 1;
        self.done_bytes.fetch_add(bytes, Ordering::Relaxed);
        // An empty line means the caller is reporting some other way - `--json` collects every
        // report and prints one document at the end - and a counter printed into the middle of
        // that document would make it unparseable.
        if line.is_empty() {
            self.advance();
            return;
        }
        self.print(&format!(
            "[{done:>width$}/{}] {line}",
            self.total,
            width = digits(self.total)
        ));
    }

    /// Redraws the bar without printing a result line.
    fn advance(&self) {
        if let Ok(mut render) = self.render.lock() {
            self.draw(&mut render, true);
        }
    }

    /// Writes a line to stdout without letting it collide with the bar.
    pub fn print(&self, line: &str) {
        let Ok(mut render) = self.render.lock() else {
            println!("{line}");
            return;
        };
        self.erase(&mut render);
        println!("{line}");
        self.draw(&mut render, true);
    }

    /// Takes the bar off the screen for good, before the closing summary is printed.
    pub fn finish(&self) {
        if let Ok(mut render) = self.render.lock() {
            self.erase(&mut render);
        }
    }

    fn erase(&self, render: &mut Render) {
        if !self.enabled || !render.drawn {
            return;
        }
        let mut stderr = std::io::stderr();
        let _ = write!(stderr, "\r\x1b[2K");
        let _ = stderr.flush();
        render.drawn = false;
    }

    fn draw(&self, render: &mut Render, force: bool) {
        if !self.enabled {
            return;
        }
        if !force && render.last_draw.elapsed() < REDRAW_EVERY {
            return;
        }
        render.last_draw = Instant::now();

        let done = self.done.load(Ordering::Relaxed);
        let fraction = done as f64 / self.total as f64;
        let filled = (fraction * BAR_WIDTH as f64).round().min(BAR_WIDTH as f64) as usize;
        let bar: String = "=".repeat(filled) + &" ".repeat(BAR_WIDTH - filled);

        let elapsed = self.started.elapsed();
        let done_bytes = self.done_bytes.load(Ordering::Relaxed);
        let rate = done_bytes as f64 / elapsed.as_secs_f64().max(0.001);

        let mut stderr = std::io::stderr();
        let _ = write!(
            stderr,
            "\r\x1b[2K[{done:>width$}/{total}] {percent:>3.0}% [{bar}] {rate}/s  ETA {eta}  {name}",
            width = digits(self.total),
            total = self.total,
            percent = fraction * 100.0,
            rate = bytes(rate as u64),
            eta = self.eta(done, done_bytes, elapsed),
            name = truncate(&render.current, NAME_WIDTH),
        );
        let _ = stderr.flush();
        render.drawn = true;
    }

    /// Estimates from bytes processed rather than files finished.
    ///
    /// File counts are a poor predictor here: a directory holding one 4 GB recording and a
    /// thousand thumbnails is 99.9% "done" by count while nearly all the work remains. Bytes
    /// track the actual cost closely enough to be worth showing, and the estimate is marked
    /// unknown until enough of the run is behind us to mean anything.
    fn eta(&self, done: usize, done_bytes: u64, elapsed: Duration) -> String {
        if done == 0 || done_bytes == 0 || self.total_bytes == 0 {
            return "--".to_string();
        }
        if done == self.total {
            return "0s".to_string();
        }
        let fraction = done_bytes as f64 / self.total_bytes as f64;
        if fraction <= 0.0 {
            return "--".to_string();
        }
        let projected = elapsed.as_secs_f64() / fraction;
        duration(Duration::from_secs_f64((projected - elapsed.as_secs_f64()).max(0.0)))
    }
}

fn digits(value: usize) -> usize {
    value.to_string().len()
}

fn truncate(text: &str, width: usize) -> String {
    let count = text.chars().count();
    if count <= width {
        return text.to_string();
    }
    // Keeps the tail, which is where a name like `IMG_0001.mp4` differs from its neighbours.
    let tail: String = text.chars().skip(count - width + 1).collect();
    format!("~{tail}")
}

pub fn bytes(value: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = value as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit + 1 < UNITS.len() {
        size /= 1024.0;
        unit += 1;
    }
    let name = UNITS.get(unit).copied().unwrap_or("B");
    if unit == 0 {
        format!("{value} {name}")
    } else {
        format!("{size:.1} {name}")
    }
}

pub fn duration(value: Duration) -> String {
    let seconds = value.as_secs();
    match seconds {
        0..60 => format!("{seconds}s"),
        60..3600 => format!("{}m{:02}s", seconds / 60, seconds % 60),
        _ => format!("{}h{:02}m", seconds / 3600, (seconds % 3600) / 60),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_are_reported_in_the_largest_unit_that_fits() {
        assert_eq!(bytes(512), "512 B");
        assert_eq!(bytes(1024), "1.0 KB");
        assert_eq!(bytes(27_848_255), "26.6 MB");
        assert_eq!(bytes(22 * 1024 * 1024 * 1024), "22.0 GB");
    }

    #[test]
    fn durations_stay_readable_at_every_scale() {
        assert_eq!(duration(Duration::from_secs(9)), "9s");
        assert_eq!(duration(Duration::from_secs(75)), "1m15s");
        assert_eq!(duration(Duration::from_secs(7325)), "2h02m");
    }

    /// A long name must still leave the counter and ETA readable, so it loses its head.
    #[test]
    fn long_names_are_truncated_from_the_front() {
        assert_eq!(truncate("short.mp4", 30), "short.mp4");
        let long = "a_very_long_recording_name_from_a_camera.mp4";
        let cut = truncate(long, 20);
        assert_eq!(cut.chars().count(), 20);
        assert!(cut.starts_with('~') && cut.ends_with("camera.mp4"));
    }

    /// Nothing may be drawn when the output is a pipe, or a log file fills with escapes.
    #[test]
    fn progress_is_silent_when_disabled() {
        let progress = Progress::new(10, 1000, Visibility::Never);
        assert!(!progress.enabled);
        progress.file_started(Path::new("x.mp4"));
        progress.finish();
    }

    #[test]
    fn an_empty_run_never_enables_the_bar() {
        assert!(!Progress::new(0, 0, Visibility::Always).enabled);
    }

    #[test]
    fn the_estimate_is_unknown_until_something_has_finished() {
        let progress = Progress::new(4, 4000, Visibility::Never);
        assert_eq!(progress.eta(0, 0, Duration::from_secs(1)), "--");
        assert_eq!(progress.eta(4, 4000, Duration::from_secs(1)), "0s");
    }

    /// Half the bytes in ten seconds means about ten seconds left.
    #[test]
    fn the_estimate_extrapolates_from_bytes_processed() {
        let progress = Progress::new(4, 4000, Visibility::Never);
        assert_eq!(progress.eta(2, 2000, Duration::from_secs(10)), "10s");
    }
}
