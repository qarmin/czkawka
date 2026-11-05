//! Deterministic damage generator.
//!
//! Repair quality cannot be measured against files whose original content is unknown, so the
//! development corpus is produced here: healthy input plus a seed yields a damaged file *and*
//! a record of exactly which bytes stopped being trustworthy. Tests use that record to assert
//! the strong property - that a lossless repair reproduces the undamaged region bit for bit.

use std::fmt;
use std::ops::Range;

use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

/// A value in `0.0..=1.0`. Constructing one out of range is impossible, so damage positions
/// can never silently land outside the file.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Fraction(f64);

impl Fraction {
    pub const ZERO: Self = Self(0.0);
    pub const HALF: Self = Self(0.5);
    pub const ONE: Self = Self(1.0);

    pub fn new(value: f64) -> Option<Self> {
        (value.is_finite() && (0.0..=1.0).contains(&value)).then_some(Self(value))
    }

    pub fn get(self) -> f64 {
        self.0
    }

    /// Scales a byte count, saturating at `total`.
    fn of(self, total: u64) -> u64 {
        let scaled = (total as f64 * self.0).round();
        if scaled < 0.0 { 0 } else { (scaled as u64).min(total) }
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

/// A length expressed either absolutely or relative to the file size.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ByteSpan {
    Bytes(u64),
    Fraction(Fraction),
}

impl ByteSpan {
    fn resolve(self, total: u64) -> u64 {
        match self {
            Self::Bytes(n) => n.min(total),
            Self::Fraction(f) => f.of(total),
        }
    }
}

impl fmt::Display for ByteSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes(n) => write!(f, "{n}b"),
            Self::Fraction(fr) => write!(f, "{fr}x"),
        }
    }
}

/// The damage classes modelled in v1. Removing a range (which shifts every following offset)
/// is deliberately absent: it needs a resynchronization mechanism nothing else here shares.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DamageKind {
    /// Interrupted transfer or recording: everything past a point is simply gone.
    TruncateTail { keep: Fraction },
    /// Failing disk sector: a run of bytes reads back as zeros, file size unchanged.
    ZeroRange { at: Fraction, len: ByteSpan },
    /// Failing flash cell: a run of bytes reads back as garbage, file size unchanged.
    RandomRange { at: Fraction, len: ByteSpan },
    /// The leading bytes, which normally carry the magic number and geometry, are scrambled.
    HeaderNoise { len: ByteSpan },
}

impl DamageKind {
    /// Stable identifier used as a key in the snapshot table of recovery metrics.
    pub fn label(self) -> String {
        match self {
            Self::TruncateTail { keep } => format!("truncate_keep{keep}"),
            Self::ZeroRange { at, len } => format!("zero_at{at}_len{len}"),
            Self::RandomRange { at, len } => format!("random_at{at}_len{len}"),
            Self::HeaderNoise { len } => format!("header_len{len}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DamageSpec {
    pub kind: DamageKind,
    pub seed: u64,
}

impl DamageSpec {
    pub fn new(kind: DamageKind, seed: u64) -> Self {
        Self { kind, seed }
    }

    pub fn label(&self) -> String {
        format!("{}_seed{}", self.kind.label(), self.seed)
    }
}

/// A damaged file together with the ground truth about what was done to it.
#[derive(Debug, Clone)]
pub struct DamagedFile {
    pub data: Vec<u8>,
    /// Ranges expressed in *original* file coordinates whose content is no longer trustworthy.
    pub damaged: Vec<Range<u64>>,
    /// Offset of the first original byte that is missing entirely, when the file was shortened.
    pub truncated_at: Option<u64>,
    pub original_len: u64,
}

impl DamagedFile {
    /// Length of the leading region that is still byte-identical to the original. A lossless
    /// repair must reproduce at least this much content unchanged.
    pub fn intact_prefix_len(&self) -> u64 {
        let first_damage = self.damaged.iter().map(|r| r.start).min();
        match (first_damage, self.truncated_at) {
            (Some(a), Some(b)) => a.min(b),
            (Some(a), None) => a,
            (None, Some(b)) => b,
            (None, None) => self.original_len,
        }
    }

    /// True when the given original offset still holds its original value.
    pub fn is_intact_at(&self, offset: u64) -> bool {
        if self.truncated_at.is_some_and(|t| offset >= t) {
            return false;
        }
        !self.damaged.iter().any(|r| r.contains(&offset))
    }
}

/// Applies `spec` to `original`, returning the damaged bytes and what was damaged.
///
/// Deterministic: the same `(original, spec)` always produces the same output, which is what
/// makes the snapshot table of recovery metrics meaningful across runs and machines.
pub fn apply_damage(original: &[u8], spec: &DamageSpec) -> DamagedFile {
    let total = original.len() as u64;
    let mut rng = ChaCha8Rng::seed_from_u64(spec.seed);

    match spec.kind {
        DamageKind::TruncateTail { keep } => truncate_tail(original, keep),
        DamageKind::ZeroRange { at, len } => overwrite_range(original, at, len, &mut rng, Fill::Zeros),
        DamageKind::RandomRange { at, len } => overwrite_range(original, at, len, &mut rng, Fill::Random),
        DamageKind::HeaderNoise { len } => {
            let span = ByteSpan::Bytes(len.resolve(total));
            let start = Fraction(0.0);
            overwrite_range(original, start, span, &mut rng, Fill::Random)
        }
    }
}

#[derive(Clone, Copy)]
enum Fill {
    Zeros,
    Random,
}

fn truncate_tail(original: &[u8], keep: Fraction) -> DamagedFile {
    let total = original.len() as u64;
    let kept = keep.of(total);
    let data = original.get(..kept as usize).unwrap_or(original).to_vec();

    DamagedFile {
        data,
        damaged: Vec::new(),
        truncated_at: (kept < total).then_some(kept),
        original_len: total,
    }
}

fn overwrite_range(original: &[u8], at: Fraction, len: ByteSpan, rng: &mut ChaCha8Rng, fill: Fill) -> DamagedFile {
    let total = original.len() as u64;
    let requested = len.resolve(total).max(1).min(total);
    let start = at.of(total).min(total.saturating_sub(requested));
    let end = (start + requested).min(total);

    let mut data = original.to_vec();
    if let Some(slice) = data.get_mut(start as usize..end as usize) {
        match fill {
            Fill::Zeros => slice.fill(0),
            Fill::Random => rng.fill(slice),
        }
    }

    DamagedFile {
        data,
        damaged: if start < end { std::iter::once(start..end).collect() } else { Vec::new() },
        truncated_at: None,
        original_len: total,
    }
}

/// The damage set used by the regression corpus: one instance of each class, at positions
/// chosen to hit structurally different parts of a file.
pub fn standard_corpus(seed: u64) -> Vec<DamageSpec> {
    let f = |v: f64| Fraction::new(v).unwrap_or(Fraction(0.0));
    vec![
        DamageSpec::new(DamageKind::TruncateTail { keep: f(0.8) }, seed),
        DamageSpec::new(DamageKind::TruncateTail { keep: f(0.5) }, seed),
        DamageSpec::new(DamageKind::TruncateTail { keep: f(0.2) }, seed),
        DamageSpec::new(
            DamageKind::ZeroRange {
                at: f(0.5),
                len: ByteSpan::Bytes(4096),
            },
            seed,
        ),
        DamageSpec::new(
            DamageKind::RandomRange {
                at: f(0.3),
                len: ByteSpan::Bytes(4096),
            },
            seed,
        ),
        DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(32) }, seed),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<u8> {
        (0..1000u32).map(|i| (i % 251) as u8).collect()
    }

    fn frac(v: f64) -> Fraction {
        Fraction::new(v).unwrap()
    }

    #[test]
    fn fraction_rejects_values_outside_the_unit_interval() {
        assert!(Fraction::new(-0.1).is_none());
        assert!(Fraction::new(1.1).is_none());
        assert!(Fraction::new(f64::NAN).is_none());
        assert!(Fraction::new(0.0).is_some());
        assert!(Fraction::new(1.0).is_some());
    }

    #[test]
    fn truncation_keeps_the_requested_prefix_verbatim() {
        let original = sample();
        let damaged = apply_damage(&original, &DamageSpec::new(DamageKind::TruncateTail { keep: frac(0.4) }, 1));

        assert_eq!(damaged.data.len(), 400);
        assert_eq!(damaged.data, original[..400]);
        assert_eq!(damaged.truncated_at, Some(400));
        assert_eq!(damaged.intact_prefix_len(), 400);
    }

    #[test]
    fn zeroing_preserves_file_length_and_reports_the_range() {
        let original = sample();
        let spec = DamageSpec::new(
            DamageKind::ZeroRange {
                at: frac(0.5),
                len: ByteSpan::Bytes(100),
            },
            7,
        );
        let damaged = apply_damage(&original, &spec);

        assert_eq!(damaged.data.len(), original.len());
        assert_eq!(damaged.damaged, vec![500..600]);
        assert!(damaged.data[500..600].iter().all(|&b| b == 0));
        assert_eq!(damaged.data[..500], original[..500]);
        assert_eq!(damaged.data[600..], original[600..]);
        assert_eq!(damaged.intact_prefix_len(), 500);
    }

    #[test]
    fn header_noise_touches_only_the_leading_bytes() {
        let original = sample();
        let spec = DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(16) }, 3);
        let damaged = apply_damage(&original, &spec);

        assert_eq!(damaged.data.len(), original.len());
        assert_eq!(damaged.damaged, vec![0..16]);
        assert_eq!(damaged.data[16..], original[16..]);
        assert_eq!(damaged.intact_prefix_len(), 0);
    }

    #[test]
    fn same_seed_reproduces_identical_bytes() {
        let original = sample();
        let spec = DamageSpec::new(
            DamageKind::RandomRange {
                at: frac(0.25),
                len: ByteSpan::Bytes(64),
            },
            42,
        );

        assert_eq!(apply_damage(&original, &spec).data, apply_damage(&original, &spec).data);
    }

    #[test]
    fn different_seeds_produce_different_bytes() {
        let original = sample();
        let kind = DamageKind::RandomRange {
            at: frac(0.25),
            len: ByteSpan::Bytes(64),
        };

        let first = apply_damage(&original, &DamageSpec::new(kind, 1));
        let second = apply_damage(&original, &DamageSpec::new(kind, 2));
        assert_ne!(first.data, second.data);
    }

    #[test]
    fn a_range_at_the_very_end_is_clamped_inside_the_file() {
        let original = sample();
        let spec = DamageSpec::new(
            DamageKind::ZeroRange {
                at: frac(1.0),
                len: ByteSpan::Bytes(300),
            },
            5,
        );
        let damaged = apply_damage(&original, &spec);

        assert_eq!(damaged.data.len(), original.len());
        assert_eq!(damaged.damaged, vec![700..1000]);
    }

    #[test]
    fn is_intact_at_reflects_both_damage_and_truncation() {
        let original = sample();
        let zeroed = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::ZeroRange {
                    at: frac(0.5),
                    len: ByteSpan::Bytes(10),
                },
                1,
            ),
        );
        assert!(zeroed.is_intact_at(499));
        assert!(!zeroed.is_intact_at(500));
        assert!(zeroed.is_intact_at(510));

        let cut = apply_damage(&original, &DamageSpec::new(DamageKind::TruncateTail { keep: frac(0.5) }, 1));
        assert!(cut.is_intact_at(499));
        assert!(!cut.is_intact_at(500));
    }

    #[test]
    fn empty_and_tiny_inputs_never_panic() {
        for len in 0..4usize {
            let original = vec![0xABu8; len];
            for spec in standard_corpus(9) {
                let damaged = apply_damage(&original, &spec);
                assert!(damaged.data.len() <= original.len());
            }
        }
    }

    #[test]
    fn standard_corpus_labels_are_unique() {
        let corpus = standard_corpus(1);
        let mut labels: Vec<String> = corpus.iter().map(DamageSpec::label).collect();
        labels.sort_unstable();
        let count = labels.len();
        labels.dedup();
        assert_eq!(labels.len(), count);
    }
}
