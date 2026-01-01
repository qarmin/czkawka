use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::{mem, panic};

use crossbeam_channel::Sender;
use fun_time::fun_time;
use little_exif::metadata::Metadata;
use log::debug;
use rayon::prelude::*;

use crate::common::cache::{CACHE_VERSION, extract_loaded_cache, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::consts::EXIF_FILES_EXTENSIONS;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData};
use crate::tools::exif_remover::{ExifEntry, ExifRemover, ExifRemoverParameters, Info};

impl ExifRemover {
    pub fn new(params: ExifRemoverParameters) -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::ExifRemover),
            information: Info::default(),
            exif_files: vec![],
            files_to_check: Default::default(),
            params,
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
                            exif_tags: vec![],
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
    fn load_cache(&mut self) -> (BTreeMap<String, ExifEntry>, BTreeMap<String, ExifEntry>, BTreeMap<String, ExifEntry>) {
        let loaded_hash_map;

        let mut records_already_cached: BTreeMap<String, ExifEntry> = Default::default();
        let mut non_cached_files_to_check: BTreeMap<String, ExifEntry> = Default::default();
        let files_to_check = mem::take(&mut self.files_to_check);

        if self.common_data.use_cache {
            let (messages, loaded_items) = load_cache_from_file_generalized_by_path::<ExifEntry>(&get_exif_remover_cache_file(), self.get_delete_outdated_cache(), &files_to_check);
            self.get_text_messages_mut().extend_with_another_messages(messages);
            loaded_hash_map = loaded_items.unwrap_or_default();

            extract_loaded_cache(&loaded_hash_map, files_to_check, &mut records_already_cached, &mut non_cached_files_to_check);
        } else {
            loaded_hash_map = Default::default();
            non_cached_files_to_check = files_to_check;
        }
        (loaded_hash_map, records_already_cached, non_cached_files_to_check)
    }

    #[fun_time(message = "save_to_cache", level = "debug")]
    fn save_to_cache(&mut self, vec_file_entry: &[ExifEntry], loaded_hash_map: BTreeMap<String, ExifEntry>) {
        if self.common_data.use_cache {
            // Must save all results to file, old loaded from file with all currently counted results
            let mut all_results: BTreeMap<String, ExifEntry> = Default::default();

            for file_entry in vec_file_entry.iter().cloned() {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }
            for (_name, file_entry) in loaded_hash_map {
                all_results.insert(file_entry.path.to_string_lossy().to_string(), file_entry);
            }

            let messages = save_cache_to_file_generalized(&get_exif_remover_cache_file(), &all_results, self.common_data.save_also_as_json, 0);
            self.get_text_messages_mut().extend_with_another_messages(messages);
        }
    }

    #[fun_time(message = "check_exif_in_files", level = "debug")]
    pub(crate) fn check_exif_in_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.files_to_check.is_empty() {
            return WorkContinueStatus::Continue;
        }

        let (loaded_hash_map, records_already_cached, non_cached_files_to_check) = self.load_cache();

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
                    return file_entry;
                }

                let size = file_entry.size;
                let res = extract_exif_tags(&file_entry.path);

                progress_handler.increase_items(1);
                progress_handler.increase_size(size);

                match res {
                    Ok(tags) if !tags.is_empty() => {
                        file_entry.exif_tags = tags;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        file_entry.error = Some(e);
                    }
                }

                file_entry
            })
            .collect();
        debug!("check_exif_in_files - finished extracting EXIF data");

        vec_file_entry.extend(records_already_cached.into_values());

        self.save_to_cache(&vec_file_entry, loaded_hash_map);
        progress_handler.join_thread();

        if !self.params.ignored_tags.is_empty() {
            for entry in &mut vec_file_entry {
                entry.exif_tags.retain(|tag| !self.params.ignored_tags.contains(tag));
            }
        }

        self.exif_files = vec_file_entry.into_iter().filter(|f| f.error.is_none() && !f.exif_tags.is_empty()).collect();

        self.information.number_of_files_with_exif = self.exif_files.len();
        debug!("Found {} files with EXIF data.", self.information.number_of_files_with_exif);

        self.files_to_check = Default::default();

        WorkContinueStatus::Continue
    }
}

fn extract_exif_tags(path: &Path) -> Result<Vec<String>, String> {
    let metadata = panic::catch_unwind(|| Metadata::new_from_path(path))
        .map_err(|e| format!("Panic occurred while reading EXIF: {e:?}"))?
        .map_err(|e| format!("Failed to read EXIF: {e}"))?;

    let mut tags = Vec::new();

    for tag in &metadata {
        let tag_name = format!("{tag:?}");
        if let Some(pos) = tag_name.find('(') {
            #[expect(clippy::string_slice)] // Safe, because pos is from find
            tags.push(tag_name[..pos].to_string());
        } else {
            tags.push(tag_name);
        }
    }

    tags.sort();
    tags.dedup();

    Ok(tags)
}

// Nom-exif implementation
// Probably will use this version in future
// fn extract_exif_tags2(path: &Path) -> Result<Vec<String>, String> {
//     let res = panic::catch_unwind(|| {
//         let mut parser = MediaParser::new();
//         let ms = MediaSource::file_path(path).map_err(|e| format!("Failed to open file: {e}"))?;
//         let mut results = vec![];
//         if !ms.has_exif() {
//             return Ok(results);
//         }
//         let exif_iter: ExifIter = parser.parse(ms).map_err(|e| format!("Failed to parse EXIF data: {e}"))?;
//         for exif_entry in exif_iter {
//             results.push(exif_entry.tag().map_or_else(|| "Unknown".to_string(), |t| format!("{t:?}")));
//         }
//         Ok(results)
//     });
//
//     res.unwrap_or_else(|_| {
//         let message = create_crash_message("nom-exif", path.to_string_lossy().as_ref(), "https://github.com/mindeng/nom-exif");
//         error!("{message}");
//         Err("Panic in get_rotation_from_exif".to_string())
//     })
// }

pub fn get_exif_remover_cache_file() -> String {
    format!("cache_exif_remover_{CACHE_VERSION}.bin")
}
