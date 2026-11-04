use std::path::Path;

pub const CATEGORY_COUNT: usize = 10;

/// Coarse file type used to colour disk usage charts and to filter them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum FileCategory {
    Folder,
    Audio,
    Video,
    Image,
    Archive,
    Document,
    Code,
    DiskImage,
    Binary,
    #[default]
    Other,
}

impl FileCategory {
    pub const ALL: [Self; CATEGORY_COUNT] = [
        Self::Folder,
        Self::Audio,
        Self::Video,
        Self::Image,
        Self::Archive,
        Self::Document,
        Self::Code,
        Self::DiskImage,
        Self::Binary,
        Self::Other,
    ];

    pub const fn index(self) -> usize {
        self as usize
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    /// Stable English name, meant for CLI and JSON output (GUIs translate it themselves).
    pub const fn name(self) -> &'static str {
        match self {
            Self::Folder => "Folder",
            Self::Audio => "Audio",
            Self::Video => "Video",
            Self::Image => "Image",
            Self::Archive => "Archive",
            Self::Document => "Document",
            Self::Code => "Code",
            Self::DiskImage => "Disk image",
            Self::Binary => "Binary",
            Self::Other => "Other",
        }
    }

    /// Default chart colour as RGB.
    pub const fn color(self) -> [u8; 3] {
        match self {
            Self::Folder => [105, 115, 135],
            Self::Audio => [240, 202, 87],
            Self::Video => [228, 87, 88],
            Self::Image => [174, 110, 247],
            Self::Archive => [224, 151, 78],
            Self::Document => [84, 160, 255],
            Self::Code => [72, 201, 176],
            Self::DiskImage => [76, 201, 240],
            Self::Binary => [120, 126, 255],
            Self::Other => [141, 153, 174],
        }
    }

    pub fn from_path(path: &Path) -> Self {
        path.extension().and_then(|extension| extension.to_str()).map_or(Self::Other, Self::from_extension)
    }

    pub fn from_extension(extension: &str) -> Self {
        match extension.to_ascii_lowercase().as_str() {
            "mp3" | "flac" | "wav" | "aac" | "ogg" | "opus" | "m4a" | "wma" | "aiff" | "alac" => Self::Audio,
            "mp4" | "mkv" | "avi" | "mov" | "webm" | "m4v" | "wmv" | "mpg" | "mpeg" | "ts" | "flv" => Self::Video,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif" | "svg" | "heic" | "avif" | "jxl" | "raw" | "cr2" | "nef" | "arw" | "dng" | "rw2" | "raf" | "orf"
            | "3fr" | "fff" | "pef" | "srw" | "rwl" | "x3f" => Self::Image,
            "zip" | "7z" | "rar" | "tar" | "gz" | "xz" | "bz2" | "zst" | "cab" | "lz4" => Self::Archive,
            "pdf" | "doc" | "docx" | "odt" | "rtf" | "txt" | "md" | "xls" | "xlsx" | "ods" | "ppt" | "pptx" | "odp" | "epub" => Self::Document,
            "rs" | "c" | "h" | "hpp" | "cpp" | "cc" | "py" | "js" | "tsx" | "jsx" | "java" | "kt" | "go" | "php" | "rb" | "swift" | "cs" | "toml" | "json" | "yaml" | "yml"
            | "xml" | "html" | "css" | "scss" | "sh" | "slint" | "gd" | "comp" | "vert" | "frag" | "geom" | "wasm" | "d" => Self::Code,
            "iso" | "img" | "dmg" | "vmdk" | "qcow2" | "vdi" | "vhd" | "vhdx" => Self::DiskImage,
            "exe" | "dll" | "so" | "bin" | "appimage" | "deb" | "rpm" | "msi" | "apk" | "msu" | "ocx" | "node" | "rlib" | "rmeta" | "o" | "a" | "obj" | "pdb" | "bc" | "class"
            | "pyc" => Self::Binary,
            _ => Self::Other,
        }
    }

    /// Reads the file header, so it is much slower than [`Self::from_path`]; use only for files whose extension is not recognized.
    pub fn from_content(path: &Path) -> Option<Self> {
        let kind = infer::get_from_path(path).ok().flatten()?;
        Self::from_mime(kind.mime_type())
    }

    fn from_mime(mime: &str) -> Option<Self> {
        let mime = mime.to_ascii_lowercase();
        if mime.starts_with("audio/") {
            Some(Self::Audio)
        } else if mime.starts_with("video/") {
            Some(Self::Video)
        } else if mime.starts_with("image/") {
            Some(Self::Image)
        } else if ["zip", "tar", "rar", "7z", "archive", "compressed"].iter().any(|part| mime.contains(part)) {
            Some(Self::Archive)
        } else if mime.starts_with("text/") || ["pdf", "document", "sheet", "presentation"].iter().any(|part| mime.contains(part)) {
            Some(Self::Document)
        } else if ["elf", "executable", "sharedlib", "mach-binary", "pe32"].iter().any(|part| mime.contains(part)) {
            Some(Self::Binary)
        } else {
            None
        }
    }
}
