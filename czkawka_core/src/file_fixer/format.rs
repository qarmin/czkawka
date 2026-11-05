use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::file_fixer::io::read_u16_be;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Jpeg,
    Png,
    Mp4,
    Avi,
    Gif,
    Bmp,
    Wav,
    Flac,
    WebP,
    Tiff,
    Zip,
    Mkv,
    Pdf,
    Mpeg,
}

impl Format {
    pub const ALL: [Self; 14] = [
        Self::Jpeg,
        Self::Png,
        Self::Mp4,
        Self::Avi,
        Self::Gif,
        Self::Bmp,
        Self::Wav,
        Self::Flac,
        Self::WebP,
        Self::Tiff,
        Self::Zip,
        Self::Mkv,
        Self::Pdf,
        Self::Mpeg,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Jpeg => "jpeg",
            Self::Png => "png",
            Self::Mp4 => "mp4",
            Self::Avi => "avi",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Wav => "wav",
            Self::Flac => "flac",
            Self::WebP => "webp",
            Self::Tiff => "tiff",
            Self::Zip => "zip",
            Self::Mkv => "mkv",
            Self::Pdf => "pdf",
            Self::Mpeg => "mpeg",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Mp4 => "mp4",
            Self::Avi => "avi",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Wav => "wav",
            Self::Flac => "flac",
            Self::WebP => "webp",
            Self::Tiff => "tiff",
            Self::Zip => "zip",
            Self::Mkv => "mkv",
            Self::Pdf => "pdf",
            Self::Mpeg => "mpg",
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// How confident we are, which is also the order in which detection methods are tried.
/// A damaged file may have lost the very bytes a format is normally recognized by, so a
/// weaker source is not a failure - it just means more candidates get generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionSource {
    /// Found a surviving structure deeper in the file (`IDAT`, `mdat`, `SOS`).
    InteriorSignature,
    /// The file name said so.
    Extension,
    /// The leading magic bytes said so.
    Magic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Detection {
    pub format: Format,
    pub source: DetectionSource,
}

const PNG_MAGIC: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
const JPEG_MAGIC: [u8; 3] = [0xFF, 0xD8, 0xFF];
const RIFF_MAGIC: [u8; 4] = *b"RIFF";
const AVI_FORM_TYPE: [u8; 4] = *b"AVI ";
const WAV_FORM_TYPE: [u8; 4] = *b"WAVE";
const WEBP_FORM_TYPE: [u8; 4] = *b"WEBP";
const FLAC_MAGIC: [u8; 4] = *b"fLaC";
const TIFF_MAGIC_LE: [u8; 4] = [b'I', b'I', 0x2A, 0x00];
const TIFF_MAGIC_BE: [u8; 4] = [b'M', b'M', 0x00, 0x2A];
const ZIP_MAGIC: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];
const MKV_MAGIC: [u8; 4] = [0x1A, 0x45, 0xDF, 0xA3];
const PDF_MAGIC: [u8; 5] = *b"%PDF-";
/// A program stream opens with a pack header, a raw video stream with a sequence header. Both
/// begin with the same three-byte start-code prefix, which is what the fourth byte selects on.
const MPEG_START_CODE: [u8; 3] = [0x00, 0x00, 0x01];
const MPEG_PACK_HEADER: u8 = 0xBA;
const MPEG_SEQUENCE_HEADER: u8 = 0xB3;
const MPEG_SYSTEM_HEADER: u8 = 0xBB;

/// Bytes past this point are not scanned for interior signatures. Damage that pushes the
/// first recognizable structure further than this is beyond what guessing can help with,
/// and an unbounded scan would make a hostile file arbitrarily expensive.
const INTERIOR_SCAN_LIMIT: usize = 1 << 20;

pub fn detect_by_magic(data: &[u8]) -> Option<Format> {
    if data.starts_with(&PNG_MAGIC) {
        return Some(Format::Png);
    }
    if data.starts_with(&JPEG_MAGIC) {
        return Some(Format::Jpeg);
    }
    if data.get(4..8) == Some(b"ftyp") {
        return Some(Format::Mp4);
    }
    if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
        return Some(Format::Gif);
    }
    if data.starts_with(b"BM") {
        return Some(Format::Bmp);
    }
    if data.starts_with(&FLAC_MAGIC) {
        return Some(Format::Flac);
    }
    if data.starts_with(&TIFF_MAGIC_LE) || data.starts_with(&TIFF_MAGIC_BE) {
        return Some(Format::Tiff);
    }
    if data.starts_with(&ZIP_MAGIC) {
        return Some(Format::Zip);
    }
    if data.starts_with(&MKV_MAGIC) {
        return Some(Format::Mkv);
    }
    if data.starts_with(&PDF_MAGIC) {
        return Some(Format::Pdf);
    }
    if data.starts_with(&MPEG_START_CODE) && matches!(data.get(3), Some(&MPEG_PACK_HEADER | &MPEG_SEQUENCE_HEADER | &MPEG_SYSTEM_HEADER)) {
        return Some(Format::Mpeg);
    }
    if data.get(0..4) == Some(&RIFF_MAGIC) {
        let form = data.get(8..12);
        if form == Some(&AVI_FORM_TYPE) {
            return Some(Format::Avi);
        }
        if form == Some(&WAV_FORM_TYPE) {
            return Some(Format::Wav);
        }
        if form == Some(&WEBP_FORM_TYPE) {
            return Some(Format::WebP);
        }
        return None;
    }
    None
}

pub fn detect_by_extension(path: &Path) -> Option<Format> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" | "jpe" | "jfif" => Some(Format::Jpeg),
        "png" => Some(Format::Png),
        // MOV/M4V/3GP/HEIC/HEIF share MP4's ISO-BMFF `ftyp` box structure, so the existing
        // remux/trim/rebuild strategies apply to them unchanged; this only extends the
        // extension fallback used when the `ftyp` magic itself is damaged.
        "mp4" | "m4v" | "mov" | "m4a" | "3gp" | "3g2" | "heic" | "heif" => Some(Format::Mp4),
        "avi" => Some(Format::Avi),
        "gif" => Some(Format::Gif),
        "bmp" | "dib" => Some(Format::Bmp),
        "wav" | "wave" => Some(Format::Wav),
        "flac" => Some(Format::Flac),
        "webp" => Some(Format::WebP),
        "tif" | "tiff" => Some(Format::Tiff),
        "zip" => Some(Format::Zip),
        "mkv" | "webm" => Some(Format::Mkv),
        "pdf" => Some(Format::Pdf),
        // VOB and MOD are program streams under another name; M2V and MPV are the raw video
        // stream, which the same strategies handle because ffmpeg demuxes both.
        "mpg" | "mpeg" | "mpe" | "m2p" | "vob" | "mod" | "m2v" | "mpv" => Some(Format::Mpeg),
        _ => None,
    }
}

/// Last resort for files whose header is gone: look for structures that live deeper in the
/// file and would have survived damage confined to the first few bytes.
pub fn detect_by_interior(data: &[u8]) -> Vec<Format> {
    let window = data.get(..INTERIOR_SCAN_LIMIT.min(data.len())).unwrap_or(data);
    let mut found = Vec::new();

    if contains(window, *b"IHDR") || contains(window, *b"IDAT") || contains(window, *b"IEND") {
        found.push(Format::Png);
    }
    if contains(window, *b"mdat") || contains(window, *b"moov") || contains(window, *b"moof") {
        found.push(Format::Mp4);
    }
    if contains_jpeg_marker(window) {
        found.push(Format::Jpeg);
    }
    found
}

fn contains(haystack: &[u8], needle: [u8; 4]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

/// Looks for a define-Huffman-table or start-of-scan segment, the two a JPEG cannot be decoded
/// without and which therefore appear in every real file.
///
/// A marker is only two bytes, which is nowhere near enough on its own: a 50 MB MPEG recording
/// contains 728 `FF C4` pairs and 303 `FF DA` pairs purely by chance, and matching on those
/// made the tool announce such a file as a JPEG, fail to repair it, and report a perfectly good
/// recording as damaged beyond help. What makes a marker real is that it *chains*: its length
/// field leads to another marker, exactly. Two links of that chain are asked for here, which
/// random data supplies about once per twenty gigabytes.
fn contains_jpeg_marker(haystack: &[u8]) -> bool {
    const LINKS_REQUIRED: usize = 2;

    (0..haystack.len().saturating_sub(4))
        .filter(|offset| matches!(haystack.get(*offset..offset + 2), Some([0xFF, 0xC4 | 0xDA])))
        .any(|offset| marker_chain_length(haystack, offset) >= LINKS_REQUIRED)
}

/// How many segments can be walked from `offset`, following each one's declared length.
fn marker_chain_length(data: &[u8], offset: usize) -> usize {
    /// A segment carries its own two length bytes, so anything shorter is not one.
    const MIN_SEGMENT: usize = 2;

    let mut at = offset;
    let mut links = 0;
    while links < 8 {
        let Some([0xFF, marker]) = data.get(at..at + 2) else {
            break;
        };
        // A padding byte or a stuffed zero introduces no segment to follow.
        if *marker == 0xFF || *marker == 0x00 {
            break;
        }
        let Some(length) = read_u16_be(data, at + 2).map(usize::from) else {
            break;
        };
        if length < MIN_SEGMENT {
            break;
        }
        at += 2 + length;
        links += 1;
    }
    links
}

/// Detection results ordered strongest first. More than one is normal for a badly damaged
/// file; the repair engine tries each and lets scoring discard the wrong guesses.
pub fn detect(path: &Path, data: &[u8], allow_interior_scan: bool) -> Vec<Detection> {
    let mut out = Vec::new();
    let mut push = |format, source| {
        if !out.iter().any(|d: &Detection| d.format == format) {
            out.push(Detection { format, source });
        }
    };

    if let Some(format) = detect_by_magic(data) {
        push(format, DetectionSource::Magic);
    }
    if let Some(format) = detect_by_extension(path) {
        push(format, DetectionSource::Extension);
    }
    if allow_interior_scan {
        for format in detect_by_interior(data) {
            push(format, DetectionSource::InteriorSignature);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn magic_wins_over_a_lying_extension() {
        let mut data = PNG_MAGIC.to_vec();
        data.extend_from_slice(b"\x00\x00\x00\x0dIHDR");
        let detections = detect(&PathBuf::from("photo.jpg"), &data, false);

        assert_eq!(detections[0].format, Format::Png);
        assert_eq!(detections[0].source, DetectionSource::Magic);
        assert_eq!(detections[1].format, Format::Jpeg);
        assert_eq!(detections[1].source, DetectionSource::Extension);
    }

    #[test]
    fn mp4_is_recognized_by_ftyp_at_offset_four() {
        let data = b"\x00\x00\x00\x20ftypisom".to_vec();
        assert_eq!(detect_by_magic(&data), Some(Format::Mp4));
    }

    #[test]
    fn avi_is_recognized_by_its_riff_form_type() {
        let data = b"RIFF\x00\x00\x00\x00AVI LIST".to_vec();
        assert_eq!(detect_by_magic(&data), Some(Format::Avi));
    }

    #[test]
    fn a_riff_file_that_is_not_avi_wav_or_webp_is_not_claimed() {
        let data = b"RIFF\x00\x00\x00\x00XXXXfmt ".to_vec();
        assert_eq!(detect_by_magic(&data), None);
    }

    #[test]
    fn wav_and_webp_are_told_apart_by_their_riff_form_type() {
        assert_eq!(detect_by_magic(b"RIFF\x00\x00\x00\x00WAVEfmt "), Some(Format::Wav));
        assert_eq!(detect_by_magic(b"RIFF\x00\x00\x00\x00WEBPVP8 "), Some(Format::WebP));
    }

    #[test]
    fn gif_is_recognized_by_either_version_tag() {
        assert_eq!(detect_by_magic(b"GIF87a\x00\x00\x00\x00\x00\x00"), Some(Format::Gif));
        assert_eq!(detect_by_magic(b"GIF89a\x00\x00\x00\x00\x00\x00"), Some(Format::Gif));
    }

    #[test]
    fn bmp_is_recognized_by_its_two_byte_magic() {
        assert_eq!(detect_by_magic(b"BM\x00\x00\x00\x00\x00\x00"), Some(Format::Bmp));
    }

    #[test]
    fn flac_is_recognized_by_its_stream_marker() {
        assert_eq!(detect_by_magic(b"fLaC\x00\x00\x00\x00"), Some(Format::Flac));
    }

    #[test]
    fn tiff_is_recognized_in_either_byte_order() {
        assert_eq!(detect_by_magic(&[b'I', b'I', 0x2A, 0x00, 0, 0, 0, 0]), Some(Format::Tiff));
        assert_eq!(detect_by_magic(&[b'M', b'M', 0x00, 0x2A, 0, 0, 0, 0]), Some(Format::Tiff));
    }

    #[test]
    fn zip_is_recognized_by_its_local_file_header() {
        assert_eq!(detect_by_magic(&[0x50, 0x4B, 0x03, 0x04, 0, 0, 0, 0]), Some(Format::Zip));
    }

    #[test]
    fn mkv_is_recognized_by_its_ebml_header() {
        assert_eq!(detect_by_magic(&[0x1A, 0x45, 0xDF, 0xA3, 0, 0, 0, 0]), Some(Format::Mkv));
    }

    #[test]
    fn pdf_is_recognized_by_its_header_comment() {
        assert_eq!(detect_by_magic(b"%PDF-1.7\n"), Some(Format::Pdf));
    }

    #[test]
    fn heic_and_3gp_extensions_fall_back_to_mp4s_iso_bmff_strategies() {
        assert_eq!(detect_by_extension(&PathBuf::from("a.heic")), Some(Format::Mp4));
        assert_eq!(detect_by_extension(&PathBuf::from("a.3gp")), Some(Format::Mp4));
    }

    #[test]
    fn every_format_round_trips_through_its_own_extension() {
        for format in Format::ALL {
            let path = PathBuf::from(format!("a.{}", format.extension()));
            assert_eq!(detect_by_extension(&path), Some(format), "{format}");
        }
    }

    #[test]
    fn interior_scan_finds_png_with_a_destroyed_header() {
        let mut data = vec![0u8; 64];
        data.extend_from_slice(b"\x00\x00\x00\x00IDAT");
        assert!(detect_by_magic(&data).is_none());
        assert_eq!(detect_by_interior(&data), vec![Format::Png]);
    }

    /// A marker only counts when its length field leads to another marker - the chain is what
    /// separates a real segment from two coincidental bytes.
    #[test]
    fn interior_scan_finds_jpeg_by_a_chain_of_segments() {
        let mut data = vec![0u8; 32];
        // DHT of 12 bytes, then DQT of 8, then the scan header: three links.
        data.extend_from_slice(&[0xFF, 0xC4, 0x00, 0x0C]);
        data.extend_from_slice(&[0u8; 10]);
        data.extend_from_slice(&[0xFF, 0xDB, 0x00, 0x08]);
        data.extend_from_slice(&[0u8; 6]);
        data.extend_from_slice(&[0xFF, 0xDA, 0x00, 0x0C]);

        assert_eq!(detect_by_interior(&data), vec![Format::Jpeg]);
    }

    /// The case this strictness exists for: a 50 MB MPEG recording holds hundreds of `FF C4`
    /// pairs by chance, and claiming it is a JPEG made the tool report it as damaged.
    #[test]
    fn a_lone_marker_in_arbitrary_data_is_not_a_jpeg() {
        let mut data = vec![0u8; 32];
        data.extend_from_slice(&[0xFF, 0xDA, 0x00, 0x0C]);
        assert!(detect_by_interior(&data).is_empty(), "one marker proves nothing");

        // Pseudo-random bytes, the shape of compressed video, must not look like a JPEG.
        let noise: Vec<u8> = (0..1u32 << 18u32).map(|index| (index.wrapping_mul(2_654_435_761) >> 13) as u8).collect();
        assert!(!detect_by_interior(&noise).contains(&Format::Jpeg));
    }

    #[test]
    fn interior_scan_is_opt_in() {
        let mut data = vec![0u8; 64];
        data.extend_from_slice(b"\x00\x00\x00\x00mdat");
        assert!(detect(&PathBuf::from("clip"), &data, false).is_empty());
        assert_eq!(detect(&PathBuf::from("clip"), &data, true).len(), 1);
    }

    #[test]
    fn extension_matching_ignores_case() {
        assert_eq!(detect_by_extension(&PathBuf::from("a.JPEG")), Some(Format::Jpeg));
        assert_eq!(detect_by_extension(&PathBuf::from("a.Mp4")), Some(Format::Mp4));
        assert_eq!(detect_by_extension(&PathBuf::from("a.txt")), None);
        assert_eq!(detect_by_extension(&PathBuf::from("noext")), None);
    }

    #[test]
    fn short_input_never_panics() {
        for len in 0..12 {
            let data = vec![0xFFu8; len];
            let _ = detect(&PathBuf::from("x.bin"), &data, true);
        }
    }
}
