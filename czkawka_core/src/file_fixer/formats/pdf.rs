//! PDF repair.
//!
//! The cross-reference table is where PDFs break. It is written last, it lives at the end of
//! the file, and it is a list of byte offsets - so anything that shortens the file, appends to
//! it, or rewrites part of it in place leaves a document whose objects are all present and
//! whose table points at the wrong places. Readers then refuse the whole file.
//!
//! Objects say who they are (`12 0 obj`), so the table is not needed to find them: it can be
//! written again from what the file itself contains. Nothing is re-encoded and no content is
//! dropped, which is why rebuilding the table is allowed even in safe mode - the same standing
//! ZIP's central directory has, for the same reason.
//!
//! Two details make the difference between a rebuild that works and one that produces an empty
//! document. Objects may be packed inside another object's compressed stream, invisible to any
//! scan, and those are decompressed and written back out as ordinary objects. And the trailer
//! has to name the catalog: a surviving one is used, and failing that the object that declares
//! itself `/Type /Catalog` is found by looking.
//!
//! See [`super::pdf_objects`] for the scanning and the table writer.

use super::pdf_objects::{self, Object, XrefEntry};
use crate::file_fixer::candidate::{Candidate, Fidelity, Payload, Score};
use crate::file_fixer::engine::{RepairContext, Repairer, Strategy};
use crate::file_fixer::error::Result;
use crate::file_fixer::format::Format;
use crate::file_fixer::mode::Capabilities;

pub struct PdfRepairer;

impl Repairer for PdfRepairer {
    fn format(&self) -> Format {
        Format::Pdf
    }

    fn strategies(&self, _ctx: &RepairContext<'_>) -> Vec<Box<dyn Strategy>> {
        vec![
            Box::new(Verbatim),
            Box::new(RebuildXref),
            Box::new(TruncateToLastObject),
            Box::new(RebuildCatalog),
            Box::new(RebuildHeader),
        ]
    }
}

/// Version written into a rebuilt header. High enough to cover the features a document may
/// already be using, which matters because the header is what a reader consults before
/// deciding whether it understands the rest.
const REBUILT_VERSION: &str = "%PDF-1.7\n";

/// What scanning the document found.
struct Survey {
    objects: Vec<Object>,
    root: Option<(u32, u16)>,
    has_header: bool,
}

impl Survey {
    fn of(ctx: &RepairContext<'_>) -> Self {
        let objects = pdf_objects::scan_objects(ctx.data, || ctx.should_stop());
        Self {
            root: pdf_objects::find_root(ctx.data, &objects),
            has_header: pdf_objects::header_version(ctx.data).is_some(),
            objects,
        }
    }

    fn complete_objects(&self) -> usize {
        self.objects.iter().filter(|object| object.is_complete()).count()
    }

    fn all_complete(&self) -> bool {
        !self.objects.is_empty() && self.complete_objects() == self.objects.len()
    }

    /// True when the file's own table is present, points at the objects it names, and the
    /// document ends the way a complete one does.
    fn is_intact(&self, data: &[u8]) -> bool {
        if !self.has_header || !pdf_objects::ends_properly(data) || self.root.is_none() {
            return false;
        }
        let Some(offset) = pdf_objects::declared_xref_offset(data) else {
            return false;
        };
        if pdf_objects::table_is_accurate(data, offset) {
            return true;
        }
        // A cross-reference *stream* keeps the same information inside a compressed object.
        // Reading it would mean decoding it; that it is a real object at the declared offset,
        // in a file whose objects are all complete, is as far as this check goes.
        pdf_objects::scan_objects(data.get(offset..).unwrap_or_default(), || false)
            .first()
            .is_some_and(|object| object.start == 0)
            && self.all_complete()
    }
}

/// Writes a document holding `prefix` plus a table describing everything in it.
///
/// Objects hidden inside compressed object streams are written out first, as ordinary objects,
/// so the table can point at them. The original bytes are never rewritten - only appended to.
fn rebuild(ctx: &RepairContext<'_>, prefix: &[u8], survey: &Survey) -> Option<Vec<u8>> {
    let mut out = prefix.to_vec();
    if !out.ends_with(b"\n") {
        out.push(b'\n');
    }

    let mut entries: Vec<XrefEntry> = survey
        .objects
        .iter()
        .filter(|object| object.is_complete() && object.start < prefix.len())
        .map(|object| XrefEntry {
            number: object.number,
            offset: object.start,
            generation: object.generation,
        })
        .collect();

    let mut root = survey.root;
    if !pdf_objects::is_encrypted(prefix) {
        let limit = usize::try_from(ctx.allocation_limit).unwrap_or(pdf_objects::MAX_EXPANSION);
        for expanded in pdf_objects::expand_object_streams(prefix, &survey.objects, limit) {
            if entries.iter().any(|entry| entry.number == expanded.number) {
                continue;
            }
            // In a document that packs its objects away, the catalog is usually one of them,
            // so the trailer's target only becomes findable once they are unpacked.
            if root.is_none() && pdf_objects::is_catalog(&expanded.body) {
                root = Some((expanded.number, 0));
            }
            entries.push(XrefEntry {
                number: expanded.number,
                offset: out.len(),
                generation: 0,
            });
            out.extend_from_slice(format!("{} 0 obj\n", expanded.number).as_bytes());
            out.extend_from_slice(&expanded.body);
            out.extend_from_slice(b"\nendobj\n");
        }
    }

    if entries.is_empty() {
        return None;
    }
    pdf_objects::write_xref(&mut out, &entries, root?);
    Some(out)
}

fn score_of(objects: usize, total: Option<usize>) -> Score {
    Score {
        content_recovered: objects as u64,
        content_total: total.map(|total| total as u64),
        decoder_errors: 0,
        decodes_cleanly: true,
        streams_complete: total.is_some_and(|total| total <= objects),
    }
}

/// The table is there and points at what it says it does.
struct Verbatim;

impl Strategy for Verbatim {
    fn name(&self) -> &str {
        "pdf_verbatim"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let survey = Survey::of(ctx);
        if !survey.all_complete() || !survey.is_intact(ctx.data) {
            return Ok(None);
        }
        let count = survey.objects.len();

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(ctx.data.to_vec()),
            score: score_of(count, Some(count)),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// Every object is there; the table that indexes them is missing, stale, or points into
/// nothing. Appending a correct one discards nothing at all, so this works in safe mode.
struct RebuildXref;

impl Strategy for RebuildXref {
    fn name(&self) -> &str {
        "pdf_rebuild_xref"
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let survey = Survey::of(ctx);
        if !survey.has_header || !survey.all_complete() || survey.is_intact(ctx.data) {
            return Ok(None);
        }
        let Some(out) = rebuild(ctx, ctx.data, &survey) else {
            return Ok(None);
        };
        let count = survey.objects.len();

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(count, Some(count)),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// The document stops inside an object. Keep the objects that finished.
///
/// What is lost with the missing bytes is whatever those objects held - pages at the end of
/// the document, usually, since writers lay them out in order. The pages that remain still
/// render, and references to objects that are gone read as null, which is what a reader does
/// with a free entry.
struct TruncateToLastObject;

impl Strategy for TruncateToLastObject {
    fn name(&self) -> &str {
        "pdf_truncate"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            truncate: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        let survey = Survey::of(ctx);
        if survey.all_complete() || survey.complete_objects() == 0 {
            return Ok(None);
        }
        let end = survey.objects.iter().filter_map(|object| object.end).max().unwrap_or(ctx.data.len());
        let Some(prefix) = ctx.data.get(..end) else {
            return Ok(None);
        };
        let Some(out) = rebuild(ctx, prefix, &survey) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            // Nothing states how many objects the document was meant to have once the table is
            // gone, and the ones past the cut left no trace to count.
            score: score_of(survey.complete_objects(), None),
            fidelity: Fidelity::Lossless,
            reconstructed: Vec::new(),
        }))
    }
}

/// No catalog survived, but pages did.
///
/// A document's catalog and page tree are written near the end of the file, so a truncation
/// that leaves most of the content intact frequently takes the one object that says where the
/// pages are. The pages themselves are still there and still complete - what is missing is the
/// index above them, and that can be written: a page tree listing the surviving page objects,
/// and a catalog pointing at it.
///
/// The pages are original; their order is their order in the file, which is how writers emit
/// them. What is invented is the structure around them, and anything a page inherited from the
/// tree that is gone - a page size, say - is inherited from the new tree instead.
struct RebuildCatalog;

impl Strategy for RebuildCatalog {
    fn name(&self) -> &str {
        "pdf_rebuild_catalog"
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
        if survey.root.is_some() || survey.objects.is_empty() {
            return Ok(None);
        }

        // A page whose content stream was beyond the damage renders as a blank sheet. Listing
        // it would produce a document that opens, shows nothing, and looks like a successful
        // repair - so only pages whose contents are still in the file are kept.
        let present: Vec<u32> = survey.objects.iter().filter(|object| object.is_complete()).map(|object| object.number).collect();
        let pages: Vec<&Object> = survey
            .objects
            .iter()
            .filter(|object| {
                let body = ctx.data.get(object.body(ctx.data)).unwrap_or_default();
                let contents = pdf_objects::content_references(body);
                object.is_complete() && pdf_objects::is_page(body) && !contents.is_empty() && contents.iter().all(|number| present.contains(number))
            })
            .collect();
        if pages.is_empty() {
            return Ok(None);
        }

        let end = survey.objects.iter().filter_map(|object| object.end).max().unwrap_or(0);
        let Some(prefix) = ctx.data.get(..end) else {
            return Ok(None);
        };
        let mut out = prefix.to_vec();
        if !out.ends_with(b"\n") {
            out.push(b'\n');
        }

        let mut entries: Vec<XrefEntry> = survey
            .objects
            .iter()
            .filter(|object| object.is_complete())
            .map(|object| XrefEntry {
                number: object.number,
                offset: object.start,
                generation: object.generation,
            })
            .collect();

        let highest = entries.iter().map(|entry| entry.number).max().unwrap_or(0);
        let (tree, catalog) = (highest + 1, highest + 2);
        let kids: Vec<String> = pages.iter().map(|page| format!("{} {} R", page.number, page.generation)).collect();

        entries.push(XrefEntry {
            number: tree,
            offset: out.len(),
            generation: 0,
        });
        out.extend_from_slice(
            format!(
                "{tree} 0 obj\n<< /Type /Pages /Kids [{}] /Count {} /MediaBox [0 0 612 792] >>\nendobj\n",
                kids.join(" "),
                pages.len()
            )
            .as_bytes(),
        );
        entries.push(XrefEntry {
            number: catalog,
            offset: out.len(),
            generation: 0,
        });
        out.extend_from_slice(format!("{catalog} 0 obj\n<< /Type /Catalog /Pages {tree} 0 R >>\nendobj\n").as_bytes());

        pdf_objects::write_xref(&mut out, &entries, (catalog, 0));
        log::debug!("pdf: rebuilt a catalog over {} surviving page(s)", pages.len());

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(survey.complete_objects(), None),
            fidelity: Fidelity::Reconstructed,
            reconstructed: std::iter::once(0..survey.complete_objects() as u64).collect(),
        }))
    }
}

/// The `%PDF-` comment that identifies the file is gone.
///
/// Readers look for it in the first bytes and refuse the file without it. Writing a new one in
/// front of the original bytes costs nothing: every offset in the document is about to be
/// recomputed anyway, since the table has to be rebuilt for the new layout.
struct RebuildHeader;

impl Strategy for RebuildHeader {
    fn name(&self) -> &str {
        "pdf_rebuild_header"
    }

    fn requires(&self) -> Capabilities {
        Capabilities {
            guess: true,
            ..Capabilities::NONE
        }
    }

    fn run(&self, ctx: &RepairContext<'_>) -> Result<Option<Candidate>> {
        if pdf_objects::header_version(ctx.data).is_some() {
            return Ok(None);
        }

        let mut prefix = REBUILT_VERSION.as_bytes().to_vec();
        prefix.extend_from_slice(ctx.data);
        let survey = Survey {
            objects: pdf_objects::scan_objects(&prefix, || ctx.should_stop()),
            root: None,
            has_header: true,
        };
        let survey = Survey {
            root: pdf_objects::find_root(&prefix, &survey.objects),
            ..survey
        };
        if survey.complete_objects() == 0 {
            return Ok(None);
        }

        let complete = survey.complete_objects();
        let Some(out) = rebuild(ctx, &prefix, &survey) else {
            return Ok(None);
        };

        Ok(Some(Candidate {
            strategy: self.name().to_string(),
            data: Payload::Bytes(out),
            score: score_of(complete, None),
            // The objects are the original bytes; only the identifying comment in front of
            // them is new, and nothing in the file can confirm which version it claimed.
            fidelity: Fidelity::Reconstructed,
            reconstructed: std::iter::once(0..complete as u64).collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::pdf_objects::sample_pdf;
    use super::*;
    use crate::file_fixer::corrupt::{ByteSpan, DamageKind, DamageSpec, Fraction, apply_damage};
    use crate::file_fixer::engine::{RepairOptions, RepairResult, repair_data};
    use crate::file_fixer::mode::Mode;
    use crate::file_fixer::report::Outcome;

    fn repair(data: &[u8], mode: Mode) -> RepairResult {
        let options = RepairOptions { mode, ..RepairOptions::default() };
        repair_data(Path::new("document.pdf"), data, &options)
    }

    fn bytes(result: &RepairResult) -> Vec<u8> {
        result
            .winner
            .as_ref()
            .and_then(|winner| winner.data.as_bytes())
            .expect("document payloads stay in memory")
            .to_vec()
    }

    /// Reads a repaired document the way a reader would: follow `startxref`, check the table
    /// against the objects, find the catalog.
    fn opens_cleanly(data: &[u8]) -> bool {
        let Some(offset) = pdf_objects::declared_xref_offset(data) else {
            return false;
        };
        pdf_objects::header_version(data).is_some()
            && pdf_objects::ends_properly(data)
            && pdf_objects::table_is_accurate(data, offset)
            && pdf_objects::find_root(data, &pdf_objects::scan_objects(data, || false)).is_some()
    }

    #[test]
    fn a_healthy_document_is_left_alone() {
        let original = sample_pdf();
        assert!(opens_cleanly(&original), "the fixture must be a working document");

        let result = repair(&original, Mode::Balanced);
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("pdf_verbatim"));
        assert_eq!(result.report.content_recovered, Some(4));
        assert_eq!(bytes(&result), original);
    }

    /// The common real failure: the table's offsets no longer match the objects, because
    /// something rewrote part of the file. Every object is present, so nothing is lost.
    #[test]
    fn a_stale_table_is_rewritten_in_safe_mode() {
        let mut data = sample_pdf();
        // Shift every object by inserting a comment after the header, which is exactly what an
        // editor that does not update the table leaves behind.
        let insert = b"% a comment that moves everything\n";
        for (index, byte) in insert.iter().enumerate() {
            data.insert(9 + index, *byte);
        }
        assert!(!opens_cleanly(&data), "the table must now be wrong");

        let result = repair(&data, Mode::Safe);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("pdf_rebuild_xref"));
        assert_eq!(result.report.outcome, Outcome::FullyRepaired);

        let fixed = bytes(&result);
        assert!(opens_cleanly(&fixed));
        assert_eq!(fixed.get(..data.len()), Some(&data[..]), "the original bytes are kept and only appended to");
    }

    #[test]
    fn a_document_with_no_table_at_all_is_rebuilt() {
        let original = sample_pdf();
        let xref = pdf_objects::find_keyword_last(&original, b"xref").expect("the table");
        let damaged = original.get(..xref).expect("the objects alone").to_vec();

        let result = repair(&damaged, Mode::Safe);
        assert_eq!(result.report.winning_strategy.as_deref(), Some("pdf_rebuild_xref"));
        assert!(opens_cleanly(&bytes(&result)));
    }

    #[test]
    fn a_document_cut_inside_an_object_keeps_the_objects_that_finished() {
        let original = sample_pdf();
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
        assert_eq!(result.report.winning_strategy.as_deref(), Some("pdf_truncate"));
        assert_eq!(result.report.outcome, Outcome::PartiallyRecovered);

        let fixed = bytes(&result);
        assert!(opens_cleanly(&fixed));
        let recovered = pdf_objects::scan_objects(&fixed, || false);
        assert!(!recovered.is_empty() && recovered.len() < 4);
        assert!(recovered.iter().all(|object| object.is_complete()));
    }

    /// Objects packed into a compressed stream are invisible to a scan; a table that does not
    /// list them describes a document missing most of its content.
    #[test]
    fn objects_inside_a_compressed_stream_reach_the_rebuilt_table() {
        let bodies = "<< /Type /Catalog /Pages 6 0 R >> << /Type /Pages /Count 0 >>";
        let header = "5 0 6 34 ";
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(format!("{header}{bodies}").as_bytes(), 6);

        let mut data = Vec::new();
        data.extend_from_slice(b"%PDF-1.5\n");
        data.extend_from_slice(
            format!(
                "1 0 obj\n<< /Type /ObjStm /N 2 /First {} /Length {} /Filter /FlateDecode >>\nstream\n",
                header.len(),
                compressed.len()
            )
            .as_bytes(),
        );
        data.extend_from_slice(&compressed);
        data.extend_from_slice(b"\nendstream\nendobj\n");

        let result = repair(&data, Mode::Safe);
        let fixed = bytes(&result);
        assert!(opens_cleanly(&fixed), "the rebuilt document must open");

        let recovered = pdf_objects::scan_objects(&fixed, || false);
        assert!(recovered.iter().any(|object| object.number == 5), "the catalog was inside the compressed stream");
        assert!(recovered.iter().any(|object| object.number == 6));
    }

    #[test]
    fn a_destroyed_header_is_written_back_in_front_of_the_objects() {
        let original = sample_pdf();
        let damaged = apply_damage(&original, &DamageSpec::new(DamageKind::HeaderNoise { len: ByteSpan::Bytes(8) }, 3));

        assert!(repair(&damaged.data, Mode::Balanced).winner.is_none());

        let result = repair(&damaged.data, Mode::Aggressive);
        let winner = result.winner.as_ref().expect("the objects survived");
        assert_eq!(winner.strategy, "pdf_rebuild_header");

        let fixed = bytes(&result);
        assert!(opens_cleanly(&fixed));
        assert_eq!(pdf_objects::header_version(&fixed).as_deref(), Some("1.7"));
    }

    #[test]
    fn safe_mode_will_not_drop_a_half_written_object() {
        let original = sample_pdf();
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
        let original = sample_pdf();
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
            let recovered = repair(&damaged.data, Mode::Aggressive).report.content_recovered.unwrap_or(0);
            assert!(recovered >= previous, "keep {keep} gave {recovered} after {previous}");
            previous = recovered;
        }
    }

    #[test]
    fn short_and_hostile_inputs_never_panic() {
        let original = sample_pdf();
        for len in 0..original.len().min(400) {
            let mut data = original.clone();
            data.truncate(len);
            let _ = repair(&data, Mode::Aggressive);
        }
    }
}
