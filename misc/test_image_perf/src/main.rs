use image_hasher::{FilterType, HashAlg, HasherConfig};
use rayon::prelude::*;
use std::env;
use std::process::Command;
use walkdir::WalkDir;

const DIR_TO_CHECK: &str = "/media/rafal/Kotyk/Rafał/Camera";

const ITERATIONS: usize = 5;
const HASH_ALG: HashAlg = HashAlg::Gradient;
const FILTER_TYPE: FilterType = FilterType::Lanczos3;
const HASH_SIZE: u32 = 64;

#[cfg(not(feature = "fast_image_resize"))]
const MODE: &str = "NORMAL";
#[cfg(feature = "fast_image_resize")]
const MODE: &str = "FAST_RESIZE";

fn main() {
    if !is_running_as_sudo() {
        println!("Please run this program as root");
        return;
    }

    clean_disk_cache();

    let collected_image_files = WalkDir::new(DIR_TO_CHECK)
        .into_iter()
        .flatten()
        .map(|e| e.path().to_path_buf())
        .filter_map(|e| {
            let ext = e.extension().unwrap_or_default().to_str().unwrap_or_default().to_lowercase();
            if ["jpg", "png", "jpeg", "webp"].contains(&ext.as_str()) {
                return Some(e.to_str().unwrap_or_default().to_string());
            }
            None
        })
        .collect::<Vec<String>>();

    println!("Collected {} image files", collected_image_files.len());

    let mut times = vec![];

    for i in 0..ITERATIONS {
        println!("Iteration {}", i + 1);
        clean_disk_cache();

        let start = std::time::Instant::now();

        collected_image_files.par_iter().for_each(|e| {
            let _ = hash_image(e);
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
    let img = image::open(hash_image).map_err(|e| format!("Cannot open image file \"{hash_image}\": {e}"))?;

    let hasher_config = HasherConfig::new().hash_size(HASH_SIZE, HASH_SIZE).hash_alg(HASH_ALG).resize_filter(FILTER_TYPE);
    let hasher = hasher_config.to_hasher();
    let _hash = hasher.hash_image(&img);

    Ok(())
}

fn clean_disk_cache() {
    let _sync = Command::new("sync").output().expect("Failed to execute sync");
    let _drop_caches = Command::new("sh")
        .arg("-c")
        .arg("echo 3 > /proc/sys/vm/drop_caches")
        .output()
        .expect("Failed to execute drop_caches");
}

fn is_running_as_sudo() -> bool {
    match env::var("EUID") {
        Ok(euid) => euid == "0",
        Err(_) => match env::var("USER") {
            Ok(user) => user == "root",
            Err(_) => false,
        },
    }
}
