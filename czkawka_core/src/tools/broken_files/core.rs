use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::{mem, panic};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use log::{debug, error};
use lopdf::Document;
use rayon::prelude::*;

use crate::common::cache::{CACHE_BROKEN_FILES_VERSION, extract_loaded_cache, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::consts::{AUDIO_FILES_EXTENSIONS, IMAGE_RS_BROKEN_FILES_EXTENSIONS, PDF_FILES_EXTENSIONS, VIDEO_FILES_EXTENSIONS, ZIP_FILES_EXTENSIONS};
use crate::common::create_crash_message;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::process_utils::run_command_interruptible;
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::helpers::audio_checker;
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
        let mut extensions = Vec::new();
        let vec_extensions = [
            (CheckedTypes::PDF, PDF_FILES_EXTENSIONS),
            (CheckedTypes::AUDIO, AUDIO_FILES_EXTENSIONS),
            (CheckedTypes::ARCHIVE, ZIP_FILES_EXTENSIONS),
            (CheckedTypes::IMAGE, IMAGE_RS_BROKEN_FILES_EXTENSIONS),
            (CheckedTypes::VIDEO, VIDEO_FILES_EXTENSIONS),
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
                        let broken_entry = fe.into_broken_entry();
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

    fn check_broken_image(mut file_entry: BrokenEntry) -> BrokenEntry {
        let mut file_entry_clone = file_entry.clone();

        panic::catch_unwind(|| {
            match image::open(&file_entry.path) {
                Ok(img) => {
                    if img.width() == 0 || img.height() == 0 {
                        file_entry.error_string = "Image has zero width or height".to_string();
                    }
                }
                Err(e) => {
                    file_entry.error_string = e.to_string();
                }
            }
            file_entry
        })
        .unwrap_or_else(|_| {
            let message = create_crash_message("Image-rs", &file_entry_clone.path.to_string_lossy(), "https://github.com/Serial-ATA/lofty-rs");
            error!("{message}");
            file_entry_clone.error_string = message;
            file_entry_clone
        })
    }
    fn check_broken_zip(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
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
    fn check_broken_audio(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
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
    fn check_broken_pdf(mut file_entry: BrokenEntry) -> BrokenEntry {
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
            file_entry
        })
        .unwrap_or_else(|_| {
            let message = create_crash_message("lopdf", &file_entry_clone.path.to_string_lossy(), "https://github.com/J-F-Liu/lopdf");
            error!("{message}");
            file_entry_clone.error_string = message;
            file_entry_clone
        })
    }

    // None if stopped, otherwise Some
    fn check_broken_video(mut file_entry: BrokenEntry, stop_flag: &Arc<AtomicBool>) -> Option<BrokenEntry> {
        let ffprobe_errors = [("moov atom not found", Some("broken file structure"))];

        let mut command = Command::new("ffprobe");
        command.arg(&file_entry.path);

        match run_command_interruptible(command, stop_flag) {
            None => return None,
            Some(Err(e)) => {
                debug!("Failed to run ffprobe on {:?}: {}", file_entry.path, e);
                file_entry.error_string = format!("Failed to run ffprobe: {}", e);
                return Some(file_entry);
            }
            Some(Ok(output)) => {
                let combined = format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));

                // use std::io::Write;
                // if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("a.txt") {
                //     let _ = writeln!(f, "{}", combined);
                // }

                if let Some((error_message, additional_message)) = ffprobe_errors.iter().find(|(err, _)| combined.contains(err)) {
                    file_entry.error_string = format!("{error_message}{}", additional_message.map(|e| format!(" ({e})")).unwrap_or_default());
                    return Some(file_entry);
                } else if !output.status.success() {
                    file_entry.error_string = format!("ffprobe exited with non-zero status: {}", output.status);
                    return Some(file_entry);
                }
            }
        }

        let ffmpeg_message = [
            ("Output file does not contain any stream", Some("cannot find video stream - possible not even video file")),
            ("corrupt decoded frame", Some("may be still playable")),
            ("Invalid data found when processing input", Some("generic error"))
        ];
        let ffmpeg_allowed_messages = [
            "Input buffer exhausted before END element found", // Looks like quite popular message, so ignoring it
            "Invalid color space", // https://fftrac-bg.ffmpeg.org/ticket/11020 - seems to be non-fatal
        ];

        let mut command = Command::new("ffmpeg");
        command.arg("-v").arg("error").arg("-xerror").arg("-threads").arg("1").arg("-i").arg(&file_entry.path).arg("-f").arg("null").arg("-");

        match run_command_interruptible(command, stop_flag) {
            None => return None,
            Some(Err(e)) => {
                debug!("Failed to run ffmpeg on {:?}: {}", file_entry.path, e);
                file_entry.error_string = format!("Failed to run ffmpeg: {}", e);
            }
            Some(Ok(output)) => {
                let combined = format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));

                // use std::io::Write;
                // if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("b.txt") {
                //     let _ = writeln!(f, "{} --- \n{}", file_entry.path.to_string_lossy(), combined);
                // }

                if ffmpeg_allowed_messages.iter().any(|msg| combined.contains(msg)) {
                    // Allowed message, do nothing
                } else if let Some((error_message, additional_message)) = ffmpeg_message.iter().find(|(err, _)| combined.contains(err)) {
                    file_entry.error_string = format!("{error_message}{}", additional_message.map(|e| format!(" ({e})")).unwrap_or_default());
                } else if !output.status.success() {
                    file_entry.error_string = format!("ffmpeg exited with non-zero status: {}", output.status);
                }
            }
        }

        Some(file_entry)
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

    fn check_file(file_entry: BrokenEntry, stop_flag: &Arc<AtomicBool>) -> Option<Option<BrokenEntry>> {
        match check_extension_availability(&file_entry.path) {
            TypeOfFile::Image => Some(Some(Self::check_broken_image(file_entry))),
            TypeOfFile::ArchiveZip => Some(Self::check_broken_zip(file_entry)),
            TypeOfFile::Audio => Some(Self::check_broken_audio(file_entry)),
            TypeOfFile::Pdf => Some(Some(Self::check_broken_pdf(file_entry))),
            TypeOfFile::Video => Self::check_broken_video(file_entry, stop_flag).map(Some),
            TypeOfFile::Unknown => {
                error!("Unknown file type of: {:?}", file_entry);
                Some(None)
            }
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
                let res = Self::check_file(file_entry, stop_flag);

                progress_handler.increase_items(1);
                progress_handler.increase_size(size);

                res
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
            for (name, file_entry) in loaded_hash_map {
                all_results.insert(name, file_entry);
            }

            let messages = save_cache_to_file_generalized(&get_broken_files_cache_file(), &all_results, self.common_data.save_also_as_json, 0);
            self.get_text_messages_mut().extend_with_another_messages(messages);
        }
    }
}

#[expect(clippy::string_slice)] // Valid, because we address go to dot, which is known ascii character
fn check_extension_availability(full_name: &Path) -> TypeOfFile {
    let Some(file_name) = full_name.file_name() else {
        error!("Missing file name in file - \"{}\"", full_name.to_string_lossy());
        debug_assert!(false, "Missing file name in file - \"{}\"", full_name.to_string_lossy());
        return TypeOfFile::Unknown;
    };

    // Faster manual conversion than using Path::extension()
    let Some(file_name_str) = file_name.to_str() else { return TypeOfFile::Unknown };
    let Some(extension_idx) = file_name_str.rfind('.') else { return TypeOfFile::Unknown };
    let extension_str = &file_name_str[extension_idx + 1..];

    let extension_lowercase = extension_str.to_ascii_lowercase();

    if IMAGE_RS_BROKEN_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        TypeOfFile::Image
    } else if ZIP_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        TypeOfFile::ArchiveZip
    } else if PDF_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        TypeOfFile::Pdf
    } else if AUDIO_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        TypeOfFile::Audio
    } else if VIDEO_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        TypeOfFile::Video
    } else {
        error!("File with unknown extension: \"{}\" - {extension_lowercase}", full_name.to_string_lossy());
        debug_assert!(false, "File with unknown extension - \"{}\" - {extension_lowercase}", full_name.to_string_lossy());
        TypeOfFile::Unknown
    }
}

pub fn get_broken_files_cache_file() -> String {
    format!("cache_broken_files_{CACHE_BROKEN_FILES_VERSION}.bin")
}
