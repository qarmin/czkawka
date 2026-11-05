//! Rebuilding a playable file for input whose index is gone entirely.
//!
//! This is the case the whole project exists for. `moov` holds every sample offset *and* the
//! decoder parameters, and encoders write it at the end of the file by default, so a truncated
//! recording keeps all of its media data and loses every description of it. ffmpeg cannot open
//! such a file at all: `ffprobe` reports "moov atom not found".
//!
//! Recovery works by scanning the media data for video units (see [`super::mp4_scan`]), writing
//! them out as an elementary stream, and letting ffmpeg build a container around it. Audio is
//! not recovered: AAC in an MP4 is stored raw with no sync word, so its chunks can be stepped
//! over but not identified.

use super::ffmpeg::DecodeReport;
use super::mp4::{candidate_from, reconstructed_span, temp_output};
use super::mp4_profile::Profile;
use super::{ffmpeg, mp4_boxes, mp4_profile, mp4_scan};
use crate::file_fixer::candidate::{Candidate, Fidelity};
use crate::file_fixer::engine::{RepairContext, Strategy};
use crate::file_fixer::error::{FixerError, Result};
use crate::file_fixer::mode::Capabilities;

/// Rebuild a playable file for input whose index is gone entirely.
///
/// This is the case the whole project exists for: `moov` holds every sample offset and the
/// decoder parameters, and encoders write it at the end by default, so a truncated recording
/// keeps its media data and loses all description of it. ffmpeg cannot open such a file at
/// all - `ffprobe` reports no streams.
///
/// The media data is scanned for video units, which are then written out as an elementary
/// stream and muxed. Audio is not recovered: AAC in an MP4 has no sync word, so the
/// interleaved audio chunks can only be stepped over, not identified.
///
/// Needs the `guess` capability, and the result is always reported as reconstructed: the
/// timing comes from the reference's frame rate rather than from the file, and the scan cannot
/// prove that every unit it accepted was real.
pub struct RebuildIndex;

impl Strategy for RebuildIndex {
    fn name(&self) -> &str {
        "mp4_rebuild_index"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if !ffmpeg::is_available() {
            return Err(FixerError::FfmpegMissing);
        }
        // Found by signature when needed: damage to the leading bytes breaks the box chain
        // but leaves the media data itself perfectly readable.
        let Some(mdat) = mp4_boxes::find_media_data(ctx.data) else {
            return Ok(None);
        };

        let mut best: Option<(tempfile::NamedTempFile, DecodeReport)> = None;
        for profile in most_promising(ctx, &mdat, profiles_for(ctx, &mdat))? {
            if ctx.should_stop() {
                break;
            }
            let Some((output, report)) = build(ctx, &mdat, &profile, None)? else {
                continue;
            };
            if best.as_ref().is_none_or(|(_, current)| fit(&report) > fit(current)) {
                best = Some((output, report));
            }
        }

        Ok(best.and_then(|(output, report)| {
            let mut candidate = candidate_from(self.name(), ctx, output, &report, None, true)?;
            // The scan cannot prove the units it accepted were real, so this never claims to
            // be faithful even when the decoder happens not to complain.
            candidate.fidelity = Fidelity::Reconstructed;
            candidate.reconstructed = reconstructed_span(Fidelity::Reconstructed, report.frames);
            Some(candidate)
        }))
    }
}

/// Every encoder configuration worth trying against this file.
///
/// The buried-index source is what makes a carved file recoverable with no reference at all,
/// which is the common case for recovery-tool output: the `moov` is inside the oversized
/// `mdat`, so nothing at the top level names it, but `avcC` is right there and describes this
/// camera's encoder settings. What is left after that is the half of a carved collection with
/// no index anywhere, and for those the batch itself is the reference library.
fn profiles_for(ctx: &RepairContext<'_>, mdat: &mp4_boxes::BoxHeader) -> Vec<Profile> {
    let mut profiles = Vec::new();
    mp4_profile::merge(&mut profiles, mp4_profile::from_in_band(ctx.data, mdat));
    mp4_profile::merge(&mut profiles, mp4_profile::from_buried(ctx.data));
    for reference in ctx.references {
        match std::fs::read(reference) {
            Ok(data) => mp4_profile::merge(&mut profiles, mp4_profile::from_reference(&data)),
            Err(error) => log::warn!("cannot read reference {}: {error}", reference.display()),
        }
    }
    mp4_profile::merge(&mut profiles, mp4_profile::learned());
    profiles
}

/// How much media data to scan when ranking profiles against each other.
///
/// A wrong profile is rejected by the scanner within the first few frames - its NAL ceiling or
/// its parameter set identifiers do not match - so a prefix separates the candidates just as
/// well as the whole file and costs a fraction as much.
const RANKING_PREFIX: usize = 4 << 20;

/// How many scan-ranked profiles get a decode probe.
const PROFILES_TO_PROBE: usize = 12;

/// How many of the probed profiles get a full rebuild.
const PROFILES_TO_BUILD: usize = 2;

/// Frames to decode when probing, which only has to answer "does this configuration fit".
const PROBE_UNITS: usize = 8;

/// Frames the decoder produced without complaining about them.
///
/// Frame count alone picks the wrong profile: a configuration whose resolution does not match
/// accepts the slices anyway and emits a full set of garbage pictures, one complaint each, so
/// it can easily out-count the configuration that is actually right.
fn fit(report: &DecodeReport) -> (u64, i64) {
    (report.frames.saturating_sub(u64::from(report.errors)), -i64::from(report.errors))
}

/// Narrows dozens of candidate configurations down to the few worth rebuilding with.
///
/// Two stages, cheapest first. Scanning is pure arithmetic and rejects anything whose NAL
/// ceiling or parameter set identifiers are wrong, but it cannot tell a right resolution from
/// a wrong one - both scan identically. Decoding eight frames can, and at that length it costs
/// little more than starting ffmpeg.
fn most_promising(ctx: &RepairContext<'_>, mdat: &mp4_boxes::BoxHeader, profiles: Vec<Profile>) -> Result<Vec<Profile>> {
    if profiles.len() <= 1 {
        return Ok(profiles);
    }
    let prefix = mdat.payload.start..mdat.payload.end.min(mdat.payload.start + RANKING_PREFIX);

    let mut scanned: Vec<(usize, Profile)> = profiles
        .into_iter()
        .map(|profile| (mp4_scan::scan(ctx.data, prefix.clone(), &profile).slices(), profile))
        .filter(|(slices, _)| *slices > 0)
        .collect();
    scanned.sort_by_key(|(slices, _)| std::cmp::Reverse(*slices));
    scanned.truncate(PROFILES_TO_PROBE);

    let mut probed: Vec<((u64, i64), Profile)> = Vec::with_capacity(scanned.len());
    for (_, profile) in scanned {
        if ctx.should_stop() {
            break;
        }
        if let Some((_, report)) = build(ctx, mdat, &profile, Some(PROBE_UNITS))? {
            probed.push((fit(&report), profile));
        }
    }

    probed.sort_by_key(|(fit, _)| std::cmp::Reverse(*fit));
    probed.truncate(PROFILES_TO_BUILD);
    Ok(probed.into_iter().map(|(_, profile)| profile).collect())
}

/// Scans, writes an elementary stream and lets ffmpeg build a container around it.
///
/// `max_units` caps the stream at the opening frames, which is what turns a rebuild into a
/// probe: the same code path, a hundredth of the work.
fn build(ctx: &RepairContext<'_>, mdat: &mp4_boxes::BoxHeader, profile: &Profile, max_units: Option<usize>) -> Result<Option<(tempfile::NamedTempFile, DecodeReport)>> {
    let region = match max_units {
        Some(_) => mdat.payload.start..mdat.payload.end.min(mdat.payload.start + RANKING_PREFIX),
        None => mdat.payload.clone(),
    };
    let mut report = mp4_scan::scan(ctx.data, region, profile);
    if report.slices() == 0 {
        return Ok(None);
    }
    if let Some(max) = max_units {
        report.units.truncate(max);
    }

    let elementary = tempfile::Builder::new()
        .suffix(&format!(".{}", profile.codec.extension()))
        .tempfile()
        .map_err(|error| FixerError::write("<temp>", error))?;
    let mut writer = std::io::BufWriter::new(&elementary);
    mp4_scan::write_annex_b(ctx.data, &report, profile, &mut writer).map_err(|error| FixerError::write("<temp>", error))?;
    drop(writer);

    let output = temp_output()?;
    // A refused stream is this *profile* failing, not the repair failing. Propagating it threw
    // away every remaining candidate: on a real carved collection one harvested configuration
    // carried a corrupt parameter set, ffmpeg said "sps_id 32 out of range", and a 1.2 GB file
    // that another configuration recovers 2,000 frames from was reported unrecoverable.
    if let Err(error) = ffmpeg::mux_elementary_stream(elementary.path(), output.path(), profile.codec.demuxer(), profile.frame_rate) {
        log::debug!("{}: a configuration produced a stream ffmpeg refused: {error}", ctx.path.display());
        return Ok(None);
    }

    let decoded = ffmpeg::decode_report(output.path())?;
    if decoded.frames == 0 {
        return Ok(None);
    }
    Ok(Some((output, decoded)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<u8> {
        std::fs::read("tmp/a.mp4").unwrap_or_default()
    }

    #[test]
    fn a_file_with_no_media_data_offers_no_profile() {
        let data = [0u8; 64];
        let mdat = mp4_boxes::BoxHeader {
            kind: *b"mdat",
            start: 0,
            payload: 0..data.len(),
            declared_end: data.len() as u64,
            status: mp4_boxes::BoxStatus::Complete,
        };
        assert!(mp4_profile::from_in_band(&data, &mdat).is_none());
    }

    /// Without a reference and without in-band parameter sets there is nothing to learn from,
    /// so no profile is available and the strategy has nothing to try.
    #[test]
    fn no_reference_and_no_in_band_sets_means_no_profiles() {
        let data = sample();
        if data.is_empty() {
            return;
        }
        let cut = data.len() * 60 / 100;
        let broken = &data[..cut];
        let mdat = mp4_boxes::top_level(broken).into_iter().find(|b| b.is(*b"mdat")).expect("media data survives truncation");

        assert!(mp4_profile::from_in_band(broken, &mdat).is_none());
    }
}
