use std::time::{Duration, Instant};

pub struct Timer {
    base: String,
    start_time: Instant,
    last_time: Instant,
    times: Vec<(String, Duration)>,
}

impl Timer {
    pub fn new(base: &str) -> Self {
        Self {
            base: base.to_string(),
            start_time: Instant::now(),
            last_time: Instant::now(),
            times: Vec::new(),
        }
    }

    pub fn checkpoint(&mut self, name: &str) {
        let elapsed = self.last_time.elapsed();
        self.times.push((name.to_string(), elapsed));
        self.last_time = Instant::now();
    }

    pub fn report(&mut self, in_one_line: bool) -> String {
        let all_elapsed = self.start_time.elapsed();
        self.times.push(("Everything".to_string(), all_elapsed));

        let joiner = if in_one_line { ", " } else { ", \n" };
        self.times
            .iter()
            .map(|(name, time)| format!("{} - {name}: {time:?}", self.base))
            .collect::<Vec<_>>()
            .join(joiner)
    }
}
