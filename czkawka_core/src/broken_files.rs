use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::{fs, mem, panic};

use crate::common::{
    check_if_stop_received, create_crash_message, prepare_thread_handler_common, send_info_and_wait_for_ending_all_threads, AUDIO_FILES_EXTENSIONS,
    IMAGE_RS_BROKEN_FILES_EXTENSIONS, PDF_FILES_EXTENSIONS, ZIP_FILES_EXTENSIONS,
};
use crate::common_cache::{extract_loaded_cache, get_broken_files_cache_file, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common_dir_traversal::{DirTraversalBuilder, DirTraversalResult, FileEntry, ToolType};
use crate::common_tool::{CommonData, CommonToolData, DeleteMethod};
use crate::common_traits::*;
use crate::progress_data::{CurrentStage, ProgressData};
use crossbeam_channel::{Receiver, Sender};
use fun_time::fun_time;
use log::debug;
use pdf::file::FileOptions;
use pdf::object::ParseOptions;
use pdf::PdfError;
use pdf::PdfError::Try;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BrokenEntry {
    pub path: PathBuf,
    pub modified_date: u64,
    pub size: u64,
    pub type_of_file: TypeOfFile,
    pub error_string: String,
}
impl ResultEntry for BrokenEntry {
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

impl FileEntry {
    fn into_broken_entry(self) -> BrokenEntry {
        BrokenEntry {
            size: self.size,
            path: self.path,
            modified_date: self.modified_date,

            type_of_file: TypeOfFile::Unknown,
            error_string: String::new(),
        }
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
    #[derive(PartialEq, Copy, Clone, Debug)]
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

pub struct BrokenFilesParameters {
    pub checked_types: CheckedTypes,
}

impl BrokenFilesParameters {
    pub fn new(checked_types: CheckedTypes) -> Self {
        Self { checked_types }
    }
}

pub struct BrokenFiles {
    common_data: CommonToolData,
    information: Info,
    files_to_check: BTreeMap<String, BrokenEntry>,
    broken_files: Vec<BrokenEntry>,
    params: BrokenFilesParameters,
}

impl BrokenFiles {
    pub fn new(params: BrokenFilesParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::BrokenFiles),
            information: Info::default(),
            files_to_check: Default::default(),
            broken_files: Default::default(),
            params,
        }
    }

    #[fun_time(message = "find_broken_files", level = "info")]
    pub fn find_broken_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) {
        self.prepare_items();
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
    fn check_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        let zip_extensions = ZIP_FILES_EXTENSIONS.iter().collect::<HashSet<_>>();
        let audio_extensions = AUDIO_FILES_EXTENSIONS.iter().collect::<HashSet<_>>();
        let pdf_extensions = PDF_FILES_EXTENSIONS.iter().collect::<HashSet<_>>();
        let images_extensions = IMAGE_RS_BROKEN_FILES_EXTENSIONS.iter().collect::<HashSet<_>>();

        let mut extensions = Vec::new();
        let vec_extensions = [
            (CheckedTypes::PDF, PDF_FILES_EXTENSIONS),
            (CheckedTypes::AUDIO, AUDIO_FILES_EXTENSIONS),
            (CheckedTypes::ARCHIVE, ZIP_FILES_EXTENSIONS),
            (CheckedTypes::IMAGE, IMAGE_RS_BROKEN_FILES_EXTENSIONS),
        ];
        for (checked_type, extensions_to_add) in &vec_extensions {
            if self.get_params().checked_types.contains(*checked_type) {
                extensions.extend_from_slice(extensions_to_add);
            }
        }

        self.common_data.extensions.set_and_validate_allowed_extensions(&extensions);
        if !self.common_data.extensions.set_any_extensions() {
            return true;
        }

        let result = DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_receiver(stop_receiver)
            .progress_sender(progress_sender)
            .common_data(&self.common_data)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.files_to_check = grouped_file_entries
                    .into_values()
                    .flatten()
                    .map(|fe| {
                        let mut broken_entry = fe.into_broken_entry();
                        broken_entry.type_of_file = check_extension_availability(broken_entry.get_path(), &images_extensions, &zip_extensions, &audio_extensions, &pdf_extensions);
                        (broken_entry.path.to_string_lossy().to_string(), broken_entry)
                    })
                    .collect();
                self.common_data.text_messages.warnings.extend(warnings);
                debug!("check_files - Found {} files to check.", self.files_to_check.len());
                true
            }

            DirTraversalResult::Stopped => false,
        }
    }

    fn check_broken_image(&self, mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        let mut file_entry_clone = file_entry.clone();

        panic::catch_unwind(|| {
            if let Err(e) = image::open(&file_entry.path) {
                let error_string = e.to_string();
                // This error is a problem with image library, remove check when https://github.com/image-rs/jpeg-decoder/issues/130 will be fixed
                if error_string.contains("spectral selection is not allowed in non-progressive scan") {
                    return None;
                }
                file_entry.error_string = error_string;
            }
            Some(file_entry)
        })
        .unwrap_or_else(|_| {
            let message = create_crash_message("Image-rs", &file_entry_clone.path.to_string_lossy(), "https://github.com/Serial-ATA/lofty-rs");
            println!("{message}");
            file_entry_clone.error_string = message;
            Some(file_entry_clone)
        })
    }
    fn check_broken_zip(&self, mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
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
    fn check_broken_audio(&self, mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                let mut file_entry_clone = file_entry.clone();

                panic::catch_unwind(|| {
                    if let Err(e) = audio_checker::parse_audio_file(file) {
                        file_entry.error_string = e.to_string();
                    }
                    Some(file_entry)
                })
                .unwrap_or_else(|_| {
                    let message = create_crash_message("Symphonia", &file_entry_clone.path.to_string_lossy(), "https://github.com/pdeljanov/Symphonia");
                    println!("{message}");
                    file_entry_clone.error_string = message;
                    Some(file_entry_clone)
                })
            }
            Err(_inspected) => None,
        }
    }
    fn check_broken_pdf(&self, mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        let parser_options = ParseOptions::tolerant(); // Only show as broken files with really big bugs

        let mut file_entry_clone = file_entry.clone();
        panic::catch_unwind(|| {
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
        })
        .unwrap_or_else(|_| {
            let message = create_crash_message("PDF-rs", &file_entry_clone.path.to_string_lossy(), "https://github.com/pdf-rs/pdf");
            println!("{message}");
            file_entry_clone.error_string = message;
            Some(file_entry_clone)
        })
    }

    #[fun_time(message = "load_cache", level = "debug")]
    fn load_cache(&mut self) -> (BTreeMap<String, BrokenEntry>, BTreeMap<String, BrokenEntry>, BTreeMap<String, BrokenEntry>) {
        let loaded_hash_map;

        let mut records_already_cached: BTreeMap<String, BrokenEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, BrokenEntry> = Default::default();
        let files_to_check = mem::take(&mut self.files_to_check);

        if self.common_data.use_cache {
            let (messages, loaded_items) =
                load_cache_from_file_generalized_by_path::<BrokenEntry>(&get_broken_files_cache_file(), self.get_delete_outdated_cache(), &files_to_check);
            self.get_text_messages_mut().extend_with_another_messages(messages);
            loaded_hash_map = loaded_items.unwrap_or_default();

            extract_loaded_cache(&loaded_hash_map, files_to_check, &mut records_already_cached, &mut non_cached_files_to_check);
        } else {
            loaded_hash_map = Default::default();
            non_cached_files_to_check = files_to_check;
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    #[fun_time(message = "look_for_broken_files", level = "debug")]
    fn look_for_broken_files(&mut self, stop_receiver: Option<&Receiver<()>>, progress_sender: Option<&Sender<ProgressData>>) -> bool {
        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache();

        let (progress_thread_handle, progress_thread_run, atomic_counter, _check_was_stopped) =
            prepare_thread_handler_common(progress_sender, CurrentStage::BrokenFilesChecking, non_cached_files_to_check.len(), self.get_test_type());

        debug!("look_for_broken_files - started finding for broken files");
        let mut vec_file_entry: Vec<BrokenEntry> = non_cached_files_to_check
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
            .flatten()
            .collect::<Vec<BrokenEntry>>();
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
    fn save_to_cache(&mut self, vec_file_entry: &[BrokenEntry], loaded_hash_map: BTreeMap<String, BrokenEntry>) {
        if self.common_data.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, BrokenEntry> = Default::default();

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
                        self.common_data.text_messages.warnings.push(file_entry.path.to_string_lossy().to_string());
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
    pub const fn get_broken_files(&self) -> &Vec<BrokenEntry> {
        &self.broken_files
    }

    pub fn get_params(&self) -> &BrokenFilesParameters {
        &self.params
    }

    pub const fn get_information(&self) -> &Info {
        &self.information
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
            self.common_data.directories.included_directories,
            self.common_data.directories.excluded_directories,
            self.common_data.excluded_items.get_excluded_items()
        )?;

        if !self.broken_files.is_empty() {
            writeln!(writer, "Found {} broken files.", self.information.number_of_broken_files)?;
            for file_entry in &self.broken_files {
                writeln!(writer, "\"{}\" - {}", file_entry.path.to_string_lossy(), file_entry.error_string)?;
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

fn check_extension_availability(
    full_name: &Path,
    images_extensions: &HashSet<&&'static str>,
    zip_extensions: &HashSet<&&'static str>,
    audio_extensions: &HashSet<&&'static str>,
    pdf_extensions: &HashSet<&&'static str>,
) -> TypeOfFile {
    let Some(extension) = full_name.extension() else {
        debug_assert!(false, "Missing extension");
        return TypeOfFile::Unknown;
    };

    let Some(extension_str) = extension.to_str() else {
        debug_assert!(false, "Extension not really fully str");
        return TypeOfFile::Unknown;
    };
    let extension_lowercase = extension_str.to_ascii_lowercase();

    if images_extensions.contains(&extension_lowercase.as_str()) {
        TypeOfFile::Image
    } else if zip_extensions.contains(&extension_lowercase.as_str()) {
        TypeOfFile::ArchiveZip
    } else if audio_extensions.contains(&extension_lowercase.as_str()) {
        TypeOfFile::Audio
    } else if pdf_extensions.contains(&extension_lowercase.as_str()) {
        TypeOfFile::PDF
    } else {
        eprintln!("File with unknown extension: \"{}\" - {extension_lowercase}", full_name.to_string_lossy());
        debug_assert!(false, "File with unknown extension");
        TypeOfFile::Unknown
    }
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

fn validate_pdf_error(file_entry: &mut BrokenEntry, e: PdfError) -> PdfError {
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
