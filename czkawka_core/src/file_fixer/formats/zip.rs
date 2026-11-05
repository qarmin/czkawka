//! ZIP repair.
//!
//! An archive says everything twice. Each member carries a local header giving its name, its
//! compression method, its sizes and a CRC-32 of its contents; the central directory at the
//! end repeats all of it. Nothing lives only in the directory - so the single most common
//! complaint about a "corrupted archive", where the tail of the file is gone and no tool will
//! open it any more, is repaired by walking the members from the front and writing a new
//! directory for the ones that are there.
//!
//! The per-member checksum is what makes this honest. A recovered member is not merely present,
//! it decompresses to exactly the bytes whose CRC the header recorded - the same standard
//! FLAC's per-frame checksum sets. Members that cannot be checked (an encrypted one, a
//! compression method this tool does not implement, one too large to decompress inside the
//! allocation limit) are kept and counted, but the report does not claim them as verified.
//!
//! The one thing here that is not byte-faithful is the last member of a truncated archive: its
//! data stops part way, and what decompresses is a genuine prefix of the original file but not
//! the whole of it. It is written out under a `.partial` suffix so nothing mistakes it for the
//! real thing, and only aggressive mode produces it at all.

use std::borrow::Cow;

use super::zip_entries::{self, Entry, Rebuilt, Verdict};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct ZipRepairer;

impl Repairer for ZipRepairer {
    fn format(&self) -> Format {
        Format::Zip
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![Box::new(Verbatim), Box::new(RebuildDirectory), Box::new(DropBrokenMembers), Box::new(SalvagePartialMember)]
    }
}

/// Largest member this will decompress just to check it. Bigger ones are kept unverified
/// rather than making a repair take minutes on an archive full of video files.
const MAX_VERIFIED_MEMBER: u64 = 64 << 20;

struct Member {
    entry: Entry,
    verdict: Verdict,
}

/// What walking the archive found.
struct Survey {
    members: Vec<Member>,
    /// The record that closes an undamaged archive, when it is still there.
    end: Option<zip_entries::EndOfDirectory>,
}

impl Survey {
    fn of(ctx: &RepairContext<'_>) -> Self {
        let entries = zip_entries::scan_entries(ctx.data, || ctx.should_stop());
        let limit = ctx.allocation_limit.min(MAX_VERIFIED_MEMBER);

        let members = entries
            .into_iter()
            .map(|entry| {
                // A search that ran out of time must not report unchecked members as damaged:
                // dropping a member because the clock stopped would lose real content.
                let verdict = if ctx.should_stop() {
                    Verdict::Unchecked
                } else {
                    zip_entries::verify(ctx.data, &entry, limit)
                };
                Member { entry, verdict }
            })
            .collect();

        Self {
            members,
            end: zip_entries::find_end_of_directory(ctx.data),
        }
    }

    /// Members worth carrying into a repaired archive: their data is present, and it either
    /// matched its checksum or could not be checked at all.
    fn usable(&self) -> impl Iterator<Item = &Member> {
        self.members.iter().filter(|member| matches!(member.verdict, Verdict::Intact | Verdict::Unchecked))
    }

    fn usable_count(&self) -> usize {
        self.usable().count()
    }

    /// True when every member walked is present and undamaged.
    fn all_usable(&self) -> bool {
        !self.members.is_empty() && self.usable_count() == self.members.len()
    }

    /// How many members the archive originally held, when anything still says.
    ///
    /// The end record is the direct answer. When it is gone, the central directory it pointed
    /// at often is not - damage takes the tail of a file, and the directory entries that
    /// survive still name members that used to be there. Counting them recovers the
    /// denominator, which is the difference between reporting "80% recovered" and reporting a
    /// truncated archive as fully repaired.
    fn declared(&self, data: &[u8]) -> Option<u64> {
        if let Some(end) = self.end {
            return Some(u64::from(end.entries));
        }
        let after_members = self.members.last().map_or(0, |member| member.entry.end);
        let surviving = data
            .get(after_members..)
            .map_or(0, |tail| tail.windows(4).filter(|window| *window == zip_entries::CENTRAL_HEADER).count());
        (surviving > 0).then_some(surviving as u64)
    }

    /// True when the archive closes the way an undamaged one does: an end record at the very
    /// end of the file, pointing at a directory that is really there and counting exactly the
    /// members that were walked.
    fn is_closed(&self, data: &[u8]) -> bool {
        let Some(end) = self.end else {
            return false;
        };
        let directory = end.directory_offset as usize;
        data.get(directory..directory + 4) == Some(&zip_entries::CENTRAL_HEADER)
            && directory < end.start
            && usize::from(end.entries) == self.members.len()
            && end.start + 22 <= data.len()
    }
}

/// A member as it will appear in a rebuilt archive.
struct Output<'a> {
    entry: Entry,
    body: Cow<'a, [u8]>,
}

fn outputs<'a>(data: &'a [u8], survey: &Survey) -> Vec<Output<'a>> {
    survey
        .usable()
        .map(|member| Output {
            entry: member.entry.clone(),
            body: Cow::Borrowed(data.get(member.entry.data.clone()).unwrap_or_default()),
        })
        .collect()
}

/// Writes a fresh archive: every member's header, its data, then a directory describing them.
fn rebuild(members: &[Output<'_>]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut directory = Vec::with_capacity(members.len());

    for member in members {
        let offset = out.len() as u64;
        zip_entries::write_local_header(&mut out, &member.entry);
        out.extend_from_slice(&member.body);
        directory.push(Rebuilt {
            name: member.entry.name.clone(),
            method: member.entry.method,
            modified: member.entry.modified,
            crc32: member.entry.crc32,
            compressed_size: member.entry.compressed_size,
            uncompressed_size: member.entry.uncompressed_size,
            offset,
        });
    }

    zip_entries::write_directory(&mut out, &directory);
    out
}

/// `total` is deliberately allowed to stay unknown: an archive whose end record and directory
/// are both gone gives no way to tell how many members it once held, and filling that in with
/// the number recovered would turn "some of it came back" into "fully repaired".
fn score_of(members: usize, total: Option<u64>) -> Score {
    Score {
        content_recovered: members as u64,
        content_total: total,
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: total.is_some_and(|total| total <= members as u64),
    }
}

/// The archive closes properly and every member checks out.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "zip_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let survey = Survey::of(ctx);
        if !survey.all_usable() || !survey.is_closed(ctx.data) {
            return Ok(None);
        }
        let count = survey.members.len();

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: score_of(count, Some(count as u64)),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Every member is there; the directory that indexes them is not, or does not match.
///
/// This is the repair for the classic "the archive is corrupt but the files inside are fine"
/// case - a download that lost its last kilobytes, an archive whose writer crashed before
/// closing it, one with junk appended. Nothing is discarded and every member still matches its
/// own checksum, so it needs no capability at all and works in safe mode.
struct RebuildDirectory;

impl Strategy for RebuildDirectory {
    fn name(&self) -> &str {
        "zip_rebuild_directory"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let survey = Survey::of(ctx);
        if !survey.all_usable() || survey.is_closed(ctx.data) {
            return Ok(None);
        }
        let count = survey.members.len();

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(rebuild(&outputs(ctx.data, &survey))),
            score: score_of(count, survey.declared(ctx.data).map(|declared| declared.max(count as u64))),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Some members are cut short or fail their checksum. Keep the ones that do not.
struct DropBrokenMembers;

impl Strategy for DropBrokenMembers {
    fn name(&self) -> &str {
        "zip_drop_broken_members"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let survey = Survey::of(ctx);
        if survey.all_usable() {
            return Ok(None);
        }
        let kept = survey.usable_count();
        if kept == 0 {
            return Ok(None);
        }
        let total = survey.declared(ctx.data).map(|declared| declared.max(kept as u64));

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(rebuild(&outputs(ctx.data, &survey))),
            score: score_of(kept, total),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Also keep what decompressed of the member the archive stops inside.
///
/// The bytes recovered are a genuine prefix of that file - deflate produces its output in
/// order, and every byte it emitted came from compressed data that arrived intact. What is not
/// genuine is the file *as a whole*: it is incomplete, so it is stored under a `.partial`
/// suffix rather than under a name that promises the original.
struct SalvagePartialMember;

impl Strategy for SalvagePartialMember {
    fn name(&self) -> &str {
        "zip_salvage_partial_member"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let survey = Survey::of(ctx);
        let Some(broken) = survey.members.iter().find(|member| member.verdict == Verdict::Incomplete) else {
            return Ok(None);
        };
        let Some(payload) = ctx.data.get(broken.entry.data.clone()) else {
            return Ok(None);
        };

        let recovered = match broken.entry.method {
            zip_entries::METHOD_STORED => payload.to_vec(),
            zip_entries::METHOD_DEFLATE => zip_entries::inflate(payload, ctx.allocation_limit).0,
            // An encrypted or otherwise unreadable member yields nothing partial; its bytes
            // mean nothing without the decoder that was going to read them.
            _ => return Ok(None),
        };
        if recovered.is_empty() {
            return Ok(None);
        }

        let kept = survey.usable_count();
        let mut members = outputs(ctx.data, &survey);
        let mut name = broken.entry.name.clone();
        name.extend_from_slice(b".partial");

        members.push(Output {
            entry: Entry {
                name,
                flags: 0,
                method: zip_entries::METHOD_STORED,
                crc32: crc32fast::hash(&recovered),
                compressed_size: recovered.len() as u64,
                uncompressed_size: recovered.len() as u64,
                ..broken.entry.clone()
            },
            body: Cow::Owned(recovered),
        });

        let total = survey.declared(ctx.data).map(|declared| declared.max(kept as u64 + 1));
        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(rebuild(&members)),
            score: score_of(kept + 1, total),
            fidelity: Fidelity::Reconstructed,
            reconstructed: std::iter::once(kept as u64..kept as u64 + 1).collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::zip_entries::sample_zip;
    use super::*;
    use crate::file_fixer::corrupt::{DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    fn repair(data: &[u8], mode: Mode) -> RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(Path::new("archive.zip"), data, &options)
    }

    fn bytes(result: &RepairResult) -> Vec<u8> {
        result
            .winner
            .as_ref()
            .and_then(|winner| winner.data.as_bytes())
            .expect("archive payloads stay in memory")
            .to_vec()
    }

    /// Reads a repaired archive back the way any other tool would: walk the members, check each
    /// against its own checksum, and return what they hold.
    fn contents(data: &[u8]) -> Vec<(String, Vec<u8>)> {
        zip_entries::scan_entries(data, || false)
            .into_iter()
            .map(|entry| {
                assert_eq!(
                    zip_entries::verify(data, &entry, 1 << 24),
                    Verdict::Intact,
                    "{} must match its checksum",
                    entry.name_lossy()
                );
                let payload = data.get(entry.data.clone()).unwrap_or_default();
                let body = match entry.method {
                    zip_entries::METHOD_DEFLATE => zip_entries::inflate(payload, 1 << 24).0,
                    _ => payload.to_vec(),
                };
                (entry.name_lossy(), body)
            })
            .collect()
    }

    fn deflated(contents: &[u8]) -> Vec<u8> {
        miniz_oxide::deflate::compress_to_vec(contents, 6)
    }

    #[test]
    fn a_healthy_archive_is_left_alone() {
        let original = sample_zip(&[("a.txt", b"hello"), ("b.txt", b"world")]);
        let result = repair(&original, Mode::Balanced);

        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("zip_verbatim"));
        assert_eq!(result.report.content_recovered, Some(2));
        assert_eq!(bytes(&result), original);
    }

    /// The single most common real complaint about a ZIP: the tail is gone, every file inside
    /// is fine, and no tool will open it. Nothing is discarded, so safe mode repairs it - and
    /// the directory entries that survived say how many members there were supposed to be.
    #[test]
    fn an_archive_whose_end_record_is_gone_is_rebuilt_in_safe_mode() {
        let original = sample_zip(&[("a.txt", b"hello"), ("b.txt", b"world")]);
        let end = zip_entries::find_end_of_directory(&original).expect("end record");
        let damaged = original.get(..end.start).expect("everything but the end record").to_vec();

        let result = repair(&damaged, Mode::Safe);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("zip_rebuild_directory"));
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.content_total, Some(2), "the surviving directory says two");

        let fixed = bytes(&result);
        assert_eq!(contents(&fixed), vec![("a.txt".to_string(), b"hello".to_vec()), ("b.txt".to_string(), b"world".to_vec())]);
        assert!(Survey::of_data(&fixed).is_closed(&fixed), "the result closes properly");
    }

    /// With the directory gone as well, nothing in the file says how many members it held.
    /// Every member present still comes back, and the report says "some" rather than "all",
    /// because a member that was entirely beyond the cut leaves no trace to count.
    #[test]
    fn an_archive_with_no_directory_at_all_is_rebuilt_but_not_claimed_complete() {
        let original = sample_zip(&[("a.txt", b"hello"), ("b.txt", b"world")]);
        let directory = zip_entries::find_end_of_directory(&original).expect("end record").directory_offset as usize;
        let damaged = original.get(..directory).expect("the members alone").to_vec();

        let result = repair(&damaged, Mode::Safe);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("zip_rebuild_directory"));
        assert_eq!(result.report.outcome, Outcome::PartiallyRecovered);
        assert_eq!(result.report.content_total, None, "nothing left to count against");
        assert_eq!(
            contents(&bytes(&result)),
            vec![("a.txt".to_string(), b"hello".to_vec()), ("b.txt".to_string(), b"world".to_vec())]
        );
    }

    #[test]
    fn a_member_cut_in_half_is_dropped_and_the_rest_kept() {
        let big: Vec<u8> = (0..20_000u32).map(|index| (index % 251) as u8).collect();
        let original = sample_zip(&[("small.txt", b"kept"), ("big.bin", &big)]);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.5).unwrap(),
                },
                1,
            ),
        );

        let result = repair(&damaged.data, Mode::Balanced);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("zip_drop_broken_members"));
        assert_eq!(result.report.outcome, Outcome::PartiallyRecovered);

        let fixed = bytes(&result);
        assert_eq!(contents(&fixed), vec![("small.txt".to_string(), b"kept".to_vec())]);
    }

    /// A member whose contents were altered fails its checksum, and must not be handed back as
    /// recovered content.
    #[test]
    fn altered_contents_are_dropped_rather_than_passed_off_as_recovered() {
        let mut data = sample_zip(&[("a.txt", b"the quick brown fox"), ("b.txt", b"jumps over")]);
        let first = zip_entries::scan_entries(&data, || false).first().cloned().expect("one member");
        if let Some(byte) = data.get_mut(first.data.start) {
            *byte ^= 0xFF;
        }

        let result = repair(&data, Mode::Balanced);
        let fixed = bytes(&result);
        assert_eq!(contents(&fixed), vec![("b.txt".to_string(), b"jumps over".to_vec())]);
        assert_eq!(result.report.content_recovered, Some(1));
    }

    /// The tail of a truncated archive is usually its largest file, and most of it is there.
    /// What decompresses is real content; the file as a whole is not, so it is renamed.
    #[test]
    fn most_of_a_half_written_member_is_salvaged_under_a_partial_name() {
        let big: Vec<u8> = (0..200_000u32).map(|index| (index % 251) as u8).collect();
        let compressed = deflated(&big);
        let mut data = Vec::new();
        let entry = Entry {
            start: 0,
            name: b"video.bin".to_vec(),
            flags: 0,
            method: zip_entries::METHOD_DEFLATE,
            modified: (0, 0),
            crc32: crc32fast::hash(&big),
            compressed_size: compressed.len() as u64,
            uncompressed_size: big.len() as u64,
            data: 0..0,
            complete: true,
            end: 0,
        };
        zip_entries::write_local_header(&mut data, &entry);
        data.extend_from_slice(&compressed);
        data.truncate(data.len() - compressed.len() / 3);

        assert!(repair(&data, Mode::Balanced).winner.is_none());

        let result = repair(&data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("most of it decompressed");
        assert_eq!(winner.strategy, "zip_salvage_partial_member");
        assert!(!winner.is_lossless(), "an incomplete file is not the original file");

        let recovered = contents(&bytes(&result));
        let (name, body) = recovered.first().expect("one member");
        assert_eq!(name, "video.bin.partial");
        assert!(!body.is_empty() && body.len() < big.len());
        assert_eq!(
            *body,
            big.get(..body.len()).expect("a prefix of the original").to_vec(),
            "every salvaged byte must be original content"
        );
    }

    /// Junk in front of the first member - a self-extracting stub, or a damaged first sector -
    /// is stepped over by scanning for the local headers.
    #[test]
    fn members_are_found_even_when_the_file_does_not_start_with_one() {
        let original = sample_zip(&[("a.txt", b"hello")]);
        let mut damaged = vec![0x7Fu8; 2048];
        damaged.extend_from_slice(&original);

        let result = repair(&damaged, Mode::Balanced);
        assert!(result.winner.is_some());
        assert_eq!(contents(&bytes(&result)), vec![("a.txt".to_string(), b"hello".to_vec())]);
    }

    #[test]
    fn safe_mode_will_not_drop_a_broken_member() {
        let big: Vec<u8> = (0..20_000u32).map(|index| (index % 251) as u8).collect();
        let original = sample_zip(&[("small.txt", b"kept"), ("big.bin", &big)]);
        let damaged = apply_damage(
            &original,
            &DamageSpec::new(
                DamageKind::TruncateTail {
                    keep: Fraction::new(0.5).unwrap(),
                },
                1,
            ),
        );

        let result = repair(&damaged.data, Mode::Safe);
        assert!(result.winner.is_none());
        assert!(result.report.declined_capabilities.contains(&"truncate".to_string()));
    }

    #[test]
    fn recovery_grows_with_the_amount_of_surviving_data() {
        let members: Vec<(String, Vec<u8>)> = (0..8).map(|index| (format!("file{index}.bin"), vec![index as u8; 2000])).collect();
        let borrowed: Vec<(&str, &[u8])> = members.iter().map(|(name, body)| (name.as_str(), body.as_slice())).collect();
        let original = sample_zip(&borrowed);

        let mut previous = 0;
        for keep in [0.3, 0.5, 0.7, 0.9] {
            let damaged = apply_damage(
                &original,
                &DamageSpec::new(
                    DamageKind::TruncateTail {
                        keep: Fraction::new(keep).unwrap(),
                    },
                    1,
                ),
            );
            let recovered = repair(&damaged.data, Mode::Balanced).report.content_recovered.unwrap_or(0);
            assert!(recovered >= previous, "keep {keep} gave {recovered} after {previous}");
            previous = recovered;
        }
    }

    #[test]
    fn short_and_hostile_inputs_never_panic() {
        let original = sample_zip(&[("a.txt", b"hello")]);
        for len in 0..original.len() {
            let mut data = original.clone();
            data.truncate(len);
            let _ = repair(&data, Mode::Aggressive);
        }
    }

    impl Survey {
        /// Surveying outside a repair context, for assertions about a finished result.
        fn of_data(data: &[u8]) -> Self {
            Self {
                members: zip_entries::scan_entries(data, || false)
                    .into_iter()
                    .map(|entry| {
                        let verdict = zip_entries::verify(data, &entry, 1 << 24);
                        Member { entry, verdict }
                    })
                    .collect(),
                end: zip_entries::find_end_of_directory(data),
            }
        }
    }
}
