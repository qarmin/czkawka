

use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{mem, panic};

use anyhow::Context;
use crossbeam_channel::Sender;
use fun_time::fun_time;
use humansize::{BINARY, format_size};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::prelude::*;
use lofty::read_from;
use log::{debug, error};
use rayon::prelude::*;
use rusty_chromaprint::{Configuration, Fingerprinter, match_fingerprints};
use serde::{Deserialize, Serialize};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::common::cache::{extract_loaded_cache, load_cache_from_file_generalized_by_path, save_cache_to_file_generalized};
use crate::common::consts::AUDIO_FILES_EXTENSIONS;
use crate::common::create_crash_message;
use crate::common::dir_traversal::{DirTraversalBuilder, DirTraversalResult};
use crate::common::model::{CheckingMethod, FileEntry, ToolType, WorkContinueStatus};
use crate::common::progress_data::{CurrentStage, ProgressData};
use crate::common::progress_stop_handler::{check_if_stop_received, prepare_thread_handler_common};
use crate::common::tool_data::{CommonData, CommonToolData, DeleteMethod};
use crate::common::traits::*;
use crate::tools::same_music::{Info, MusicEntry, SameMusic, SameMusicParameters};

impl DebugPrint for SameMusic {
    #[allow(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) {
            return;
        }
        println!("---------------DEBUG PRINT---------------");
        println!("Found files music - {}", self.music_entries.len());
        println!("Found duplicated files music - {}", self.duplicated_music_entries.len());
        self.debug_print_common();
        println!("-----------------------------------------");
    }
}

impl PrintResults for SameMusic {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        if !self.duplicated_music_entries.is_empty() {
            writeln!(writer, "{} music files which have similar friends\n\n.", self.duplicated_music_entries.len())?;

            for vec_file_entry in &self.duplicated_music_entries {
                writeln!(writer, "Found {} music files which have similar friends", vec_file_entry.len())?;
                for file_entry in vec_file_entry {
                    write_music_entry(writer, file_entry)?;
                }
                writeln!(writer)?;
            }
        } else if !self.duplicated_music_entries_referenced.is_empty() {
            writeln!(writer, "{} music files which have similar friends\n\n.", self.duplicated_music_entries_referenced.len())?;
            for (file_entry, vec_file_entry) in &self.duplicated_music_entries_referenced {
                writeln!(writer, "Found {} music files which have similar friends", vec_file_entry.len())?;
                writeln!(writer)?;
                write_music_entry(writer, file_entry)?;
                for file_entry in vec_file_entry {
                    write_music_entry(writer, file_entry)?;
                }
                writeln!(writer)?;
            }
        } else {
            write!(writer, "Not found any similar music files.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        if self.get_use_reference() {
            self.save_results_to_file_as_json_internal(file_name, &self.duplicated_music_entries_referenced, pretty_print)
        } else {
            self.save_results_to_file_as_json_internal(file_name, &self.duplicated_music_entries, pretty_print)
        }
    }
}

fn write_music_entry<T: Write>(writer: &mut T, file_entry: &MusicEntry) -> std::io::Result<()> {
    writeln!(
        writer,
        "TT: {}  -  TA: {}  -  Y: {}  -  L: {}  -  G: {}  -  B: {}  -  P: \"{}\"",
        file_entry.track_title,
        file_entry.track_artist,
        file_entry.year,
        file_entry.length,
        file_entry.genre,
        file_entry.bitrate,
        file_entry.path.to_string_lossy()
    )
}

impl CommonData for SameMusic {
    type Info = Info;
    type Parameters = SameMusicParameters;

    fn get_information(&self) -> Self::Info {
        self.information.clone()
    }
    fn get_params(&self) -> Self::Parameters {
        self.params.clone()
    }
    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }
    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }
    fn get_check_method(&self) -> CheckingMethod {
        self.get_params().check_type
    }
    fn found_any_broken_files(&self) -> bool {
        self.information.number_of_duplicates > 0
    }
}


impl DeletingItems for SameMusic {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        if self.get_cd().delete_method == DeleteMethod::None {
            return WorkContinueStatus::Continue;
        }
        let files_to_delete = self.duplicated_music_entries.clone();
        self.delete_advanced_elements_and_add_to_messages(stop_flag, progress_sender, files_to_delete)
    }
}
