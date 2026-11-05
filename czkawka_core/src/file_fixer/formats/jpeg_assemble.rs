//! Writing a JPEG back out from original headers and a prefix of the original scan data.
//!
//! Nothing is re-encoded here. The height in the frame header is rewritten, the entropy data is
//! copied up to a whole-unit boundary, and the unused bits of the final byte are padded the way
//! the specification asks for.

use super::jpeg_integrity::Analysis;
use super::jpeg_markers;
use super::jpeg_scan::BitPosition;
use crate::file_fixer::error::{FixerError, Result};

/// Assembles a JPEG from the original headers plus a prefix of the original scan data.
///
/// `height` replaces the value in the frame header; the caller guarantees it covers whole MCU
/// rows, because a JPEG cannot describe a partial one.
pub fn assemble(data: &[u8], analysis: &Analysis, height: u16, cut: BitPosition) -> Result<Vec<u8>> {
    let frame = analysis.frame().ok_or_else(|| FixerError::malformed("<jpeg>", "no frame header"))?;
    let scan_start = analysis.scan_data_offset().ok_or_else(|| FixerError::malformed("<jpeg>", "no scan header"))?;

    let headers = data
        .get(..scan_start)
        .ok_or_else(|| FixerError::malformed("<jpeg>", "scan header runs past the end of the file"))?;
    let entropy = data
        .get(scan_start..cut.next_byte)
        .ok_or_else(|| FixerError::malformed("<jpeg>", "scan cut point is past the end of the file"))?;

    let mut out = Vec::with_capacity(headers.len() + entropy.len() + 2);
    out.extend_from_slice(headers);
    out.extend_from_slice(entropy);

    patch_height(&mut out, frame.height_offset, height)?;
    pad_trailing_bits(&mut out, scan_start, cut);
    out.extend_from_slice(&[0xFF, jpeg_markers::EOI]);
    Ok(out)
}

pub(super) fn patch_height(out: &mut [u8], offset: usize, height: u16) -> Result<()> {
    let field = out
        .get_mut(offset..offset + 2)
        .ok_or_else(|| FixerError::malformed("<jpeg>", "frame height field is outside the file"))?;
    field.copy_from_slice(&height.to_be_bytes());
    Ok(())
}

/// The unused low bits of the final entropy byte belong to an MCU that is being dropped.
/// JPEG pads with one bits, and a byte that is already `0xFF` needs nothing, so its stuffing
/// byte is left untouched.
pub(super) fn pad_trailing_bits(out: &mut [u8], scan_start: usize, cut: BitPosition) {
    if cut.bits_left == 0 || cut.current_byte < scan_start {
        return;
    }
    if let Some(byte) = out.get_mut(cut.current_byte) {
        *byte |= (1u8 << cut.bits_left) - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn padding_fills_the_unused_bits_with_ones() {
        let mut out = vec![0xFF, 0xD8, 0b1010_0000];
        pad_trailing_bits(
            &mut out,
            2,
            BitPosition {
                next_byte: 3,
                current_byte: 2,
                bits_left: 5,
            },
        );
        assert_eq!(out[2], 0b1011_1111);
    }

    #[test]
    fn a_byte_boundary_needs_no_padding() {
        let mut out = vec![0x12, 0x34];
        let before = out.clone();
        pad_trailing_bits(
            &mut out,
            0,
            BitPosition {
                next_byte: 2,
                current_byte: 1,
                bits_left: 0,
            },
        );
        assert_eq!(out, before);
    }

    /// The cut point can only ever land inside the entropy data, never in the headers.
    #[test]
    fn a_position_before_the_scan_is_left_alone() {
        let mut out = vec![0xAA; 4];
        let before = out.clone();
        pad_trailing_bits(
            &mut out,
            3,
            BitPosition {
                next_byte: 2,
                current_byte: 1,
                bits_left: 4,
            },
        );
        assert_eq!(out, before);
    }

    #[test]
    fn the_height_field_is_replaced_in_place() {
        let mut out = vec![0x00, 0x00, 0xFF, 0xFF, 0x11];
        patch_height(&mut out, 2, 0x0140).expect("in range");
        assert_eq!(out, vec![0x00, 0x00, 0x01, 0x40, 0x11]);
        assert!(patch_height(&mut out, 4, 1).is_err());
    }
}
