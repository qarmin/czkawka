use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::{mem, panic};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::{debug, error};
use lopdf::Document;
use rayon::prelude::*;

use crate::common::cache::{CACHE_VERSION, extract_loaded_cache, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::consts::{AUDIO_FILES_EXTENSIONS, IMAGE_RS_BROKEN_FILES_EXTENSIONS, PDF_FILES_EXTENSIONS, ZIP_FILES_EXTENSIONS};
use crate::common::create_crash_message;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::traits::ResultEntry;
use crate::tools::broken_files::{BrokenEntry, BrokenFiles, BrokenFilesParameters, CheckedTypes, Info, TypeOfFile};

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

    #[fun_time(message = "check_files", level = "debug")]
    pub(crate) fn check_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
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
        // TODO, responsibility should be moved to CLI/GUI
        // assert!(self.common_data.extensions.set_any_extensions(), "This should be checked before");
        if !self.common_data.extensions.set_any_extensions() {
            return WorkContinueStatus::Continue;
        }

        let result = DirTraversalBuilder::new()
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
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
                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
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
            error!("{message}");
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
                    error!("{message}");
                    file_entry_clone.error_string = message;
                    Some(file_entry_clone)
                })
            }
            Err(_inspected) => None,
        }
    }
    fn check_broken_pdf(&self, mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        let mut file_entry_clone = file_entry.clone();
        panic::catch_unwind(|| {
            match File::open(&file_entry.path) {
                Ok(file) => {
                    if let Err(e) = Document::load_from(file) {
                        file_entry.error_string = e.to_string();
                    }
                }
                Err(e) => {
                    file_entry.error_string = e.to_string();
                }
            }
            Some(file_entry)
        })
        .unwrap_or_else(|_| {
            let message = create_crash_message("lopdf", &file_entry_clone.path.to_string_lossy(), "https://github.com/J-F-Liu/lopdf");
            error!("{message}");
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

    fn check_file(&self, file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match file_entry.type_of_file {
            TypeOfFile::Image => self.check_broken_image(file_entry),
            TypeOfFile::ArchiveZip => self.check_broken_zip(file_entry),
            TypeOfFile::Audio => self.check_broken_audio(file_entry),
            TypeOfFile::PDF => self.check_broken_pdf(file_entry),
            // This means that cache read invalid value because maybe cache comes from different czkawka version
            TypeOfFile::Unknown => None,
        }
    }

    #[fun_time(message = "look_for_broken_files", level = "debug")]
    pub(crate) fn look_for_broken_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.files_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache();

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::BrokenFilesChecking,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            non_cached_files_to_check.values().map(|item| item.size).sum::<u64>(),
        );

        let non_cached_files_to_check = non_cached_files_to_check.into_iter().collect::<Vec<_>>();

        debug!("look_for_broken_files - started finding for broken files");
        let mut vec_file_entry: Vec<BrokenEntry> = non_cached_files_to_check
            .into_par_iter()
            .with_max_len(3)
            .map(|(_, file_entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let size = file_entry.size;
                let res = self.check_file(file_entry);

                progress_handler.increase_items(1);
                progress_handler.increase_size(size);

                Some(res)
            })
            .while_some()
            .flatten()
            .collect::<Vec<BrokenEntry>>();
        debug!("look_for_broken_files - ended finding for broken files");

        progress_handler.join_thread();

        // Just connect loaded results with already calculated
        vec_file_entry.extend(records_already_cached.into_values());

        self.save_to_cache(&vec_file_entry, loaded_hash_map);

        self.broken_files = vec_file_entry.into_iter().filter_map(|f| if f.error_string.is_empty() { None } else { Some(f) }).collect();

        self.information.number_of_broken_files = self.broken_files.len();
        debug!("Found {} broken files.", self.information.number_of_broken_files);
        // Clean unused data
        self.files_to_check = Default::default();

        WorkContinueStatus::Continue
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
}

#[allow(clippy::string_slice)] // Valid, because we address go to dot, which is known ascii character
fn check_extension_availability(
    full_name: &Path,
    images_extensions: &HashSet<&&'static str>,
    zip_extensions: &HashSet<&&'static str>,
    audio_extensions: &HashSet<&&'static str>,
    pdf_extensions: &HashSet<&&'static str>,
) -> TypeOfFile {
    let Some(file_name) = full_name.file_name() else {
        error!("Missing file name in file - \"{}\"", full_name.to_string_lossy());
        debug_assert!(false, "Missing file name in file - \"{}\"", full_name.to_string_lossy());
        return TypeOfFile::Unknown;
    };
    let Some(file_name_str) = file_name.to_str() else { return TypeOfFile::Unknown };
    let Some(extension_idx) = file_name_str.rfind('.') else { return TypeOfFile::Unknown };
    let extension_str = &file_name_str[extension_idx + 1..];

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
        error!("File with unknown extension: \"{}\" - {extension_lowercase}", full_name.to_string_lossy());
        debug_assert!(false, "File with unknown extension - \"{}\" - {extension_lowercase}", full_name.to_string_lossy());
        TypeOfFile::Unknown
    }
}

pub fn get_broken_files_cache_file() -> String {
    format!("cache_broken_files_{CACHE_VERSION}.bin")
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::path::Path;

    use super::*;

    #[test]
    fn test_check_extension_availability_image() {
        let images_extensions: HashSet<&&str> = ["jpg", "png", "gif"].iter().collect();
        let zip_extensions: HashSet<&&str> = HashSet::new();
        let audio_extensions: HashSet<&&str> = HashSet::new();
        let pdf_extensions: HashSet<&&str> = HashSet::new();

        let path = Path::new("test.jpg");
        assert_eq!(
            check_extension_availability(path, &images_extensions, &zip_extensions, &audio_extensions, &pdf_extensions),
            TypeOfFile::Image
        );
    }

    #[test]
    fn test_check_extension_availability_zip() {
        let images_extensions: HashSet<&&str> = HashSet::new();
        let zip_extensions: HashSet<&&str> = ["zip", "rar"].iter().collect();
        let audio_extensions: HashSet<&&str> = HashSet::new();
        let pdf_extensions: HashSet<&&str> = HashSet::new();

        let path = Path::new("test.zip");
        assert_eq!(
            check_extension_availability(path, &images_extensions, &zip_extensions, &audio_extensions, &pdf_extensions),
            TypeOfFile::ArchiveZip
        );
    }

    #[test]
    fn test_check_extension_availability_audio() {
        let images_extensions: HashSet<&&str> = HashSet::new();
        let zip_extensions: HashSet<&&str> = HashSet::new();
        let audio_extensions: HashSet<&&str> = ["mp3", "wav"].iter().collect();
        let pdf_extensions: HashSet<&&str> = HashSet::new();

        let path = Path::new("test.mp3");
        assert_eq!(
            check_extension_availability(path, &images_extensions, &zip_extensions, &audio_extensions, &pdf_extensions),
            TypeOfFile::Audio
        );
    }

    #[test]
    fn test_check_extension_availability_pdf() {
        let images_extensions: HashSet<&&str> = HashSet::new();
        let zip_extensions: HashSet<&&str> = HashSet::new();
        let audio_extensions: HashSet<&&str> = HashSet::new();
        let pdf_extensions: HashSet<&&str> = std::iter::once(&"pdf").collect();

        let path = Path::new("test.pdf");
        assert_eq!(
            check_extension_availability(path, &images_extensions, &zip_extensions, &audio_extensions, &pdf_extensions),
            TypeOfFile::PDF
        );
    }

    #[test]
    fn test_check_extension_availability_no_extension() {
        let images_extensions: HashSet<&&str> = HashSet::new();
        let zip_extensions: HashSet<&&str> = HashSet::new();
        let audio_extensions: HashSet<&&str> = HashSet::new();
        let pdf_extensions: HashSet<&&str> = HashSet::new();

        let path = Path::new("test");
        assert_eq!(
            check_extension_availability(path, &images_extensions, &zip_extensions, &audio_extensions, &pdf_extensions),
            TypeOfFile::Unknown
        );
    }

    #[test]
    fn test_check_no_extension() {
        let images_extensions: HashSet<&&str> = HashSet::new();
        let zip_extensions: HashSet<&&str> = HashSet::new();
        let audio_extensions: HashSet<&&str> = ["mp3", "wav"].iter().collect();
        let pdf_extensions: HashSet<&&str> = HashSet::new();

        let path = Path::new("/home/.mp3");
        assert_eq!(
            check_extension_availability(path, &images_extensions, &zip_extensions, &audio_extensions, &pdf_extensions),
            TypeOfFile::Audio
        );
    }
}
