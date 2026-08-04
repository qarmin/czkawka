use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub extensions: Vec<String>,
    pub organization_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizerConfig {
    pub workflows: Vec<Workflow>,
}

impl OrganizerConfig {
    pub fn default_workflows() -> Vec<Workflow> {
        vec![
            Workflow {
                name: "Music".to_string(),
                extensions: vec!["mp3".to_string(), "flac".to_string(), "ogg".to_string(), "m4a".to_string()],
                organization_pattern: "Music/{artist}/{album}/{title}.{ext}".to_string(),
            },
            Workflow {
                name: "Photos".to_string(),
                extensions: vec!["jpg".to_string(), "jpeg".to_string(), "png".to_string(), "raw".to_string(), "heic".to_string()],
                organization_pattern: "Photos/{year}/{month}/{day}/{filename}.{ext}".to_string(),
            },
            Workflow {
                name: "Videos".to_string(),
                extensions: vec!["mp4".to_string(), "mkv".to_string(), "mov".to_string(), "avi".to_string()],
                organization_pattern: "Videos/{year}/{filename}.{ext}".to_string(),
            },
        ]
    }
}
