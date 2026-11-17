// Based on bk-tree 0.5.0 by Eugene Bulkin (MIT), https://github.com/eugene-bulkin/rust-bk-tree
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Result as FmtResult};

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
    /// Pairs of (distance from this node, node index), sorted by distance.
    children: Vec<(u32, u32)>,
}

impl<K> BKNode<K> {
    fn max_child_distance(&self) -> u32 {
        self.children.last().map_or(0, |(distance, _)| *distance)
    }
}

impl<K: Debug> Debug for BKNode<K> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.debug_map().entry(&self.key, &self.children).finish()
    }
}

/// A [BK-tree](https://en.wikipedia.org/wiki/BK-tree) stored in a single arena, the first node is the root.
pub struct BKTree<K, M> {
    nodes: Vec<BKNode<K>>,
    metric: M,
}

impl<K, M: Metric<K>> BKTree<K, M> {
    pub fn new(metric: M) -> Self {
        Self { nodes: Vec::new(), metric }
    }

    /// Adds a key to the tree. A key equal to an already present one is ignored.
    pub fn add(&mut self, key: K) {
        let Some(root) = self.nodes.first() else {
            self.nodes.push(BKNode { key, children: Vec::new() });
            return;
        };
        let mut current_index = 0;
        let mut current_distance = self.metric.distance(&root.key, &key);
        while current_distance > 0 {
            let Some(current) = self.nodes.get(current_index) else {
                return;
            };
            match current.children.binary_search_by_key(&current_distance, |(distance, _)| *distance) {
                Ok(position) => {
                    current_index = current.children.get(position).map_or(0, |(_, index)| *index as usize);
                    let Some(next) = self.nodes.get(current_index) else {
                        return;
                    };
                    current_distance = self.metric.distance(&next.key, &key);
                }
                Err(position) => {
                    let new_index = u32::try_from(self.nodes.len()).expect("BK-tree cannot hold more than u32::MAX nodes");
                    self.nodes.push(BKNode { key, children: Vec::new() });
                    if let Some(parent) = self.nodes.get_mut(current_index) {
                        parent.children.insert(position, (current_distance, new_index));
                    }
                    return;
                }
            }
        }
    }

    /// Finds all keys within `tolerance` of `key`, as `(distance, key)` pairs in unspecified order.
    pub fn find<'a, 'q, Q: ?Sized>(&'a self, key: &'q Q, tolerance: u32) -> Find<'a, 'q, K, Q, M>
    where
        K: Borrow<Q>,
        M: Metric<Q>,
    {
        Find {
            nodes: &self.nodes,
            candidates: if self.nodes.is_empty() { Vec::new() } else { vec![0] },
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
    nodes: &'a [BKNode<K>],
    candidates: Vec<u32>,
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
        while let Some(candidate) = self.candidates.pop() {
            let Some(current) = self.nodes.get(candidate as usize) else {
                continue;
            };
            let distance_cutoff = current.max_child_distance().saturating_add(self.tolerance);
            let Some(distance) = self.metric.threshold_distance(self.key, current.key.borrow(), distance_cutoff) else {
                continue;
            };
            let min_distance = distance.saturating_sub(self.tolerance);
            let max_distance = distance.saturating_add(self.tolerance);
            let first = current.children.partition_point(|(child_distance, _)| *child_distance < min_distance);
            let last = current.children.partition_point(|(child_distance, _)| *child_distance <= max_distance);
            if let Some(matching_children) = current.children.get(first..last) {
                self.candidates.extend(matching_children.iter().map(|(_, index)| *index));
            }
            if distance <= self.tolerance {
                return Some((distance, &current.key));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests;
