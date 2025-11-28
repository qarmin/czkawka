use std::time::{Duration, Instant};

/// Timer for measuring elapsed time between checkpoints.
///
/// # How to use - examples
///
/// Basic usage:
/// ```
/// use czkawka_core::helpers::debug_timer::Timer;
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// let mut timer = Timer::new("MyTimer");
/// sleep(Duration::from_millis(50));
/// timer.checkpoint("step1");
/// sleep(Duration::from_millis(30));
/// timer.checkpoint("step2");
/// let report = timer.report("all_steps", false);
/// println!("{}", report);
/// ```
///
/// Output example:
/// ```text
/// MyTimer - step1: 50.0ms,
/// MyTimer - step2: 30.0ms,
/// MyTimer - all_steps: 80.0ms
/// ```
///
/// One-line output:
/// ```
/// use czkawka_core::helpers::debug_timer::Timer;
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// let mut timer = Timer::new("MyTimer");
/// sleep(Duration::from_millis(10));
/// timer.checkpoint("a");
/// sleep(Duration::from_millis(20));
/// timer.checkpoint("b");
/// let report = timer.report("total", true);
/// println!("{}", report);
/// ```
///
/// Output example:
/// ```text
/// MyTimer - a: 10.0ms, b: 20.0ms, total: 30.0ms
/// ```
pub struct Timer {
    /// Name or label for the timer.
    base: String,
    /// Time when the timer was started.
    start_time: Instant,
    /// Time of the last checkpoint.
    last_time: Instant,
    /// List of (checkpoint name, duration since last checkpoint).
    times: Vec<(String, Duration)>,
}

impl Timer {
    /// Creates a new timer with a given label.
    pub fn new(base: &str) -> Self {
        Self {
            base: base.to_string(),
            start_time: Instant::now(),
            last_time: Instant::now(),
            times: Vec::new(),
        }
    }

    /// Records a checkpoint with the given name.
    pub fn checkpoint(&mut self, name: &str) {
        let elapsed = self.last_time.elapsed();
        self.times.push((name.to_string(), elapsed));
        self.last_time = Instant::now();
    }

    /// Returns a formatted report of all checkpoints and total time.
    ///
    /// If `in_one_line` is true, outputs all checkpoints in a single line.
    /// Otherwise, outputs each checkpoint on a separate line.
    pub fn report(&mut self, all_steps_name: &str, in_one_line: bool) -> String {
        let all_elapsed = self.start_time.elapsed();
        self.times.push((all_steps_name.to_string(), all_elapsed));

        if in_one_line {
            let times = self.times.iter().map(|(name, time)| format!("{name}: {time:?}")).collect::<Vec<_>>().join(", ");
            format!("{} - {}", self.base, times)
        } else {
            self.times
                .iter()
                .map(|(name, time)| format!("{} - {name}: {time:?}", self.base))
                .collect::<Vec<_>>()
                .join(", \n")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn test_timer_basic_functionality() {
        let mut timer = Timer::new("TestTimer");
        assert_eq!(timer.base, "TestTimer");
        assert_eq!(timer.times.len(), 0);

        sleep(Duration::from_millis(10));
        timer.checkpoint("step1");
        assert_eq!(timer.times.len(), 1);
        assert_eq!(timer.times[0].0, "step1");

        sleep(Duration::from_millis(10));
        timer.checkpoint("step2");
        assert_eq!(timer.times.len(), 2);
        assert_eq!(timer.times[1].0, "step2");
    }

    #[test]
    fn test_timer_report_multiline() {
        let mut timer = Timer::new("MultilineTimer");
        sleep(Duration::from_millis(5));
        timer.checkpoint("checkpoint1");
        sleep(Duration::from_millis(5));
        timer.checkpoint("checkpoint2");

        let report = timer.report("total", false);
        assert!(report.contains("MultilineTimer - checkpoint1:"));
        assert!(report.contains("MultilineTimer - checkpoint2:"));
        assert!(report.contains("MultilineTimer - total:"));
        assert!(report.contains(", \n"));
    }

    #[test]
    fn test_timer_report_oneline() {
        let mut timer = Timer::new("OnelineTimer");
        sleep(Duration::from_millis(5));
        timer.checkpoint("a");
        sleep(Duration::from_millis(5));
        timer.checkpoint("b");

        let report = timer.report("final", true);
        assert!(report.starts_with("OnelineTimer - "));
        assert!(report.contains("a:"));
        assert!(report.contains("b:"));
        assert!(report.contains("final:"));
        assert!(report.contains(", "));
        assert!(!report.contains("\n"));
    }

    #[test]
    fn test_timer_no_checkpoints() {
        let mut timer = Timer::new("EmptyTimer");
        let report = timer.report("done", false);
        assert!(report.contains("EmptyTimer - done:"));
        assert_eq!(report.matches('\n').count(), 0);
    }

    #[test]
    fn test_timer_elapsed_time_accumulates() {
        let mut timer = Timer::new("AccumulateTimer");
        sleep(Duration::from_millis(20));
        timer.checkpoint("step1");

        assert!(timer.times[0].1.as_millis() >= 15);

        sleep(Duration::from_millis(20));
        timer.checkpoint("step2");

        assert!(timer.times[1].1.as_millis() >= 15);
    }
}
