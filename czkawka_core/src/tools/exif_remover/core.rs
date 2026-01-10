use std::collections::BTreeMap;
use std::io::{Read, Seek};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::{fs, mem, panic};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use little_exif::exif_tag::ExifTag;
use little_exif::filetype::FileExtension;
use little_exif::ifd::ExifTagGroup;
use little_exif::metadata::Metadata;
use log::{debug, error, info};
use rayon::prelude::*;

use crate::common::cache::{CACHE_VERSION, extract_loaded_cache, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::consts::EXIF_FILES_EXTENSIONS;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::tools::exif_remover::{ExifEntry, ExifRemover, ExifRemoverParameters, ExifTagInfo, ExifTagsFixerParams, Info};

impl ExifRemover {
    pub fn new(params: ExifRemoverParameters) -> Self {
        let mut additional_excluded_tags = BTreeMap::new();

        let tiff_disabled_tags = vec![
            "ImageWidth",
            "ImageHeight",
            "BitsPerSample",
            "Compression",
            "PhotometricInterpretation",
            "StripOffsets",
            "SamplesPerPixel",
            "RowsPerStrip",
            "StripByteCounts",
            "PlanarConfiguration",
        ];
        for i in ["tif", "tiff"] {
            additional_excluded_tags.insert(i, tiff_disabled_tags.clone());
        }
        Self {
            common_data: CommonToolData::new(ToolType::ExifRemover),
            information: Info::default(),
            exif_files: Vec::new(),
            files_to_check: Default::default(),
            params,
            additional_excluded_tags,
        }
    }

    #[fun_time(message = "find_exif_files", level = "debug")]
    pub(crate) fn find_exif_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        self.common_data.extensions.set_and_validate_allowed_extensions(EXIF_FILES_EXTENSIONS);
        if !self.common_data.extensions.set_any_extensions() {
            return WorkContinueStatus::Continue;
        }
        let result = DirTraversalBuilder::new()
            .common_data(&self.common_data)
            .group_by(|_fe| ())
            .stop_flag(stop_flag)
            .progress_sender(progress_sender)
            .build()
            .run();

        match result {
            DirTraversalResult::SuccessFiles { grouped_file_entries, warnings } => {
                self.files_to_check = grouped_file_entries
                    .into_values()
                    .flatten()
                    .map(|fe| {
                        let exif_entry = ExifEntry {
                            path: fe.path.clone(),
                            size: fe.size,
                            modified_date: fe.modified_date,
                            exif_tags: Vec::new(),
                            error: None,
                        };
                        (fe.path.to_string_lossy().to_string(), exif_entry)
                    })
                    .collect();

                self.common_data.text_messages.warnings.extend(warnings);
                debug!("find_exif_files - Found {} files to check.", self.files_to_check.len());

                WorkContinueStatus::Continue
            }

            DirTraversalResult::Stopped => WorkContinueStatus::Stop,
        }
    }

    #[fun_time(message = "load_cache", level = "debug")]
    fn load_cache(
        &mut self,
        _stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
    ) -> (BTreeMap<String, ExifEntry>, BTreeMap<String, ExifEntry>, BTreeMap<String, ExifEntry>) {
        let loaded_hash_map;

        let mut records_already_cached: BTreeMap<String, ExifEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, ExifEntry> = Default::default();
        let files_to_check = mem::take(&mut self.files_to_check);

        if self.common_data.use_cache {
            let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::ExifRemoverCacheLoading, 0, self.get_test_type(), 0);

            let (messages, loaded_items) = load_cache_from_file_generalized_by_path::<ExifEntry>(&get_exif_remover_cache_file(), self.get_delete_outdated_cache(), &files_to_check);
            self.get_text_messages_mut().extend_with_another_messages(messages);
            loaded_hash_map = loaded_items.unwrap_or_default();

            extract_loaded_cache(&loaded_hash_map, files_to_check, &mut records_already_cached, &mut non_cached_files_to_check);

            progress_handler.join_thread();
        } else {
            loaded_hash_map = Default::default();
            non_cached_files_to_check = files_to_check;
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    #[fun_time(message = "save_to_cache", level = "debug")]
    fn save_to_cache(
        &mut self,
        vec_file_entry: &[ExifEntry],
        loaded_hash_map: BTreeMap<String, ExifEntry>,
        _stop_flag: &Arc<AtomicBool>,
        progress_sender: Option<&Sender<ProgressData>>,
    ) {
        if self.common_data.use_cache {
            let progress_handler = prepare_thread_handler_common(progress_sender, CurrentStage::ExifRemoverCacheSaving, 0, self.get_test_type(), 0);

            let mut all_results: BTreeMap<String, ExifEntry> = Default::default();

            for file_entry in vec_file_entry.iter().cloned() {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            for (name, file_entry) in loaded_hash_map {
                all_results.insert(name, file_entry);
            }

            let messages = save_cache_to_file_generalized(&get_exif_remover_cache_file(), &all_results, self.common_data.save_also_as_json, 0);
            self.get_text_messages_mut().extend_with_another_messages(messages);

            progress_handler.join_thread();
        }
    }

    #[fun_time(message = "check_exif_in_files", level = "debug")]
    pub(crate) fn check_exif_in_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.files_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache(stop_flag, progress_sender);

        let progress_handler = prepare_thread_handler_common(
            progress_sender,
            CurrentStage::ExifRemoverExtractingTags,
            non_cached_files_to_check.len(),
            self.get_test_type(),
            non_cached_files_to_check.values().map(|item| item.size).sum::<u64>(),
        );

        let non_cached_files_to_check = non_cached_files_to_check.into_iter().collect::<Vec<_>>();

        debug!("check_exif_in_files - started extracting EXIF data");
        let mut vec_file_entry: Vec<ExifEntry> = non_cached_files_to_check
            .into_par_iter()
            .map(|(_, mut file_entry)| {
                if check_if_stop_received(stop_flag) {
                    return None;
                }

                let size = file_entry.size;
                let res = extract_exif_tags(&file_entry.path);

                progress_handler.increase_items(1);
                progress_handler.increase_size(size);

                match res {
                    Ok(tags) if !tags.is_empty() => {
                        file_entry.exif_tags = tags.into_iter().map(|(name, code, group)| ExifTagInfo { name, code, group }).collect();
                    }
                    Ok(_) => {}
                    Err(e) => {
                        file_entry.error = Some(e);
                    }
                }

                Some(file_entry)
            })
            .while_some()
            .collect();
        debug!("check_exif_in_files - finished extracting EXIF data");

        progress_handler.join_thread();

        vec_file_entry.extend(records_already_cached.into_values());

        self.save_to_cache(&vec_file_entry, loaded_hash_map, stop_flag, progress_sender);

        if check_if_stop_received(stop_flag) {
            return WorkContinueStatus::Stop;
        }

        // After saving to cache, remove ignored tags - because in cache we need to store full info about tags
        for entry in &mut vec_file_entry {
            let extension = entry.path.extension().and_then(|ext| ext.to_str()).unwrap_or("").to_lowercase();
            if let Some(additional_ignored_tags) = self.additional_excluded_tags.get(&extension.as_str()) {
                entry.exif_tags.retain(|tag_item| !additional_ignored_tags.contains(&tag_item.name.as_str()));
            }
            if self.params.ignored_tags.is_empty() {
                continue;
            }

            entry.exif_tags.retain(|tag_item| !self.params.ignored_tags.contains(&tag_item.name));
        }

        self.exif_files = vec_file_entry.into_iter().filter(|f| f.error.is_none() && !f.exif_tags.is_empty()).collect();
        self.exif_files.iter_mut().for_each(|file| file.exif_tags.sort_unstable_by_key(|tag| tag.name.clone()));

        self.information.number_of_files_with_exif = self.exif_files.len();
        debug!("Found {} files with EXIF data.", self.information.number_of_files_with_exif);

        self.files_to_check = Default::default();

        WorkContinueStatus::Continue
    }

    #[fun_time(message = "fix_files", level = "debug")]
    pub(crate) fn fix_files(&mut self, stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>, fix_params: ExifTagsFixerParams) -> WorkContinueStatus {
        info!("Starting optimization of {} video files", self.exif_files.len());

        self.exif_files.par_iter_mut().for_each(|entry| {
            if check_if_stop_received(stop_flag) {
                return;
            }

            let exif_data_to_remove: Vec<(u16, String)> = entry.exif_tags.iter().map(|item_tag| (item_tag.code, item_tag.group.clone())).collect();
            match clean_exif_tags(&entry.path.to_string_lossy(), &exif_data_to_remove, fix_params.override_file) {
                Ok(_number_removed_tags) => {}
                Err(e) => {
                    entry.error = Some(e);
                }
            }
        });

        WorkContinueStatus::Continue
    }
}

pub fn clean_exif_tags(file_path: &str, tags_to_remove: &[(u16, String)], override_file: bool) -> Result<u32, String> {
    panic::catch_unwind(|| {
        let file_path = Path::new(file_path);
        let mut file_data = fs::read(file_path).map_err(|e| e.to_string())?;
        let mut cursor = std::io::Cursor::new(&file_data);
        let ext = auto_detect_file_extension(&mut cursor).ok_or_else(|| "Failed to detect file type".to_string())?;
        let metadata = Metadata::new_from_vec(&file_data, ext).map_err(|e| format!("Failed to read EXIF: {e}"))?;

        let mut new_metadata = metadata;
        let mut tags_removed: u32 = 0;
        for (tag_u16, tag_group) in tags_to_remove {
            let Ok(tag_group) = string_to_exif_tag_group(tag_group) else {
                error!("Unknown EXIF tag group string: {tag_group}, skipping tag removal.");
                continue;
            };

            // TODO https://github.com/TechnikTobi/little_exif/pull/92
            let Ok(tag) = ExifTag::from_u16(*tag_u16, &tag_group) else {
                continue;
            };
            new_metadata.remove_tag(tag);
            tags_removed += 1;
        }

        new_metadata.write_to_vec(&mut file_data, ext).map_err(|e| e.to_string())?;
        if override_file {
            fs::write(file_path, file_data).map_err(|e| e.to_string())?;
        } else {
            let extension = file_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
            let new_file_path = file_path.with_extension(format!("czkawka_cleaned_exif.{extension}"));
            fs::write(new_file_path, file_data).map_err(|e| e.to_string())?;
        }

        Ok(tags_removed)
    })
    .map_err(|e| format!("Panic occurred while reading EXIF: {e:?}"))?
    .map_err(|e: String| format!("Failed to remove EXIF from file {file_path} - {e}"))
}

fn extract_exif_tags(path: &Path) -> Result<Vec<(String, u16, String)>, String> {
    panic::catch_unwind(|| {
        let file_path = Path::new(path);
        let data = fs::read(file_path).map_err(|e| e.to_string())?;
        let mut cursor = std::io::Cursor::new(&data);
        let ext = auto_detect_file_extension(&mut cursor).ok_or_else(|| "Failed to detect file type".to_string())?;
        let metadata = Metadata::new_from_vec(&data, ext).map_err(|e| format!("Failed to read EXIF: {e}"))?;

        let mut tags = Vec::new();

        for tag in &metadata {
            let tag_name = format!("{tag:?}");
            let tag_u16 = tag.as_u16();
            let tag_group = exif_tag_group_to_string(tag.get_group());
            if let Some(pos) = tag_name.find('(') {
                #[expect(clippy::string_slice)] // Safe, because pos is from find
                tags.push((tag_name[..pos].to_string(), tag_u16, tag_group));
            } else {
                tags.push((tag_name, tag_u16, tag_group));
            }
        }

        Ok(tags)
    })
    .map_err(|e| format!("Panic occurred while reading \"{}\" - EXIF: {e:?}", path.to_string_lossy()))?
}

pub(crate) fn auto_detect_file_extension<T: Seek + Read>(cursor: &mut T) -> Option<FileExtension> {
    let mut buffer = [0; 32];
    let Ok(n) = cursor.read(&mut buffer) else {
        return None;
    };
    if n < 4 {
        return None;
    }

    match buffer {
            // PNG
            [0x89, 0x50, 0x4E, 0x47, ..] => {
                Some(FileExtension::PNG { as_zTXt_chunk: true })
            }

            // JP(E)G
            [0xFF, 0xD8, ..] => {
                Some(FileExtension::JPEG)
            }

            // TIFF, little endian
            [0x49, 0x49, 0x2A, 0x00, ..] |
            [0x4D, 0x4D, 0x00, 0x2A, ..] => {
                Some(FileExtension::TIFF)
            }

            // WebP
            [0x52, 0x49, 0x46, 0x46, _, _, _, _, 0x57, 0x45, 0x42, 0x50, ..] =>
                {
                    Some(FileExtension::WEBP)
                }

            // A "naked" JXL codestream that can't hold metadata
            // See: https://www.loc.gov/preservation/digital/formats/fdd/fdd000538.shtml
            [0xFF, 0x0A, ..] => {
                Some(FileExtension::NAKED_JXL)
            }

            // JXL (in ISO_BMFF container)
            // In this case, the JXL file starts with the JXL signature box
            // 4 bytes for length       J     X     L  space more stuff
            [0x00, 0x00, 0x00, 0x0C, 0x4A, 0x58, 0x4C, 0x20, 0x0D, 0x0A, 0x87, 0x0A, ..] =>
                {
                    Some(FileExtension::JXL)
                }

            // HEIC/HEIF/AVIF
            // length       f     t     y     p
            [_, _, _, _, 0x66, 0x74, 0x79, 0x70, 0x68, 0x65, 0x69, 0x63 | 0x66, ..] |
[_, _, _, _, 0x66, 0x74, 0x79, 0x70, 0x61, 0x76, 0x69, 0x66, ..]  // avif
            =>
                {
                    Some(FileExtension::HEIF)
                }

            _ => {
                None
            }
        }
}

pub fn file_extension_to_string(extension: FileExtension) -> &'static str {
    match extension {
        FileExtension::PNG { .. } => "PNG",
        FileExtension::JPEG => "JPEG",
        FileExtension::TIFF => "TIFF",
        FileExtension::WEBP => "WEBP",
        FileExtension::NAKED_JXL => "NAKED_JXL",
        FileExtension::JXL => "JXL",
        FileExtension::HEIF => "HEIF",
    }
}
pub fn string_to_file_extension(s: &str) -> FileExtension {
    match s {
        "PNG" => FileExtension::PNG { as_zTXt_chunk: true },
        "JPEG" => FileExtension::JPEG,
        "TIFF" => FileExtension::TIFF,
        "WEBP" => FileExtension::WEBP,
        "NAKED_JXL" => FileExtension::NAKED_JXL,
        "JXL" => FileExtension::JXL,
        "HEIF" => FileExtension::HEIF,
        _ => {
            error!("Unknown file extension string: {s}, defaulting to JPEG");
            FileExtension::JPEG
        } // Default to JPEG
    }
}

// Nom-exif implementation
// Probably will use this version in future
// fn extract_exif_tags2(path: &Path) -> Result<Vec<String>, String> {
//     let res = panic::catch_unwind(|| {
//         let mut parser = nom_exif::MediaParser::new();
//         let ms = nom_exif::MediaSource::file_path(path).map_err(|e| format!("Failed to open file: {e}"))?;
//         let mut results = Vec::new();
//         if !ms.has_exif() {
//             return Ok(results);
//         }
//         let exif_iter: nom_exif::ExifIter = parser.parse(ms).map_err(|e| format!("Failed to parse EXIF data: {e}"))?;
//         for exif_entry in exif_iter {
//             results.push(exif_entry.tag().map_or_else(|| "Unknown".to_string(), |t| format!("{t:?}")));
//         }
//
//         Ok(results)
//     });
//
//     res.unwrap_or_else(|_| {
//         let message = crate::common::create_crash_message("nom-exif", path.to_string_lossy().as_ref(), "https://github.com/mindeng/nom-exif");
//         error!("{message}");
//         Err("Panic in get_rotation_from_exif".to_string())
//     })
// }

pub fn string_to_exif_tag_group(tag: &str) -> Result<ExifTagGroup, String> {
    match tag {
        "EXIF" => Ok(ExifTagGroup::EXIF),
        "INTEROP" => Ok(ExifTagGroup::INTEROP),
        "GPS" => Ok(ExifTagGroup::GPS),
        "GENERIC" => Ok(ExifTagGroup::GENERIC),
        _ => Err(format!("Unknown EXIF tag group: {tag}")),
    }
}

pub fn exif_tag_group_to_string(tag_group: ExifTagGroup) -> String {
    match tag_group {
        ExifTagGroup::EXIF => "EXIF".to_string(),
        ExifTagGroup::INTEROP => "INTEROP".to_string(),
        ExifTagGroup::GPS => "GPS".to_string(),
        ExifTagGroup::GENERIC => "GENERIC".to_string(),
    }
}

pub fn get_exif_remover_cache_file() -> String {
    format!("cache_exif_remover_{CACHE_VERSION}.bin")
}
