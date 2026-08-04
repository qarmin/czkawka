use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use crossbeam_channel::Sender;

use crate::common::model::ToolType;
use crate::common::progress_data::ProgressData;
use crate::common::tool_data::{CommonToolData, CommonData as CommonDataTrait};
use crate::common::traits::Search;

pub mod config;

#[derive(Debug, Clone)]
pub struct MediaMetadata {
    pub artist: Option<String>,
    pub album: Option<String>,
    pub title: Option<String>,
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct MoveOperation {
    pub source: PathBuf,
    pub destination: PathBuf,
}

#[derive(Clone, Default)]
pub struct OrganizerInfo;

#[derive(Clone, Default)]
pub struct OrganizerParameters;

pub struct MediaOrganizer {
    pub common_data: CommonToolData,
    pub workflows: Vec<config::Workflow>,
    pub pending_operations: Vec<MoveOperation>,
    pub parameters: OrganizerParameters,
    pub info: OrganizerInfo,
}

impl Default for MediaOrganizer {
    fn default() -> Self {
        Self {
            common_data: CommonToolData::new(ToolType::Duplicate), // Mock ToolType for now
            workflows: config::OrganizerConfig::default_workflows(),
            pending_operations: Vec::new(),
            parameters: OrganizerParameters::default(),
            info: OrganizerInfo::default(),
        }
    }
}

impl MediaOrganizer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_included_paths(&mut self, mut paths: Vec<PathBuf>) {
        self.common_data.directories.included_directories.append(&mut paths);
    }

    pub fn generate_destination_path(file_path: &Path, base_dest: &Path, pattern: &str, metadata: &MediaMetadata) -> PathBuf {
        let mut result = pattern.to_string();

        // Basic interpolation
        result = result.replace("{artist}", metadata.artist.as_deref().unwrap_or("Unknown Artist"));
        result = result.replace("{album}", metadata.album.as_deref().unwrap_or("Unknown Album"));
        result = result.replace("{title}", metadata.title.as_deref().unwrap_or("Unknown Title"));

        let year_str = metadata.year.map(|y| y.to_string()).unwrap_or_else(|| "Unknown".to_string());
        result = result.replace("{year}", &year_str);

        let month_str = metadata.month.map(|m| format!("{:02}", m)).unwrap_or_else(|| "Unknown".to_string());
        result = result.replace("{month}", &month_str);

        let day_str = metadata.day.map(|d| format!("{:02}", d)).unwrap_or_else(|| "Unknown".to_string());
        result = result.replace("{day}", &day_str);

        if let Some(file_name) = file_path.file_stem() {
            result = result.replace("{filename}", &file_name.to_string_lossy());
        }

        if let Some(ext) = file_path.extension() {
            result = result.replace("{ext}", &ext.to_string_lossy());
        }

        base_dest.join(result)
    }

    pub fn organize_files(&mut self, _stop_flag: &Arc<AtomicBool>, _progress_sender: Option<&Sender<ProgressData>>) {
        // Mock implementation for now, will scan directories and prepare move operations
        // and populate self.pending_operations
    }

    pub fn move_files(&self, operations: Vec<MoveOperation>, dry_run: bool) {
        if dry_run {
            for op in operations {
                println!("Would move {:?} to {:?}", op.source, op.destination);
            }
            return;
        }

        for op in operations {
            if let Some(parent) = op.destination.parent() {
                let _ = fs::create_dir_all(parent);
            }
            // Add actual rename / copy logic with conflict resolution
            let _ = fs::rename(&op.source, &op.destination);
        }
    }
}

impl CommonDataTrait for MediaOrganizer {
    type Info = OrganizerInfo;
    type Parameters = OrganizerParameters;

    fn get_cd(&self) -> &CommonToolData {
        &self.common_data
    }

    fn get_cd_mut(&mut self) -> &mut CommonToolData {
        &mut self.common_data
    }

    fn get_information(&self) -> Self::Info {
        self.info.clone()
    }

    fn get_params(&self) -> Self::Parameters {
        self.parameters.clone()
    }

    fn found_any_items(&self) -> bool {
        !self.pending_operations.is_empty()
    }
}

impl Search for MediaOrganizer {
    fn search(&mut self, stop_flag: &Arc<AtomicBool>, progress_sender: Option<&Sender<ProgressData>>) {
        self.organize_files(stop_flag, progress_sender);
    }
}
