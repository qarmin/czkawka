use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use czkawka_core::common::bk_tree::{BKTree, Metric};
use hamming_bitwise_fast::hamming_bitwise_fast;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

const HASHES_COUNT: usize = 5_000;
const QUERIES_COUNT: usize = 200;
const VARIANTS_PER_BASE: usize = 3;
const FLIPPED_BITS_PER_VARIANT: usize = 3;

// (hash size in bytes, search tolerance), the same sizes as image hashes of size 8, 16, 32 and 64
const CASES: [(usize, u32); 4] = [(8, 2), (32, 5), (128, 10), (512, 20)];

struct Hamming;

impl Metric<Vec<u8>> for Hamming {
    fn distance(&self, a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        hamming_bitwise_fast(a, b)
    }

    fn threshold_distance(&self, a: &Vec<u8>, b: &Vec<u8>, threshold: u32) -> Option<u32> {
        Some(hamming_bitwise_fast(a, b)).filter(|distance| *distance <= threshold)
    }
}

/// Random hashes where every base hash is followed by a few slightly modified copies, like similar images.
fn clustered_hashes(count: usize, byte_size: usize, seed: u64) -> Vec<Vec<u8>> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut hashes = Vec::with_capacity(count);
    while hashes.len() < count {
        let base: Vec<u8> = (0..byte_size).map(|_| rng.random()).collect();
        for _ in 0..VARIANTS_PER_BASE {
            let mut variant = base.clone();
            for _ in 0..FLIPPED_BITS_PER_VARIANT {
                let bit = rng.random_range(0..byte_size * 8);
                if let Some(byte) = variant.get_mut(bit / 8) {
                    *byte ^= 1 << (bit % 8);
                }
            }
            hashes.push(variant);
        }
        hashes.push(base);
    }
    hashes.truncate(count);
    hashes
}

fn bench_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("bk_tree_build");
    group.sample_size(20);
    for (byte_size, _) in CASES {
        let hashes = clustered_hashes(HASHES_COUNT, byte_size, 1);
        group.bench_with_input(BenchmarkId::from_parameter(byte_size), &hashes, |b, hashes| {
            b.iter(|| {
                let mut tree = BKTree::new(Hamming);
                tree.extend(hashes.iter().cloned());
                black_box(tree)
            });
        });
    }
    group.finish();
}

fn bench_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("bk_tree_find");
    group.sample_size(20);
    for (byte_size, tolerance) in CASES {
        let hashes = clustered_hashes(HASHES_COUNT, byte_size, 1);
        let unknown_queries = clustered_hashes(QUERIES_COUNT, byte_size, 2);
        let queries: Vec<&Vec<u8>> = hashes.iter().step_by(HASHES_COUNT / QUERIES_COUNT).chain(&unknown_queries).collect();

        let mut tree = BKTree::new(Hamming);
        tree.extend(hashes.iter().cloned());
        group.bench_function(BenchmarkId::from_parameter(byte_size), |b| {
            b.iter(|| queries.iter().map(|query| tree.find(black_box(*query), tolerance).count()).sum::<usize>());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_build, bench_find);
criterion_main!(benches);
