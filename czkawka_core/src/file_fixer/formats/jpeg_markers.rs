//! Parsing the marker segments that precede a JPEG's entropy-coded data.
//!
//! Only what the scanner and the repair strategies need: frame geometry, component sampling,
//! Huffman tables, the restart interval, and where the scan data begins. No pixels are
//! produced here.

use crate::file_fixer::io::read_u32_be;

pub const SOI: u8 = 0xD8;
pub const EOI: u8 = 0xD9;
pub const SOS: u8 = 0xDA;
pub const DHT: u8 = 0xC4;
pub const DRI: u8 = 0xDD;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameKind {
    /// SOF0/SOF1: a single sequential scan, which is what the entropy scanner understands.
    Baseline,
    /// SOF2: coefficients are spread over several scans with spectral selection.
    Progressive,
    /// Lossless, hierarchical, or arithmetic-coded variants.
    Unsupported,
}

impl FrameKind {
    fn from_marker(marker: u8) -> Option<Self> {
        match marker {
            0xC0 | 0xC1 => Some(Self::Baseline),
            0xC2 => Some(Self::Progressive),
            0xC3 | 0xC5..=0xC7 | 0xC9..=0xCB | 0xCD..=0xCF => Some(Self::Unsupported),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FrameComponent {
    pub id: u8,
    /// Horizontal and vertical sampling factors, 1..=4.
    pub h: u8,
    pub v: u8,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub kind: FrameKind,
    pub precision: u8,
    pub width: u16,
    pub height: u16,
    /// Offset of the two-byte height field, which a lossless crop rewrites in place.
    pub height_offset: usize,
    pub components: Vec<FrameComponent>,
}

impl Frame {
    /// Largest horizontal and vertical sampling factors, which size an interleaved unit.
    pub fn max_sampling(&self) -> (u8, u8) {
        let h_max = self.components.iter().map(|c| c.h).max().unwrap_or(1).max(1);
        let v_max = self.components.iter().map(|c| c.v).max().unwrap_or(1).max(1);
        (h_max, v_max)
    }

    /// Pixel size of one minimum coded unit.
    pub fn mcu_size(&self) -> (u32, u32) {
        let (h_max, v_max) = self.max_sampling();
        (u32::from(h_max) * 8, u32::from(v_max) * 8)
    }

    /// How many MCUs make up one row of the image.
    pub fn mcus_per_row(&self) -> u32 {
        let (mcu_width, _) = self.mcu_size();
        u32::from(self.width).div_ceil(mcu_width.max(1))
    }

    pub fn mcu_rows(&self) -> u32 {
        let (_, mcu_height) = self.mcu_size();
        u32::from(self.height).div_ceil(mcu_height.max(1))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScanComponent {
    pub id: u8,
    pub dc_table: u8,
    pub ac_table: u8,
}

#[derive(Debug, Clone)]
pub struct Scan {
    /// First byte of entropy-coded data, just past the scan header.
    pub data_offset: usize,
    pub components: Vec<ScanComponent>,
}

#[derive(Debug, Clone)]
pub struct HuffmanSpec {
    /// 0 for a DC table, 1 for an AC table.
    pub class: u8,
    pub id: u8,
    pub counts: [u8; 16],
    pub values: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct Layout {
    pub frame: Option<Frame>,
    pub scans: Vec<Scan>,
    pub huffman: Vec<HuffmanSpec>,
    /// MCUs between restart markers; zero means the encoder emitted none.
    pub restart_interval: u16,
}

impl Layout {
    /// The table a scan asks for, from the file when it carries one and from the specification
    /// when it does not.
    ///
    /// Falling back is not a guess: a file without `DHT` is *defined* to use the standard
    /// tables, and a decoder is expected to supply them. Refusing such a file reported three
    /// perfectly good photographs in a carved collection as damaged beyond repair.
    pub fn huffman_table(&self, class: u8, id: u8) -> Option<std::borrow::Cow<'_, HuffmanSpec>> {
        // Later definitions replace earlier ones, so search from the end.
        self.huffman
            .iter()
            .rev()
            .find(|t| t.class == class && t.id == id)
            .map(std::borrow::Cow::Borrowed)
            .or_else(|| super::jpeg_tables::standard(class, id).map(std::borrow::Cow::Owned))
    }

    /// A file the entropy scanner can work on: baseline, 8-bit, one interleaved scan.
    pub fn is_scannable(&self) -> bool {
        let Some(frame) = &self.frame else {
            return false;
        };
        let Some(scan) = self.scans.first() else {
            return false;
        };
        // A single-component scan is allowed even in a multi-component frame: the geometry
        // handles it, and grayscale files are exactly that case.
        let scan_covers_frame = scan.components.len() == frame.components.len() || scan.components.len() == 1;

        frame.kind == FrameKind::Baseline
            && frame.precision == 8
            // A zero dimension is not a tiny image, it is a file whose size is declared
            // elsewhere or not at all; treating it as complete reported damage as healthy.
            && frame.width > 0
            && frame.height > 0
            && !frame.components.is_empty()
            && scan_covers_frame
            && frame
                .components
                .iter()
                .all(|c| c.h >= 1 && c.h <= 4 && c.v >= 1 && c.v <= 4)
    }
}

pub(super) fn read_u16_be(data: &[u8], offset: usize) -> Option<u16> {
    let bytes: [u8; 2] = data.get(offset..offset + 2)?.try_into().ok()?;
    Some(u16::from_be_bytes(bytes))
}

/// Walks the marker chain up to and including the first scan header.
///
/// Stops describing segments once entropy data begins; locating the end of that data is the
/// scanner's job, not this one's.
pub fn parse(data: &[u8]) -> Layout {
    let mut layout = Layout::default();
    if data.get(..2) != Some(&[0xFF, SOI]) {
        return layout;
    }

    let mut offset = 2;
    while offset + 1 < data.len() {
        if data.get(offset) != Some(&0xFF) {
            offset += 1;
            continue;
        }
        let Some(&marker) = data.get(offset + 1) else { break };
        match marker {
            0xFF => {
                // Fill bytes are legal padding before a marker.
                offset += 1;
                continue;
            }
            // An EOI before any scan header means there is no image data to describe.
            EOI => break,
            // `FF 00` is a stuffed byte and `FF 01` a temporary marker; neither introduces a
            // segment. Real files do contain them between segments, and reading a length after
            // one sends the walk off into nothing.
            0x00 | 0x01 | 0xD0..=0xD7 => {
                offset += 2;
                continue;
            }
            _ => {}
        }

        let Some(length) = read_u16_be(data, offset + 2) else {
            break;
        };
        let length = usize::from(length);
        if length < 2 {
            break;
        }
        let payload_start = offset + 4;
        let Some(payload) = data.get(payload_start..offset + 2 + length) else {
            break;
        };

        apply_segment(&mut layout, marker, payload, payload_start);
        if marker == SOS {
            layout.scans.push(Scan {
                data_offset: offset + 2 + length,
                components: parse_scan_components(payload),
            });
            break;
        }
        offset += 2 + length;
    }

    if layout.scans.is_empty() {
        layout.scans.extend(super::jpeg_recover::recover_scan_header(data, &layout));
    }
    layout
}

fn apply_segment(layout: &mut Layout, marker: u8, payload: &[u8], payload_start: usize) {
    if let Some(kind) = FrameKind::from_marker(marker) {
        if layout.frame.is_none() {
            layout.frame = parse_frame(kind, payload, payload_start);
        }
        return;
    }
    match marker {
        DHT => parse_huffman_tables(payload, &mut layout.huffman),
        DRI => {
            if let Some(interval) = read_u16_be(payload, 0) {
                layout.restart_interval = interval;
            }
        }
        _ => {}
    }
}

fn parse_frame(kind: FrameKind, payload: &[u8], payload_start: usize) -> Option<Frame> {
    let precision = *payload.first()?;
    let height = read_u16_be(payload, 1)?;
    let width = read_u16_be(payload, 3)?;
    let count = usize::from(*payload.get(5)?);

    let mut components = Vec::with_capacity(count);
    for index in 0..count {
        let base = 6 + index * 3;
        let id = *payload.get(base)?;
        let sampling = *payload.get(base + 1)?;
        components.push(FrameComponent {
            id,
            h: sampling >> 4,
            v: sampling & 0x0F,
        });
    }

    Some(Frame {
        kind,
        precision,
        width,
        height,
        height_offset: payload_start + 1,
        components,
    })
}

fn parse_huffman_tables(payload: &[u8], out: &mut Vec<HuffmanSpec>) {
    let mut offset = 0;
    while let Some(&header) = payload.get(offset) {
        let Some(counts_slice) = payload.get(offset + 1..offset + 17) else {
            return;
        };
        let Ok(counts) = <[u8; 16]>::try_from(counts_slice) else {
            return;
        };
        let total: usize = counts.iter().map(|&c| usize::from(c)).sum();
        let Some(values) = payload.get(offset + 17..offset + 17 + total) else {
            return;
        };

        out.push(HuffmanSpec {
            class: header >> 4,
            id: header & 0x0F,
            counts,
            values: values.to_vec(),
        });
        offset += 17 + total;
    }
}

pub(super) fn parse_scan_components(payload: &[u8]) -> Vec<ScanComponent> {
    let Some(&count) = payload.first() else {
        return Vec::new();
    };
    let count = usize::from(count);
    let mut components = Vec::with_capacity(count);

    for index in 0..count {
        let base = 1 + index * 2;
        let (Some(&id), Some(&tables)) = (payload.get(base), payload.get(base + 1)) else {
            break;
        };
        components.push(ScanComponent {
            id,
            dc_table: tables >> 4,
            ac_table: tables & 0x0F,
        });
    }
    components
}

/// True when the bytes look like a JPEG at all, used before spending effort on them.
pub fn looks_like_jpeg(data: &[u8]) -> bool {
    data.starts_with(&[0xFF, SOI]) || read_u32_be(data, 0).is_some_and(|w| w >> 8 == 0x00FF_D8FF >> 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/Rome.jpg").unwrap_or_default()
    }

    #[test]
    fn geometry_matches_the_sampling_factors() {
        let frame = Frame {
            kind: FrameKind::Baseline,
            precision: 8,
            width: 201,
            height: 203,
            height_offset: 0,
            components: vec![
                FrameComponent { id: 1, h: 2, v: 2 },
                FrameComponent { id: 2, h: 1, v: 1 },
                FrameComponent { id: 3, h: 1, v: 1 },
            ],
        };

        assert_eq!(frame.mcu_size(), (16, 16));
        assert_eq!(frame.mcus_per_row(), 13);
        assert_eq!(frame.mcu_rows(), 13);
    }

    #[test]
    fn a_greyscale_frame_has_eight_pixel_mcus() {
        let frame = Frame {
            kind: FrameKind::Baseline,
            precision: 8,
            width: 16,
            height: 9,
            height_offset: 0,
            components: vec![FrameComponent { id: 1, h: 1, v: 1 }],
        };

        assert_eq!(frame.mcu_size(), (8, 8));
        assert_eq!(frame.mcus_per_row(), 2);
        assert_eq!(frame.mcu_rows(), 2);
    }

    #[test]
    fn later_huffman_definitions_win() {
        let layout = Layout {
            huffman: vec![
                HuffmanSpec {
                    class: 0,
                    id: 0,
                    counts: [0; 16],
                    values: vec![1],
                },
                HuffmanSpec {
                    class: 0,
                    id: 0,
                    counts: [0; 16],
                    values: vec![2],
                },
            ],
            ..Layout::default()
        };
        assert_eq!(layout.huffman_table(0, 0).expect("defined twice").values, vec![2]);

        // A table the file never defined falls back to the specification's, which is what a
        // file with no `DHT` at all relies on.
        let standard = layout.huffman_table(1, 0).expect("the standard AC table");
        assert_eq!(standard.values.len(), 162);
        assert!(layout.huffman_table(1, 7).is_none(), "no standard table has that id");
    }

    #[test]
    fn nothing_parses_out_of_data_that_is_not_a_jpeg() {
        let layout = parse(&[0u8; 64]);
        assert!(layout.frame.is_none());
        assert!(layout.scans.is_empty());
        assert!(!layout.is_scannable());
    }

    #[test]
    fn truncating_the_input_never_panics() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        for len in 0..data.len().min(1200) {
            let _ = parse(&data[..len]);
        }
    }

    #[test]
    fn the_real_sample_parses_as_a_scannable_baseline_file() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let layout = parse(&data);
        let frame = layout.frame.as_ref().expect("Rome.jpg has a frame header");

        assert_eq!(frame.kind, FrameKind::Baseline);
        assert_eq!((frame.width, frame.height), (201, 203));
        assert_eq!(frame.components.len(), 3);
        assert_eq!(frame.mcu_size(), (16, 16));
        assert_eq!(layout.restart_interval, 0);
        assert_eq!(layout.scans.len(), 1);
        assert_eq!(layout.huffman.len(), 4);
        assert!(layout.is_scannable());

        // The height field must be addressable in place for a lossless crop.
        let patched = u16::from_be_bytes([data[frame.height_offset], data[frame.height_offset + 1]]);
        assert_eq!(patched, frame.height);
    }

    /// Found in a decoder's crash-test corpus: a frame declaring a zero dimension has no coded
    /// units at all, so "every unit decoded" was trivially true and the file was called healthy.
    #[test]
    fn a_zero_dimension_is_not_scannable() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let layout = parse(&data);
        assert!(layout.is_scannable());
        let height_offset = layout.frame.expect("a frame").height_offset;

        for offset in [height_offset, height_offset + 2] {
            let mut variant = data.clone();
            variant[offset] = 0;
            variant[offset + 1] = 0;
            assert!(!parse(&variant).is_scannable(), "zero at {offset} must be rejected");
        }
    }

    /// Found in a real photograph: a stuffed `FF 00` sitting between header segments was read
    /// as a marker introducing a 59877-byte segment, which sent the walk past the end.
    #[test]
    fn a_stuffed_byte_between_segments_does_not_derail_the_walk() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let reference = parse(&data);
        let scan_offset = reference.scans.first().expect("a scan").data_offset;

        // Splice the pair in just before the scan header, where a real file had it.
        let insert_at = data[..scan_offset].windows(2).rposition(|w| w == [0xFF, 0xDA]).expect("SOS");
        let mut spliced = data[..insert_at].to_vec();
        spliced.extend_from_slice(&[0xFF, 0x00]);
        spliced.extend_from_slice(&data[insert_at..]);

        let layout = parse(&spliced);
        assert!(layout.is_scannable(), "the walk must survive a stuffed byte");
        assert_eq!(layout.huffman.len(), reference.huffman.len());
    }

    #[test]
    fn a_file_with_restart_markers_reports_its_interval() {
        let Ok(data) = std::fs::read("tmp/Rome_rst.jpg") else {
            return;
        };
        assert_eq!(parse(&data).restart_interval, 4);
    }
}
