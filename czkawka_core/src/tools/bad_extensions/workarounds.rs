pub(crate) const DISABLED_EXTENSIONS: &[&str] = &["file", "cache", "bak", "data", "tmp"]; // Such files can have any type inside

// This adds several workarounds for bugs/invalid recognizing types by external libraries
// ("real_content_extension", "current_file_extension")
pub(crate) const WORKAROUNDS: &[(&str, &str)] = &[
    // Wine/Windows
    ("der", "cat"),
    ("exe", "acm"),
    ("exe", "ax"),
    ("exe", "bck"),
    ("exe", "com"),
    ("exe", "cpl"),
    ("exe", "dll16"),
    ("exe", "dll"),
    ("exe", "drv16"),
    ("exe", "drv"),
    ("exe", "ds"),
    ("exe", "efi"),
    ("exe", "exe16"),
    ("exe", "fon"), // Type of font or something else
    ("exe", "mod16"),
    ("exe", "msstyles"),
    ("exe", "mui"),
    ("exe", "mun"),
    ("exe", "orig"),
    ("exe", "ps1xml"),
    ("exe", "rll"),
    ("exe", "rs"),
    ("exe", "scr"),
    ("exe", "signed"),
    ("exe", "sys"),
    ("exe", "tlb"),
    ("exe", "tsp"),
    ("exe", "vdm"),
    ("exe", "vxd"),
    ("exe", "winmd"),
    ("gz", "loggz"),
    ("xml", "adml"),
    ("xml", "admx"),
    ("xml", "camp"),
    ("xml", "cdmp"),
    ("xml", "cdxml"),
    ("xml", "dgml"),
    ("xml", "diagpkg"),
    ("xml", "gmmp"),
    ("xml", "library-ms"),
    ("xml", "man"),
    ("xml", "manifest"),
    ("xml", "msc"),
    ("xml", "mum"),
    ("xml", "resx"),
    ("zip", "wmz"),
    // Games specific extensions - cannot be used here common extensions like zip
    ("gz", "h3m"),     // Heroes 3
    ("zip", "hashdb"), // Gog
    ("c2", "zip"),     // King of the Dark Age
    ("c2", "bmp"),     // King of the Dark Age
    ("c2", "avi"),     // King of the Dark Age
    ("c2", "exe"),     // King of the Dark Age
    // Raw images
    ("tif", "nef"),
    ("tif", "dng"),
    ("tif", "arw"),
    // Other
    ("der", "keystore"),  // Godot/Android keystore
    ("exe", "pyd"),       // Python/Mingw
    ("gz", "blend"),      // Blender
    ("gz", "crate"),      // Cargo
    ("gz", "svgz"),       // Archive svg
    ("gz", "tgz"),        // Archive
    ("heic", "heif"),     // Image
    ("heif", "heic"),     // Image
    ("html", "dtd"),      // Mingw
    ("html", "ent"),      // Mingw
    ("html", "md"),       // Markdown
    ("html", "svelte"),   // Svelte
    ("jpg", "jfif"),      // Photo format
    ("m4v", "mp4"),       // m4v and mp4 are interchangeable
    ("mobi", "azw3"),     // Ebook format
    ("mpg", "vob"),       // Weddings in parts have usually vob extension
    ("obj", "bin"),       // Multiple apps, Czkawka, Nvidia, Windows
    ("obj", "o"),         // Compilators
    ("odp", "otp"),       // LibreOffice
    ("ods", "ots"),       // Libreoffice
    ("odt", "ott"),       // Libreoffice
    ("ogg", "ogv"),       // Audio format
    ("pem", "key"),       // curl, openssl
    ("png", "kpp"),       // Krita presets
    ("pptx", "ppsx"),     // Powerpoint
    ("sh", "bash"),       // Linux
    ("sh", "guess"),      // GNU
    ("sh", "lua"),        // Lua
    ("sh", "js"),         // Javascript
    ("sh", "pl"),         // Gnome/Linux
    ("sh", "pm"),         // Gnome/Linux
    ("sh", "py"),         // Python
    ("sh", "pyx"),        // Python
    ("sh", "rs"),         // Rust
    ("sh", "sample"),     // Git
    ("xml", "bsp"),       // Quartus
    ("xml", "cbp"),       // CodeBlocks config
    ("xml", "cfg"),       // Multiple apps - Godot
    ("xml", "cmb"),       // Cambalache
    ("xml", "conf"),      // Multiple apps - Python
    ("xml", "config"),    // Multiple apps - QT Creator
    ("xml", "dae"),       // 3D models
    ("xml", "docbook"),   //
    ("xml", "fb2"),       //
    ("xml", "filters"),   // Visual studio
    ("xml", "gir"),       // GTK
    ("xml", "glade"),     // Glade
    ("xml", "iml"),       // Intelij Idea
    ("xml", "kdenlive"),  // KDenLive
    ("xml", "lang"),      // ?
    ("xml", "nuspec"),    // Nuget
    ("xml", "policy"),    // SystemD
    ("xml", "qsys"),      // Quartus
    ("xml", "sopcinfo"),  // Quartus
    ("xml", "svg"),       // SVG
    ("xml", "ui"),        // Cambalache, Glade
    ("xml", "user"),      // Qtcreator
    ("xml", "vbox"),      // VirtualBox
    ("xml", "vbox-prev"), // VirtualBox
    ("xml", "vcproj"),    // VisualStudio
    ("xml", "vcxproj"),   // VisualStudio
    ("xml", "xba"),       // Libreoffice
    ("xml", "xcd"),       // Libreoffice files
    ("zip", "apk"),       // Android apk
    ("zip", "cbz"),       // Comics
    ("zip", "dat"),       // Multiple - python, brave
    ("zip", "doc"),       // Word
    ("zip", "docx"),      // Word
    ("zip", "epub"),      // Ebook format
    ("zip", "jar"),       // Java
    ("zip", "kra"),       // Krita
    ("zip", "kgm"),       // Krita
    ("zip", "nupkg"),     // Nuget packages
    ("zip", "odg"),       // Libreoffice
    ("zip", "pptx"),      // Powerpoint
    ("zip", "whl"),       // Python packages
    ("zip", "xlsx"),      // Excel
    ("zip", "xpi"),       // Firefox extensions
    ("zip", "zcos"),      // Scilab
    // Probably invalid
    ("html", "svg"),
    ("xml", "html"),
    // Probably bug in external library
    ("msi", "ppt"), // Not sure why ppt is not recognized
    ("msi", "doc"), // Not sure why doc is not recognized
    ("exe", "xls"), // Not sure why xls is not recognized
];
