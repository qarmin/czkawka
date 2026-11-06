use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter, Result as FmtResult};

use fnv::FnvHashMap;

/// A *metric* (distance function), which should follow the metric axioms:
///
/// * `distance(a, b) == 0` if and only if `a == b`
/// * `distance(a, b) == distance(b, a)`
/// * `distance(a, c) <= distance(a, b) + distance(b, c)`
///
/// If any of these rules are broken, the BK-tree may give unexpected results.
pub trait Metric<K: ?Sized> {
    fn distance(&self, a: &K, b: &K) -> u32;

    /// Returns `Some(distance)` only when the distance is not greater than `threshold`.
    fn threshold_distance(&self, a: &K, b: &K, threshold: u32) -> Option<u32>;
}

struct BKNode<K> {
    key: K,
    /// Children indexed by their distance from this node.
    children: FnvHashMap<u32, Self>,
    max_child_distance: Option<u32>,
}

impl<K> BKNode<K> {
    fn new(key: K) -> Self {
        Self {
            key,
            children: FnvHashMap::default(),
            max_child_distance: None,
        }
    }

    // Does not verify that `distance` really is the metric distance between this node and `key`.
    fn add_child(&mut self, distance: u32, key: K) {
        self.children.insert(distance, Self::new(key));
        self.max_child_distance = self.max_child_distance.max(Some(distance));
    }
}

impl<K: Debug> Debug for BKNode<K> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_map().entry(&self.key, &self.children).finish()
    }
}

/// A [BK-tree](https://en.wikipedia.org/wiki/BK-tree).
pub struct BKTree<K, M> {
    root: Option<BKNode<K>>,
    metric: M,
}

impl<K, M: Metric<K>> BKTree<K, M> {
    pub fn new(metric: M) -> Self {
        Self { root: None, metric }
    }

    /// Adds a key to the tree. A key equal to an already present one is ignored.
    pub fn add(&mut self, key: K) {
        let Some(root) = &mut self.root else {
            self.root = Some(BKNode::new(key));
            return;
        };
        let mut current_node = root;
        let mut current_distance = self.metric.distance(&current_node.key, &key);
        while current_distance > 0 && current_node.children.contains_key(&current_distance) {
            let next_node = current_node.children.get_mut(&current_distance).expect("child existence was checked in loop condition");
            current_node = next_node;
            current_distance = self.metric.distance(&current_node.key, &key);
        }
        if current_distance > 0 {
            current_node.add_child(current_distance, key);
        }
    }

    /// Finds all keys within `tolerance` of `key`, as `(distance, key)` pairs in unspecified order.
    pub fn find<'a, 'q, Q: ?Sized>(&'a self, key: &'q Q, tolerance: u32) -> Find<'a, 'q, K, Q, M>
    where
        K: Borrow<Q>,
        M: Metric<Q>,
    {
        Find {
            candidates: self.root.iter().collect(),
            tolerance,
            metric: &self.metric,
            key,
        }
    }

    pub fn find_exact<Q: ?Sized>(&self, key: &Q) -> Option<&K>
    where
        K: Borrow<Q>,
        M: Metric<Q>,
    {
        self.find(key, 0).next().map(|(_, found_key)| found_key)
    }
}

impl<K, M: Metric<K>> Extend<K> for BKTree<K, M> {
    fn extend<I: IntoIterator<Item = K>>(&mut self, keys: I) {
        for key in keys {
            self.add(key);
        }
    }
}

/// Iterator returned by [`BKTree::find`].
pub struct Find<'a, 'q, K, Q: ?Sized, M> {
    candidates: VecDeque<&'a BKNode<K>>,
    tolerance: u32,
    metric: &'a M,
    key: &'q Q,
}

impl<'a, K, Q: ?Sized, M> Iterator for Find<'a, '_, K, Q, M>
where
    K: Borrow<Q>,
    M: Metric<Q>,
{
    type Item = (u32, &'a K);

    fn next(&mut self) -> Option<(u32, &'a K)> {
        while let Some(current) = self.candidates.pop_front() {
            let distance_cutoff = current.max_child_distance.unwrap_or(0) + self.tolerance;
            let Some(distance) = self.metric.threshold_distance(self.key, current.key.borrow(), distance_cutoff) else {
                continue;
            };
            let min_distance = distance.saturating_sub(self.tolerance);
            let max_distance = distance.saturating_add(self.tolerance);
            self.candidates.extend(
                current
                    .children
                    .iter()
                    .filter(|(child_distance, _)| (min_distance..=max_distance).contains(child_distance))
                    .map(|(_, child_node)| child_node),
            );
            if distance <= self.tolerance {
                return Some((distance, &current.key));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
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
        let root = tree.root.expect("tree has keys");
        assert_eq!(root.key, vec![1, 0]);
        assert_eq!(root.children.get(&1).map(|node| node.key.clone()), Some(vec![3, 0]));
    }

    #[test]
    fn duplicate_keys_are_ignored() {
        let tree = tree_with(&[[1, 1], [1, 1]]);
        assert!(tree.root.expect("tree has keys").children.is_empty());
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
}
