use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
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

impl bk_tree::Metric<Vec<u8>> for Hamming {
    fn distance(&self, a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        hamming_bitwise_fast(a, b)
    }

    fn threshold_distance(&self, a: &Vec<u8>, b: &Vec<u8>, threshold: u32) -> Option<u32> {
        Some(hamming_bitwise_fast(a, b)).filter(|distance| *distance <= threshold)
    }
}

impl czkawka_bk_tree::Metric<Vec<u8>> for Hamming {
    fn distance(&self, a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        hamming_bitwise_fast(a, b)
    }

    fn threshold_distance(&self, a: &Vec<u8>, b: &Vec<u8>, threshold: u32) -> Option<u32> {
        Some(hamming_bitwise_fast(a, b)).filter(|distance| *distance <= threshold)
    }
}

struct HammingU64;

impl czkawka_bk_tree::Metric<Vec<u64>> for HammingU64 {
    fn distance(&self, a: &Vec<u64>, b: &Vec<u64>) -> u32 {
        a.iter().zip(b).map(|(x, y)| (x ^ y).count_ones()).sum()
    }

    fn threshold_distance(&self, a: &Vec<u64>, b: &Vec<u64>, threshold: u32) -> Option<u32> {
        Some(self.distance(a, b)).filter(|distance| *distance <= threshold)
    }
}

fn to_words(hashes: &[Vec<u8>]) -> Vec<Vec<u64>> {
    hashes
        .iter()
        .map(|hash| hash.as_chunks::<8>().0.iter().map(|chunk| u64::from_le_bytes(*chunk)).collect())
        .collect()
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
    let mut group = c.benchmark_group("bktree_build");
    group.sample_size(20);
    for (byte_size, _) in CASES {
        let hashes = clustered_hashes(HASHES_COUNT, byte_size, 1);
        group.bench_with_input(BenchmarkId::new("original", byte_size), &hashes, |b, hashes| {
            b.iter(|| {
                let mut tree = bk_tree::BKTree::new(Hamming);
                tree.extend(hashes.iter().cloned());
                black_box(tree)
            });
        });
        let hashes_u64 = to_words(&hashes);
        group.bench_with_input(BenchmarkId::new("vendored_u64", byte_size), &hashes_u64, |b, hashes| {
            b.iter(|| {
                let mut tree = czkawka_bk_tree::BKTree::new(HammingU64);
                tree.extend(hashes.iter().cloned());
                black_box(tree)
            });
        });
        group.bench_with_input(BenchmarkId::new("vendored", byte_size), &hashes, |b, hashes| {
            b.iter(|| {
                let mut tree = czkawka_bk_tree::BKTree::new(Hamming);
                tree.extend(hashes.iter().cloned());
                black_box(tree)
            });
        });
    }
    group.finish();
}

fn bench_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("bktree_find");
    group.sample_size(20);
    for (byte_size, tolerance) in CASES {
        let hashes = clustered_hashes(HASHES_COUNT, byte_size, 1);
        let queries = clustered_hashes(QUERIES_COUNT, byte_size, 2);
        let queries: Vec<&Vec<u8>> = hashes.iter().step_by(HASHES_COUNT / QUERIES_COUNT).chain(&queries).collect();

        let mut original = bk_tree::BKTree::new(Hamming);
        original.extend(hashes.iter().cloned());
        group.bench_function(BenchmarkId::new("original", byte_size), |b| {
            b.iter(|| queries.iter().map(|query| original.find(black_box(*query), tolerance).count()).sum::<usize>());
        });

        let mut vendored = czkawka_bk_tree::BKTree::new(Hamming);
        vendored.extend(hashes.iter().cloned());
        let hashes_u64 = to_words(&hashes);
        let queries_u64 = to_words(&queries.iter().map(|query| (*query).clone()).collect::<Vec<_>>());
        let mut vendored_u64 = czkawka_bk_tree::BKTree::new(HammingU64);
        vendored_u64.extend(hashes_u64.iter().cloned());
        group.bench_function(BenchmarkId::new("vendored_u64", byte_size), |b| {
            b.iter(|| queries_u64.iter().map(|query| vendored_u64.find(black_box(query), tolerance).count()).sum::<usize>());
        });

        group.bench_function(BenchmarkId::new("vendored", byte_size), |b| {
            b.iter(|| queries.iter().map(|query| vendored.find(black_box(*query), tolerance).count()).sum::<usize>());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_build, bench_find);
criterion_main!(benches);
