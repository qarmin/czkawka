//! This is a collection of string metrics that are suitable for use with a
//! BK-tree.

#[cfg(feature = "serde")]
extern crate serde;

use Metric;

extern crate triple_accel;
use self::triple_accel::{levenshtein, levenshtein::levenshtein_simd_k};

/// This calculates the Levenshtein distance between two strings.
///
/// The [distance metric itself][1] is calculated using the [Wagner-Fischer][2]
/// dynamic programming algorithm.
///
/// # Examples
///
/// ```
/// use bk_tree::Metric;
/// use bk_tree::metrics::Levenshtein;
///
/// assert_eq!(Levenshtein.distance("bar", "baz"), 1);
/// assert_eq!(Levenshtein.distance("kitten", "sitting"), 3);
/// ```
///
/// [1]: https://en.wikipedia.org/wiki/Levenshtein_distance
/// [2]: https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Levenshtein;

impl<K: AsRef<str> + ?Sized> Metric<K> for Levenshtein {
    fn distance(&self, a: &K, b: &K) -> u32 {
        let a_bytes = a.as_ref().as_bytes();
        let b_bytes = b.as_ref().as_bytes();
        levenshtein(a_bytes, b_bytes)
    }

    fn threshold_distance(&self, a: &K, b: &K, threshold: u32) -> Option<u32> {
        let a_bytes = a.as_ref().as_bytes();
        let b_bytes = b.as_ref().as_bytes();
        levenshtein_simd_k(a_bytes, b_bytes, threshold)
    }
}
