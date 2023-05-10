use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::mem;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use crossbeam_channel::Receiver;
use futures::channel::mpsc::UnboundedSender;
use mime_guess::get_mime_extensions;
use rayon::prelude::*;

use crate::common::{prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads};
use crate::common_dir_traversal::{CheckingMethod, DirTraversalBuilder, DirTraversalResult, FileEntry, ProgressData, ToolType};
use crate::common_directory::Directories;
use crate::common_extensions::Extensions;
use crate::common_items::ExcludedItems;
use crate::common_messages::Messages;
use crate::common_traits::*;

static DISABLED_EXTENSIONS: &[&str] = &["file", "cache", "bak", "data"]; // Such files can have any type inside

// This adds several workarounds for bugs/invalid recognizing types by external libraries
// ("real_content_extension", "current_file_extension")
const WORKAROUNDS: &[(&str, &str)] = &[
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
    // Other
    ("exe", "pyd"),       // Python/Mingw
    ("gz", "blend"),      // Blender
    ("gz", "crate"),      // Cargo
    ("gz", "svgz"),       // Archive svg
    ("gz", "tgz"),        // Archive
    ("html", "dtd"),      // Mingw
    ("html", "ent"),      // Mingw
    ("html", "md"),       // Markdown
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
    ("pptx", "ppsx"),     // Powerpoint
    ("sh", "bash"),       // Linux
    ("sh", "guess"),      // GNU
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
    ("zip", "cbr"),       // Comics
    ("zip", "dat"),       // Multiple - python, brave
    ("zip", "doc"),       // Word
    ("zip", "docx"),      // Word
    ("zip", "jar"),       // Java
    ("zip", "kra"),       // Krita
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

#[derive(Clone)]
pub struct BadFileEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub current_extension: String,
    pub proper_extensions: String,
}

/// Info struck with helpful information's about results
#[derive(Default)]
pub struct Info {
    pub number_of_files_with_bad_extension: usize,
}

impl Info {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }
}

pub struct BadExtensions {
    tool_type: ToolType,
    text_messages: Messages,
    information: Info,
    files_to_check: Vec<FileEntry>,
    bad_extensions_files: Vec<BadFileEntry>,
    directories: Directories,
    allowed_extensions: Extensions,
    excluded_items: ExcludedItems,
    minimal_file_size: u64,
    maximal_file_size: u64,
    recursive_search: bool,
    stopped_search: bool,
    save_also_as_json: bool,
    include_files_without_extension: bool,
}

impl BadExtensions {
    #[must_use]
    pub fn new() -> Self {
        Self {
            tool_type: ToolType::BadExtensions,
            text_messages: Messages::new(),
            information: Info::new(),
            recursive_search: true,
            allowed_extensions: Extensions::new(),
            directories: Directories::new(),
            excluded_items: ExcludedItems::new(),
            files_to_check: Default::default(),
            stopped_search: false,
            minimal_file_size: 8192,
            maximal_file_size: u64::MAX,
            bad_extensions_files: Default::default(),
            save_also_as_json: false,
            include_files_without_extension: true,
        }
    }

    pub fn find_bad_extensions_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.directories.optimize_directories(self.recursive_search, &mut self.text_messages);
        if !self.check_files(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        if !self.look_for_bad_extensions_files(stop_receiver, progress_sender) {
            self.stopped_search = true;
            return;
        }
        self.debug_print();
    }

    #[must_use]
    pub fn get_stopped_search(&self) -> bool {
        self.stopped_search
    }

    #[must_use]
    pub const fn get_bad_extensions_files(&self) -> &Vec<BadFileEntry> {
        &self.bad_extensions_files
    }

    pub fn set_maximal_file_size(&mut self, maximal_file_size: u64) {
        self.maximal_file_size = match maximal_file_size {
            0 => 1,
            t => t,
        };
    }
    pub fn set_minimal_file_size(&mut self, minimal_file_size: u64) {
        self.minimal_file_size = match minimal_file_size {
            0 => 1,
            t => t,
        };
    }
    #[cfg(target_family = "unix")]
    pub fn set_exclude_other_filesystems(&mut self, exclude_other_filesystems: bool) {
        self.directories.set_exclude_other_filesystems(exclude_other_filesystems);
    }
    #[cfg(not(target_family = "unix"))]
    pub fn set_exclude_other_filesystems(&mut self, _exclude_other_filesystems: bool) {}

    #[must_use]
    pub const fn get_text_messages(&self) -> &Messages {
        &self.text_messages
    }

    #[must_use]
    pub const fn get_information(&self) -> &Info {
        &self.information
    }

    pub fn set_save_also_as_json(&mut self, save_also_as_json: bool) {
        self.save_also_as_json = save_also_as_json;
    }

    pub fn set_recursive_search(&mut self, recursive_search: bool) {
        self.recursive_search = recursive_search;
    }

    pub fn set_included_directory(&mut self, included_directory: Vec<PathBuf>) -> bool {
        self.directories.set_included_directory(included_directory, &mut self.text_messages)
    }

    pub fn set_excluded_directory(&mut self, excluded_directory: Vec<PathBuf>) {
        self.directories.set_excluded_directory(excluded_directory, &mut self.text_messages);
    }
    pub fn set_allowed_extensions(&mut self, allowed_extensions: String) {
        self.allowed_extensions.set_allowed_extensions(allowed_extensions, &mut self.text_messages);
    }

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) {
        self.excluded_items.set_excluded_items(excluded_items, &mut self.text_messages);
    }

    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let result = DirTraversalBuilder::new()
            .root_dirs(self.directories.included_directories.clone())
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .minimal_file_size(self.minimal_file_size)
            .maximal_file_size(self.maximal_file_size)
            .directories(self.directories.clone())
            .allowed_extensions(self.allowed_extensions.clone())
            .excluded_items(self.excluded_items.clone())
            .recursive_search(self.recursive_search)
            .build()
            .run();
        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                if let Some(files_to_check) = grouped_file_entries.get(&()) {
                    self.files_to_check = files_to_check.clone();
                }
                self.text_messages.warnings.extend(warnings);

                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        }
    }

    fn look_for_bad_extensions_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 1, 1, self.files_to_check.len(), CheckingMethod::None, self.tool_type);

        let files_to_check = mem::take(&mut self.files_to_check);

        let mut hashmap_workarounds: HashMap<&str, Vec<&str>> = Default::default();
        for (proper, found) in WORKAROUNDS {
            // This should be enabled when items will have only 1 possible workaround items, but looks that some have 2 or even more, so at least for now this is disabled
            // if hashmap_workarounds.contains_key(found) {
            //     panic!("Already have {} key", found);
            // }
            hashmap_workarounds.entry(found).or_insert_with(Vec::new).push(proper);
        }

        self.bad_extensions_files = self.verify_extensions(files_to_check, &atomic_counter, stop_receiver, &check_was_stopped, &hashmap_workarounds);

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        // Break if stop was clicked
        if check_was_stopped.load(Ordering::Relaxed) {
            return false;
        }

        self.information.number_of_files_with_bad_extension = self.bad_extensions_files.len();

        // Clean unused data
        self.files_to_check = Default::default();

        true
    }

    fn verify_extensions(
        &self,
        files_to_check: Vec<FileEntry>,
        atomic_counter: &Arc<AtomicUsize>,
        stop_receiver: Option<&Receiver<()>>,
        check_was_stopped: &AtomicBool,
        hashmap_workarounds: &HashMap<&str, Vec<&str>>,
    ) -> Vec<BadFileEntry> {
        files_to_check
            .into_par_iter()
            .map(|file_entry| {
                atomic_counter.fetch_add(1, Ordering::Relaxed);
                if stop_receiver.is_some() && stop_receiver.unwrap().try_recv().is_ok() {
                    check_was_stopped.store(true, Ordering::Relaxed);
                    return None;
                }

                // Check what exactly content file contains
                let kind = match infer::get_from_path(&file_entry.path) {
                    Ok(k) => match k {
                        Some(t) => t,
                        None => return Some(None),
                    },
                    Err(_) => return Some(None),
                };
                let proper_extension = kind.extension();

                let Some(current_extension) = self.get_and_validate_extension(&file_entry, proper_extension) else {
                    return Some(None);
                };

                // Check for all extensions that file can use(not sure if it is worth to do it)
                let (mut all_available_extensions, valid_extensions) = self.check_for_all_extensions_that_file_can_use(hashmap_workarounds, &current_extension, proper_extension);

                if all_available_extensions.is_empty() {
                    // Not found any extension
                    return Some(None);
                } else if current_extension.is_empty() {
                    if !self.include_files_without_extension {
                        return Some(None);
                    }
                } else if all_available_extensions.take(&current_extension).is_some() {
                    // Found proper extension
                    return Some(None);
                }

                Some(Some(BadFileEntry {
                    path: file_entry.path,
                    modified_date: file_entry.modified_date,
                    size: file_entry.size,
                    current_extension,
                    proper_extensions: valid_extensions,
                }))
            })
            .while_some()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect::<Vec<_>>()
    }

    fn get_and_validate_extension(&self, file_entry: &FileEntry, proper_extension: &str) -> Option<String> {
        let current_extension;
        // Extract current extension from file
        if let Some(extension) = file_entry.path.extension() {
            let extension = extension.to_string_lossy().to_lowercase();
            if DISABLED_EXTENSIONS.contains(&extension.as_str()) {
                return None;
            }
            // Text longer than 10 characters is not considered as extension
            if extension.len() > 10 {
                current_extension = String::new();
            } else {
                current_extension = extension;
            }
        } else {
            current_extension = String::new();
        }

        // Already have proper extension, no need to do more things
        if current_extension == proper_extension {
            return None;
        }
        Some(current_extension)
    }

    fn check_for_all_extensions_that_file_can_use(
        &self,
        hashmap_workarounds: &HashMap<&str, Vec<&str>>,
        current_extension: &str,
        proper_extension: &str,
    ) -> (BTreeSet<String>, String) {
        let mut all_available_extensions: BTreeSet<String> = Default::default();
        let valid_extensions = if current_extension.is_empty() {
            String::new()
        } else {
            for mim in mime_guess::from_ext(proper_extension) {
                if let Some(all_ext) = get_mime_extensions(&mim) {
                    for ext in all_ext {
                        all_available_extensions.insert((*ext).to_string());
                    }
                }
            }

            // Workarounds
            if let Some(vec_pre) = hashmap_workarounds.get(current_extension) {
                for pre in vec_pre {
                    if all_available_extensions.contains(*pre) {
                        all_available_extensions.insert(current_extension.to_string());
                        break;
                    }
                }
            }

            let mut guessed_multiple_extensions = format!("({proper_extension}) - ");
            for ext in &all_available_extensions {
                guessed_multiple_extensions.push_str(ext);
                guessed_multiple_extensions.push(',');
            }
            guessed_multiple_extensions.pop();

            guessed_multiple_extensions
        };
        (all_available_extensions, valid_extensions)
    }
}

impl Default for BadExtensions {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for BadExtensions {
    #[allow(dead_code)]
    #[allow(unreachable_code)]
    /// Debugging printing - only available on debug build
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("### Information's");

        println!("Errors size - {}", self.text_messages.errors.len());
        println!("Warnings size - {}", self.text_messages.warnings.len());
        println!("Messages size - {}", self.text_messages.messages.len());

        println!("### Other");

        println!("Excluded items - {:?}", self.excluded_items.items);
        println!("Included directories - {:?}", self.directories.included_directories);
        println!("Excluded directories - {:?}", self.directories.excluded_directories);
        println!("Recursive search - {}", self.recursive_search);
        println!("-----------------------------------------");
    }
}

impl SaveResults for BadExtensions {
    fn save_results_to_file(&mut self, file_name: &str) -> bool {
        let file_name: String = match file_name {
            "" => "results.txt".to_string(),
            k => k.to_string(),
        };

        let file_handler = match File::create(&file_name) {
            Ok(t) => t,
            Err(e) => {
                self.text_messages.errors.push(format!("Failed to create file {file_name}, reason {e}"));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.directories.included_directories, self.directories.excluded_directories, self.excluded_items.items
        ) {
            self.text_messages.errors.push(format!("Failed to save results to file {file_name}, reason {e}"));
            return false;
        }

        if !self.bad_extensions_files.is_empty() {
            writeln!(writer, "Found {} files with invalid extension.", self.information.number_of_files_with_bad_extension).unwrap();
            for file_entry in &self.bad_extensions_files {
                writeln!(writer, "{} ----- {}", file_entry.path.display(), file_entry.proper_extensions).unwrap();
            }
        } else {
            write!(writer, "Not found any files with invalid extension.").unwrap();
        }

        true
    }
}

impl PrintResults for BadExtensions {
    /// Print information's about duplicated entries
    /// Only needed for CLI
    fn print_results(&self) {
        println!("Found {} files with invalid extension.\n", self.information.number_of_files_with_bad_extension);
        for file_entry in &self.bad_extensions_files {
            println!("{} ----- {}", file_entry.path.display(), file_entry.proper_extensions);
        }
    }
}
