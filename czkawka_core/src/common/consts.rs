pub const DEFAULT_THREAD_SIZE: usize = 8 * 1024 * 1024; // 8 MB
pub const DEFAULT_WORKER_THREAD_SIZE: usize = 4 * 1024 * 1024; // 4 MB
pub const VIDEO_RESOLUTION_LIMIT: u32 = 16 * 1024; // Not processing is a problem, but overflows, when width * height overflows u64 in gui, so with such limit can i32 can be used safely

pub const RAW_IMAGE_EXTENSIONS: &[&str] = &[
    "ari", "cr3", "cr2", "crw", "erf", "raf", "3fr", "kdc", "dcs", "dcr", "iiq", "mos", "mef", "mrw", "nef", "nrw", "orf", "rw2", "pef", "srw", "arw", "srf", "sr2",
];
#[cfg(feature = "libavif")]
pub const IMAGE_RS_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "bmp", "tiff", "tif", "tga", "ff", "jif", "jfi", "webp", "gif", "ico", "exr", "qoi", "jxl", "avif",
];
#[cfg(not(feature = "libavif"))]
pub const IMAGE_RS_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "bmp", "tiff", "tif", "tga", "ff", "jif", "jfi", "webp", "gif", "ico", "exr", "qoi", "jxl",
];
#[cfg(feature = "libavif")]
pub const IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "bmp", "webp", "exr", "qoi", "jxl", "avif"];
#[cfg(not(feature = "libavif"))]
pub const IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "bmp", "webp", "exr", "qoi", "jxl"];
#[cfg(feature = "libavif")]
pub const IMAGE_RS_BROKEN_FILES_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "gif", "bmp", "ico", "jfif", "jpe", "pnz", "dib", "webp", "exr", "avif", "jxl",
];
#[cfg(not(feature = "libavif"))]
pub const IMAGE_RS_BROKEN_FILES_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "gif", "bmp", "ico", "jfif", "jpe", "pnz", "dib", "webp", "exr", "jxl",
];
pub const HEIC_EXTENSIONS: &[&str] = &["heif", "heifs", "heic", "heics", "avci", "avcs", "hif"];
pub const ZIP_FILES_EXTENSIONS: &[&str] = &["zip", "jar"];
pub const SEVENZ_FILES_EXTENSIONS: &[&str] = &["7z"];
pub const GZ_FILES_EXTENSIONS: &[&str] = &["gz", "tgz"];
pub const TAR_FILES_EXTENSIONS: &[&str] = &["tar"];
pub const ZST_FILES_EXTENSIONS: &[&str] = &["zst", "tzst"];
// WOFF and WOFF2 are excluded: they wrap TTF/OTF in a compressed container that ttf-parser
pub const FONT_FILES_EXTENSIONS: &[&str] = &["ttf", "otf", "ttc"];
// Markup/structured-text files - validated as JSON, XML, TOML or YAML
pub const JSON_FILES_EXTENSIONS: &[&str] = &["json"];
pub const XML_FILES_EXTENSIONS: &[&str] = &["xml", "xhtml", "xsd", "xsl", "xslt", "rss", "atom"];
pub const SVG_FILES_EXTENSIONS: &[&str] = &["svg", "svgz"];
pub const TOML_FILES_EXTENSIONS: &[&str] = &["toml"];
pub const YAML_FILES_EXTENSIONS: &[&str] = &["yaml", "yml"];
pub const BZ2_FILES_EXTENSIONS: &[&str] = &["bz2", "tbz2", "tbz"];
pub const XZ_FILES_EXTENSIONS: &[&str] = &["xz", "txz"];
pub const PDF_FILES_EXTENSIONS: &[&str] = &["pdf"];
// Extensions lofty-rs can read tags from (see lofty::file::EXTENSIONS).
pub const AUDIO_FILES_TAGS_EXTENSIONS: &[&str] = &[
    "mp3", "mp2", "mp1", "flac", "wav", "wave", "ogg", "oga", "opus", "aac", "wma", "aiff", "aif", "afc", "aifc", "m4a", "m4b", "m4p", "mp4a", "mp2a", "mpga", "m3a", "ape", "wv",
    "mpc", "spx", "weba", "pcm",
];
// Extensions symphonia can decode (features = "all": ogg/isomp4/mkv/aiff/wav/caf; codecs aac/alac/flac/mp1-3/pcm/vorbis).
// No opus (codec absent), no wma (ASF format absent - files would be falsely flagged as broken).
pub const AUDIO_FILES_CONTENT_EXTENSIONS: &[&str] = &[
    "mp3", "mp2", "mp1", "mp2a", "mpga", "m3a", "flac", "wav", "wave", "pcm", "ogg",
    "oga", // Vorbis works; OGG Opus returns UnsupportedCodec which broken_files treats as not-broken
    "aac", "m4a", "m4b", "m4p", "mp4a", "aiff", "aif", "aifc", "weba",
];
pub const VIDEO_FILES_EXTENSIONS: &[&str] = &[
    "mp4", "m4v", "mkv", "avi", "mov", "webm", "flv", "wmv", // Popular
    "mpeg", "mpg", "mpe", "m2ts", "vob", "evo", // MPEG / broadcast, "ts"
    "3gp", "3g2", "f4v", "f4p", "f4a", "f4b", // Mobile / legacy
    "qt", "m4p", "mpv", // Apple / ISO BMFF
    "ogv", "rm", "rmvb", "asf", // Streaming / recording
    "dv", "mxf", "roq", "nsv", "yuv", // Professional
    "y4m", "h264", "h265", "hevc", "av1", "vp8", "vp9", // Raw / uncompressed
    "amv", "drc", "gifv", "smk", "bik", // Older / games
];

pub const TEXT_FILES_EXTENSIONS: &[&str] = &["txt", "md", "csv", "log", "ini", "json", "xml", "yaml", "yml", "toml", "doc", "docx", "rtf", "odt"];

// "dng" - is theoretically a tiff file, but little_exif have problem with saving metadata to it
pub const EXIF_FILES_EXTENSIONS: &[&str] = &["jpg", "jpeg", "jfif", "png", "tiff", "tif", "avif", "jxl", "webp", "heic", "heif"];
