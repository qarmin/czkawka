//! Compares the portable directory walk with reading the NTFS master file table on chosen paths.
//!
//! Run (the MFT method works only on Windows, from an elevated terminal):
//! `cargo bench -p czkawka_core --features ntfs_mft --bench disk_usage_ntfs_benchmark -- --path C:\Users --path D:\ --runs 5 --drop-caches`
//!
//! `--drop-caches` empties the OS file cache before every run, so each run is cold; it needs root on Linux
//! (writes to `/proc/sys/vm/drop_caches`). Elsewhere use `--drop-caches-command "<program> <args>"`,
//! e.g. a standby list cleaner such as RAMMap's `RAMMap64.exe -Et`.
#![expect(clippy::print_stdout, clippy::print_stderr, reason = "benchmark reports its results on the terminal")]

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::disk_usage::{ContentDetection, DiskUsage, DiskUsageParameters, Info};

const DEFAULT_RUNS: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Method {
    Portable,
    NtfsMft,
}

impl Method {
    const fn name(self) -> &'static str {
        match self {
            Self::Portable => "portable walk",
            Self::NtfsMft => "NTFS MFT",
        }
    }
}

enum CacheDropping {
    None,
    Builtin,
    Command(String),
}

struct Arguments {
    paths: Vec<PathBuf>,
    runs: usize,
    cache_dropping: CacheDropping,
}

fn parse_arguments() -> Arguments {
    let mut arguments = Arguments {
        paths: Vec::new(),
        runs: DEFAULT_RUNS,
        cache_dropping: CacheDropping::None,
    };
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--path" => arguments.paths.extend(args.next().map(PathBuf::from)),
            "--runs" => arguments.runs = args.next().and_then(|runs| runs.parse().ok()).filter(|runs| *runs > 0).unwrap_or(DEFAULT_RUNS),
            "--drop-caches" => arguments.cache_dropping = CacheDropping::Builtin,
            "--drop-caches-command" => arguments.cache_dropping = args.next().map_or(CacheDropping::None, CacheDropping::Command),
            _ => {} // cargo passes e.g. `--bench`
        }
    }
    if arguments.paths.is_empty() {
        eprintln!("Pass at least one directory with `--path <dir>` (repeatable); options: --runs <n>, --drop-caches, --drop-caches-command \"<cmd>\"");
    }
    arguments
}

fn drop_caches(cache_dropping: &CacheDropping) {
    let result = match cache_dropping {
        CacheDropping::None => return,
        CacheDropping::Builtin => drop_caches_builtin(),
        CacheDropping::Command(command) => run_command(command),
    };
    if let Err(error) = result {
        eprintln!("Dropping caches failed, the run is warm: {error}");
    }
}

#[cfg(target_os = "linux")]
fn drop_caches_builtin() -> Result<(), String> {
    std::fs::write("/proc/sys/vm/drop_caches", "3").map_err(|error| format!("cannot write /proc/sys/vm/drop_caches (needs root): {error}"))
}

#[cfg(not(target_os = "linux"))]
fn drop_caches_builtin() -> Result<(), String> {
    Err("no built-in way on this system, use --drop-caches-command".to_string())
}

fn run_command(command: &str) -> Result<(), String> {
    let mut parts = command.split_whitespace();
    let program = parts.next().ok_or("empty command")?;
    let status = Command::new(program).args(parts).status().map_err(|error| format!("cannot run `{command}`: {error}"))?;
    status.success().then_some(()).ok_or_else(|| format!("`{command}` exited with {status}"))
}

fn scan(path: &Path, method: Method) -> (Duration, Info) {
    let params = DiskUsageParameters {
        read_ntfs_mft: method == Method::NtfsMft,
        ..DiskUsageParameters::new(ContentDetection::ExtensionOnly)
    };
    let mut tool = DiskUsage::new(params);
    tool.set_included_paths(vec![path.to_path_buf()]);
    let start = Instant::now();
    tool.search(&Arc::new(AtomicBool::new(false)), None);
    (start.elapsed(), tool.get_information())
}

struct MethodResult {
    times: Vec<Duration>,
    info: Info,
}

fn measure(path: &Path, method: Method, arguments: &Arguments) -> MethodResult {
    let mut times = Vec::with_capacity(arguments.runs);
    let mut info = Info::default();
    for _ in 0..arguments.runs {
        drop_caches(&arguments.cache_dropping);
        let (time, run_info) = scan(path, method);
        times.push(time);
        info = run_info;
    }
    times.sort_unstable();
    MethodResult { times, info }
}

fn report(method: Method, result: &MethodResult) {
    let (Some(min), Some(median)) = (result.times.first(), result.times.get(result.times.len() / 2)) else {
        return;
    };
    let mean = result.times.iter().sum::<Duration>() / result.times.len() as u32;
    println!(
        "  {:<14} min {min:>10.2?}  median {median:>10.2?}  mean {mean:>10.2?}  files {:>9}  dirs {:>8}  size {} B",
        method.name(),
        result.info.number_of_files,
        result.info.number_of_directories,
        result.info.total_size
    );
    if let Some(reason) = &result.info.ntfs_mft_fallback {
        println!("    WARNING: the MFT was not used, so this result is the portable walk: {reason}");
    }
}

fn main() {
    let arguments = parse_arguments();
    for path in &arguments.paths {
        println!("{} ({} runs per method)", path.display(), arguments.runs);
        let portable = measure(path, Method::Portable, &arguments);
        report(Method::Portable, &portable);
        let mft = measure(path, Method::NtfsMft, &arguments);
        report(Method::NtfsMft, &mft);
        if mft.info.ntfs_mft_fallback.is_none()
            && let (Some(portable_median), Some(mft_median)) = (portable.times.get(portable.times.len() / 2), mft.times.get(mft.times.len() / 2))
        {
            println!(
                "  MFT is {:.1}x faster (median)",
                portable_median.as_secs_f64() / mft_median.as_secs_f64().max(f64::EPSILON)
            );
        }
    }
}
