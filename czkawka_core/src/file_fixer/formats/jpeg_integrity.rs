//! Deciding how much of a JPEG scan can be trusted.
//!
//! JPEG carries no checksum over its entropy data, so altered bytes cannot be detected the way
//! a PNG chunk CRC detects them. Huffman codes are self-synchronizing: after an altered byte
//! the decoder keeps producing *valid* symbols that simply mean something else, so a scan can
//! run to the last MCU and report success while half the image is wrong.
//!
//! The one signal available is **alignment**. A healthy file's scan consumes exactly up to its
//! end marker; a desynchronized one stops somewhere else.

use super::jpeg_markers::{self, Frame, Layout};
use super::jpeg_scan::{self, BitPosition, ScanReport, StopReason};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integrity {
    /// Every MCU decoded and the scan ended exactly at the file's end marker.
    Whole,
    /// Every MCU decoded but at the wrong offset: content was altered at an unknown point.
    Desynchronized,
    /// The data ran out at the end of the file, which is what truncation looks like. Nothing
    /// was altered, so the MCU rows that decoded are the original ones.
    Truncated,
    /// The scan stopped early with data still to come, so some byte before it was altered.
    /// The rows that decoded are probably original, but nothing here can prove it.
    Interrupted,
}

impl Integrity {
    /// Whether the rows this scan produced are provably the original ones.
    pub fn is_provable(self) -> bool {
        matches!(self, Self::Whole | Self::Truncated)
    }
}

/// What a scan pass concluded about a file, shared by every strategy.
pub struct Analysis {
    pub layout: Layout,
    pub report: ScanReport,
    pub integrity: Integrity,
}

impl Analysis {
    pub fn of(data: &[u8]) -> Option<Self> {
        let layout = jpeg_markers::parse(data);
        let report = jpeg_scan::scan(data, &layout)?;
        let integrity = classify(data, &report);
        Some(Self { layout, report, integrity })
    }

    pub fn frame(&self) -> Option<&Frame> {
        self.layout.frame.as_ref()
    }

    /// Where to cut the entropy data to keep only whole MCU rows.
    pub fn cut(&self) -> Option<BitPosition> {
        self.report.row_boundaries.last().copied()
    }

    pub fn scan_data_offset(&self) -> Option<usize> {
        self.layout.scans.first().map(|scan| scan.data_offset)
    }

    /// Image rows covered by one row of coded units, which is the unit a crop works in.
    pub fn rows_per_unit_row(&self) -> u32 {
        super::jpeg_geometry::scan_geometry(&self.layout).map_or(8, |geometry| geometry.pixel_rows_per_unit_row.max(1))
    }
}

fn classify(data: &[u8], report: &ScanReport) -> Integrity {
    let complete = report.stop.is_clean_end() && report.mcus_decoded == report.mcus_expected;
    let cut = report.row_boundaries.last().map_or(0, |boundary| boundary.next_byte);
    let end_marker = find_end_marker(data);

    match (complete, end_marker) {
        (true, None) => Integrity::Whole,
        (true, Some(offset)) if only_fill_between(data, cut, offset) => Integrity::Whole,
        // A marker sitting exactly at the scan end means the encoder stopped there too, which
        // is what a correctly consumed scan looks like even when other segments follow. Real
        // files do that: a second `DHT` for a following scan, or a trailing restart marker.
        (true, Some(_)) if starts_a_marker(data, cut) => Integrity::Whole,
        (true, Some(_)) => Integrity::Desynchronized,
        (false, None) if report.stop == StopReason::OutOfData => Integrity::Truncated,
        (false, _) => Integrity::Interrupted,
    }
}

/// Whether `offset` begins a marker rather than more entropy data.
fn starts_a_marker(data: &[u8], offset: usize) -> bool {
    data.get(offset) == Some(&0xFF) && data.get(offset + 1).is_some_and(|&next| next != 0x00)
}

/// Offset of the file's own end marker, searched from the back.
///
/// Searching forwards would be wrong: valid entropy data always stuffs `FF` as `FF 00`, so a
/// genuine `FF D9` cannot appear inside it, but damaged data can contain that pair by chance.
pub fn find_end_marker(data: &[u8]) -> Option<usize> {
    let last = data.len().checked_sub(1)?;
    (0..last)
        .rev()
        .find(|&index| data.get(index) == Some(&0xFF) && data.get(index + 1) == Some(&jpeg_markers::EOI))
}

/// Only `FF` fill bytes, which are legal padding, may sit between the scan and its end marker.
fn only_fill_between(data: &[u8], from: usize, to: usize) -> bool {
    to >= from && data.get(from..to).is_some_and(|gap| gap.iter().all(|&byte| byte == 0xFF))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/Rome.jpg").unwrap_or_default()
    }

    fn damaged(kind: DamageKind) -> Vec<u8> {
        apply_damage(&sample(), &DamageSpec::new(kind, 1)).data
    }

    /// The regression that motivated this module. Huffman codes keep decoding after an altered
    /// byte, so the scan reaches the last MCU and reports success while more than half the image
    /// is wrong. Only its misalignment with the file's own end marker gives it away.
    #[test]
    fn altered_bytes_that_still_decode_are_caught_by_end_marker_alignment() {
        if sample().is_empty() {
            return;
        }
        assert_eq!(Analysis::of(&sample()).map(|a| a.integrity), Some(Integrity::Whole));

        let zeroed = damaged(DamageKind::ZeroRange {
            at: Fraction::HALF,
            len: ByteSpan::Bytes(4096),
        });
        let analysis = Analysis::of(&zeroed).expect("still scannable");
        assert_eq!(analysis.integrity, Integrity::Desynchronized);
        assert_eq!(analysis.report.mcus_decoded, analysis.report.mcus_expected);
    }

    #[test]
    fn truncation_and_alteration_are_told_apart() {
        if sample().is_empty() {
            return;
        }
        let cut = damaged(DamageKind::TruncateTail {
            keep: Fraction::new(0.8).unwrap(),
        });
        assert_eq!(Analysis::of(&cut).map(|a| a.integrity), Some(Integrity::Truncated));

        let altered = damaged(DamageKind::RandomRange {
            at: Fraction::new(0.3).unwrap(),
            len: ByteSpan::Bytes(4096),
        });
        assert_eq!(Analysis::of(&altered).map(|a| a.integrity), Some(Integrity::Interrupted));
    }

    #[test]
    fn the_end_marker_is_found_from_the_back() {
        assert_eq!(find_end_marker(&[0x00, 0xFF, 0xD9]), Some(1));
        assert_eq!(find_end_marker(&[0xFF, 0xD9, 0x00, 0xFF, 0xD9]), Some(3));
        assert_eq!(find_end_marker(&[0xFF, 0xD8, 0x00]), None);
        assert_eq!(find_end_marker(&[]), None);
        assert_eq!(find_end_marker(&[0xFF]), None);
    }

    #[test]
    fn fill_bytes_are_tolerated_before_the_end_marker() {
        let data = [0x12, 0xFF, 0xFF, 0xFF, 0xD9];
        assert!(only_fill_between(&data, 1, 3));
        assert!(only_fill_between(&data, 3, 3));
        assert!(!only_fill_between(&data, 0, 3));
        assert!(!only_fill_between(&data, 4, 1));
    }

    #[test]
    fn a_marker_is_told_apart_from_entropy_data() {
        assert!(starts_a_marker(&[0xFF, 0xC4], 0));
        assert!(starts_a_marker(&[0x00, 0xFF, 0xD1], 1));
        // A stuffed byte is data, not a marker.
        assert!(!starts_a_marker(&[0xFF, 0x00], 0));
        assert!(!starts_a_marker(&[0xDE, 0xCE], 0));
        assert!(!starts_a_marker(&[0xFF], 0));
    }

    #[test]
    fn only_provable_states_are_marked_as_such() {
        assert!(Integrity::Whole.is_provable());
        assert!(Integrity::Truncated.is_provable());
        assert!(!Integrity::Desynchronized.is_provable());
        assert!(!Integrity::Interrupted.is_provable());
    }

    #[test]
    fn data_that_is_not_a_jpeg_yields_no_analysis() {
        assert!(Analysis::of(&[0u8; 64]).is_none());
    }
}
