//! Decoding rows out of a PNG and writing them back, independent of any repair strategy.

use std::io::Cursor;

use png::{Decoder, Encoder, Info, Limits};

use crate::file_fixer::engine::RepairContext;
use crate::file_fixer::error::{FixerError, Result};

/// Rows recovered from a decode attempt, plus what stopped it.
pub struct PartialDecode {
    pub info: Info<'static>,
    /// Concatenated raw rows in the image's own colour type and bit depth.
    pub rows: Vec<u8>,
    pub rows_recovered: u32,
    pub declared_height: u32,
    /// `Some` when decoding ended before the last row.
    pub stopped_by: Option<String>,
}

impl PartialDecode {
    pub fn is_complete(&self) -> bool {
        self.stopped_by.is_none() && self.rows_recovered >= self.declared_height
    }
}

/// Decodes as many rows as `data` allows. Returns `None` when even the header is unusable.
pub fn decode_rows(ctx: &RepairContext<'_>, data: &[u8], ignore_crc: bool) -> Option<PartialDecode> {
    let mut decoder = Decoder::new(Cursor::new(data));
    decoder.set_limits(Limits {
        bytes: usize::try_from(ctx.allocation_limit).unwrap_or(usize::MAX),
    });
    decoder.ignore_checksums(ignore_crc);
    // A malformed text chunk says nothing about the pixels, but the decoder refuses the whole
    // file over one. Seen in the wild on an otherwise perfect image.
    decoder.set_ignore_text_chunk(true);

    let mut reader = decoder.read_info().ok()?;
    let info = reader.info().clone();
    let declared_height = info.height;

    // Adam7 rows arrive in pass order and each covers a subset of the image, so a partial
    // interlaced decode cannot be laid out as a shorter progressive image. Those files are
    // left to the whole-image strategies.
    if info.interlaced {
        return None;
    }

    let mut rows = Vec::new();
    let mut rows_recovered = 0u32;
    let stopped_by = loop {
        match reader.next_row() {
            Ok(Some(row)) => {
                rows.extend_from_slice(row.data());
                rows_recovered += 1;
            }
            Ok(None) => break None,
            Err(error) => break Some(error.to_string()),
        }
    };

    Some(PartialDecode {
        info,
        rows,
        rows_recovered,
        declared_height,
        stopped_by,
    })
}

/// Decodes the whole image in one go, returning its height.
///
/// Used for the health check rather than the row-by-row path, because interlaced images arrive
/// as Adam7 passes there and a healthy one would look unreadable.
pub fn decode_whole(ctx: &RepairContext<'_>, data: &[u8], ignore_crc: bool) -> Option<u32> {
    let mut decoder = Decoder::new(Cursor::new(data));
    decoder.set_limits(Limits {
        bytes: usize::try_from(ctx.allocation_limit).unwrap_or(usize::MAX),
    });
    decoder.ignore_checksums(ignore_crc);
    decoder.set_ignore_text_chunk(true);

    let mut reader = decoder.read_info().ok()?;
    let mut buffer = vec![0; reader.output_buffer_size().expect("png buffer size fits in memory")];
    reader.next_frame(&mut buffer).ok().map(|info| info.height)
}

/// Writes the recovered rows back out as a standalone PNG of `height` rows, carrying over the
/// palette, transparency and colour metadata of the original.
pub fn encode(decoded: &PartialDecode, height: u32) -> Result<Vec<u8>> {
    let mut info = decoded.info.clone();
    info.height = height;
    info.interlaced = false;
    info.animation_control = None;
    info.frame_control = None;

    let expected = usize::try_from(height).unwrap_or(usize::MAX).saturating_mul(row_bytes(&info));
    let rows = decoded
        .rows
        .get(..expected)
        .ok_or_else(|| FixerError::malformed("<png>", "recovered rows are shorter than the requested height"))?;

    let mut out = Vec::new();
    let encoder = Encoder::with_info(Cursor::new(&mut out), info).map_err(|e| FixerError::malformed("<png>", e))?;
    let mut writer = encoder.write_header().map_err(|e| FixerError::malformed("<png>", e))?;
    writer.write_image_data(rows).map_err(|e| FixerError::malformed("<png>", e))?;
    writer.finish().map_err(|e| FixerError::malformed("<png>", e))?;
    Ok(out)
}

pub fn row_bytes(info: &Info<'_>) -> usize {
    let channels = info.color_type.samples();
    let bits = usize::try_from(info.width).unwrap_or(0) * channels * usize::from(info.bit_depth as u8);
    bits.div_ceil(8)
}

#[cfg(test)]
mod tests {
    use png::{BitDepth, ColorType};

    use super::*;

    #[test]
    fn row_length_accounts_for_sub_byte_depths() {
        let mut info = Info::default();
        info.width = 10;
        info.color_type = ColorType::Grayscale;
        info.bit_depth = BitDepth::One;
        assert_eq!(row_bytes(&info), 2);

        info.bit_depth = BitDepth::Eight;
        info.color_type = ColorType::Rgba;
        assert_eq!(row_bytes(&info), 40);

        info.width = 3;
        info.color_type = ColorType::Rgb;
        info.bit_depth = BitDepth::Sixteen;
        assert_eq!(row_bytes(&info), 18);
    }

    #[test]
    fn a_zero_width_image_needs_no_row_bytes() {
        let mut info = Info::default();
        info.width = 0;
        info.color_type = ColorType::Rgba;
        info.bit_depth = BitDepth::Eight;
        assert_eq!(row_bytes(&info), 0);
    }
}
