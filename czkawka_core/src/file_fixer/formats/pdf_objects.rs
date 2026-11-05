//! Finding the objects in a PDF, and writing a cross-reference table for them.
//!
//! A PDF is a heap of numbered objects plus a table saying where each one is. The table is the
//! fragile part: it is written last, it is written at the end of the file, and every editor
//! that touches the document appends another one. Practically every "damaged PDF" is a
//! document whose objects are all present and whose table no longer points at them.
//!
//! Objects announce themselves - `12 0 obj` - so they can be found without the table, which is
//! what makes the repair possible. The hard part is not finding them but the two ways a modern
//! PDF hides them: a stream's binary payload can contain anything, including the very keywords
//! being searched for, and since PDF 1.5 objects may be packed *inside* another object's
//! compressed stream, where no scan will ever see them. Both are handled below - streams are
//! stepped over by their declared length where possible, and object streams are decompressed
//! and their contents written out as ordinary objects.

use std::ops::Range;

/// Objects numbered above this are a misparse, not a document.
const MAX_OBJECT_NUMBER: u32 = 10_000_000;

/// Ceiling on how much an object stream may expand to, so a hostile file cannot ask for the
/// whole address space.
pub const MAX_EXPANSION: usize = 256 << 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    pub number: u32,
    pub generation: u16,
    /// Offset of the object number that introduces it.
    pub start: usize,
    /// Offset just past `endobj`, when it was found.
    pub end: Option<usize>,
}

impl Object {
    pub fn is_complete(&self) -> bool {
        self.end.is_some()
    }

    /// The object's body, from its header to its end.
    pub fn body(&self, data: &[u8]) -> Range<usize> {
        self.start..self.end.unwrap_or(data.len())
    }
}

/// Finds every object the file announces, in the order they appear.
///
/// Later definitions of the same object number replace earlier ones, which is how PDF's own
/// incremental updates work: the newest copy is the live one.
pub fn scan_objects(data: &[u8], mut should_stop: impl FnMut() -> bool) -> Vec<Object> {
    let mut objects: Vec<Object> = Vec::new();
    let mut offset = 0;

    while offset < data.len() {
        if should_stop() {
            break;
        }
        let Some(found) = find_keyword(data, offset, b"obj") else {
            break;
        };
        let Some((number, generation, start)) = read_object_header(data, found) else {
            offset = found + 3;
            continue;
        };

        let end = find_object_end(data, found + 3);
        offset = end.unwrap_or(data.len()).max(found + 3);

        // A newer definition of the same number supersedes the old one.
        if let Some(previous) = objects.iter_mut().find(|object| object.number == number) {
            *previous = Object { number, generation, start, end };
        } else {
            objects.push(Object { number, generation, start, end });
        }
    }
    objects
}

/// Reads the `12 0 ` that must precede an `obj` keyword, returning where it starts.
fn read_object_header(data: &[u8], keyword_at: usize) -> Option<(u32, u16, usize)> {
    let mut cursor = skip_space_backwards(data, keyword_at)?;
    let (generation, after_generation) = read_number_backwards(data, cursor)?;
    cursor = skip_space_backwards(data, after_generation)?;
    let (number, start) = read_number_backwards(data, cursor)?;

    if number == 0 || number > MAX_OBJECT_NUMBER {
        return None;
    }
    Some((number, u16::try_from(generation).unwrap_or(0), start))
}

fn skip_space_backwards(data: &[u8], from: usize) -> Option<usize> {
    let mut cursor = from;
    let mut seen = false;
    while cursor > 0 && data.get(cursor - 1).is_some_and(|byte| byte.is_ascii_whitespace()) {
        cursor -= 1;
        seen = true;
    }
    seen.then_some(cursor)
}

/// Reads a decimal number that ends at `from`, returning it and where it starts.
fn read_number_backwards(data: &[u8], from: usize) -> Option<(u32, usize)> {
    let mut start = from;
    while start > 0 && data.get(start - 1).is_some_and(u8::is_ascii_digit) {
        start -= 1;
    }
    if start == from || from - start > 10 {
        return None;
    }
    let text = std::str::from_utf8(data.get(start..from)?).ok()?;
    Some((text.parse().ok()?, start))
}

/// Where an object ends, stepping over any stream it contains.
///
/// A stream's payload is arbitrary bytes and may well contain the word `endobj`, so the
/// declared length is used to jump past it whenever it is a direct number. When the length is
/// itself an indirect reference - legal, and produced by streaming writers that do not know
/// the size in advance - there is nothing to do but look for `endstream`.
fn find_object_end(data: &[u8], from: usize) -> Option<usize> {
    let mut cursor = from;
    loop {
        let stream = find_stream_start(data, cursor);
        let end_object = find_keyword(data, cursor, b"endobj");

        match (stream, end_object) {
            (Some(stream_at), Some(end_at)) if stream_at < end_at => {
                let payload = stream_payload_start(data, stream_at);
                cursor = match declared_length(data, from, stream_at) {
                    Some(length) if payload + length <= data.len() => payload + length,
                    _ => find_keyword(data, payload, b"endstream")? + 9,
                };
            }
            (_, Some(end_at)) => return Some(end_at + 6),
            (_, None) => return None,
        }
    }
}

/// The word `stream` that opens a payload, skipping the `endstream` that closes one - the
/// shorter keyword is a suffix of the longer, so a plain search finds both.
fn find_stream_start(data: &[u8], from: usize) -> Option<usize> {
    let mut at = from;
    loop {
        let found = find_keyword(data, at, b"stream")?;
        if found < 3 || data.get(found - 3..found) != Some(b"end") {
            return Some(found);
        }
        at = found + 6;
    }
}

/// The `/Length` of the stream that starts at `stream_at`, when it is written directly.
///
/// `None` for the indirect form, `/Length 12 0 R`, where the number is an object number and
/// resolving it would mean reading the object it names - which a damaged file may not have.
fn declared_length(data: &[u8], dictionary_start: usize, stream_at: usize) -> Option<usize> {
    let dictionary = data.get(dictionary_start..stream_at)?;
    let at = find_keyword(dictionary, 0, b"/Length")? + 7;
    let text = String::from_utf8_lossy(dictionary.get(at..)?);
    let mut words = text.split_whitespace();

    let length: usize = words.next()?.parse().ok()?;
    let indirect = words.next().is_some_and(|word| word.parse::<u32>().is_ok()) && words.next().is_some_and(|word| word.starts_with('R'));
    (!indirect).then_some(length)
}

/// Where a stream's bytes begin: past the keyword and the end-of-line that must follow it.
fn stream_payload_start(data: &[u8], keyword_at: usize) -> usize {
    let mut cursor = keyword_at + 6;
    if data.get(cursor) == Some(&b'\r') {
        cursor += 1;
    }
    if data.get(cursor) == Some(&b'\n') {
        cursor += 1;
    }
    cursor
}

pub fn find_keyword(data: &[u8], from: usize, keyword: &[u8]) -> Option<usize> {
    let window = data.get(from..)?;
    window.windows(keyword.len()).position(|candidate| candidate == keyword).map(|position| from + position)
}

/// Searches backwards, which is how the trailer and the start of the table are found.
pub fn find_keyword_last(data: &[u8], keyword: &[u8]) -> Option<usize> {
    data.windows(keyword.len()).rposition(|candidate| candidate == keyword)
}

/// The catalog object a rebuilt trailer has to point at.
///
/// A surviving trailer says which it is. When there is none - or it is the part that was
/// damaged - the catalog says so about itself, and there is exactly one object claiming to be
/// one in a healthy document.
pub fn find_root(data: &[u8], objects: &[Object]) -> Option<(u32, u16)> {
    if let Some(root) = find_root_in_trailer(data) {
        return Some(root);
    }
    objects
        .iter()
        .find(|object| is_catalog(data.get(object.body(data)).unwrap_or_default()))
        .map(|object| (object.number, object.generation))
}

/// True when an object's body declares itself the document catalog.
pub fn is_catalog(body: &[u8]) -> bool {
    contains_name(body, b"/Catalog") && contains_name(body, b"/Type")
}

/// The objects a page's `/Contents` entry points at, which may be one reference or an array of
/// them. An empty result means the page names no content at all.
pub fn content_references(body: &[u8]) -> Vec<u32> {
    let Some(at) = find_keyword(body, 0, b"/Contents") else {
        return Vec::new();
    };
    let tail = body.get(at + 9..).unwrap_or_default();
    let end = find_keyword(tail, 0, b"/").unwrap_or(tail.len());
    // Both forms reduce to the same thing once the brackets are spaces: every "n g R" triple.
    let text = String::from_utf8_lossy(tail.get(..end).unwrap_or_default()).replace(['[', ']'], " ");

    let words: Vec<&str> = text.split_whitespace().collect();
    let mut references = Vec::new();
    for (index, word) in words.iter().enumerate() {
        if *word != "R" || index < 2 {
            continue;
        }
        if let Some(Ok(number)) = words.get(index - 2).map(|word| word.parse::<u32>()) {
            references.push(number);
        }
    }
    references
}

/// True when an object's body declares itself a single page.
///
/// `/Pages` is a different thing - the node that groups them - and starts the same way, so the
/// character after the name has to be checked.
pub fn is_page(body: &[u8]) -> bool {
    let mut from = 0;
    while let Some(at) = find_keyword(body, from, b"/Page") {
        let next = body.get(at + 5).copied().unwrap_or(b' ');
        if !next.is_ascii_alphanumeric() {
            return true;
        }
        from = at + 5;
    }
    false
}

fn find_root_in_trailer(data: &[u8]) -> Option<(u32, u16)> {
    let mut search_from = data.len();
    while let Some(at) = find_keyword_last(data.get(..search_from)?, b"/Root") {
        if let Some(reference) = read_reference(data, at + 5) {
            return Some(reference);
        }
        search_from = at;
    }
    None
}

/// Reads `12 0 R` at `from`.
fn read_reference(data: &[u8], from: usize) -> Option<(u32, u16)> {
    let window = data.get(from..(from + 32).min(data.len()))?;
    let text = String::from_utf8_lossy(window);
    let mut parts = text.split_whitespace();
    let number: u32 = parts.next()?.parse().ok()?;
    let generation: u16 = parts.next()?.parse().ok()?;
    (parts.next()? == "R").then_some((number, generation))
}

fn contains_name(data: &[u8], name: &[u8]) -> bool {
    find_keyword(data, 0, name).is_some()
}

/// An object recovered from inside another object's compressed stream.
pub struct Expanded {
    pub number: u32,
    /// The object's body, ready to be written out between `N 0 obj` and `endobj`.
    pub body: Vec<u8>,
}

/// Decompresses every object stream and returns the objects packed inside them.
///
/// Since PDF 1.5 a document may pack most of its objects into the compressed stream of a
/// single `/ObjStm` object. Those objects are invisible to any scan of the file, and a
/// cross-reference table that does not list them describes a document missing most of its
/// content - so they are decompressed and written back out as ordinary objects, where the
/// rebuilt table can point at them.
pub fn expand_object_streams(data: &[u8], objects: &[Object], limit: usize) -> Vec<Expanded> {
    let mut expanded = Vec::new();
    let mut budget = limit.min(MAX_EXPANSION);

    for object in objects {
        let body = data.get(object.body(data)).unwrap_or_default();
        if !contains_name(body, b"/ObjStm") {
            continue;
        }
        let Some(stream) = stream_payload(data, object) else {
            continue;
        };
        let decoded = super::deflate::inflate(stream, budget, true);
        if decoded.data.is_empty() {
            continue;
        }
        budget = budget.saturating_sub(decoded.data.len());

        let Some(count) = dictionary_number(body, b"/N") else {
            continue;
        };
        let Some(first) = dictionary_number(body, b"/First") else {
            continue;
        };
        expanded.extend(split_object_stream(&decoded.data, count, first));
        if budget == 0 {
            break;
        }
    }
    expanded
}

/// The raw bytes of an object's stream, between the keyword and `endstream`.
fn stream_payload<'a>(data: &'a [u8], object: &Object) -> Option<&'a [u8]> {
    let body = object.body(data);
    let keyword = find_keyword(data, body.start, b"stream")?;
    if keyword >= body.end {
        return None;
    }
    let start = stream_payload_start(data, keyword);
    let end = match declared_length(data, body.start, keyword) {
        Some(length) if start + length <= data.len() => start + length,
        _ => find_keyword(data, start, b"endstream")?,
    };
    data.get(start..end.min(body.end))
}

/// Reads `/Name 123` out of an object's dictionary.
fn dictionary_number(body: &[u8], name: &[u8]) -> Option<usize> {
    let at = find_keyword(body, 0, name)? + name.len();
    let digits: Vec<u8> = body
        .get(at..)?
        .iter()
        .skip_while(|byte| byte.is_ascii_whitespace())
        .take_while(|byte| byte.is_ascii_digit())
        .copied()
        .collect();
    std::str::from_utf8(&digits).ok()?.parse().ok()
}

/// Splits a decompressed object stream into the objects it holds.
///
/// The stream begins with `count` pairs of numbers - object number and offset - followed by
/// the bodies themselves, starting at `first`.
fn split_object_stream(decoded: &[u8], count: usize, first: usize) -> Vec<Expanded> {
    let header = decoded.get(..first.min(decoded.len())).unwrap_or_default();
    let text = String::from_utf8_lossy(header);
    let numbers: Vec<usize> = text.split_whitespace().filter_map(|word| word.parse().ok()).collect();

    let mut out = Vec::with_capacity(count);
    for index in 0..count {
        let Some(number) = numbers.get(index * 2).copied() else {
            break;
        };
        let Some(offset) = numbers.get(index * 2 + 1).copied() else {
            break;
        };
        let start = first + offset;
        let end = match numbers.get(index * 2 + 3).copied() {
            Some(next) => (first + next).min(decoded.len()),
            None => decoded.len(),
        };
        let Some(body) = decoded.get(start.min(decoded.len())..end.max(start.min(decoded.len()))) else {
            continue;
        };
        let Ok(number) = u32::try_from(number) else {
            continue;
        };
        if number == 0 || body.is_empty() {
            continue;
        }
        out.push(Expanded { number, body: body.to_vec() });
    }
    out
}

/// One entry of the table being written.
pub struct XrefEntry {
    pub number: u32,
    pub offset: usize,
    pub generation: u16,
}

/// Writes a cross-reference table, trailer and end marker describing `entries`.
///
/// The table is written as a single contiguous section covering object numbers zero to the
/// highest in use, with the numbers nothing was found for marked free. That is a legal table
/// and the simplest correct one to produce - a reader handles a free entry by treating any
/// reference to it as null, which is what a missing object amounts to anyway.
pub fn write_xref(out: &mut Vec<u8>, entries: &[XrefEntry], root: (u32, u16)) {
    let highest = entries.iter().map(|entry| entry.number).max().unwrap_or(0);
    let size = highest as usize + 1;
    let start = out.len();

    out.extend_from_slice(b"xref\n");
    out.extend_from_slice(format!("0 {size}\n").as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");

    for number in 1..=highest {
        match entries.iter().find(|entry| entry.number == number) {
            Some(entry) => {
                out.extend_from_slice(format!("{:010} {:05} n \n", entry.offset, entry.generation).as_bytes());
            }
            None => out.extend_from_slice(b"0000000000 65535 f \n"),
        }
    }

    out.extend_from_slice(b"trailer\n");
    out.extend_from_slice(format!("<< /Size {size} /Root {} {} R >>\n", root.0, root.1).as_bytes());
    out.extend_from_slice(format!("startxref\n{start}\n%%EOF\n").as_bytes());
}

/// Where the file says its table is, from the `startxref` at the end.
pub fn declared_xref_offset(data: &[u8]) -> Option<usize> {
    let at = find_keyword_last(data, b"startxref")? + 9;
    let window = data.get(at..(at + 32).min(data.len()))?;
    let text = String::from_utf8_lossy(window);
    text.split_whitespace().next()?.parse().ok()
}

/// True when the file ends the way a complete PDF does.
pub fn ends_properly(data: &[u8]) -> bool {
    let tail = data.get(data.len().saturating_sub(64)..).unwrap_or_default();
    find_keyword(tail, 0, b"%%EOF").is_some()
}

/// Reads the version from the header comment, which also confirms the file starts like a PDF.
pub fn header_version(data: &[u8]) -> Option<String> {
    let line = data.get(..16)?;
    let text = String::from_utf8_lossy(line);
    let version = text.strip_prefix("%PDF-")?;
    let digits: String = version.chars().take_while(|character| character.is_ascii_digit() || *character == '.').collect();
    (!digits.is_empty()).then_some(digits)
}

/// True when the document is encrypted, in which case streams cannot be read here: the bytes
/// are ciphertext, and decompressing them would produce noise rather than objects.
pub fn is_encrypted(data: &[u8]) -> bool {
    find_keyword_last(data, b"/Encrypt").is_some()
}

/// Walks a classic cross-reference table and checks that what it points at is really there.
///
/// This is the check that separates "the table is fine" from "the table is stale", which is
/// what an editor that wrote an update wrongly leaves behind. Every in-use entry has to land on
/// the object it claims.
pub fn table_is_accurate(data: &[u8], at: usize) -> bool {
    if data.get(at..at + 4) != Some(b"xref") {
        return false;
    }
    let Some(text) = data.get(at + 4..).map(|tail| {
        let end = find_keyword(tail, 0, b"trailer").unwrap_or(tail.len().min(1 << 20));
        String::from_utf8_lossy(tail.get(..end).unwrap_or_default()).into_owned()
    }) else {
        return false;
    };

    let mut words = text.split_whitespace().peekable();
    let mut checked = 0;
    while let Some(first) = words.next() {
        let (Ok(start), Some(Ok(count))) = (first.parse::<u32>(), words.next().map(str::parse::<u32>)) else {
            return false;
        };
        for index in 0..count {
            let (Some(offset), Some(generation), Some(kind)) = (words.next(), words.next(), words.next()) else {
                return false;
            };
            if kind != "n" {
                continue;
            }
            let (Ok(offset), Ok(generation)) = (offset.parse::<usize>(), generation.parse::<u16>()) else {
                return false;
            };
            let expected = format!("{} {generation} obj", start + index);
            if data.get(offset..offset + expected.len()) != Some(expected.as_bytes()) {
                return false;
            }
            checked += 1;
        }
        if words.peek().is_none() {
            break;
        }
    }
    checked > 0
}

#[cfg(test)]
/// A small but genuine PDF: catalog, page tree, one page, and a content stream.
pub fn sample_pdf() -> Vec<u8> {
    let mut out = Vec::new();
    let mut offsets = Vec::new();
    out.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");

    let contents = b"BT /F1 24 Tf 72 700 Td (file_fixer) Tj ET";
    let bodies: [Vec<u8>; 4] = [
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R >>".to_vec(),
        {
            let mut stream = format!("<< /Length {} >>\nstream\n", contents.len()).into_bytes();
            stream.extend_from_slice(contents);
            stream.extend_from_slice(b"\nendstream");
            stream
        },
    ];

    for (index, body) in bodies.iter().enumerate() {
        offsets.push(XrefEntry {
            number: index as u32 + 1,
            offset: out.len(),
            generation: 0,
        });
        out.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    }

    write_xref(&mut out, &offsets, (1, 0));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn never() -> impl FnMut() -> bool {
        || false
    }

    #[test]
    fn a_built_document_walks_back_to_its_objects() {
        let data = sample_pdf();
        let objects = scan_objects(&data, never());

        assert_eq!(objects.len(), 4);
        assert!(objects.iter().all(Object::is_complete));
        assert_eq!(objects.first().map(|object| object.number), Some(1));
        assert_eq!(header_version(&data).as_deref(), Some("1.4"));
        assert!(ends_properly(&data));
    }

    #[test]
    fn the_table_the_file_declares_points_at_it() {
        let data = sample_pdf();
        let offset = declared_xref_offset(&data).expect("startxref");
        assert_eq!(data.get(offset..offset + 4), Some(&b"xref"[..]));
    }

    #[test]
    fn the_catalog_is_found_from_the_trailer_and_from_the_object_itself() {
        let data = sample_pdf();
        let objects = scan_objects(&data, never());
        assert_eq!(find_root(&data, &objects), Some((1, 0)));

        // With the trailer gone, the object that calls itself a catalog still answers.
        let trailer = find_keyword_last(&data, b"trailer").expect("trailer");
        let without = data.get(..trailer).expect("prefix").to_vec();
        let objects = scan_objects(&without, never());
        assert_eq!(find_root(&without, &objects), Some((1, 0)));
    }

    /// A stream's payload is arbitrary bytes, and the scan must not be fooled by the keywords
    /// it can contain.
    #[test]
    fn keywords_inside_a_stream_do_not_end_the_object() {
        let payload = b"endobj 9 0 obj endstream trap";
        let mut data = Vec::new();
        data.extend_from_slice(b"%PDF-1.4\n");
        let start = data.len();
        data.extend_from_slice(format!("1 0 obj\n<< /Length {} >>\nstream\n", payload.len()).as_bytes());
        data.extend_from_slice(payload);
        data.extend_from_slice(b"\nendstream\nendobj\n");
        data.extend_from_slice(b"2 0 obj\n<< /Type /Catalog >>\nendobj\n");

        let objects = scan_objects(&data, never());
        assert_eq!(objects.len(), 2, "the trap object number must not become an object");
        assert_eq!(objects.first().map(|object| object.start), Some(start));
        assert!(objects.iter().all(Object::is_complete));
    }

    #[test]
    fn a_later_definition_of_an_object_replaces_the_earlier_one() {
        let mut data = sample_pdf();
        let second = data.len();
        data.extend_from_slice(b"3 0 obj\n<< /Type /Page /Rotate 90 >>\nendobj\n");

        let objects = scan_objects(&data, never());
        assert_eq!(objects.iter().filter(|object| object.number == 3).count(), 1);
        assert_eq!(objects.iter().find(|object| object.number == 3).map(|object| object.start), Some(second));
    }

    #[test]
    fn objects_packed_into_a_compressed_stream_are_recovered() {
        let bodies = "<< /Type /Catalog /Pages 6 0 R >> << /Type /Pages /Count 0 >>";
        let header = "5 0 6 34 ";
        let decoded = format!("{header}{bodies}");
        let compressed = miniz_oxide::deflate::compress_to_vec_zlib(decoded.as_bytes(), 6);

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

        let objects = scan_objects(&data, never());
        let expanded = expand_object_streams(&data, &objects, 1 << 20);

        assert_eq!(expanded.len(), 2);
        assert_eq!(expanded.first().map(|object| object.number), Some(5));
        assert_eq!(expanded.get(1).map(|object| object.number), Some(6));
        assert!(expanded.first().is_some_and(|object| String::from_utf8_lossy(&object.body).contains("/Catalog")));
    }

    #[test]
    fn a_document_cut_inside_an_object_reports_it_incomplete() {
        let mut data = sample_pdf();
        data.truncate(data.len() / 2);

        let objects = scan_objects(&data, never());
        assert!(!objects.is_empty());
        assert!(objects.iter().any(|object| !object.is_complete()));
    }

    #[test]
    fn short_and_hostile_input_never_panics() {
        for len in 0..64usize {
            let data = vec![b'%'; len];
            let _ = scan_objects(&data, never());
            let _ = header_version(&data);
            let _ = declared_xref_offset(&data);
        }
        let _ = scan_objects(b"0 0 obj obj obj 999999999999 0 obj", never());
    }
}
