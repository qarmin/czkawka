//! The Huffman tables the JPEG specification defines, for files that do not carry their own.
//!
//! A `DHT` segment is not mandatory. The specification publishes four tables in Annex K and a
//! file may simply rely on them, leaving the decoder to supply them - which every decoder does.
//! Cameras that emit a fixed quantization table often omit `DHT` for the same reason: the bytes
//! are identical in every file they produce.
//!
//! Without this, such a file has no tables to build a scanner from, and the scanner's honest
//! answer - "I cannot read this entropy stream" - comes out as `unrecoverable`, which says the
//! photograph is damaged beyond repair. Three files in a carved collection of 827 JPEGs were
//! exactly this: perfectly good photographs, no `DHT`, condemned.
//!
//! The values are transcribed from the specification. A single wrong byte would desynchronize
//! the scan within the first few blocks, and the test at the bottom checks them against a real
//! file that carries no tables of its own.

use super::jpeg_markers::HuffmanSpec;

/// The standard table for `class` (0 for DC, 1 for AC) and `id` (0 for luminance, 1 for
/// chrominance). Identifiers above one have no standard table.
pub fn standard(class: u8, id: u8) -> Option<HuffmanSpec> {
    let (counts, values): (&[u8; 16], &[u8]) = match (class, id) {
        (0, 0) => (&DC_LUMINANCE_COUNTS, &DC_VALUES),
        (0, 1) => (&DC_CHROMINANCE_COUNTS, &DC_VALUES),
        (1, 0) => (&AC_LUMINANCE_COUNTS, &AC_LUMINANCE_VALUES),
        (1, 1) => (&AC_CHROMINANCE_COUNTS, &AC_CHROMINANCE_VALUES),
        _ => return None,
    };
    Some(HuffmanSpec {
        class,
        id,
        counts: *counts,
        values: values.to_vec(),
    })
}

const DC_LUMINANCE_COUNTS: [u8; 16] = [0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0];
const DC_CHROMINANCE_COUNTS: [u8; 16] = [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0];

/// Both DC tables code the same twelve symbols; only the code lengths differ.
const DC_VALUES: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

const AC_LUMINANCE_COUNTS: [u8; 16] = [0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4, 4, 0, 0, 1, 0x7D];
const AC_CHROMINANCE_COUNTS: [u8; 16] = [0, 2, 1, 2, 4, 4, 3, 4, 7, 5, 4, 4, 0, 1, 2, 0x77];

const AC_LUMINANCE_VALUES: [u8; 162] = [
    0x01, 0x02, 0x03, 0x00, 0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32, 0x81, 0x91, 0xA1, 0x08, 0x23, 0x42, 0xB1, 0xC1, 0x15,
    0x52, 0xD1, 0xF0, 0x24, 0x33, 0x62, 0x72, 0x82, 0x09, 0x0A, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x43,
    0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78,
    0x79, 0x7A, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xB2,
    0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xE1, 0xE2, 0xE3,
    0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA,
];

const AC_CHROMINANCE_VALUES: [u8; 162] = [
    0x00, 0x01, 0x02, 0x03, 0x11, 0x04, 0x05, 0x21, 0x31, 0x06, 0x12, 0x41, 0x51, 0x07, 0x61, 0x71, 0x13, 0x22, 0x32, 0x81, 0x08, 0x14, 0x42, 0x91, 0xA1, 0xB1, 0xC1, 0x09, 0x23,
    0x33, 0x52, 0xF0, 0x15, 0x62, 0x72, 0xD1, 0x0A, 0x16, 0x24, 0x34, 0xE1, 0x25, 0xF1, 0x17, 0x18, 0x19, 0x1A, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A,
    0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x73, 0x74, 0x75, 0x76, 0x77,
    0x78, 0x79, 0x7A, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9,
    0xAA, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xE2,
    0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA,
];

/// Strips every `DHT` segment from a JPEG, leaving a file that is only readable through the
/// standard tables. Used by the tests to build the case this module exists for.
#[cfg(test)]
pub fn without_huffman_tables(data: &[u8]) -> Vec<u8> {
    let mut out = data.get(..2).unwrap_or_default().to_vec();
    let mut offset = 2;

    while offset + 4 <= data.len() {
        if data.get(offset) != Some(&0xFF) {
            offset += 1;
            continue;
        }
        let marker = data.get(offset + 1).copied().unwrap_or(0);
        if marker == 0xDA {
            out.extend_from_slice(data.get(offset..).unwrap_or_default());
            break;
        }
        let Some(length) = crate::file_fixer::io::read_u16_be(data, offset + 2).map(usize::from) else {
            break;
        };
        if marker != 0xC4 {
            out.extend_from_slice(data.get(offset..offset + 2 + length).unwrap_or_default());
        }
        offset += 2 + length;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::super::jpeg_scan::HuffmanTable;
    use super::*;

    /// A Huffman table is only valid if its code lengths account for exactly as many symbols as
    /// it lists, so this catches a transcription slip in the counts.
    #[test]
    fn every_standard_table_is_internally_consistent() {
        for (class, id) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            let table = standard(class, id).expect("the four standard tables exist");
            let declared: usize = table.counts.iter().map(|count| usize::from(*count)).sum();
            assert_eq!(declared, table.values.len(), "class {class} id {id}: {declared} codes for {} symbols", table.values.len());
        }
        assert!(standard(1, 2).is_none(), "only two identifiers have a standard table");
    }

    /// Every standard table has to build into a usable canonical Huffman table - the same
    /// construction a table read from the file goes through.
    #[test]
    fn every_standard_table_builds() {
        for (class, id) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
            let spec = standard(class, id).expect("a standard table");
            assert!(HuffmanTable::build(&spec).is_some(), "class {class} id {id} does not build");
        }
    }

    /// The symbol lists have no duplicates: a repeated value would mean two codes decode to the
    /// same symbol and one of them is a typo.
    #[test]
    fn the_symbol_lists_are_free_of_duplicates() {
        for (class, id) in [(0, 0), (1, 0), (1, 1)] {
            let table = standard(class, id).expect("a standard table");
            let mut seen = table.values.clone();
            seen.sort_unstable();
            let count = seen.len();
            seen.dedup();
            assert_eq!(seen.len(), count, "class {class} id {id} lists a symbol twice");
        }
    }
}
