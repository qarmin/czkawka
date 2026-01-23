use std::io::prelude::*;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::consts::AUDIO_FILES_EXTENSIONS;
use crate::common::model::{CheckingMethod, WorkContinueStatus};
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::tools::same_music::core::format_audio_duration;
use crate::tools::same_music::{Info, MusicEntry, SameMusic, SameMusicParameters};

impl AllTraits for SameMusic {}

impl Search for SameMusic {
    #[fun_time(message = "find_same_music", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            if self.prepare_items(Some(AUDIO_FILES_EXTENSIONS)).is_err() {
                return;
            }
            self.common_data.use_reference_folders = !self.common_data.directories.reference_directories.is_empty() || !self.common_data.directories.reference_files.is_empty();
            if self.check_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            match self.params.check_type {
                CheckingMethod::AudioTags => {
                    if self.read_tags(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                        self.common_data.stopped_search = true;
                        return;
                    }
                    if self.check_for_duplicate_tags(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                        self.common_data.stopped_search = true;
                        return;
                    }
                }
                CheckingMethod::AudioContent => {
                    if self.read_tags(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                        self.common_data.stopped_search = true;
                        return;
                    }
                    if self.calculate_fingerprint(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                        self.common_data.stopped_search = true;
                        return;
                    }
                    if self.check_for_duplicate_fingerprints(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                        self.common_data.stopped_search = true;
                        return;
                    }
                }
                _ => panic!(),
            }
            if self.delete_files(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
            }
        })();

        self.information.scanning_time = start_time.elapsed();

        if !self.common_data.stopped_search {
            self.debug_print();
        }
    }
}

impl DebugPrint for SameMusic {
    #[expect(clippy::print_stdout)]
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
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
        format_audio_duration(file_entry.length),
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
