use hamming_bitwise_fast::hamming_bitwise_fast;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

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

/// Few distinct bytes make many near and exactly equal hashes.
fn similar_hashes(count: usize, byte_size: usize, seed: u64) -> Vec<Vec<u8>> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..count).map(|_| (0..byte_size).map(|_| rng.random_range(0..4u8) * 0x55).collect()).collect()
}

fn assert_same_results(byte_size: usize, seed: u64) {
    let hashes = similar_hashes(600, byte_size, seed);
    let queries = similar_hashes(60, byte_size, seed + 1000);

    let mut original = bk_tree::BKTree::new(Hamming);
    original.extend(hashes.iter().cloned());
    let mut vendored = czkawka_bk_tree::BKTree::new(Hamming);
    vendored.extend(hashes.iter().cloned());

    for query in hashes.iter().take(30).chain(&queries) {
        for tolerance in [0, 1, 2, 5, 10, 40] {
            let mut expected: Vec<_> = original.find(query, tolerance).map(|(distance, key)| (distance, key.clone())).collect();
            let mut found: Vec<_> = vendored.find(query, tolerance).map(|(distance, key)| (distance, key.clone())).collect();
            expected.sort();
            found.sort();
            assert_eq!(expected, found, "byte_size={byte_size} seed={seed} tolerance={tolerance}");
        }
        assert_eq!(original.find_exact(query), vendored.find_exact(query));
    }
}

#[test]
fn find_returns_the_same_results_as_the_original_crate() {
    for byte_size in [1, 8, 32, 128] {
        for seed in 0..5 {
            assert_same_results(byte_size, seed);
        }
    }
}
