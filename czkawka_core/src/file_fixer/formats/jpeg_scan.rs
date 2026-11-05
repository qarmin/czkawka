//! Walking a baseline JPEG's entropy-coded data to find where it stops being decodable.
//!
//! No crate offers this. `zune-jpeg` silently returns a full-size buffer whose tail is
//! garbage, and `jpeg-decoder` returns a hard error with no rows at all; neither can say
//! *how much* of the scan was good. That answer is the whole basis of a lossless crop, so it
//! is computed here.
//!
//! Only the entropy layer is decoded: Huffman symbols and the bits they introduce. There is
//! no dequantization, no inverse DCT and no upsampling, because the question is "does this
//! bitstream parse", not "what colour is this pixel".

use super::jpeg_geometry::{self, ScanGeometry};
use super::jpeg_markers::{HuffmanSpec, Layout, Scan};

/// Canonical Huffman table in the decode-friendly form from the JPEG specification.
pub struct HuffmanTable {
    /// Largest code of each length, or -1 when no code has that length.
    max_code: [i32; 17],
    min_code: [i32; 17],
    /// Index into `values` of the first code of each length.
    value_index: [usize; 17],
    values: Vec<u8>,
}

impl HuffmanTable {
    #[expect(clippy::indexing_slicing, reason = "code length is bounded to 1..=16 by the loop and every array here is [_; 17]")]
    pub fn build(spec: &HuffmanSpec) -> Option<Self> {
        let mut max_code = [-1i32; 17];
        let mut min_code = [0i32; 17];
        let mut value_index = [0usize; 17];

        let mut code = 0i32;
        let mut index = 0usize;
        for length in 1..=16usize {
            let count = usize::from(*spec.counts.get(length - 1)?);
            if count == 0 {
                max_code[length] = -1;
                code <<= 1;
                continue;
            }
            value_index[length] = index;
            min_code[length] = code;
            code += i32::try_from(count).ok()?;
            index += count;
            max_code[length] = code - 1;
            // A code must fit in its own length, otherwise the table is malformed.
            if max_code[length] >= (1 << length) {
                return None;
            }
            code <<= 1;
        }

        if index != spec.values.len() {
            return None;
        }
        Some(Self {
            max_code,
            min_code,
            value_index,
            values: spec.values.clone(),
        })
    }

    fn decode(&self, reader: &mut BitReader<'_>) -> Option<u8> {
        let mut code = 0i32;
        for length in 1..=16usize {
            code = (code << 1) | i32::from(reader.next_bit()?);
            let max = *self.max_code.get(length)?;
            if max >= 0 && code <= max {
                let offset = usize::try_from(code - self.min_code.get(length)?).ok()?;
                return self.values.get(self.value_index.get(length)? + offset).copied();
            }
        }
        None
    }
}

/// Position inside the entropy stream, precise enough to cut the file at a bit boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BitPosition {
    /// Index of the next unread raw byte.
    pub next_byte: usize,
    /// Index of the raw byte currently being consumed.
    pub current_byte: usize,
    /// Unread bits remaining in the current byte.
    pub bits_left: u8,
}

struct BitReader<'a> {
    data: &'a [u8],
    next_byte: usize,
    current_byte: usize,
    current: u8,
    bits_left: u8,
    /// Set when a marker other than a stuffed `FF 00` was reached.
    marker: Option<u8>,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8], start: usize) -> Self {
        Self {
            data,
            next_byte: start,
            current_byte: start,
            current: 0,
            bits_left: 0,
            marker: None,
        }
    }

    fn position(&self) -> BitPosition {
        BitPosition {
            next_byte: self.next_byte,
            current_byte: self.current_byte,
            bits_left: self.bits_left,
        }
    }

    /// Pulls the next data byte, unstuffing `FF 00` and stopping at any real marker.
    fn load(&mut self) -> Option<()> {
        let byte = *self.data.get(self.next_byte)?;
        if byte != 0xFF {
            self.current_byte = self.next_byte;
            self.next_byte += 1;
            self.current = byte;
            self.bits_left = 8;
            return Some(());
        }

        match self.data.get(self.next_byte + 1) {
            Some(0x00) => {
                self.current_byte = self.next_byte;
                self.next_byte += 2;
                self.current = 0xFF;
                self.bits_left = 8;
                Some(())
            }
            Some(&marker) => {
                self.marker = Some(marker);
                None
            }
            None => None,
        }
    }

    fn next_bit(&mut self) -> Option<u8> {
        if self.bits_left == 0 {
            self.load()?;
        }
        self.bits_left -= 1;
        Some((self.current >> self.bits_left) & 1)
    }

    /// Consumes `count` bits. Their value is irrelevant here, only that they exist.
    fn skip_bits(&mut self, count: u8) -> Option<()> {
        for _ in 0..count {
            self.next_bit()?;
        }
        Some(())
    }

    /// Drops the remainder of the current byte, as required before a restart marker.
    fn align_to_byte(&mut self) {
        self.bits_left = 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    /// Every MCU the frame declares was decoded.
    Complete,
    /// The scan ended at a marker, normally `EOI`.
    Marker(u8),
    /// The data ran out mid-symbol.
    OutOfData,
    /// A bit pattern matched no code in the active table.
    InvalidCode,
    /// A restart marker was expected at this MCU boundary and something else was there.
    MissingRestart,
}

impl StopReason {
    pub fn is_clean_end(self) -> bool {
        matches!(self, Self::Complete | Self::Marker(0xD9))
    }
}

#[derive(Debug, Clone)]
pub struct ScanReport {
    pub mcus_decoded: u32,
    pub mcus_expected: u32,
    /// Cut points after each fully decoded MCU row, indexed by completed row count minus one.
    pub row_boundaries: Vec<BitPosition>,
    pub stop: StopReason,
}

impl ScanReport {
    /// Number of MCU rows that decoded end to end.
    pub fn complete_rows(&self) -> u32 {
        u32::try_from(self.row_boundaries.len()).unwrap_or(u32::MAX)
    }
}

struct Tables {
    dc: Vec<HuffmanTable>,
    ac: Vec<HuffmanTable>,
    /// Blocks per MCU for each scan component, in scan order.
    blocks: Vec<u32>,
}

fn build_tables(layout: &Layout, scan: &Scan, geometry: &ScanGeometry) -> Option<Tables> {
    let mut dc = Vec::with_capacity(scan.components.len());
    let mut ac = Vec::with_capacity(scan.components.len());

    for component in &scan.components {
        dc.push(HuffmanTable::build(layout.huffman_table(0, component.dc_table)?.as_ref())?);
        ac.push(HuffmanTable::build(layout.huffman_table(1, component.ac_table)?.as_ref())?);
    }
    Some(Tables {
        dc,
        ac,
        blocks: geometry.blocks_per_unit.clone(),
    })
}

/// Decodes one 8x8 block's coefficients, consuming exactly the bits they occupy.
fn decode_block(reader: &mut BitReader<'_>, dc: &HuffmanTable, ac: &HuffmanTable) -> Result<(), StopReason> {
    let symbol = dc.decode(reader).ok_or(stop_of(reader))?;
    reader.skip_bits(symbol & 0x0F).ok_or(stop_of(reader))?;

    let mut index = 1usize;
    while index < 64 {
        let symbol = ac.decode(reader).ok_or(stop_of(reader))?;
        let run = usize::from(symbol >> 4);
        let size = symbol & 0x0F;

        if size == 0 {
            if run == 15 {
                index += 16;
                continue;
            }
            break; // end of block
        }

        index += run;
        if index > 63 {
            return Err(StopReason::InvalidCode);
        }
        reader.skip_bits(size).ok_or(stop_of(reader))?;
        index += 1;
    }
    Ok(())
}

/// A failed read means either a marker interrupted the stream or the data simply ended.
fn stop_of(reader: &BitReader<'_>) -> StopReason {
    match reader.marker {
        Some(marker) => StopReason::Marker(marker),
        None if reader.next_byte >= reader.data.len() => StopReason::OutOfData,
        None => StopReason::InvalidCode,
    }
}

fn consume_restart(reader: &mut BitReader<'_>, index: u32) -> bool {
    reader.align_to_byte();
    let expected = 0xD0 + u8::try_from(index % 8).unwrap_or(0);
    let at = reader.next_byte;

    if reader.data.get(at) == Some(&0xFF) && reader.data.get(at + 1) == Some(&expected) {
        reader.next_byte = at + 2;
        reader.current_byte = reader.next_byte;
        reader.marker = None;
        return true;
    }
    false
}

/// Walks the scan MCU by MCU, recording how far the bitstream stays coherent.
pub fn scan(data: &[u8], layout: &Layout) -> Option<ScanReport> {
    let scan_header = layout.scans.first()?;
    if !layout.is_scannable() {
        return None;
    }
    let geometry = jpeg_geometry::scan_geometry(layout)?;
    let tables = build_tables(layout, scan_header, &geometry)?;

    let per_row = geometry.units_per_row.max(1);
    let expected = geometry.total_units();
    let mut reader = BitReader::new(data, scan_header.data_offset);
    let mut row_boundaries = Vec::with_capacity(geometry.unit_rows as usize);
    let mut decoded = 0u32;

    let stop = loop {
        if decoded == expected {
            break StopReason::Complete;
        }
        if layout.restart_interval > 0
            && decoded > 0
            && decoded.is_multiple_of(u32::from(layout.restart_interval))
            && !consume_restart(&mut reader, decoded / u32::from(layout.restart_interval) - 1)
        {
            break StopReason::MissingRestart;
        }

        if let Err(reason) = decode_mcu(&mut reader, &tables) {
            break reason;
        }
        decoded += 1;
        if decoded.is_multiple_of(per_row) {
            row_boundaries.push(reader.position());
        }
    };

    Some(ScanReport {
        mcus_decoded: decoded,
        mcus_expected: expected,
        row_boundaries,
        stop,
    })
}

fn decode_mcu(reader: &mut BitReader<'_>, tables: &Tables) -> Result<(), StopReason> {
    for (index, &blocks) in tables.blocks.iter().enumerate() {
        let dc = tables.dc.get(index).ok_or(StopReason::InvalidCode)?;
        let ac = tables.ac.get(index).ok_or(StopReason::InvalidCode)?;
        for _ in 0..blocks {
            decode_block(reader, dc, ac)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::jpeg_markers;
    use super::*;

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/Rome.jpg").unwrap_or_default()
    }

    fn sample_with_restarts() -> Vec<u8> {
        std::fs::read("tmp/Rome_rst.jpg").unwrap_or_default()
    }

    #[test]
    fn a_malformed_table_is_rejected_rather_than_trusted() {
        // Sixteen codes of length one cannot exist; only two do.
        let spec = HuffmanSpec {
            class: 0,
            id: 0,
            counts: [16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            values: vec![0; 16],
        };
        assert!(HuffmanTable::build(&spec).is_none());
    }

    #[test]
    fn a_table_whose_value_count_disagrees_with_its_counts_is_rejected() {
        let spec = HuffmanSpec {
            class: 0,
            id: 0,
            counts: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            values: vec![0; 5],
        };
        assert!(HuffmanTable::build(&spec).is_none());
    }

    #[test]
    fn stuffed_ff_bytes_read_back_as_data() {
        let data = [0xFF, 0x00, 0x0F];
        let mut reader = BitReader::new(&data, 0);

        for _ in 0..8 {
            assert_eq!(reader.next_bit(), Some(1));
        }
        assert_eq!(reader.position().next_byte, 2);
        for _ in 0..4 {
            assert_eq!(reader.next_bit(), Some(0));
        }
    }

    #[test]
    fn a_real_marker_stops_the_reader() {
        let data = [0x0F, 0xFF, 0xD9];
        let mut reader = BitReader::new(&data, 0);

        for _ in 0..8 {
            assert!(reader.next_bit().is_some());
        }
        assert!(reader.next_bit().is_none());
        assert_eq!(reader.marker, Some(0xD9));
    }

    #[test]
    fn a_healthy_file_decodes_every_mcu() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let layout = jpeg_markers::parse(&data);
        let report = scan(&data, &layout).expect("Rome.jpg is scannable");

        assert_eq!(report.mcus_expected, 169);
        assert_eq!(report.mcus_decoded, 169);
        assert_eq!(report.complete_rows(), 13);
        assert!(report.stop.is_clean_end(), "{:?}", report.stop);
    }

    #[test]
    fn a_healthy_file_with_restart_markers_also_decodes_fully() {
        let data = sample_with_restarts();
        if data.is_empty() {
            return;
        }
        let layout = jpeg_markers::parse(&data);
        assert_eq!(layout.restart_interval, 4);

        let report = scan(&data, &layout).expect("the restart variant is scannable");
        assert_eq!(report.mcus_decoded, report.mcus_expected);
        assert!(report.stop.is_clean_end(), "{:?}", report.stop);
    }

    #[test]
    fn truncation_stops_the_scan_partway_and_keeps_the_rows_before_it() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let layout = jpeg_markers::parse(&data);

        for percent in [90usize, 70, 50, 30] {
            let cut = data.len() * percent / 100;
            let report = scan(&data[..cut], &layout).expect("scannable");

            assert!(report.mcus_decoded < report.mcus_expected, "{percent}%");
            assert!(!report.stop.is_clean_end(), "{percent}% -> {:?}", report.stop);
            assert!(report.complete_rows() < 13);
            // Row boundaries must stay inside the data they were measured on.
            for boundary in &report.row_boundaries {
                assert!(boundary.next_byte <= cut);
            }
        }
    }

    #[test]
    fn more_data_never_recovers_fewer_rows() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let layout = jpeg_markers::parse(&data);
        let mut previous = 0;

        for percent in [30usize, 50, 70, 90, 100] {
            let cut = data.len() * percent / 100;
            let rows = scan(&data[..cut], &layout).map_or(0, |r| r.complete_rows());
            assert!(rows >= previous, "{percent}% gave {rows} after {previous}");
            previous = rows;
        }
    }

    #[test]
    fn a_non_scannable_file_is_declined_rather_than_guessed_at() {
        assert!(scan(&[0u8; 32], &jpeg_markers::parse(&[0u8; 32])).is_none());
    }

    #[test]
    fn hostile_bytes_in_the_scan_never_panic() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let layout = jpeg_markers::parse(&data);
        let scan_start = layout.scans[0].data_offset;

        for seed in 0..64usize {
            let mut corrupted = data.clone();
            let at = scan_start + seed * 37;
            if let Some(byte) = corrupted.get_mut(at) {
                *byte = (seed * 7) as u8;
            }
            let _ = scan(&corrupted, &layout);
        }
    }
}
