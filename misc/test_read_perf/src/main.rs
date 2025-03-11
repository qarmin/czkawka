use czkawka_core::tools::duplicate::{hash_calculation, DuplicateEntry, HashType};
use humansize::{format_size, BINARY};
use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use strum::Display;
use walkdir::WalkDir;

const DIR_TO_CHECK: &str = "/home/rafal/TODO/B/Nefs and raws";
const ITERATIONS: usize = 1;

thread_local! {
    static BUFFER: RefCell<Vec<u8>> = RefCell::new(vec![0u8; 1024 * 1024]);
}

static GLOBAL_HDD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[allow(unused)]
#[derive(Copy, Clone, Display, Hash, Eq, PartialEq)]
enum MODES {
    ARR16,
    ARR256,
    VEC16,
    VEC1024,
    VEC1024LOCKING,
    VEC1024THREAD,
}

fn main() {
    if !is_running_as_sudo() {
        println!("Please run this program as root");
        return;
    }
    let files = collect_files_to_test();

    clean_disk_cache();

    let mut hashmap: HashMap<MODES, Vec<u128>> = HashMap::new();

    let modes = [MODES::VEC1024, MODES::VEC1024LOCKING];
    for i in 0..ITERATIONS {
        for mode in modes.iter() {
            println!("Iteration {}/{} in mode {}", i, ITERATIONS, mode);
            clean_disk_cache();

            let start = std::time::Instant::now();
            match mode {
                MODES::ARR16 => array16(&files),
                MODES::ARR256 => array256(&files),
                MODES::VEC16 => vec16(&files),
                MODES::VEC1024 => vec1024(&files),
                MODES::VEC1024THREAD => vec1024_thread(&files),
                MODES::VEC1024LOCKING => vec1024_locking(&files),
            }

            println!("Iteration {}/{} finished in mode {mode} with time: {} ms", i, ITERATIONS, start.elapsed().as_millis());

            hashmap.entry(*mode).or_insert_with(Vec::new).push(start.elapsed().as_micros());
        }
    }

    for (mode, times) in hashmap {
        let results_to_remove = times.len() / 4;

        let total_time = times.iter().sum::<u128>();
        let all_iterations_time = total_time as f64 / 1000.0;

        let times = if ITERATIONS > 4 {
            times
                .iter()
                .enumerate()
                .filter(|(idx, _value)| idx < &results_to_remove || idx >= &(times.len() - results_to_remove))
                .map(|e| *e.1)
                .collect::<Vec<_>>()
        } else {
            times
        };

        let total_time_without_extremes = total_time - times.iter().min().cloned().unwrap_or_default() - times.iter().max().cloned().unwrap_or_default();
        let all_iterations_time_without_extremes = total_time_without_extremes as f64 / 1000.0;
        println!(
            "Mode {}, {} iterations, total time: {} ms, per iteration time {} ms, total time without extremes: {} ms, per iteration time without extremes {} ms",
            mode,
            ITERATIONS,
            all_iterations_time,
            all_iterations_time / ITERATIONS as f64,
            all_iterations_time_without_extremes,
            all_iterations_time_without_extremes / ITERATIONS as f64
        );
    }
}

fn array16(files: &Vec<DuplicateEntry>) {
    files.into_par_iter().for_each(|f| {
        let mut buffer = [0u8; 16 * 1024];
        let _ = hash_calculation(&mut buffer, &f, HashType::Blake3, &Arc::default(), None);
    });
}
fn array256(files: &Vec<DuplicateEntry>) {
    files.into_par_iter().for_each(|f| {
        let mut buffer = [0u8; 256 * 1024];
        let _ = hash_calculation(&mut buffer, &f, HashType::Blake3, &Arc::default(), None);
    });
}
fn vec16(files: &Vec<DuplicateEntry>) {
    files.into_par_iter().for_each(|f| {
        let mut buffer = vec![0u8; 16 * 1024];
        let _ = hash_calculation(&mut buffer, &f, HashType::Blake3, &Arc::default(), None);
    });
}
fn vec1024(files: &Vec<DuplicateEntry>) {
    files.into_par_iter().for_each(|f| {
        let mut buffer = vec![0u8; 1024 * 1024];
        let _ = hash_calculation(&mut buffer, &f, HashType::Blake3, &Arc::default(), None);
    });
}
fn vec1024_locking(files: &Vec<DuplicateEntry>) {
    files.into_par_iter().for_each(|f| {
        let _lock = GLOBAL_HDD_LOCK.lock().unwrap();
        let mut buffer = vec![0u8; 1024 * 1024];
        let _ = hash_calculation(&mut buffer, &f, HashType::Blake3, &Arc::default(), None);
    });
}
fn vec1024_thread(files: &Vec<DuplicateEntry>) {
    files.into_par_iter().for_each(|f| {
        BUFFER.with(|buffer| {
            let _ = hash_calculation(&mut buffer.borrow_mut(), &f, HashType::Blake3, &Arc::default(), None);
        });
    });
}

fn collect_files_to_test() -> Vec<DuplicateEntry> {
    let files = WalkDir::new(DIR_TO_CHECK)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
        .map(|e| DuplicateEntry {
            path: e.path().to_path_buf(),
            modified_date: e
                .metadata()
                .map(|e| e.modified().map(|e| e.duration_since(UNIX_EPOCH)).expect("TEST").expect("TEST").as_secs())
                .unwrap_or_default(),
            size: e.metadata().map(|e| e.len()).unwrap_or_default(),
            hash: "".to_string(),
        })
        .collect::<Vec<_>>();
    let size: u64 = files.iter().map(|f| f.size).sum();
    let size_str = format_size(size, BINARY);
    println!("Collected {} files to test, total size: {}", files.len(), size_str);
    files
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
