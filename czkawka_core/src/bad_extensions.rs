use std::collections::{BTreeSet, HashMap};
use std::io::prelude::*;
use std::mem;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender};
use fun_time::fun_time;
use log::debug;
use mime_guess::get_mime_extensions;
use rayon::prelude::*;
use serde::Serialize;

use crate::common::{check_if_stop_received, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads};
use crate::common_dir_traversal::{DirTraversalBuilder, DirTraversalResult, FileEntry, ToolType};
use crate::common_tool::{CommonData, CommonToolData};
use crate::common_traits::*;
use crate::progress_data::{CurrentStage, ProgressData};

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
    ("zip", "c2"),     // King of the Dark Age
    ("bmp", "c2"),     // King of the Dark Age
    ("avi", "c2"),     // King of the Dark Age
    ("exe", "c2"),     // King of the Dark Age
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
    ("zip", "cbr"),       // Comics
    ("zip", "dat"),       // Multiple - python, brave
    ("zip", "doc"),       // Word
    ("zip", "docx"),      // Word
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

#[derive(Clone, Serialize, Debug)]
pub struct BadFileEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub current_extension: String,
    pub proper_extensions_group: String,
    pub proper_extension: String,
}

impl ResultEntry for BadFileEntry {
    fn get_path(&self) -> &Path {
        &self.path
    }
    fn get_modified_date(&self) -> u64 {
        self.modified_date
    }
    fn get_size(&self) -> u64 {
        self.size
    }
}

#[derive(Default)]
pub struct Info {
    pub number_of_files_with_bad_extension: usize,
}

pub struct BadExtensionsParameters {
    pub include_files_without_extension: bool,
}

impl BadExtensionsParameters {
    pub fn new() -> Self {
        Self {
            include_files_without_extension: false,
        } // TODO add option to all modes
    }
}
impl Default for BadExtensionsParameters {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BadExtensions {
    common_data: CommonToolData,
    information: Info,
    files_to_check: Vec<FileEntry>,
    bad_extensions_files: Vec<BadFileEntry>,
    params: BadExtensionsParameters,
}

impl BadExtensions {
    pub fn new(params: BadExtensionsParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::BadExtensions),
            information: Info::default(),
            files_to_check: Default::default(),
            bad_extensions_files: Default::default(),
            params,
        }
    }

    #[fun_time(message = "find_bad_extensions_files", level = "info")]
    pub fn find_bad_extensions_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) {
        self.prepare_items();
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

    #[fun_time(message = "check_files", level = "debug")]
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.files_to_check = grouped_file_entries.into_values().flatten().collect();
                self.common_data.text_messages.warnings.extend(warnings);

                true
            }

            DirTraversalResult::Stopped => false,
        }
    }

    #[fun_time(message = "look_for_bad_extensions_files", level = "debug")]
    fn look_for_bad_extensions_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        if self.files_to_check.is_empty() {
            return true;
        }

        let (progress_thread_handle, progress_thread_run, atomic_counter, check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::BadExtensionsChecking, self.files_to_check.len(), self.get_test_type());

        let files_to_check = mem::take(&mut self.files_to_check);

        let mut hashmap_workarounds: HashMap<&str, Vec<&str>> = Default::default();
        for (proper, found) in WORKAROUNDS {
            hashmap_workarounds.entry(found).or_default().push(proper);
        }

        self.bad_extensions_files = self.verify_extensions(files_to_check, &atomic_counter, stop_receiver, &check_was_stopped, &hashmap_workarounds);

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        // Break if stop was clicked
        if check_was_stopped.load(Ordering::Relaxed) {
            return false;
        }

        self.information.number_of_files_with_bad_extension = self.bad_extensions_files.len();

        debug!("Found {} files with invalid extension.", self.information.number_of_files_with_bad_extension);

        true
    }

    #[fun_time(message = "verify_extensions", level = "debug")]
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
                if check_if_stop_received(stop_receiver) {
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
                    if !self.params.include_files_without_extension {
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
                    proper_extensions_group: valid_extensions,
                    proper_extension: proper_extension.to_string(),
                }))
            })
            .while_some()
            .flatten()
            .collect::<Vec<_>>()
    }

    #[allow(clippy::unused_self)]
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
        // TODO Isn't this a bug?
        // Why to file without extensions we set this as empty
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

impl DebugPrint for BadExtensions {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for BadExtensions {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories,
            self.common_data.directories.excluded_directories,
            self.common_data.excluded_items.get_excluded_items()
        )?;
        writeln!(writer, "Found {} files with invalid extension.\n", self.information.number_of_files_with_bad_extension)?;

        for file_entry in &self.bad_extensions_files {
            writeln!(writer, "\"{}\" ----- {}", file_entry.path.to_string_lossy(), file_entry.proper_extensions_group)?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.bad_extensions_files, pretty_print)
    }
}

impl BadExtensions {
    pub const fn get_bad_extensions_files(&self) -> &Vec<BadFileEntry> {
        &self.bad_extensions_files
    }

    pub fn get_params(&self) -> &BadExtensionsParameters {
        &self.params
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
