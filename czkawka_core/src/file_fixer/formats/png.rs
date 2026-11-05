//! PNG repair.
//!
//! Partial decoding comes free from the `png` crate: `Reader::next_row` hands back rows one at
//! a time, so whatever decoded before the damage is already in hand when the error arrives.
//! Nothing here needs to reimplement inflate or the filter chain.
//!
//! Re-encoding is not a compromise for PNG - the format is lossless, so a cropped re-encode
//! carries exactly the original pixel values.
//!
//! The subtlety is knowing *where* a partial decode stops being real. Inflate carries on for
//! a while past corrupted input before it fails, so the last rows it produces can be garbage
//! that looks exactly like data. Chunk checksums resolve this: see [`super::png_chunks`].

use super::png_image::{self, PartialDecode, decode_rows, encode};
use super::{png_chunks, png_rebuild};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy, Support};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct PngRepairer;

impl Repairer for PngRepairer {
    fn format(&self) -> Format {
        Format::Png
    }

    /// An `acTL` chunk turns the file into an animation, where `IDAT` is only the poster frame
    /// and the real content lives in `fdAT` chunks this repairer does not look at at all.
    /// Reporting such a file as fully repaired from the poster frame alone would silently drop
    /// every animation frame, so it is declined honestly instead - the same shape as JPEG's
    /// `Support::Unsupported` for variants its scanner does not understand.
    fn supports(&self, data: &[u8]) -> Support {
        if contains_actl(data) {
            return Support::Unsupported("animated PNG (APNG) is not handled".to_string());
        }
        Support::Supported
    }

    /// Ordered cheapest and most faithful first, so that a healthy or lightly damaged file
    /// short-circuits the search before the expensive options are reached.
    ///
    /// An animated PNG produces no candidates at all: every strategy here only ever looks at
    /// `IDAT`, so a "successful" repair would silently be a repair of the poster frame alone.
    /// Emptiness is what actually drives `Outcome::Unsupported` (see `supports` above for the
    /// reason text) - unlike `supports`, this is the check the engine consults.
    fn strategies(&self, ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        if contains_actl(ctx.data) {
            return Vec::new();
        }
        vec![
            Box::new(Verbatim),
            Box::new(RewriteWithoutChecksums),
            Box::new(png_rebuild::RestoreSignature),
            Box::new(TruncateToVerifiedRows),
            Box::new(DecodePastDamage),
            Box::new(png_rebuild::RebuildHeader),
        ]
    }
}

fn contains_actl(data: &[u8]) -> bool {
    data.windows(4).any(|window| window == b"acTL")
}

fn complete_score(height: u32) -> Score {
    Score {
        content_recovered: u64::from(height),
        content_total: Some(u64::from(height)),
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: true,
    }
}

/// The file is already sound; hand the original bytes back untouched.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "png_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        // A whole-image decode, not the row-by-row path: interlaced files are perfectly healthy
        // but their rows arrive as Adam7 passes, which the row reader cannot lay out.
        let Some(height) = png_image::decode_whole(ctx, ctx.data, false) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: complete_score(height),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Pixel data is whole but a checksum somewhere does not match. Re-encoding produces a file
/// with correct checksums and the same pixels.
///
/// Only applies when no *critical* chunk was altered: a bad checksum on `IDAT` means the
/// pixels themselves changed, and passing those off as recovered content would be a lie.
struct RewriteWithoutChecksums;

impl Strategy for RewriteWithoutChecksums {
    fn name(&self) -> &str {
        "png_ignore_crc"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if png_chunks::has_altered_content(ctx.data) {
            return Ok(None);
        }
        let decoded = match decode_rows(ctx, ctx.data, true) {
            Some(decoded) if decoded.is_complete() => decoded,
            // Interlaced files cannot be re-laid-out row by row; a healthy one is already
            // handled by the verbatim check above.
            _ => return Ok(None),
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(encode(&decoded, decoded.declared_height)?),
            score: complete_score(decoded.declared_height),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Keep every row that came out of *verified* bytes and drop the rest, producing a shorter
/// but wholly genuine image.
///
/// Decoding is fed only the checksum-verified prefix rather than the whole file. Feeding it
/// everything would produce extra rows: inflate does not fail at the damaged byte, it fails
/// some distance later, and the rows in between are fabricated.
struct TruncateToVerifiedRows;

impl Strategy for TruncateToVerifiedRows {
    fn name(&self) -> &str {
        "png_truncate"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(decoded) = decode_verified(ctx) else {
            return Ok(None);
        };
        if decoded.rows_recovered == 0 || decoded.is_complete() {
            return Ok(None);
        }

        let height = decoded.rows_recovered;
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(encode(&decoded, height)?),
            score: partial_score(height, decoded.declared_height, 0),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Push on past the damage and keep whatever inflate still produces.
///
/// The extra rows are not recovered content, they are what the decoder made of corrupted
/// input, so the candidate is reported as reconstructed and the affected rows are named in
/// the report. Worth having because the alternative is throwing that area away entirely.
struct DecodePastDamage;

impl Strategy for DecodePastDamage {
    fn name(&self) -> &str {
        "png_decode_past_damage"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let Some(decoded) = decode_rows(ctx, ctx.data, true) else {
            return Ok(None);
        };
        if decoded.rows_recovered == 0 || decoded.is_complete() {
            return Ok(None);
        }

        let verified_rows = decode_verified(ctx).map_or(0, |d| d.rows_recovered);
        if decoded.rows_recovered <= verified_rows {
            return Ok(None);
        }

        let height = decoded.rows_recovered;
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(encode(&decoded, height)?),
            score: partial_score(height, decoded.declared_height, height - verified_rows),
            fidelity: Fidelity::Reconstructed,
            reconstructed: std::iter::once(u64::from(verified_rows)..u64::from(height)).collect(),
        }))
    }
}

fn decode_verified(ctx: &RepairContext<'_>) -> Option<PartialDecode> {
    let verified = ctx.data.get(..png_chunks::verified_prefix_len(ctx.data))?;
    decode_rows(ctx, verified, true)
}

fn partial_score(recovered: u32, total: u32, unverified: u32) -> Score {
    Score {
        content_recovered: u64::from(recovered),
        content_total: Some(u64::from(total)),
        decoder_errors: unverified,
        decodes_cleanly: true,
        streams_complete: true,
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use png::{BitDepth, ColorType, Decoder, Encoder};

    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    fn sample_png(width: u32, height: u32) -> Vec<u8> {
        let mut pixels = Vec::with_capacity((width * height * 3) as usize);
        for y in 0..height {
            for x in 0..width {
                pixels.extend_from_slice(&[(x % 256) as u8, (y % 256) as u8, ((x + y) % 256) as u8]);
            }
        }
        encode_rgb(width, height, &pixels)
    }

    /// Incompressible pixels, so the compressed stream is large enough to span many `IDAT` chunks.
    fn noisy_png(width: u32, height: u32) -> Vec<u8> {
        let mut state = 12345u32;
        let pixels: Vec<u8> = (0..width * height * 3)
            .map(|_| {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                (state >> 24) as u8
            })
            .collect();
        encode_rgb(width, height, &pixels)
    }

    fn encode_rgb(width: u32, height: u32, pixels: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut encoder = Encoder::new(Cursor::new(&mut out), width, height);
        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(pixels).unwrap();
        writer.finish().unwrap();
        out
    }

    use super::super::png_chunks::split_idat as with_split_idat;

    fn decode_dimensions(data: &[u8]) -> (u32, u32) {
        let reader = Decoder::new(Cursor::new(data)).read_info().unwrap();
        (reader.info().width, reader.info().height)
    }

    fn decode_pixels(data: &[u8]) -> Vec<u8> {
        let mut reader = Decoder::new(Cursor::new(data)).read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size().expect("png buffer size fits in memory")];
        let info = reader.next_frame(&mut buf).unwrap();
        buf.truncate(info.buffer_size());
        buf
    }

    fn bytes(candidate: &Candidate) -> Vec<u8> {
        candidate.data.as_bytes().expect("image payloads stay in memory").to_vec()
    }

    fn repair(data: &[u8], mode: Mode) -> crate::file_fixer::engine::RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(std::path::Path::new("sample.png"), data, &options)
    }

    /// Found by running the tool over a few hundred real images: the decoder refuses a whole
    /// file over a malformed text chunk, which cannot possibly affect a pixel.
    #[test]
    fn a_malformed_text_chunk_does_not_condemn_the_image() {
        // A tEXt chunk whose keyword has no terminating null, which is what the decoder rejects.
        let original = sample_png(16, 16);
        let mut with_text = Vec::from(&original[..33]);
        png_chunks::append_chunk(&mut with_text, *b"tEXt", b"no-null-separator");
        with_text.extend_from_slice(&original[33..]);

        let result = repair(&with_text, Mode::Balanced);
        // `repair_data` does not apply the healthy short-circuit that `repair_file` does, so a
        // sound file shows up here as a verbatim full recovery.
        assert_eq!(result.report.winning_strategy.as_deref(), Some("png_verbatim"));
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
    }

    /// Found by running the tool over a few hundred real images: interlaced files are perfectly
    /// healthy, but the row-by-row reader hands back Adam7 passes rather than image rows, so the
    /// health check used to reject every one of them.
    #[test]
    fn an_interlaced_file_is_recognized_as_healthy() {
        let Ok(original) = std::fs::read("tmp/Rome_interlaced.png") else {
            return;
        };
        let mut reader = Decoder::new(Cursor::new(&original)).read_info().unwrap();
        assert!(reader.info().interlaced, "the fixture must be interlaced");
        let mut buffer = vec![0; reader.output_buffer_size().expect("png buffer size fits in memory")];
        reader.next_frame(&mut buffer).unwrap();

        let result = repair(&original, Mode::Balanced);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("png_verbatim"));
        assert_eq!(bytes(&result.winner.unwrap()), original);
    }

    #[test]
    fn a_healthy_file_is_returned_untouched() {
        let original = sample_png(64, 48);
        let result = repair(&original, Mode::Balanced);

        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("png_verbatim"));
        assert_eq!(bytes(&result.winner.unwrap()), original);
    }

    /// An `acTL` chunk means `IDAT` is only the poster frame; reporting the file fully
    /// repaired from that alone would silently drop every animation frame in the `fdAT` chunks.
    #[test]
    fn an_animated_png_is_declined_rather_than_repaired_from_the_poster_frame() {
        let mut data = sample_png(16, 16);
        assert!(!contains_actl(&data), "the plain sample must not already look animated");
        data.extend_from_slice(b"acTL");

        assert!(matches!(PngRepairer.supports(&data), Support::Unsupported(_)));
        assert_eq!(repair(&data, Mode::Aggressive).report.outcome, Outcome::Unsupported);
    }

    #[test]
    fn a_truncated_file_keeps_every_row_that_decoded() {
        let original = sample_png(64, 200);
        let spec = DamageSpec::new(
            DamageKind::TruncateTail {
                keep: Fraction::new(0.5).unwrap(),
            },
            1,
        );
        let damaged = apply_damage(&original, &spec);

        let result = repair(&damaged.data, Mode::Balanced);
        let winner = result.winner.expect("some rows should survive");

        assert_eq!(result.report.outcome, Outcome::PartiallyRecovered);
        let (width, height) = decode_dimensions(&bytes(&winner));
        assert_eq!(width, 64);
        assert!(height > 0 && height < 200, "recovered {height} rows");
        assert_eq!(u64::from(height), result.report.content_recovered.unwrap());
    }

    /// The point of the lossless modes: whatever survives must be the original data, not an
    /// approximation of it.
    #[test]
    fn recovered_rows_are_bit_identical_to_the_original() {
        let original = sample_png(64, 200);
        let spec = DamageSpec::new(
            DamageKind::TruncateTail {
                keep: Fraction::new(0.6).unwrap(),
            },
            1,
        );
        let damaged = apply_damage(&original, &spec);

        let winner = repair(&damaged.data, Mode::Balanced).winner.unwrap();
        let (_, recovered_height) = decode_dimensions(&bytes(&winner));

        let original_pixels = decode_pixels(&original);
        let recovered_pixels = decode_pixels(&bytes(&winner));
        let compared = recovered_pixels.len();

        assert!(compared > 0);
        assert_eq!(recovered_pixels, original_pixels[..compared]);
        assert_eq!(compared, (64 * 3 * recovered_height) as usize);
    }

    #[test]
    fn safe_mode_refuses_to_shorten_an_image() {
        let original = sample_png(64, 200);
        let spec = DamageSpec::new(
            DamageKind::TruncateTail {
                keep: Fraction::new(0.5).unwrap(),
            },
            1,
        );
        let damaged = apply_damage(&original, &spec);

        let result = repair(&damaged.data, Mode::Safe);
        assert_eq!(result.report.outcome, Outcome::Unrecoverable);
        assert!(result.winner.is_none());
    }

    #[test]
    fn a_broken_checksum_alone_is_repaired_without_losing_rows() {
        let mut data = sample_png(32, 32);
        // The final four bytes of the IEND chunk are its CRC; corrupting them leaves every
        // pixel intact but makes a strict decoder reject the file.
        let len = data.len();
        if let Some(crc) = data.get_mut(len - 4..) {
            crc.copy_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
        }

        let result = repair(&data, Mode::Balanced);
        let winner = result.winner.expect("content is intact");
        assert_eq!(decode_dimensions(&bytes(&winner)), (32, 32));
        assert_eq!(decode_pixels(&bytes(&winner)), decode_pixels(&sample_png(32, 32)));
    }

    /// The regression that motivated chunk verification: inflate keeps producing rows for a
    /// while after corrupted input, so a naive partial decode hands back fabricated rows and
    /// calls them recovered. In a lossless mode every row that comes out must be original.
    #[test]
    fn damage_in_the_middle_never_yields_fabricated_rows() {
        let original = with_split_idat(&noisy_png(64, 200), 2048);
        let spec = DamageSpec::new(
            DamageKind::ZeroRange {
                at: Fraction::HALF,
                len: ByteSpan::Bytes(512),
            },
            11,
        );
        let damaged = apply_damage(&original, &spec);

        let result = repair(&damaged.data, Mode::Balanced);
        let winner = result.winner.expect("the first half is recoverable");
        assert!(winner.is_lossless());

        let recovered_pixels = decode_pixels(&bytes(&winner));
        let original_pixels = decode_pixels(&original);
        let compared = recovered_pixels.len();

        assert!(compared > 0);
        assert_eq!(
            recovered_pixels,
            original_pixels[..compared],
            "a lossless candidate must not contain a single fabricated byte"
        );
    }

    /// The same file in aggressive mode may keep going past the damage, but then it has to
    /// say so rather than passing the extra rows off as recovered content.
    #[test]
    fn aggressive_mode_may_go_past_damage_but_must_declare_it() {
        let original = with_split_idat(&noisy_png(64, 200), 2048);
        let spec = DamageSpec::new(
            DamageKind::ZeroRange {
                at: Fraction::HALF,
                len: ByteSpan::Bytes(512),
            },
            11,
        );
        let damaged = apply_damage(&original, &spec);

        let lossless_rows = repair(&damaged.data, Mode::Balanced).winner.map_or(0, |w| w.score.content_recovered);
        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.expect("something is recoverable");

        if winner.score.content_recovered > lossless_rows {
            assert!(!winner.is_lossless());
            assert!(!winner.reconstructed.is_empty());
            assert_eq!(winner.reconstructed[0].start, lossless_rows);
        }
    }

    /// Checksums are per chunk, so a file whose pixel data lives in one giant `IDAT` offers
    /// no partially trustworthy state: any alteration inside it invalidates every row. The
    /// lossless modes then correctly recover nothing, and only aggressive mode salvages
    /// anything - clearly labelled as reconstructed.
    #[test]
    fn a_single_idat_chunk_leaves_the_lossless_path_nothing_to_trust() {
        let original = sample_png(64, 200);
        assert_eq!(
            png_chunks::walk(&original).iter().filter(|c| c.is(*b"IDAT")).count(),
            1,
            "fixture must have a single IDAT for this test to mean anything"
        );

        let spec = DamageSpec::new(
            DamageKind::ZeroRange {
                at: Fraction::HALF,
                len: ByteSpan::Bytes(512),
            },
            11,
        );
        let damaged = apply_damage(&original, &spec);

        assert!(repair(&damaged.data, Mode::Balanced).winner.is_none());

        let salvaged = repair(&damaged.data, Mode::Aggressive).winner;
        if let Some(candidate) = salvaged {
            assert!(!candidate.is_lossless());
            assert!(!candidate.reconstructed.is_empty());
        }
    }

    #[test]
    fn a_destroyed_header_yields_nothing_rather_than_garbage() {
        let original = sample_png(32, 32);
        let spec = DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(40) }, 5);
        let damaged = apply_damage(&original, &spec);

        let result = repair(&damaged.data, Mode::Balanced);
        assert!(result.winner.is_none());
    }
}
