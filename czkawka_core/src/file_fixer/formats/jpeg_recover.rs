//! Recognising a scan header whose marker byte was corrupted.
//!
//! Split out because it is recognition rather than parsing: the segment has to be identified
//! from its own shape, without the `FF` that normally announces it.

use super::jpeg_markers::{Layout, SOS, Scan, parse_scan_components, read_u16_be};

/// Finds a scan header whose leading `FF` was corrupted.
///
/// Seen in a real photograph: the `FF` of `FF DA` had flipped to another byte while the segment
/// behind it was untouched, which left the file unreadable to a strict walk although every
/// pixel survived. The segment describes itself well enough to be recognised on its own - its
/// length has to match its component count exactly, and every component has to be one the frame
/// declares - so this is recognition rather than guesswork.
pub(super) fn recover_scan_header(data: &[u8], layout: &Layout) -> Option<Scan> {
    let frame = layout.frame.as_ref()?;

    for offset in 0..data.len() {
        if data.get(offset) != Some(&SOS) {
            continue;
        }
        let payload_start = offset + 3;
        let Some(length) = read_u16_be(data, offset + 1).map(usize::from) else {
            continue;
        };
        let Some(count) = data.get(payload_start).copied().map(usize::from) else {
            continue;
        };

        // A scan header is exactly six bytes plus two per component, and every component it
        // names must exist in the frame.
        if count == 0 || count > frame.components.len() || length != 6 + 2 * count {
            continue;
        }
        let Some(payload) = data.get(payload_start..offset + 1 + length) else {
            continue;
        };
        let components = parse_scan_components(payload);
        if components.len() != count || !components.iter().all(|scan| frame.components.iter().any(|f| f.id == scan.id)) {
            continue;
        }

        return Some(Scan {
            data_offset: offset + 1 + length,
            components,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::super::jpeg_markers::parse;
    use super::*;

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/Rome.jpg").unwrap_or_default()
    }

    /// Found in a real photograph: the `FF` of a scan header had flipped to another byte while
    /// the segment behind it was untouched, leaving every pixel readable but the file unopenable.
    #[test]
    fn a_scan_header_with_a_damaged_marker_byte_is_recovered() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let expected = parse(&data).scans.first().expect("a scan").data_offset;
        let marker = data[..expected].windows(2).rposition(|w| w == [0xFF, SOS]).expect("SOS");

        let mut damaged = data;
        damaged[marker] = 0xAB;

        let layout = parse(&damaged);
        assert!(layout.is_scannable(), "the segment describes itself well enough to be found");
        assert_eq!(layout.scans.first().expect("a scan").data_offset, expected);
    }

    /// A damaged marker must not be invented where none exists.
    #[test]
    fn a_lone_scan_byte_is_not_mistaken_for_a_header() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let mut truncated = data[..200].to_vec();
        truncated.extend_from_slice(&[SOS, 0x00, 0x02, 0x09]);
        assert!(parse(&truncated).scans.is_empty());
    }
}
