use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use criterion::{Criterion, criterion_group, criterion_main};
use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::disk_usage::layout::{Layout, LayoutKind, LayoutOptions};
use czkawka_core::tools::disk_usage::{ContentDetection, DiskUsage, DiskUsageParameters};

const BENCH_PATH_VARIABLE: &str = "DISK_USAGE_BENCH_PATH";

/// Directory to measure: the one from `DISK_USAGE_BENCH_PATH`, or a generated tree of about 10 000 tiny files.
fn prepare_directory() -> (PathBuf, Option<PathBuf>) {
    if let Some(path) = std::env::var_os(BENCH_PATH_VARIABLE) {
        return (PathBuf::from(path), None);
    }
    let root = std::env::temp_dir().join("czkawka_disk_usage_bench");
    if root.exists() {
        fs::remove_dir_all(&root).expect("Failed to remove old benchmark directory");
    }
    for first in 0..20 {
        for second in 0..20 {
            let directory = root.join(format!("dir_{first}")).join(format!("sub_{second}"));
            fs::create_dir_all(&directory).expect("Failed to create benchmark directory");
            for file in 0..25 {
                fs::write(directory.join(format!("file_{file}.dat")), vec![0u8; 1 + file * 7]).expect("Failed to create benchmark file");
            }
        }
    }
    (root.clone(), Some(root))
}

fn scan(path: &Path) -> DiskUsage {
    let mut tool = DiskUsage::new(DiskUsageParameters::new(ContentDetection::ExtensionOnly));
    tool.set_included_paths(vec![path.to_path_buf()]);
    tool.search(&Arc::new(AtomicBool::new(false)), None);
    tool
}

fn benchmark_disk_usage(c: &mut Criterion) {
    let (path, generated) = prepare_directory();

    let mut group = c.benchmark_group("disk_usage");
    group.sample_size(20);
    group.bench_function("scan", |b| b.iter(|| black_box(scan(black_box(&path)))));

    let tool = scan(&path);
    for (name, kind) in [("treemap", LayoutKind::Treemap), ("sunburst", LayoutKind::Sunburst)] {
        let options = LayoutOptions::new(kind);
        group.bench_function(format!("layout_{name}"), |b| {
            b.iter(|| black_box(Layout::build(black_box(tool.get_tree()), black_box(&options))));
        });
    }
    group.finish();

    if let Some(generated) = generated {
        fs::remove_dir_all(generated).expect("Failed to remove benchmark directory");
    }
}

criterion_group!(benches, benchmark_disk_usage);
criterion_main!(benches);
