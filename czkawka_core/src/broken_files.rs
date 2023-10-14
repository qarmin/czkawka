use std::collections::BTreeMap;
use std::fs::{DirEntry, File, Metadata};
use std::io::prelude::*;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{fs, mem, panic};

use crossbeam_channel::Receiver;
use fun_time::fun_time;
use futures::channel::mpsc::UnboundedSender;
use log::debug;
use pdf::file::FileOptions;
use pdf::object::ParseOptions;
use pdf::PdfError;
use pdf::PdfError::Try;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::{
    check_folder_children, check_if_stop_received, create_crash_message, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads, AUDIO_FILES_EXTENSIONS,
    IMAGE_RS_BROKEN_FILES_EXTENSIONS, PDF_FILES_EXTENSIONS, ZIP_FILES_EXTENSIONS,
};
use crate::common_cache::{get_broken_files_cache_file, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common_dir_traversal::{common_get_entry_data_metadata, common_read_dir, get_lowercase_name, get_modified_time, CheckingMethod, ProgressData, ToolType};
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub type_of_file: TypeOfFile,
    pub error_string: String,
}
impl ResultEntry for FileEntry {
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

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum TypeOfFile {
    Unknown = -1,
    Image = 0,
    ArchiveZip,
    Audio,
    PDF,
}

bitflags! {
    #[derive(PartialEq, Copy, Clone)]
    pub struct CheckedTypes : u32 {
        const NONE = 0;

        const PDF = 0b1;
        const AUDIO = 0b10;
        const IMAGE = 0b100;
        const ARCHIVE = 0b1000;
    }
}

#[derive(Default)]
pub struct Info {
    pub number_of_broken_files: usize,
}

pub struct BrokenFiles {
    common_data: CommonToolData,
    information: Info,
    files_to_check: BTreeMap<String, FileEntry>,
    broken_files: Vec<FileEntry>,
    checked_types: CheckedTypes,
}

impl BrokenFiles {
    pub fn new() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::BrokenFiles),
            information: Info::default(),
            files_to_check: Default::default(),
            broken_files: Default::default(),
            checked_types: CheckedTypes::PDF | CheckedTypes::AUDIO | CheckedTypes::IMAGE | CheckedTypes::ARCHIVE,
        }
    }

    #[fun_time(message = "find_broken_files", level = "info")]
    pub fn find_broken_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) {
        self.optimize_dirs_before_start();
        if !self.check_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        if !self.look_for_broken_files(stop_receiver, progress_sender) {
            self.common_data.stopped_search = true;
            return;
        }
        self.delete_files();
        self.debug_print();
    }

    #[fun_time(message = "check_files", level = "debug")]
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let mut folders_to_check: Vec<PathBuf> = Vec::with_capacity(1024 * 2); // This should be small enough too not see to big difference and big enough to store most of paths without needing to resize vector

        // Add root folders for finding
        for id in &self.common_data.directories.included_directories {
            folders_to_check.push(id.clone());
        }

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 0, 1, 0, CheckingMethod::None, self.common_data.tool_type);

        debug!("check_files - starting to collect files");
        while !folders_to_check.is_empty() {
            if check_if_stop_received(stop_receiver) {
                send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
                return false;
            }

            let segments: Vec<_> = folders_to_check
                .par_iter()
                .map(|current_folder| {
                    let mut dir_result = vec![];
                    let mut warnings = vec![];
                    let mut fe_result = vec![];

                    let Some(read_dir) = common_read_dir(current_folder, &mut warnings) else {
                        return (dir_result, warnings, fe_result);
                    };

                    // Check every sub folder/file/link etc.
                    for entry in read_dir {
                        let Some((entry_data, metadata)) = common_get_entry_data_metadata(&entry, &mut warnings, current_folder) else {
                            continue;
                        };

                        if metadata.is_dir() {
                            check_folder_children(
                                &mut dir_result,
                                &mut warnings,
                                current_folder,
                                entry_data,
                                self.common_data.recursive_search,
                                &self.common_data.directories,
                                &self.common_data.excluded_items,
                            );
                        } else if metadata.is_file() {
                            if let Some(file_entry) = self.get_file_entry(&metadata, &atomic_counter, entry_data, &mut warnings, current_folder) {
                                fe_result.push((file_entry.path.to_string_lossy().to_string(), file_entry));
                            }
                        }
                    }
                    (dir_result, warnings, fe_result)
                })
                .collect();
            debug!("check_files - collected files");

            // Advance the frontier
            folders_to_check.clear();

            // Process collected data
            for (segment, warnings, fe_result) in segments {
                folders_to_check.extend(segment);
                self.common_data.text_messages.warnings.extend(warnings);
                for (name, fe) in fe_result {
                    self.files_to_check.insert(name, fe);
                }
            }
        }

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);
        true
    }

    fn get_file_entry(
        &self,
        metadata: &Metadata,
        atomic_counter: &Arc<AtomicUsize>,
        entry_data: &DirEntry,
        warnings: &mut Vec<String>,
        current_folder: &Path,
    ) -> Option<FileEntry> {
        atomic_counter.fetch_add(1, Ordering::Relaxed);

        let Some(file_name_lowercase) = get_lowercase_name(entry_data, warnings) else {
            return None;
        };

        if !self.common_data.allowed_extensions.matches_filename(&file_name_lowercase) {
            return None;
        }

        let type_of_file = check_extension_availability(&file_name_lowercase);

        if !check_if_file_extension_is_allowed(&type_of_file, &self.checked_types) {
            return None;
        }

        let current_file_name = current_folder.join(entry_data.file_name());
        if self.common_data.excluded_items.is_excluded(&current_file_name) {
            return None;
        }

        let fe: FileEntry = FileEntry {
            path: current_file_name.clone(),
            modified_date: get_modified_time(metadata, warnings, &current_file_name, false),
            size: metadata.len(),
            type_of_file,
            error_string: String::new(),
        };
        Some(fe)
    }

    fn check_broken_image(&self, mut file_entry: FileEntry) -> Option<FileEntry> {
        let mut file_entry_clone = file_entry.clone();

        let result = panic::catch_unwind(|| {
            if let Err(e) = image::open(&file_entry.path) {
                let error_string = e.to_string();
                // This error is a problem with image library, remove check when https://github.com/image-rs/jpeg-decoder/issues/130 will be fixed
                if error_string.contains("spectral selection is not allowed in non-progressive scan") {
                    return None;
                }
                file_entry.error_string = error_string;
            }
            Some(file_entry)
        });

        // If image crashed during opening, needs to be printed info about crashes thing
        if let Ok(image_result) = result {
            image_result
        } else {
            let message = create_crash_message("Image-rs", &file_entry_clone.path.to_string_lossy(), "https://github.com/Serial-ATA/lofty-rs");
            println!("{message}");
            file_entry_clone.error_string = message;
            Some(file_entry_clone)
        }
    }
    fn check_broken_zip(&self, mut file_entry: FileEntry) -> Option<FileEntry> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                if let Err(e) = zip::ZipArchive::new(file) {
                    file_entry.error_string = e.to_string();
                }
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }
    fn check_broken_audio(&self, mut file_entry: FileEntry) -> Option<FileEntry> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                let mut file_entry_clone = file_entry.clone();

                let result = panic::catch_unwind(|| {
                    if let Err(e) = audio_checker::parse_audio_file(file) {
                        file_entry.error_string = e.to_string();
                    }
                    Some(file_entry)
                });

                if let Ok(audio_result) = result {
                    audio_result
                } else {
                    let message = create_crash_message("Symphonia", &file_entry_clone.path.to_string_lossy(), "https://github.com/pdeljanov/Symphonia");
                    println!("{message}");
                    file_entry_clone.error_string = message;
                    Some(file_entry_clone)
                }
            }
            Err(_inspected) => None,
        }
    }
    fn check_broken_pdf(&self, mut file_entry: FileEntry) -> Option<FileEntry> {
        let parser_options = ParseOptions::tolerant(); // Only show as broken files with really big bugs

        let mut file_entry_clone = file_entry.clone();
        let result = panic::catch_unwind(|| {
            match FileOptions::cached().parse_options(parser_options).open(&file_entry.path) {
                Ok(file) => {
                    for idx in 0..file.num_pages() {
                        if let Err(e) = file.get_page(idx) {
                            let err = validate_pdf_error(&mut file_entry, e);
                            if let PdfError::InvalidPassword = err {
                                return None;
                            }
                            break;
                        }
                    }
                }
                Err(e) => {
                    if let PdfError::Io { .. } = e {
                        return None;
                    }
                    let err = validate_pdf_error(&mut file_entry, e);
                    if let PdfError::InvalidPassword = err {
                        return None;
                    }
                }
            }
            Some(file_entry)
        });
        if let Ok(pdf_result) = result {
            pdf_result
        } else {
            let message = create_crash_message("PDF-rs", &file_entry_clone.path.to_string_lossy(), "https://github.com/pdf-rs/pdf");
            println!("{message}");
            file_entry_clone.error_string = message;
            Some(file_entry_clone)
        }
    }

    #[fun_time(message = "load_cache", level = "debug")]
    fn load_cache(&mut self) -> (BTreeMap<String, FileEntry>, BTreeMap<String, FileEntry>, BTreeMap<String, FileEntry>) {
        let loaded_hash_map;

        let mut records_already_cached: BTreeMap<String, FileEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, FileEntry> = Default::default();
        let files_to_check = mem::take(&mut self.files_to_check);

        if self.common_data.use_cache {
            let (messages, loaded_items) = load_cache_from_file_generalized_by_path::<FileEntry>(&get_broken_files_cache_file(), self.get_delete_outdated_cache(), &files_to_check);
            self.get_text_messages_mut().extend_with_another_messages(messages);
            loaded_hash_map = loaded_items.unwrap_or_default();

            for (name, file_entry) in files_to_check {
                if let Some(cached_file_entry) = loaded_hash_map.get(&name) {
                    records_already_cached.insert(name.clone(), cached_file_entry.clone());
                } else {
                    non_cached_files_to_check.insert(name, file_entry);
                }
            }
        } else {
            loaded_hash_map = Default::default();
            non_cached_files_to_check = files_to_check;
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    #[fun_time(message = "look_for_broken_files", level = "debug")]
    fn look_for_broken_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&UnboundedSender<ProgressData>>) -> bool {
        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache();

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, 1, 1, non_cached_files_to_check.len(), CheckingMethod::None, self.common_data.tool_type);

        debug!("look_for_broken_files - started finding for broken files");
        let mut vec_file_entry: Vec<FileEntry> = non_cached_files_to_check
            .into_par_iter()
            .map(|(_, file_entry)| {
                atomic_counter.fetch_add(1, Ordering::Relaxed);
                if check_if_stop_received(stop_receiver) {
                    return None;
                }

                match file_entry.type_of_file {
                    TypeOfFile::Image => Some(self.check_broken_image(file_entry)),
                    TypeOfFile::ArchiveZip => Some(self.check_broken_zip(file_entry)),
                    TypeOfFile::Audio => Some(self.check_broken_audio(file_entry)),
                    TypeOfFile::PDF => Some(self.check_broken_pdf(file_entry)),
                    // This means that cache read invalid value because maybe cache comes from different czkawka version
                    TypeOfFile::Unknown => Some(None),
                }
            })
            .while_some()
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect::<Vec<FileEntry>>();
        debug!("look_for_broken_files - ended finding for broken files");

        send_info_and_wait_for_ending_all_threads(&progress_thread_run, progress_thread_handle);

        // Just connect loaded results with already calculated
        vec_file_entry.extend(records_already_cached.into_values());

        self.save_to_cache(&vec_file_entry, loaded_hash_map);

        self.broken_files = vec_file_entry
            .into_par_iter()
            .filter_map(|f| if f.error_string.is_empty() { None } else { Some(f) })
            .collect();

        self.information.number_of_broken_files = self.broken_files.len();
        debug!("Found {} broken files.", self.information.number_of_broken_files);
        // Clean unused data
        self.files_to_check = Default::default();

        true
    }
    #[fun_time(message = "save_to_cache", level = "debug")]
    fn save_to_cache(&mut self, vec_file_entry: &[FileEntry], loaded_hash_map: BTreeMap<String, FileEntry>) {
        if self.common_data.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, FileEntry> = Default::default();

            for file_entry in vec_file_entry.iter().cloned() {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            for (_name, file_entry) in loaded_hash_map {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }

            let messages = save_cache_to_file_generalized(&get_broken_files_cache_file(), &all_results, self.common_data.save_also_as_json, 0);
            self.get_text_messages_mut().extend_with_another_messages(messages);
        }
    }

    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self) {
        match self.common_data.delete_method {
            DeleteMethod::Delete => {
                for file_entry in &self.broken_files {
                    if fs::remove_file(&file_entry.path).is_err() {
                        self.common_data.text_messages.warnings.push(file_entry.path.display().to_string());
                    }
                }
            }
            DeleteMethod::None => {
                //Just do nothing
            }
            _ => {
                unreachable!()
            }
        }
    }
}

impl BrokenFiles {
    pub const fn get_broken_files(&self) -> &Vec<FileEntry> {
        &self.broken_files
    }

    pub fn set_checked_types(&mut self, checked_types: CheckedTypes) {
        self.checked_types = checked_types;
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
    }
}
impl Default for BrokenFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugPrint for BrokenFiles {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        self.debug_print_common();
    }
}

impl PrintResults for BrokenFiles {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writeln!(
            writer,
            "Results of searching {:?} with excluded directories {:?} and excluded items {:?}",
            self.common_data.directories.included_directories, self.common_data.directories.excluded_directories, self.common_data.excluded_items.items
        )?;

        if !self.broken_files.is_empty() {
            writeln!(writer, "Found {} broken files.", self.information.number_of_broken_files)?;
            for file_entry in &self.broken_files {
                writeln!(writer, "{} - {}", file_entry.path.display(), file_entry.error_string)?;
            }
        } else {
            write!(writer, "Not found any broken files.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.broken_files, pretty_print)
    }
}

fn check_extension_availability(file_name_lowercase: &str) -> TypeOfFile {
    if IMAGE_RS_BROKEN_FILES_EXTENSIONS.iter().any(|e| file_name_lowercase.ends_with(e)) {
        TypeOfFile::Image
    } else if ZIP_FILES_EXTENSIONS.iter().any(|e| file_name_lowercase.ends_with(e)) {
        TypeOfFile::ArchiveZip
    } else if AUDIO_FILES_EXTENSIONS.iter().any(|e| file_name_lowercase.ends_with(e)) {
        TypeOfFile::Audio
    } else if PDF_FILES_EXTENSIONS.iter().any(|e| file_name_lowercase.ends_with(e)) {
        TypeOfFile::PDF
    } else {
        TypeOfFile::Unknown
    }
}

fn check_if_file_extension_is_allowed(type_of_file: &TypeOfFile, checked_types: &CheckedTypes) -> bool {
    ((*type_of_file == TypeOfFile::Image) && ((*checked_types & CheckedTypes::IMAGE) == CheckedTypes::IMAGE))
        || ((*type_of_file == TypeOfFile::PDF) && ((*checked_types & CheckedTypes::PDF) == CheckedTypes::PDF))
        || ((*type_of_file == TypeOfFile::ArchiveZip) && ((*checked_types & CheckedTypes::ARCHIVE) == CheckedTypes::ARCHIVE))
        || ((*type_of_file == TypeOfFile::Audio) && ((*checked_types & CheckedTypes::AUDIO) == CheckedTypes::AUDIO))
}

fn unpack_pdf_error(e: PdfError) -> PdfError {
    if let Try {
        file: _,
        line: _,
        column: _,
        context: _,
        source,
    } = e
    {
        unpack_pdf_error(*source)
    } else {
        e
    }
}

fn validate_pdf_error(file_entry: &mut FileEntry, e: PdfError) -> PdfError {
    let mut error_string = e.to_string();
    // Workaround for strange error message https://github.com/qarmin/czkawka/issues/898
    if error_string.starts_with("Try at") {
        if let Some(start_index) = error_string.find("/pdf-") {
            error_string = format!("Decoding error in pdf-rs library - {}", &error_string[start_index..]);
        }
    }

    file_entry.error_string = error_string;
    unpack_pdf_error(e)
}

impl CommonData for BrokenFiles {
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
}
