use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::mem;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use crossbeam_channel::Receiver;
use fun_time::fun_time;
use futures::channel::mpsc::UnboundedSender;
use log::debug;
use mime_guess::get_mime_extensions;
use rayon::prelude::*;

use crate::common::{prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads};
use crate::common_dir_traversal::{CheckingMethod, DirTraversalBuilder, DirTraversalResult, FileEntry, ProgressData, ToolType};
use crate::common_tool::{CommonData, CommonToolData};
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
    // Games specific extensions - cannot be used here common extensions like zip
    ("gz", "h3m"),     // Heroes 3
    ("zip", "hashdb"), // Gog
    ("c2", "zip"),     // King of the Dark Age
    ("c2", "bmp"),     // King of the Dark Age
    ("c2", "avi"),     // King of the Dark Age
    ("c2", "exe"),     // King of the Dark Age
    // Other
    ("der", "keystore"),  // Godot/Android keystore
    ("exe", "pyd"),       // Python/Mingw
    ("gz", "blend"),      // Blender
    ("gz", "crate"),      // Cargo
    ("gz", "svgz"),       // Archive svg
    ("gz", "tgz"),        // Archive
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

#[derive(Default)]
pub struct Info {
    pub number_of_files_with_bad_extension: usize,
}

pub struct BadExtensions {
    common_data: CommonToolData,
    information: Info,
    files_to_check: Vec<FileEntry>,
    bad_extensions_files: Vec<BadFileEntry>,
    include_files_without_extension: bool,
}

impl BadExtensions {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::BadExtensions),
            information: Info::default(),
            files_to_check: Default::default(),
            bad_extensions_files: Default::default(),
            include_files_without_extension: true,
        }
    }

    #[fun_time(message = "find_bad_extensions_files")]
    pub fn find_bad_extensions_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.optimize_dirs_before_start();
        if !self.check_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        if !self.look_for_bad_extensions_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.debug_print();
    }

    #[fun_time(message = "check_files")]
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let result = DirTraversalBuilder::new()
            .root_dirs(self.common_data.directories.included_directories.clone())
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .minimal_file_size(self.common_data.minimal_file_size)
            .maximal_file_size(self.common_data.maximal_file_size)
            .directories(self.common_data.directories.clone())
            .allowed_extensions(self.common_data.allowed_extensions.clone())
            .excluded_items(self.common_data.excluded_items.clone())
            .recursive_search(self.common_data.recursive_search)
            .build()
            .run();
        debug!("check_files - collected files");
        let res = match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                if let Some(files_to_check) = grouped_file_entries.get(&()) {
                    self.files_to_check = files_to_check.clone();
                }
                self.common_data.text_messages.warnings.extend(warnings);

                true
            }
            DirTraversalResult::SuccessFolders { .. } => {
                unreachable!()
            }
            DirTraversalResult::Stopped => false,
        };
        res
    }

    #[fun_time(message = "look_for_bad_extensions_files")]
    fn look_for_bad_extensions_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 1, 1, self.files_to_check.len(), CheckingMethod::None, self.get_cd().tool_type);

        let files_to_check = mem::take(&mut self.files_to_check);

        let mut hashmap_workarounds: HashMap<&str, Vec<&str>> = Default::default();
        for (proper, found) in WORKAROUNDS {
            // This should be enabled when items will have only 1 possible workaround items, but looks that some have 2 or even more, so at least for now this is disabled
            // if hashmap_workarounds.contains_key(found) {
            //     panic!("Already have {} key", found);
            // }
            hashmap_workarounds.entry(found).or_default().push(proper);
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

    #[fun_time(message = "verify_extensions")]
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
    fn debug_print(&self) {
        #[cfg(not(debug_assertions))]
        {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        self.debug_print_common();
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
                self.common_data.text_messages.errors.push(format!("Failed to create file {file_name}, reason {e}"));
                return false;
            }
        };
        let mut writer = BufWriter::new(file_handler);

        if let Err(e) = writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories, self.common_data.directories.excluded_directories, self.common_data.excluded_items.items
        ) {
            self.common_data
                .text_messages
                .errors
                .push(format!("Failed to save results to file {file_name}, reason {e}"));
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
    fn print_results(&self) {
        println!("Found {} files with invalid extension.\n", self.information.number_of_files_with_bad_extension);
        for file_entry in &self.bad_extensions_files {
            println!("{} ----- {}", file_entry.path.display(), file_entry.proper_extensions);
        }
    }
}

impl BadExtensions {
    pub const fn get_bad_extensions_files(&self) -> &Vec<BadFileEntry> {
        &self.bad_extensions_files
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}

impl CommonData for BadExtensions {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
}
