use std::env;
use std::thread::available_parallelism;
use std::time::{Duration, Instant};

use czkawka_core::common::image::get_dynamic_image_from_path;
use image_hasher::{FilterType, HashAlg, HasherConfig};
use log::info;
use rayon::prelude::*;
use walkdir::WalkDir;

const ITERATIONS_ON_IMAGE: usize = 3;
const ITERATIONS: usize = 5;
const HASH_ALG: HashAlg = HashAlg::Gradient;
const FILTER_TYPE: FilterType = FilterType::Lanczos3;
const HASH_SIZE: u32 = 8;

const MODE: &str = "FAST_RESIZE";

fn print_items() {
    let debug_release = if cfg!(debug_assertions) { "debug" } else { "release" };

    let processors = available_parallelism().map(|e| e.get()).unwrap_or_default();

    let info = os_info::get();

    #[allow(unused_mut)]
    let mut features: Vec<&str> = Vec::new();

    #[allow(unused_mut)]
    let mut app_cpu_version = "Baseline";
    let mut os_cpu_version = "Baseline";
    if cfg!(target_feature = "sse2") {
        app_cpu_version = "x86-64-v1 (SSE2)";
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("sse2") {
        os_cpu_version = "x86-64-v1 (SSE2)";
    }

    if cfg!(target_feature = "popcnt") {
        app_cpu_version = "x86-64-v2 (SSE4.2 + POPCNT)";
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("popcnt") {
        os_cpu_version = "x86-64-v2 (SSE4.2 + POPCNT)";
    }

    if cfg!(target_feature = "avx2") {
        app_cpu_version = "x86-64-v3 (AVX2)";
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx2") {
        os_cpu_version = "x86-64-v3 (AVX2)";
    }

    if cfg!(target_feature = "avx512f") {
        app_cpu_version = "x86-64-v4 (AVX-512)";
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx512f") {
        os_cpu_version = "x86-64-v4 (AVX-512)";
    }

    // TODO - probably needs to add arm and other architectures, need help, because I don't have access to them

    info!(
        "App version: {debug_release} mode, os {} {} [{} {}], {processors} cpu/threads, features({}): [{}], app cpu version: {}, os cpu version: {}",
        info.os_type(),
        info.version(),
        env::consts::ARCH,
        info.bitness(),
        features.len(),
        features.join(", "),
        app_cpu_version,
        os_cpu_version,
    );
}

fn main() {
    handsome_logger::init().expect("TEST");
    print_items();

    #[cfg(unix)]
    {
        if !is_running_as_sudo() {
            println!("Please run this program as root");
            return;
        }

        clean_disk_cache();
    }

    let Some(test_path) = env::args().nth(1) else {
        println!("Please provide path to test images");
        return;
    };

    let all_files: Vec<_> = WalkDir::new(&test_path).into_iter().flatten().map(|e| e.path().to_path_buf()).collect();

    let all_files_len = all_files.len();

    let collected_image_files = all_files
        .into_iter()
        .filter_map(|e| {
            let ext = e.extension().unwrap_or_default().to_str().unwrap_or_default().to_lowercase();
            if ["jpg", "png", "jpeg", "webp", "crw", "nef", "arw", "dng", "avif", "cr3", "cr2"].contains(&ext.as_str()) {
                return Some(e.to_str().unwrap_or_default().to_string());
            }
            None
        })
        .collect::<Vec<String>>();

    println!(
        "Collected {} image files out of all {all_files_len} files, with mode {MODE} from {test_path}",
        collected_image_files.len()
    );

    let mut times = Vec::new();

    for i in 0..ITERATIONS {
        println!("Iteration {}", i + 1);
        #[cfg(unix)]
        clean_disk_cache();

        let start = std::time::Instant::now();

        collected_image_files.par_iter().for_each(|e| {
            for _ in 0..ITERATIONS_ON_IMAGE {
                let _ = hash_image(e);
            }
        });

        let elapsed = start.elapsed();
        println!("Iteration {} took {} ms", i + 1, elapsed.as_millis());
        times.push(elapsed.as_micros());
    }

    let total_time = times.iter().sum::<u128>();
    let all_iterations_time = total_time as f64 / 1000.0;

    let iters_without_extremes = times.len().checked_sub(2).unwrap_or_default();
    let total_time_without_extremes = total_time - times.iter().min().copied().unwrap_or_default() - times.iter().max().copied().unwrap_or_default();
    let all_iterations_time_without_extremes = total_time_without_extremes as f64 / 1000.0;
    println!(
        "Mode {}, {} iterations, total time: {} ms, per iteration time {} ms, total time without extremes: {} ms, per iteration time without extremes {} ms",
        MODE,
        ITERATIONS,
        all_iterations_time,
        all_iterations_time / ITERATIONS as f64,
        all_iterations_time_without_extremes,
        all_iterations_time_without_extremes / iters_without_extremes as f64
    );
}

fn hash_image(hash_image: &str) -> Result<(), String> {
    let img = get_dynamic_image_from_path(hash_image)?;

    let hasher_config = HasherConfig::new().hash_size(HASH_SIZE, HASH_SIZE).hash_alg(HASH_ALG).resize_filter(FILTER_TYPE);
    let hasher = hasher_config.to_hasher();
    let _hash = hasher.hash_image(&img);

    Ok(())
}

#[cfg(unix)]
fn clean_disk_cache() {
    use std::process::Command;
    let _sync = Command::new("sync").output().expect("Failed to execute sync");
    let _drop_caches = Command::new("sh")
        .arg("-c")
        .arg("echo 3 > /proc/sys/vm/drop_caches")
        .output()
        .expect("Failed to execute drop_caches");
}

#[cfg(unix)]
fn is_running_as_sudo() -> bool {
    match env::var("EUID") {
        Ok(euid) => euid == "0",
        Err(_) => match env::var("USER") {
            Ok(user) => user == "root",
            Err(_) => false,
        },
    }
}

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
