pub const DEFAULT_THREAD_SIZE: usize = 8 * 1024 * 1024; // 8 MB
pub const DEFAULT_WORKER_THREAD_SIZE: usize = 4 * 1024 * 1024; // 4 MB

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
pub const HEIC_EXTENSIONS: &[&str] = &["heif", "heifs", "heic", "heics", "avci", "avcs"];
pub const ZIP_FILES_EXTENSIONS: &[&str] = &["zip", "jar"];
pub const PDF_FILES_EXTENSIONS: &[&str] = &["pdf"];
pub const AUDIO_FILES_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "wav", "ogg", "m4a", "aac", "aiff", "pcm", "aif", "aiff", "aifc", "m3a", "mp2", "mp4a", "mp2a", "mpga", "wave", "weba", "wma", "oga",
];
pub const VIDEO_FILES_EXTENSIONS: &[&str] = &[
    "mp4", "mpv", "flv", "mp4a", "webm", "mpg", "mp2", "mpeg", "m4p", "m4v", "avi", "wmv", "qt", "mov", "swf", "mkv", "mts", "m2ts",
];
