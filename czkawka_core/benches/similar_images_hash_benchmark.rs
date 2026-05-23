// Benchmark Vec<u8> (current ImHash) vs [u8; N] (const-generic candidate)
// for hot paths in similar-image comparison.
//
// Hash byte sizes correspond to image_hasher hash_size:
//   hash_size  8  -> N =  8 bytes
//   hash_size 16  -> N = 32 bytes
//   hash_size 32  -> N = 128 bytes
//   hash_size 64  -> N = 512 bytes

use std::hint::black_box;
use std::time::Duration;

use bk_tree::{BKTree, Metric};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use hamming_bitwise_fast::hamming_bitwise_fast;
use indexmap::IndexMap;
use rand::SeedableRng;
use rand::RngExt;
use rand::rngs::StdRng;

// -------- Metric implementations --------

struct HammingVec;
impl Metric<Vec<u8>> for HammingVec {
    fn distance(&self, a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        hamming_bitwise_fast(a, b)
    }
    fn threshold_distance(&self, a: &Vec<u8>, b: &Vec<u8>, _t: u32) -> Option<u32> {
        Some(self.distance(a, b))
    }
}

struct HammingArr<const N: usize>;
impl<const N: usize> Metric<[u8; N]> for HammingArr<N> {
    fn distance(&self, a: &[u8; N], b: &[u8; N]) -> u32 {
        hamming_bitwise_fast(a, b)
    }
    fn threshold_distance(&self, a: &[u8; N], b: &[u8; N], _t: u32) -> Option<u32> {
        Some(self.distance(a, b))
    }
}

// -------- Data generation --------

fn random_vec(n: usize, byte_size: usize, seed: u64) -> Vec<Vec<u8>> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..n).map(|_| (0..byte_size).map(|_| rng.random::<u8>()).collect()).collect()
}

fn random_arr<const N: usize>(n: usize, seed: u64) -> Vec<[u8; N]> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..n)
        .map(|_| {
            let mut a = [0u8; N];
            for byte in &mut a {
                *byte = rng.random::<u8>();
            }
            a
        })
        .collect()
}

// Fast criterion config so the whole suite runs in ~1-2 min.
fn fast_group<'a>(c: &'a mut Criterion, name: &str) -> criterion::BenchmarkGroup<'a, criterion::measurement::WallTime> {
    let mut g = c.benchmark_group(name);
    g.warm_up_time(Duration::from_millis(500));
    g.measurement_time(Duration::from_secs(2));
    g.sample_size(10);
    g
}

// -------- 1. Raw hamming distance (Vec vs [u8;N]) --------

fn bench_hamming(c: &mut Criterion) {
    let mut group = c.benchmark_group("hamming_distance");
    group.warm_up_time(Duration::from_millis(500));
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(50);

    for byte_size in [8usize, 32, 128, 512] {
        let vec_data = random_vec(2, byte_size, 42);
        group.bench_function(BenchmarkId::new("vec", byte_size), |b| {
            b.iter(|| black_box(hamming_bitwise_fast(black_box(&vec_data[0]), black_box(&vec_data[1]))));
        });
    }

    macro_rules! arr_bench {
        ($n:literal) => {{
            let d = random_arr::<$n>(2, 42);
            group.bench_function(BenchmarkId::new("arr", $n), |b| {
                b.iter(|| black_box(hamming_bitwise_fast(black_box(&d[0]), black_box(&d[1]))));
            });
        }};
    }
    arr_bench!(8);
    arr_bench!(32);
    arr_bench!(128);
    arr_bench!(512);

    group.finish();
}

// -------- 2. BKTree::find (HOT PATH) --------

const N_HASHES: usize = 3_000;
const M_QUERIES: usize = 100;

fn bench_bktree_find(c: &mut Criterion) {
    let mut group = fast_group(c, "bktree_find");

    let tolerances: [(usize, u32); 4] = [(8, 2), (32, 5), (128, 10), (512, 20)];

    for (byte_size, tolerance) in tolerances {
        let dataset = random_vec(N_HASHES, byte_size, 1);
        let queries = random_vec(M_QUERIES, byte_size, 2);
        let mut tree = BKTree::new(HammingVec);
        for h in &dataset {
            tree.add(h.clone());
        }
        group.bench_function(BenchmarkId::new("vec", byte_size), |b| {
            b.iter(|| {
                let mut total = 0u64;
                for q in &queries {
                    total += tree.find(q, tolerance).count() as u64;
                }
                black_box(total);
            });
        });
    }

    macro_rules! arr_bench {
        ($n:literal, $tol:literal) => {{
            let dataset = random_arr::<$n>(N_HASHES, 1);
            let queries = random_arr::<$n>(M_QUERIES, 2);
            let mut tree = BKTree::new(HammingArr::<$n>);
            for h in &dataset {
                tree.add(*h);
            }
            group.bench_function(BenchmarkId::new("arr", $n), |b| {
                b.iter(|| {
                    let mut total = 0u64;
                    for q in &queries {
                        total += tree.find(q, $tol).count() as u64;
                    }
                    black_box(total);
                });
            });
        }};
    }
    arr_bench!(8, 2);
    arr_bench!(32, 5);
    arr_bench!(128, 10);
    arr_bench!(512, 20);

    group.finish();
}

// -------- 3. BKTree build --------

fn bench_bktree_build(c: &mut Criterion) {
    let mut group = fast_group(c, "bktree_build");

    for byte_size in [8usize, 32, 128, 512] {
        let dataset = random_vec(N_HASHES, byte_size, 3);
        group.bench_function(BenchmarkId::new("vec", byte_size), |b| {
            b.iter(|| {
                let mut tree = BKTree::new(HammingVec);
                for h in &dataset {
                    tree.add(h.clone());
                }
                black_box(&tree);
            });
        });
    }

    macro_rules! arr_bench {
        ($n:literal) => {{
            let dataset = random_arr::<$n>(N_HASHES, 3);
            group.bench_function(BenchmarkId::new("arr", $n), |b| {
                b.iter(|| {
                    let mut tree = BKTree::new(HammingArr::<$n>);
                    for h in &dataset {
                        tree.add(*h);
                    }
                    black_box(&tree);
                });
            });
        }};
    }
    arr_bench!(8);
    arr_bench!(32);
    arr_bench!(128);
    arr_bench!(512);

    group.finish();
}

// -------- 4. Hash clone (1024 items) --------

fn bench_clone(c: &mut Criterion) {
    let mut group = fast_group(c, "hash_clone");
    group.sample_size(30);

    for byte_size in [8usize, 32, 128, 512] {
        let dataset = random_vec(1024, byte_size, 5);
        group.bench_function(BenchmarkId::new("vec", byte_size), |b| {
            b.iter(|| {
                let cloned: Vec<Vec<u8>> = dataset.iter().map(|h| h.clone()).collect();
                black_box(cloned);
            });
        });
    }

    macro_rules! arr_bench {
        ($n:literal) => {{
            let dataset = random_arr::<$n>(1024, 5);
            group.bench_function(BenchmarkId::new("arr", $n), |b| {
                b.iter(|| {
                    let cloned: Vec<[u8; $n]> = dataset.iter().copied().collect();
                    black_box(cloned);
                });
            });
        }};
    }
    arr_bench!(8);
    arr_bench!(32);
    arr_bench!(128);
    arr_bench!(512);

    group.finish();
}

// -------- 5. IndexMap<H, u32> insert + lookup --------

fn bench_indexmap(c: &mut Criterion) {
    let mut group = fast_group(c, "indexmap_build_and_lookup");

    const N: usize = 2_000;

    for byte_size in [8usize, 32, 128, 512] {
        let dataset = random_vec(N, byte_size, 7);
        group.bench_function(BenchmarkId::new("vec", byte_size), |b| {
            b.iter(|| {
                let mut m: IndexMap<Vec<u8>, u32> = IndexMap::with_capacity(N);
                for (i, h) in dataset.iter().enumerate() {
                    m.insert(h.clone(), i as u32);
                }
                let mut sum = 0u64;
                for h in &dataset {
                    sum += *m.get(h).unwrap() as u64;
                }
                black_box(sum);
            });
        });
    }

    macro_rules! arr_bench {
        ($n:literal) => {{
            let dataset = random_arr::<$n>(N, 7);
            group.bench_function(BenchmarkId::new("arr", $n), |b| {
                b.iter(|| {
                    let mut m: IndexMap<[u8; $n], u32> = IndexMap::with_capacity(N);
                    for (i, h) in dataset.iter().enumerate() {
                        m.insert(*h, i as u32);
                    }
                    let mut sum = 0u64;
                    for h in &dataset {
                        sum += *m.get(h).unwrap() as u64;
                    }
                    black_box(sum);
                });
            });
        }};
    }
    arr_bench!(8);
    arr_bench!(32);
    arr_bench!(128);
    arr_bench!(512);

    group.finish();
}

criterion_group!(benches, bench_hamming, bench_bktree_find, bench_bktree_build, bench_clone, bench_indexmap);
criterion_main!(benches);
