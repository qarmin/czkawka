pub const DEFAULT_THREAD_SIZE: usize = 8 * 1024 * 1024; // 8 MB
pub const DEFAULT_WORKER_THREAD_SIZE: usize = 4 * 1024 * 1024; // 4 MB
pub const VIDEO_RESOLUTION_LIMIT: u32 = 16 * 1024; // Not processing is a problem, but overflows, when width * height overflows u64 in gui, so with such limit can i32 can be used safely

pub const RAW_IMAGE_EXTENSIONS: &[&str] = &[
    "ari", "cr3", "cr2", "crw", "erf", "raf", "3fr", "kdc", "dcs", "dcr", "iiq", "mos", "mef", "mrw", "nef", "nrw", "orf", "rw2", "pef", "srw", "arw", "srf", "sr2",
];
pub const JXL_IMAGE_EXTENSIONS: &[&str] = &["jxl"];
#[cfg(feature = "libavif")]
pub const IMAGE_RS_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "bmp", "tiff", "tif", "tga", "ff", "jif", "jfi", "webp", "gif", "ico", "exr", "qoi", "avif",
];
#[cfg(not(feature = "libavif"))]
pub const IMAGE_RS_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "bmp", "tiff", "tif", "tga", "ff", "jif", "jfi", "webp", "gif", "ico", "exr", "qoi"];
#[cfg(feature = "libavif")]
pub const IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "bmp", "webp", "exr", "qoi", "avif"];
#[cfg(not(feature = "libavif"))]
pub const IMAGE_RS_SIMILAR_IMAGES_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "bmp", "webp", "exr", "qoi"];
#[cfg(feature = "libavif")]
pub const IMAGE_RS_BROKEN_FILES_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "gif", "bmp", "ico", "jfif", "jpe", "pnz", "dib", "webp", "exr", "avif",
];
#[cfg(not(feature = "libavif"))]
pub const IMAGE_RS_BROKEN_FILES_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "tiff", "tif", "tga", "ff", "jif", "jfi", "gif", "bmp", "ico", "jfif", "jpe", "pnz", "dib", "webp", "exr",
];
pub const HEIC_EXTENSIONS: &[&str] = &["heif", "heifs", "heic", "heics", "avci", "avcs", "hif"];
pub const ZIP_FILES_EXTENSIONS: &[&str] = &["zip", "jar"];
pub const PDF_FILES_EXTENSIONS: &[&str] = &["pdf"];
pub const AUDIO_FILES_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "wav", "ogg", "m4a", "aac", "aiff", "pcm", "aif", "aiff", "aifc", "m3a", "mp2", "mp4a", "mp2a", "mpga", "wave", "weba", "wma", "oga",
];
pub const VIDEO_FILES_EXTENSIONS: &[&str] = &[
    "mp4", "m4v", "mkv", "avi", "mov", "webm", "flv", "wmv", // Popular
    "mpeg", "mpg", "mp2", "mpe", "m2ts", "vob", "evo", // MPEG / broadcast, "ts"
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
