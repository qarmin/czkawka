use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use crossbeam_channel::Sender;
use fun_time::fun_time;

use crate::common::model::WorkContinueStatus;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonData, CommonToolData, DeleteItemType, DeleteMethod};
use crate::common::traits::{AllTraits, DebugPrint, DeletingItems, PrintResults, Search};
use crate::tools::similar_documents::{Info, SimilarDocuments, SimilarDocumentsParameters};

impl AllTraits for SimilarDocuments {}

impl Search for SimilarDocuments {
    #[fun_time(message = "find_similar_documents", level = "info")]
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        let start_time = Instant::now();

        let () = (|| {
            if self.collect_documents(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            if self.compute_signatures(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
            }
            if self.compare_signatures(stop_flag, progress_sender) == WorkContinueStatus::Stop {
                self.common_data.stopped_search = true;
                return;
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

impl DebugPrint for SimilarDocuments {
    fn debug_print(&self) {
        if !cfg!(debug_assertions) || cfg!(test) {
            return;
        }
        self.debug_print_common();
        println!(
            "Found {} groups with {} similar documents",
            self.information.number_of_groups, self.information.number_of_similar_documents
        );
    }
}

impl PrintResults for SimilarDocuments {
    fn write_results<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        self.write_base_search_paths(writer)?;

        if !self.similar_documents.is_empty() {
            writeln!(
                writer,
                "Found {} groups with {} similar documents.",
                self.information.number_of_groups, self.information.number_of_similar_documents
            )?;
            for group in &self.similar_documents {
                writeln!(writer, "---- Group ----")?;
                for entry in group {
                    writeln!(
                        writer,
                        "\"{}\" (size: {}, similarity: {:.1}%)",
                        entry.path.to_string_lossy(),
                        entry.size,
                        entry.similarity * 100.0
                    )?;
                }
            }
        } else {
            write!(writer, "Not found any similar documents.")?;
        }

        Ok(())
    }

    fn save_results_to_file_as_json(&self, file_name: &str, pretty_print: bool) -> std::io::Result<()> {
        self.save_results_to_file_as_json_internal(file_name, &self.similar_documents, pretty_print)
    }
}

impl DeletingItems for SimilarDocuments {
    #[fun_time(message = "delete_files", level = "debug")]
    fn delete_files(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) -> WorkContinueStatus {
        match self.common_data.delete_method {
            DeleteMethod::Delete => {
                let all_entries: Vec<_> = self.similar_documents.iter().flat_map(|g| g.clone()).collect();
                self.delete_simple_elements_and_add_to_messages(stop_flag, progress_sender, DeleteItemType::DeletingFiles(all_entries))
            }
            DeleteMethod::None => WorkContinueStatus::Continue,
            _ => unreachable!(),
        }
    }
}

impl CommonData for SimilarDocuments {
    type Info = Info;
    type Parameters = SimilarDocumentsParameters;

    fn get_information(&self) -> Self::Info {
        self.information
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
    fn found_any_items(&self) -> bool {
        self.information.number_of_similar_documents > 0
    }
}
