//! How long the disk usage chart takes to compute (the layout) and to draw (the picture), the two halves of what Krokiet
//! does after every resize, zoom or change of options. Prints one line per measurement for `misc/bench_disk_usage_matrix.sh`,
//! which runs it in many build configurations and puts the results side by side.
//!
//! Settings come from environment variables, so that the same binary can be started the same way everywhere:
//! `BENCH_FILES` (size of the generated tree, 1000000 by default), `BENCH_PATH` (scan this directory instead),
//! `BENCH_RUNS` (measured runs, 7 by default), `BENCH_WIDTH` and `BENCH_HEIGHT` (size of the chart in pixels).
#![allow(dead_code, unused_imports, clippy::print_stdout, clippy::cast_precision_loss)]

#[path = "../src/disk_usage/render.rs"]
mod render;

use std::hint::black_box;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use czkawka_core::common::tool_data::CommonData;
use czkawka_core::common::traits::Search;
use czkawka_core::tools::disk_usage::category::FileCategory;
use czkawka_core::tools::disk_usage::layout::{Layout, LayoutKind, LayoutOptions};
use czkawka_core::tools::disk_usage::tree::{DiskNode, DiskTree};
use czkawka_core::tools::disk_usage::{ContentDetection, DiskUsage, DiskUsageParameters};
use render::{ChartGeometry, RenderSettings, render_chart};

const DEFAULT_FILES: usize = 1_000_000;
const DEFAULT_RUNS: usize = 7;
const DEFAULT_SIZE: (u32, u32) = (1600, 900);
/// What the options of Krokiet are by default: rings shown for 6 levels, the centre is one more.
const SUNBURST_LEVELS: usize = 7;
const MIN_ARC_ANGLE: f32 = 0.03;
const MAX_DIRECTORY_DEPTH: usize = 10;
const HIT_TEST_POINTS: usize = 64;
const HIT_TEST_REPEATS: usize = 20;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, limit: u64) -> u64 {
        self.next() % limit.max(1)
    }
}

fn environment_number<T: std::str::FromStr>(name: &str, default: T) -> T {
    std::env::var(name).ok().and_then(|value| value.parse().ok()).unwrap_or(default)
}

/// Looks like a real disk: most directories hold a few files, a few hold thousands, sizes span many orders of magnitude.
/// The same number always gives the same tree, so every build configuration measures the same thing.
fn synthetic_tree(files: usize) -> DiskTree {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    DiskTree::new(vec![synthetic_directory("/synthetic".to_string(), 0, files, &mut rng)])
}

/// A directory that holds `allowance` files together with everything below it; what its own files do not use is
/// shared between its subdirectories by random weights, so that the tree is wide on every level and not one long chain.
fn synthetic_directory(name: String, depth: usize, allowance: usize, rng: &mut Rng) -> DiskNode {
    let wanted = match rng.below(1000) {
        0..=799 => rng.below(12) + 1,
        800..=969 => rng.below(200) + 12,
        970..=997 => rng.below(5000) + 200,
        _ => rng.below(60_000) + 5000,
    } as usize;
    let own_files = if depth >= MAX_DIRECTORY_DEPTH { allowance } else { wanted.min(allowance) };

    let categories = &FileCategory::ALL[1..];
    let mut children = Vec::with_capacity(own_files);
    for index in 0..own_files {
        let bits = rng.below(28);
        let size = (1_u64 << bits) + rng.below(1 << bits.max(1));
        let category = *categories.get(rng.below(categories.len() as u64) as usize).expect("index is below the length");
        children.push(DiskNode::file(format!("file_{index}.dat"), size, category));
    }

    let remaining = allowance - own_files;
    if remaining > 0 {
        let weights = (0..=rng.below(6)).map(|_| 1 + rng.below(100)).collect::<Vec<_>>();
        let total_weight = weights.iter().sum::<u64>();
        let mut left = remaining;
        for (index, weight) in weights.iter().enumerate() {
            let share = if index + 1 == weights.len() {
                left
            } else {
                (remaining as u64 * weight / total_weight) as usize
            };
            let share = share.min(left);
            left -= share;
            if share > 0 {
                children.push(synthetic_directory(format!("dir_{index}"), depth + 1, share, rng));
            }
        }
    }
    DiskNode::directory(name, children)
}

fn scanned_tree(path: PathBuf) -> DiskTree {
    let mut tool = DiskUsage::new(DiskUsageParameters::new(ContentDetection::ExtensionOnly));
    tool.set_included_paths(vec![path]);
    tool.search(&Arc::new(AtomicBool::new(false)), None);
    std::mem::take(tool.get_tree_mut())
}

fn count_files(node: &DiskNode) -> usize {
    if node.is_directory() { node.children.iter().map(count_files).sum() } else { 1 }
}

/// The options Krokiet builds from its defaults (see `layout_options` in `src/disk_usage/view.rs`).
fn chart_options(kind: LayoutKind, (width, height): (u32, u32)) -> LayoutOptions {
    let mut options = LayoutOptions::new(kind);
    options.min_size_fraction = 1e-6;
    options.min_item_size_px = 1.0;
    options.min_expanded_directory_px = 0.0;
    options.canvas_size_px = (width as f32, height as f32);
    options.min_arc_angle = MIN_ARC_ANGLE;
    match kind {
        LayoutKind::Treemap => {
            options.aspect_ratio = width as f32 / height.max(1) as f32;
            options.nested_padding = 0.0;
        }
        LayoutKind::Sunburst => options.max_depth = SUNBURST_LEVELS,
    }
    options
}

fn median(times: &[Duration]) -> Duration {
    let mut sorted = times.to_vec();
    sorted.sort_unstable();
    sorted.get(sorted.len() / 2).copied().unwrap_or_default()
}

fn milliseconds(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn report(mode: &str, stage: &str, times: &[Duration]) {
    let fastest = times.iter().min().copied().unwrap_or_default();
    println!("RESULT {mode} {stage} {:.3} {:.3}", milliseconds(median(times)), milliseconds(fastest));
}

/// One run of what `build_and_render` does: the layout first, then the picture, each timed on its own.
fn measure_chart(tree: &DiskTree, kind: LayoutKind, size: (u32, u32), runs: usize) {
    let mode = if kind == LayoutKind::Treemap { "treemap" } else { "sunburst" };
    let options = chart_options(kind, size);
    let settings = RenderSettings {
        geometry: ChartGeometry {
            width: size.0,
            height: size.1,
            kind,
        },
        levels: options.max_depth,
        dark_theme: true,
        nested_borders: false,
    };

    let (mut layout_times, mut render_times, mut total_times) = (Vec::new(), Vec::new(), Vec::new());
    let mut items = 0;
    // The first run only warms up the caches and the thread pool
    for run in 0..=runs {
        let start = Instant::now();
        let layout = black_box(Layout::build_from_roots(black_box(&tree.roots), black_box(&options)));
        let layout_done = start.elapsed();
        let render_start = Instant::now();
        let picture = black_box(render_chart(black_box(&layout), black_box(&settings), &|| true));
        let render_done = render_start.elapsed();
        items = layout.items.len();
        drop(picture);
        if run > 0 {
            layout_times.push(layout_done);
            render_times.push(render_done);
            total_times.push(start.elapsed());
        }
    }
    println!("INFO {mode} layout_items {items}");
    report(mode, "layout", &layout_times);
    report(mode, "render", &render_times);
    report(mode, "layout_and_render", &total_times);

    let layout = Layout::build_from_roots(&tree.roots, &options);
    let points = (0..HIT_TEST_POINTS)
        .map(|index| ((index % 8) as f32 / 8.0 + 0.031, (index / 8) as f32 / 8.0 + 0.047))
        .collect::<Vec<_>>();
    let mut hit_times = Vec::new();
    for _ in 0..=runs {
        let start = Instant::now();
        for _ in 0..HIT_TEST_REPEATS {
            for &(x, y) in &points {
                black_box(layout.hit_test(black_box(x), black_box(y)));
            }
        }
        hit_times.push(start.elapsed() / (HIT_TEST_POINTS * HIT_TEST_REPEATS) as u32);
    }
    println!(
        "RESULT {mode} hit_test_per_call_us {:.3} {:.3}",
        median(&hit_times).as_secs_f64() * 1e6,
        hit_times.iter().min().copied().unwrap_or_default().as_secs_f64() * 1e6
    );
}

fn main() {
    let runs = environment_number("BENCH_RUNS", DEFAULT_RUNS).max(1);
    let size = (environment_number("BENCH_WIDTH", DEFAULT_SIZE.0), environment_number("BENCH_HEIGHT", DEFAULT_SIZE.1));
    let tree = match std::env::var_os("BENCH_PATH") {
        Some(path) => scanned_tree(PathBuf::from(path)),
        None => synthetic_tree(environment_number("BENCH_FILES", DEFAULT_FILES)),
    };
    println!("INFO build debug_assertions {}", cfg!(debug_assertions));
    println!(
        "INFO build avx2 {} avx512f {} sse4_2 {}",
        cfg!(target_feature = "avx2"),
        cfg!(target_feature = "avx512f"),
        cfg!(target_feature = "sse4.2")
    );
    println!(
        "INFO run files {} runs {runs} chart {}x{} threads {}",
        tree.roots.iter().map(count_files).sum::<usize>(),
        size.0,
        size.1,
        rayon::current_num_threads()
    );

    measure_chart(&tree, LayoutKind::Treemap, size, runs);
    measure_chart(&tree, LayoutKind::Sunburst, size, runs);
}
