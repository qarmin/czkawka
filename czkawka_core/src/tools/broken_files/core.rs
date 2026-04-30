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

use crate::common::cache::{CACHE_BROKEN_FILES_VERSION, load_and_split_cache_generalized_by_path, save_and_connect_cache_generalized_by_path};
use crate::common::consts::{
    AUDIO_FILES_EXTENSIONS, BZ2_FILES_EXTENSIONS, FONT_FILES_EXTENSIONS, GZ_FILES_EXTENSIONS, IMAGE_RS_BROKEN_FILES_EXTENSIONS, JSON_FILES_EXTENSIONS, PDF_FILES_EXTENSIONS,
    SEVENZ_FILES_EXTENSIONS, SVG_FILES_EXTENSIONS, TAR_FILES_EXTENSIONS, TOML_FILES_EXTENSIONS, VIDEO_FILES_EXTENSIONS, XML_FILES_EXTENSIONS, XZ_FILES_EXTENSIONS,
    YAML_FILES_EXTENSIONS, ZIP_FILES_EXTENSIONS, ZST_FILES_EXTENSIONS,
};
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::process_utils::run_command_interruptible;
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::common::{create_crash_message_generic, normalize_error_string};
use crate::helpers::audio_checker;
use crate::tools::broken_files::{BrokenEntry, BrokenFiles, BrokenFilesParameters, CheckedTypes, CheckedTypesSingle, Info, TypeOfFile};

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
        let error = match image::open(&file_entry.path) {
            Ok(img) => {
                if img.width() == 0 || img.height() == 0 {
                    Some("Image has zero width or height".to_string())
                } else {
                    None
                }
            }
            Err(e) => Some(normalize_error_string(&e.to_string())),
        };
        file_entry.errors.insert(CheckedTypesSingle::Image, error);
        file_entry
    }

    fn check_broken_zip(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                let error = match zip::ZipArchive::new(file) {
                    Err(e) => Some(normalize_error_string(&e.to_string())),
                    Ok(_) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Archive, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_7z(mut file_entry: BrokenEntry) -> BrokenEntry {
        let error = match sevenz_rust2::Archive::open(&file_entry.path) {
            Err(sevenz_rust2::Error::PasswordRequired) | Ok(_) => None,
            Err(e) => Some(normalize_error_string(&e.to_string())),
        };
        file_entry.errors.insert(CheckedTypesSingle::Archive, error);
        file_entry
    }

    fn check_broken_gz(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                let mut decoder = flate2::read::GzDecoder::new(file);
                let error = match std::io::copy(&mut decoder, &mut std::io::sink()) {
                    Err(e) => Some(normalize_error_string(&e.to_string())),
                    Ok(_) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Archive, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_zst(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                let error = match ruzstd::decoding::StreamingDecoder::new(file) {
                    Err(e) => Some(normalize_error_string(&e.to_string())),
                    Ok(mut decoder) => match std::io::copy(&mut decoder, &mut std::io::sink()) {
                        Err(e) => Some(normalize_error_string(&e.to_string())),
                        Ok(_) => None,
                    },
                };
                file_entry.errors.insert(CheckedTypesSingle::Archive, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_tar(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                let mut archive = tar::Archive::new(file);
                let error = match archive.entries() {
                    Err(e) => Some(normalize_error_string(&e.to_string())),
                    Ok(entries) => {
                        let mut err: Option<String> = None;
                        for entry in entries {
                            if let Err(e) = entry {
                                err = Some(normalize_error_string(&e.to_string()));
                                break;
                            }
                        }
                        err
                    }
                };
                file_entry.errors.insert(CheckedTypesSingle::Archive, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }
    fn check_broken_json(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match std::fs::read(&file_entry.path) {
            Ok(data) => {
                let error = match serde_json::from_slice::<serde_json::Value>(&data) {
                    Err(e) => Some(normalize_error_string(&e.to_string())),
                    Ok(_) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Markup, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_xml(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match std::fs::read(&file_entry.path) {
            Ok(data) => {
                let mut reader = quick_xml::Reader::from_reader(data.as_slice());
                reader.config_mut().check_end_names = true;
                let error = loop {
                    match reader.read_event() {
                        Err(e) => break Some(normalize_error_string(&e.to_string())),
                        Ok(quick_xml::events::Event::Eof) => break None,
                        Ok(_) => {}
                    }
                };
                file_entry.errors.insert(CheckedTypesSingle::Markup, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_toml(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match std::fs::read_to_string(&file_entry.path) {
            Ok(text) => {
                let error = match toml::from_str::<toml::Table>(&text) {
                    Err(e) => Some(normalize_error_string(&e.to_string())),
                    Ok(_) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Markup, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_yaml(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match std::fs::read_to_string(&file_entry.path) {
            Ok(text) => {
                let error = match yaml_rust2::YamlLoader::load_from_str(&text) {
                    Err(e) => Some(normalize_error_string(&e.to_string())),
                    Ok(_) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Markup, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_svg(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match std::fs::read(&file_entry.path) {
            Ok(data) => {
                let error = match usvg::Tree::from_data(&data, &usvg::Options::default()) {
                    Err(e) => Some(normalize_error_string(&format!("{e:?}"))),
                    Ok(_) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Markup, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_bz2(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                let mut decoder = bzip2_rs::DecoderReader::new(file);
                let error = match std::io::copy(&mut decoder, &mut std::io::sink()) {
                    Err(e) => Some(normalize_error_string(&e.to_string())),
                    Ok(_) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Archive, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_xz(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                let mut reader = std::io::BufReader::new(file);
                let error = match lzma_rs::xz_decompress(&mut reader, &mut std::io::sink()) {
                    Err(e) => Some(normalize_error_string(&e.to_string())),
                    Ok(()) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Archive, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_font(mut file_entry: BrokenEntry) -> Option<BrokenEntry> {
        match std::fs::read(&file_entry.path) {
            Ok(data) => {
                let error = match ttf_parser::Face::parse(&data, 0) {
                    Err(e) => Some(normalize_error_string(&format!("{e:?}"))),
                    Ok(_) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Font, error);
                Some(file_entry)
            }
            Err(_inspected) => None,
        }
    }

    fn check_broken_audio(mut file_entry: BrokenEntry, stop_flag: &Arc<AtomicBool>) -> Option<Option<BrokenEntry>> {
        match File::open(&file_entry.path) {
            Ok(file) => {
                let error = match audio_checker::parse_audio_file(file, stop_flag) {
                    Err(e) => {
                        let err_str = e.to_string();
                        if err_str.contains("not supported codec") {
                            None
                        } else {
                            Some(normalize_error_string(&err_str))
                        }
                    }
                    Ok(None) => return None, // stop flag was set
                    Ok(Some(())) => None,
                };
                file_entry.errors.insert(CheckedTypesSingle::Audio, error);
                Some(Some(file_entry))
            }
            Err(_inspected) => Some(None),
        }
    }
    fn check_broken_pdf(mut file_entry: BrokenEntry) -> BrokenEntry {
        let error = match File::open(&file_entry.path) {
            Ok(file) => match Document::load_from(file) {
                Err(e) => Some(normalize_error_string(&e.to_string())),
                Ok(_) => None,
            },
            Err(e) => Some(normalize_error_string(&e.to_string())),
        };
        file_entry.errors.insert(CheckedTypesSingle::Pdf, error);
        file_entry
    }

    fn check_broken_video_ffprobe(mut file_entry: BrokenEntry, stop_flag: &Arc<AtomicBool>) -> Option<BrokenEntry> {
        let ffprobe_errors = [
            ("moov atom not found", Some("broken file structure")),
            ("error reading header", Some("broken file structure")),
            ("EBML header parsing failed", None),
            ("exceeds containing master element", Some("broken file structure")),
            ("invalid frame index table", Some("broken file structure")),
            ("Invalid argument", Some("ffprobe seems to not recognize file format")),
        ];

        let mut command = Command::new("ffprobe");
        command.arg("-v").arg("error").arg(&file_entry.path);

        let error = match run_command_interruptible(command, stop_flag) {
            None => return None,
            Some(Err(e)) => {
                debug!("Failed to run ffprobe on {:?}: {}", file_entry.path, e);
                Some(normalize_error_string(&format!("Failed to run ffprobe: {e}")))
            }
            Some(Ok(output)) => {
                let combined = format!("{}{}", output.stdout.trim(), output.stderr.trim());

                if let Some((error_message, additional_message)) = ffprobe_errors.iter().find(|(err, _)| combined.contains(err)) {
                    Some(format!("{error_message}{}", additional_message.map(|e| format!(" ({e})")).unwrap_or_default()))
                } else if !output.status.success() {
                    // debug_save_file("ffprobe_failed_output.txt", &format!("{} --- \n{}", file_entry.path.to_string_lossy(), combined));
                    Some(format!("ffprobe exited with non-zero status: {}", output.status))
                } else {
                    None
                }
            }
        };
        file_entry.errors.insert(CheckedTypesSingle::VideoFfprobe, error);
        Some(file_entry)
    }

    fn check_broken_video_ffmpeg(mut file_entry: BrokenEntry, stop_flag: &Arc<AtomicBool>) -> Option<BrokenEntry> {
        let ffmpeg_message = [
            ("Output file does not contain any stream", Some("cannot find video stream - possible not even video file")),
            ("missing mandatory atoms, broken header", Some("broken file structure")),
            ("Cannot determine format of input", None),
            ("decode_slice_header error", Some("corrupted video data, may be still fully/partially playable")),
            ("Truncating packet", Some("corrupted video data, may be still fully/partially playable")),
            ("Invalid NAL unit size", Some("corrupted video data, may be still fully/partially playable")),
            (
                "exceeds containing master element ending",
                Some("corrupted video data, may be still fully/partially playable"),
            ),
            ("corrupt input packet in stream", Some("Possible corruption in audio/video stream, may be still playable")),
            (
                "invalid as first byte of an EBML number",
                Some("corrupted video data, may be still fully/partially playable"),
            ),
            // Last resort for all other errors
            ("Invalid data found when processing input", Some("generic error")), // Must be last to not override more precise errors
            // Warnings
            ("corrupt decoded frame", Some("may be still playable")),
        ];
        let ffmpeg_allowed_messages = [
            "Input buffer exhausted before END element found", // Looks like quite popular message, so ignoring it
            "Invalid color space",                             // https://fftrac-bg.ffmpeg.org/ticket/11020 - seems to be non-fatal
        ];

        let mut command = Command::new("ffmpeg");
        command
            .arg("-v")
            .arg("error")
            .arg("-xerror")
            .arg("-threads")
            .arg("1")
            .arg("-i")
            .arg(&file_entry.path)
            .arg("-f")
            .arg("null")
            .arg("-");

        let error = match run_command_interruptible(command, stop_flag) {
            None => return None,
            Some(Err(e)) => {
                debug!("Failed to run ffmpeg on {:?}: {}", file_entry.path, e);
                Some(normalize_error_string(&format!("Failed to run ffmpeg: {e}")))
            }
            Some(Ok(output)) => {
                let combined = format!("{}{}", output.stdout.trim(), output.stderr.trim());

                if ffmpeg_allowed_messages.iter().any(|msg| combined.contains(msg)) {
                    None
                } else if let Some((error_message, additional_message)) = ffmpeg_message.iter().find(|(err, _)| combined.contains(err)) {
                    Some(format!("{error_message}{}", additional_message.map(|e| format!(" ({e})")).unwrap_or_default()))
                } else if !output.status.success() {
                    // debug_save_file("ffmpeg_failed_output.txt", &format!("{} --- \n{}", file_entry.path.to_string_lossy(), combined));
                    Some(format!("ffmpeg exited with non-zero status: {}", output.status))
                } else {
                    None
                }
            }
        };
        file_entry.errors.insert(CheckedTypesSingle::VideoFfmpeg, error);
        Some(file_entry)
    }

    fn check_broken_video(mut file_entry: BrokenEntry, stop_flag: &Arc<AtomicBool>, checked_types: CheckedTypes) -> Option<BrokenEntry> {
        if checked_types.contains(CheckedTypes::VIDEO_FFPROBE) {
            file_entry = Self::check_broken_video_ffprobe(file_entry, stop_flag)?;
        }
        if checked_types.contains(CheckedTypes::VIDEO_FFMPEG) {
            file_entry = Self::check_broken_video_ffmpeg(file_entry, stop_flag)?;
        }
        Some(file_entry)
    }

    #[fun_time(message = "load_cache", level = "debug")]
    fn load_cache(&mut self) -> (BTreeMap<String, BrokenEntry>, BTreeMap<String, BrokenEntry>, BTreeMap<String, BrokenEntry>) {
        load_and_split_cache_generalized_by_path(&get_broken_files_cache_file(), mem::take(&mut self.files_to_check), self)
    }

    #[fun_time(message = "save_to_cache", level = "debug")]
    fn save_to_cache(&mut self, vec_file_entry: &[BrokenEntry], loaded_hash_map: BTreeMap<String, BrokenEntry>) {
        save_and_connect_cache_generalized_by_path(&get_broken_files_cache_file(), vec_file_entry, loaded_hash_map, self);
    }

    fn file_type_to_checked_type_single(file_type: TypeOfFile) -> CheckedTypesSingle {
        match file_type {
            TypeOfFile::Image => CheckedTypesSingle::Image,
            TypeOfFile::ArchiveZip
            | TypeOfFile::Archive7z
            | TypeOfFile::ArchiveGz
            | TypeOfFile::ArchiveTar
            | TypeOfFile::ArchiveZst
            | TypeOfFile::ArchiveBz2
            | TypeOfFile::ArchiveXz => CheckedTypesSingle::Archive,
            TypeOfFile::Audio => CheckedTypesSingle::Audio,
            TypeOfFile::Pdf => CheckedTypesSingle::Pdf,
            TypeOfFile::Video => CheckedTypesSingle::VideoFfprobe,
            TypeOfFile::Font => CheckedTypesSingle::Font,
            TypeOfFile::Json | TypeOfFile::Xml | TypeOfFile::Toml | TypeOfFile::Yaml | TypeOfFile::Svg => CheckedTypesSingle::Markup,
        }
    }

    fn check_file(file_entry: BrokenEntry, stop_flag: &Arc<AtomicBool>, checked_types: CheckedTypes) -> Option<Option<BrokenEntry>> {
        let Some(file_type) = check_extension_availability(&file_entry.path) else {
            error!("Unknown file type of: {file_entry:?}");
            debug_assert!(false, "Unknown file type: {:?}", file_entry.path);
            return Some(None);
        };

        let mut file_entry_fallback = file_entry.clone();

        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| match file_type {
            TypeOfFile::Image => Some(Some(Self::check_broken_image(file_entry))),
            TypeOfFile::ArchiveZip => Some(Self::check_broken_zip(file_entry)),
            TypeOfFile::Archive7z => Some(Some(Self::check_broken_7z(file_entry))),
            TypeOfFile::ArchiveGz => Some(Self::check_broken_gz(file_entry)),
            TypeOfFile::ArchiveTar => Some(Self::check_broken_tar(file_entry)),
            TypeOfFile::ArchiveZst => Some(Self::check_broken_zst(file_entry)),
            TypeOfFile::Font => Some(Self::check_broken_font(file_entry)),
            TypeOfFile::Json => Some(Self::check_broken_json(file_entry)),
            TypeOfFile::Xml => Some(Self::check_broken_xml(file_entry)),
            TypeOfFile::Toml => Some(Self::check_broken_toml(file_entry)),
            TypeOfFile::Yaml => Some(Self::check_broken_yaml(file_entry)),
            TypeOfFile::Svg => Some(Self::check_broken_svg(file_entry)),
            TypeOfFile::ArchiveBz2 => Some(Self::check_broken_bz2(file_entry)),
            TypeOfFile::ArchiveXz => Some(Self::check_broken_xz(file_entry)),
            TypeOfFile::Audio => Self::check_broken_audio(file_entry, stop_flag),
            TypeOfFile::Pdf => Some(Some(Self::check_broken_pdf(file_entry))),
            TypeOfFile::Video => Self::check_broken_video(file_entry, stop_flag, checked_types).map(Some),
        }));

        match result {
            Ok(v) => v,
            Err(_) => {
                let checked_type_single = Self::file_type_to_checked_type_single(file_type);
                let message = create_crash_message_generic(&file_entry_fallback.path.to_string_lossy());
                error!("{message}");
                file_entry_fallback.errors.insert(checked_type_single, Some(message));
                Some(Some(file_entry_fallback))
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
        let checked_types = self.params.checked_types;

        debug!("look_for_broken_files - started finding for broken files");
        let mut vec_file_entry: Vec<BrokenEntry> = non_cached_files_to_check
            .into_par_iter()
            .with_max_len(3)
            .map(|(_, file_entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let size = file_entry.size;
                let res = Self::check_file(file_entry, stop_flag, checked_types);

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

        self.broken_files = vec_file_entry.into_iter().filter(|f| f.has_errors()).collect();

        self.information.number_of_broken_files = self.broken_files.len();
        debug!("Found {} broken files.", self.information.number_of_broken_files);
        // Clean unused data
        self.files_to_check = Default::default();

        WorkContinueStatus::Continue
    }
}

#[expect(clippy::string_slice)] // Valid, because we address up to the dot, which is known ascii character
fn check_extension_availability(full_name: &Path) -> Option<TypeOfFile> {
    let Some(file_name) = full_name.file_name() else {
        error!("Missing file name in file - \"{}\"", full_name.to_string_lossy());
        debug_assert!(false, "Missing file name in file - \"{}\"", full_name.to_string_lossy());
        return None;
    };

    // Faster manual conversion than using Path::extension()
    let file_name_str = file_name.to_str()?;
    let extension_idx = file_name_str.rfind('.')?;
    let extension_str = &file_name_str[extension_idx + 1..];

    let extension_lowercase = extension_str.to_ascii_lowercase();

    if IMAGE_RS_BROKEN_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Image)
    } else if ZIP_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::ArchiveZip)
    } else if SEVENZ_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Archive7z)
    } else if GZ_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::ArchiveGz)
    } else if TAR_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::ArchiveTar)
    } else if ZST_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::ArchiveZst)
    } else if FONT_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Font)
    } else if JSON_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Json)
    } else if SVG_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Svg)
    } else if XML_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Xml)
    } else if TOML_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Toml)
    } else if YAML_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Yaml)
    } else if BZ2_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::ArchiveBz2)
    } else if XZ_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::ArchiveXz)
    } else if PDF_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Pdf)
    } else if AUDIO_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Audio)
    } else if VIDEO_FILES_EXTENSIONS.contains(&extension_lowercase.as_str()) {
        Some(TypeOfFile::Video)
    } else {
        error!("File with unknown extension: \"{}\" - {extension_lowercase}", full_name.to_string_lossy());
        debug_assert!(false, "File with unknown extension - \"{}\" - {extension_lowercase}", full_name.to_string_lossy());
        None
    }
}

pub fn get_broken_files_cache_file() -> String {
    format!("cache_broken_files_{CACHE_BROKEN_FILES_VERSION}.bin")
}
