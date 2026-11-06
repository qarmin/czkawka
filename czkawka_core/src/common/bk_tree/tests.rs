use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngExt, SeedableRng};

use super::*;

struct ByteHamming;

impl Metric<Vec<u8>> for ByteHamming {
    fn distance(&self, a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        a.iter().zip(b).map(|(x, y)| (x ^ y).count_ones()).sum()
    }

    fn threshold_distance(&self, a: &Vec<u8>, b: &Vec<u8>, threshold: u32) -> Option<u32> {
        Some(self.distance(a, b)).filter(|distance| *distance <= threshold)
    }
}

fn tree_with(keys: &[[u8; 2]]) -> BKTree<Vec<u8>, ByteHamming> {
    let mut tree = BKTree::new(ByteHamming);
    tree.extend(keys.iter().map(|key| key.to_vec()));
    tree
}

fn sorted_find(tree: &BKTree<Vec<u8>, ByteHamming>, key: [u8; 2], tolerance: u32) -> Vec<(u32, Vec<u8>)> {
    let mut found: Vec<_> = tree.find(&key.to_vec(), tolerance).map(|(distance, key)| (distance, key.clone())).collect();
    found.sort();
    found
}

#[test]
fn empty_tree_finds_nothing() {
    let tree: BKTree<Vec<u8>, ByteHamming> = BKTree::new(ByteHamming);
    assert_eq!(tree.find(&vec![0, 0], 100).count(), 0);
    assert_eq!(tree.find_exact(&vec![0, 0]), None);
}

#[test]
fn first_key_becomes_root() {
    let tree = tree_with(&[[1, 0], [3, 0]]);
    assert_eq!(tree.nodes.first().map(|node| node.key.clone()), Some(vec![1, 0]));
    assert_eq!(tree.nodes.first().map(|node| node.children.clone()), Some(vec![(1, 1)]));
    assert_eq!(tree.nodes.get(1).map(|node| node.key.clone()), Some(vec![3, 0]));
}

#[test]
fn duplicate_keys_are_ignored() {
    let tree = tree_with(&[[1, 1], [1, 1]]);
    assert_eq!(tree.nodes.len(), 1);
}

#[test]
fn find_respects_tolerance() {
    let tree = tree_with(&[[0, 0], [1, 0], [3, 0], [255, 255], [0, 7]]);
    assert_eq!(sorted_find(&tree, [0, 0], 0), vec![(0, vec![0, 0])]);
    assert_eq!(sorted_find(&tree, [0, 0], 1), vec![(0, vec![0, 0]), (1, vec![1, 0])]);
    assert_eq!(sorted_find(&tree, [0, 0], 3), vec![(0, vec![0, 0]), (1, vec![1, 0]), (2, vec![3, 0]), (3, vec![0, 7])]);
    assert_eq!(sorted_find(&tree, [255, 0], 0), vec![]);
}

#[test]
fn find_exact_returns_only_equal_key() {
    let tree = tree_with(&[[0, 0], [1, 0], [3, 0]]);
    assert_eq!(tree.find_exact(&vec![3, 0]), Some(&vec![3, 0]));
    assert_eq!(tree.find_exact(&vec![2, 0]), None);
}

struct AbsoluteDifference;

impl Metric<i64> for AbsoluteDifference {
    fn distance(&self, a: &i64, b: &i64) -> u32 {
        a.abs_diff(*b).try_into().unwrap_or(u32::MAX)
    }

    fn threshold_distance(&self, a: &i64, b: &i64, threshold: u32) -> Option<u32> {
        Some(self.distance(a, b)).filter(|distance| *distance <= threshold)
    }
}

struct Hamming;

impl Metric<Vec<u8>> for Hamming {
    fn distance(&self, a: &Vec<u8>, b: &Vec<u8>) -> u32 {
        a.iter().zip(b).map(|(x, y)| (x ^ y).count_ones()).sum()
    }

    fn threshold_distance(&self, a: &Vec<u8>, b: &Vec<u8>, threshold: u32) -> Option<u32> {
        Some(self.distance(a, b)).filter(|distance| *distance <= threshold)
    }
}

fn brute_force<K: Clone + Ord, M: Metric<K>>(metric: &M, keys: &[K], query: &K, tolerance: u32) -> Vec<(u32, K)> {
    let mut found: Vec<_> = keys
        .iter()
        .map(|key| (metric.distance(query, key), key.clone()))
        .filter(|(distance, _)| *distance <= tolerance)
        .collect();
    found.sort();
    found.dedup();
    found
}

fn tree_find<K: Clone + Ord, M: Metric<K>>(tree: &BKTree<K, M>, query: &K, tolerance: u32) -> Vec<(u32, K)> {
    let mut found: Vec<_> = tree.find(query, tolerance).map(|(distance, key)| (distance, key.clone())).collect();
    found.sort();
    found
}

#[test]
fn integer_metric_matches_linear_scan() {
    for seed in 0..10 {
        let mut rng = StdRng::seed_from_u64(seed);
        let keys: Vec<i64> = (0..500).map(|_| rng.random_range(-300..300)).collect();
        let mut tree = BKTree::new(AbsoluteDifference);
        tree.extend(keys.iter().copied());
        for query in -320..320 {
            for tolerance in [0, 1, 3, 17, 1000] {
                assert_eq!(
                    tree_find(&tree, &query, tolerance),
                    brute_force(&AbsoluteDifference, &keys, &query, tolerance),
                    "seed={seed} query={query} tolerance={tolerance}"
                );
            }
        }
    }
}

#[test]
fn hamming_metric_matches_linear_scan() {
    for byte_size in [1, 8, 32, 128] {
        let mut rng = StdRng::seed_from_u64(byte_size as u64);
        let keys: Vec<Vec<u8>> = (0..400).map(|_| (0..byte_size).map(|_| rng.random_range(0..8u8) << 3).collect()).collect();
        let mut tree = BKTree::new(Hamming);
        tree.extend(keys.iter().cloned());
        for query in keys.iter().take(100) {
            for tolerance in [0, 1, 2, 5, 10, 20, 1000] {
                assert_eq!(
                    tree_find(&tree, query, tolerance),
                    brute_force(&Hamming, &keys, query, tolerance),
                    "byte_size={byte_size} tolerance={tolerance}"
                );
            }
        }
    }
}

#[test]
fn insertion_order_does_not_change_the_set_of_results() {
    let mut rng = StdRng::seed_from_u64(7);
    let keys: Vec<i64> = (0..300).map(|_| rng.random_range(0..1000)).collect();
    let mut shuffled = keys.clone();
    shuffled.shuffle(&mut rng);

    let mut tree = BKTree::new(AbsoluteDifference);
    tree.extend(keys.iter().copied());
    let mut shuffled_tree = BKTree::new(AbsoluteDifference);
    shuffled_tree.extend(shuffled);

    for query in (0..1000).step_by(13) {
        assert_eq!(tree_find(&tree, &query, 25), tree_find(&shuffled_tree, &query, 25), "query={query}");
    }
}

#[test]
fn every_key_is_found_exactly_once_and_duplicates_are_stored_once() {
    let keys: Vec<i64> = vec![5, 5, 5, 9, 9, 1, 1, 1, 1, 100];
    let mut tree = BKTree::new(AbsoluteDifference);
    tree.extend(keys);
    assert_eq!(tree_find(&tree, &0, u32::MAX), vec![(1, 1), (5, 5), (9, 9), (100, 100)]);
    for key in [1, 5, 9, 100] {
        assert_eq!(tree.find_exact(&key), Some(&key));
        assert_eq!(tree.find(&key, 0).count(), 1);
    }
}

#[test]
fn zero_distance_chain_does_not_overflow_the_cutoff() {
    let mut tree = BKTree::new(AbsoluteDifference);
    tree.extend([0, 1]);
    assert_eq!(tree_find(&tree, &0, u32::MAX), vec![(0, 0), (1, 1)]);
}
