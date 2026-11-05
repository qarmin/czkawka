//! Decompressing a deflate stream that may stop part way.
//!
//! This exists because the obvious call does not do it. `miniz_oxide`'s
//! `decompress_to_vec_with_limit` hands back its working buffer when it fails, and that buffer
//! is only ever *grown* - never trimmed to the number of bytes actually produced. A truncated
//! stream therefore comes back as the real output followed by a run of zeros, which looks
//! exactly like recovered content that happens to be blank. Measured on a half-truncated
//! stream: 146 real bytes, then 366 zeros.
//!
//! Driving the decompressor directly gives the count of bytes written, so the result can be
//! trimmed to what genuinely came out. Both callers care: PNG decodes past damage to salvage
//! rows, and ZIP salvages the readable part of a member whose tail is missing.

use miniz_oxide::inflate::TINFLStatus;
use miniz_oxide::inflate::core::{DecompressorOxide, decompress, inflate_flags};

/// Where to start the output buffer before it starts doubling.
const INITIAL_CAPACITY: usize = 32 * 1024;

pub struct Inflated {
    /// Exactly the bytes the decompressor produced - no padding.
    pub data: Vec<u8>,
    /// True when the stream ended properly rather than running out or failing.
    pub complete: bool,
}

/// Decompresses `payload`, keeping whatever came out if it fails part way.
///
/// `zlib` selects the two-byte wrapper PNG uses; ZIP members are raw deflate.
pub fn inflate(payload: &[u8], limit: usize, zlib: bool) -> Inflated {
    let mut flags = inflate_flags::TINFL_FLAG_USING_NON_WRAPPING_OUTPUT_BUF;
    if zlib {
        flags |= inflate_flags::TINFL_FLAG_PARSE_ZLIB_HEADER;
    }

    let mut state = Box::<DecompressorOxide>::default();
    let mut out = vec![0u8; INITIAL_CAPACITY.min(limit.max(1))];
    let mut written = 0;
    let mut input = payload;

    loop {
        let (status, consumed, produced) = decompress(&mut state, input, &mut out, written, flags);
        written += produced;
        input = input.get(consumed..).unwrap_or_default();

        match status {
            TINFLStatus::Done => {
                out.truncate(written);
                return Inflated { data: out, complete: true };
            }
            TINFLStatus::HasMoreOutput if out.len() < limit => {
                // The whole output has to stay addressable: back-references reach into it.
                let grown = out.len().saturating_mul(2).clamp(INITIAL_CAPACITY, limit);
                out.resize(grown, 0);
            }
            _ => {
                out.truncate(written);
                return Inflated { data: out, complete: false };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<u8> {
        (0..50_000u32).map(|index| (index % 251) as u8).collect()
    }

    #[test]
    fn a_whole_stream_comes_back_exactly() {
        let original = sample();
        let compressed = miniz_oxide::deflate::compress_to_vec(&original, 6);
        let result = inflate(&compressed, 1 << 20, false);

        assert!(result.complete);
        assert_eq!(result.data, original);
    }

    #[test]
    fn a_zlib_wrapped_stream_needs_the_flag() {
        let original = sample();
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(&original, 6);

        assert!(inflate(&compressed, 1 << 20, true).complete);
        assert!(!inflate(&compressed, 1 << 20, false).complete, "raw mode must refuse it");
    }

    /// The reason this module exists: what comes back must be the bytes produced, with no
    /// trailing zeros from the working buffer.
    #[test]
    fn a_truncated_stream_yields_the_bytes_that_decompressed_and_no_padding() {
        let original = sample();
        let mut compressed = miniz_oxide::deflate::compress_to_vec(&original, 6);
        compressed.truncate(compressed.len() / 2);

        let result = inflate(&compressed, 1 << 20, false);
        assert!(!result.complete);
        assert!(!result.data.is_empty());
        assert!(result.data.len() < original.len());
        assert_eq!(
            result.data,
            original.get(..result.data.len()).expect("a prefix of the original"),
            "every byte returned must be original content"
        );
    }

    #[test]
    fn the_limit_is_respected() {
        let original = sample();
        let compressed = miniz_oxide::deflate::compress_to_vec(&original, 6);
        let result = inflate(&compressed, 1024, false);

        assert!(!result.complete);
        assert!(result.data.len() <= 1024);
    }

    #[test]
    fn nonsense_input_produces_nothing_rather_than_panicking() {
        let result = inflate(&[0xFFu8; 64], 1 << 20, false);
        assert!(!result.complete);
        assert!(inflate(&[], 1 << 20, false).data.is_empty());
    }
}
