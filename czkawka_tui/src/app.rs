use ratatui::widgets::ListState;
use std::fs;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum InputMode {
    Normal,
    Search,
    Select,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ToolType {
    Duplicate,
    EmptyFolders,
    BigFiles,
    EmptyFiles,
    Temporary,
    SimilarImages,
    SameMusic,
    InvalidSymlinks,
    BrokenFiles,
    SimilarVideos,
    BadExtensions,
    BadNames,
    VideoOptimizer,
}

impl ToolType {
    pub fn name(&self) -> &'static str {
        match self {
            ToolType::Duplicate => "Duplicates",
            ToolType::EmptyFolders => "Empty Folders",
            ToolType::BigFiles => "Big Files",
            ToolType::EmptyFiles => "Empty Files",
            ToolType::Temporary => "Temporary",
            ToolType::SimilarImages => "Similar Images",
            ToolType::SameMusic => "Same Music",
            ToolType::InvalidSymlinks => "Invalid Symlinks",
            ToolType::BrokenFiles => "Broken Files",
            ToolType::SimilarVideos => "Similar Videos",
            ToolType::BadExtensions => "Bad Extensions",
            ToolType::BadNames => "Bad Names",
            ToolType::VideoOptimizer => "Video Optimizer",
        }
    }
}

pub struct ToolState {
    pub tool_type: ToolType,
    pub filter: String,
    pub all_items: Vec<String>,
    pub list_state: ListState,
    pub selected_items: std::collections::HashSet<usize>,
}

impl ToolState {
    pub fn new(tool_type: ToolType) -> Self {
        Self {
            tool_type,
            filter: String::new(),
            all_items: Vec::new(),
            list_state: ListState::default(),
            selected_items: std::collections::HashSet::new(),
        }
    }

    pub fn filtered_items(&self) -> Vec<(usize, &String)> {
        if self.filter.is_empty() {
            self.all_items.iter().enumerate().collect()
        } else {
            self.all_items
                .iter()
                .enumerate()
                .filter(|(_, s)| s.to_lowercase().contains(&self.filter.to_lowercase()))
                .collect()
        }
    }
}

pub struct App {
    pub input_mode: InputMode,
    pub active_tool_idx: usize,
    pub tools: Vec<ToolState>,
    pub status_message: String,
    pub scan_dir: String,
}

impl Default for App {
    fn default() -> Self {
        let tools = vec![
            ToolState::new(ToolType::Duplicate),
            ToolState::new(ToolType::EmptyFolders),
            ToolState::new(ToolType::BigFiles),
            ToolState::new(ToolType::EmptyFiles),
            ToolState::new(ToolType::Temporary),
            ToolState::new(ToolType::SimilarImages),
            ToolState::new(ToolType::SameMusic),
            ToolState::new(ToolType::InvalidSymlinks),
            ToolState::new(ToolType::BrokenFiles),
            ToolState::new(ToolType::SimilarVideos),
            ToolState::new(ToolType::BadExtensions),
            ToolState::new(ToolType::BadNames),
            ToolState::new(ToolType::VideoOptimizer),
        ];

        Self {
            input_mode: InputMode::Normal,
            active_tool_idx: 0,
            tools,
            status_message: String::new(),
            scan_dir: String::new(),
        }
    }
}

impl App {
    pub fn active_tool_mut(&mut self) -> &mut ToolState {
        &mut self.tools[self.active_tool_idx]
    }

    pub fn active_tool(&self) -> &ToolState {
        &self.tools[self.active_tool_idx]
    }

    pub fn next_tool(&mut self) {
        self.active_tool_idx = (self.active_tool_idx + 1) % self.tools.len();
        self.status_message.clear();
    }

    pub fn previous_tool(&mut self) {
        if self.active_tool_idx == 0 {
            self.active_tool_idx = self.tools.len() - 1;
        } else {
            self.active_tool_idx -= 1;
        }
        self.status_message.clear();
    }

    pub fn next(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        let items = tool.filtered_items();
        if items.is_empty() {
            tool.list_state.select(None);
            return;
        }

        let i = match tool.list_state.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        tool.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        let items = tool.filtered_items();
        if items.is_empty() {
            tool.list_state.select(None);
            return;
        }

        let i = match tool.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        tool.list_state.select(Some(i));
    }

    pub fn on_filter_change(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        tool.list_state.select(Some(0));
        let items = tool.filtered_items();
        if items.is_empty() {
             tool.list_state.select(None);
        }
    }

    pub fn toggle_selection(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        if let Some(i) = tool.list_state.selected() {
            let items = tool.filtered_items();
            if i < items.len() {
                let actual_idx = items[i].0;
                if tool.selected_items.contains(&actual_idx) {
                    tool.selected_items.remove(&actual_idx);
                } else {
                    tool.selected_items.insert(actual_idx);
                }
            }
        }
    }

    pub fn select_all(&mut self) {
        let tool = self.active_tool_mut();
        let items: Vec<(usize, String)> = tool.filtered_items().into_iter().map(|(i, s)| (i, s.clone())).collect();
        for (actual_idx, _) in items {
            tool.selected_items.insert(actual_idx);
        }
    }

    pub fn deselect_all(&mut self) {
        let tool = self.active_tool_mut();
        tool.selected_items.clear();
    }

    pub fn invert_selection(&mut self) {
        let tool = self.active_tool_mut();
        let items: Vec<(usize, String)> = tool.filtered_items().into_iter().map(|(i, s)| (i, s.clone())).collect();
        for (actual_idx, _) in items {
            if tool.selected_items.contains(&actual_idx) {
                tool.selected_items.remove(&actual_idx);
            } else {
                tool.selected_items.insert(actual_idx);
            }
        }
    }

    pub fn delete_selected(&mut self) {
        let tool_idx = self.active_tool_idx;
        let paths_to_delete: Vec<(usize, String)> = {
            let tool = &self.tools[tool_idx];
            if !tool.selected_items.is_empty() {
                 let mut items_to_delete = Vec::new();
                 for idx in &tool.selected_items {
                     if let Some(s) = tool.all_items.get(*idx) {
                          items_to_delete.push((*idx, s.clone()));
                     }
                 }
                 // Sort by idx descending to avoid shifting issues when removing from vec later
                 items_to_delete.sort_by(|a, b| b.0.cmp(&a.0));
                 items_to_delete
            } else {
                let items = tool.filtered_items();
                if let Some(i) = tool.list_state.selected() {
                    if i < items.len() {
                        vec![(items[i].0, items[i].1.clone())]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
        };

        let mut deleted_count = 0;
        let tool = &mut self.tools[tool_idx];

        for (idx, path) in paths_to_delete {
            match fs::remove_file(&path) {
                Ok(_) => {
                    deleted_count += 1;
                    if tool.all_items.get(idx) == Some(&path) {
                        tool.all_items.remove(idx);
                        tool.selected_items.remove(&idx);
                    }
                },
                Err(e) => {
                    self.status_message = format!("Failed to delete {}: {}", path, e);
                }
            }
        }

        if deleted_count > 0 {
             self.status_message = format!("Deleted {} items", deleted_count);
             // Rebuild selection indices because all_items shifted
             // This is naive and will drop selections if elements shifted.
             // Ideally we'd remove them from UI first, or use a better data structure.
             // For now, clear selection after bulk delete to be safe.
             tool.selected_items.clear();

             let new_items = tool.filtered_items();
             if new_items.is_empty() {
                 tool.list_state.select(None);
             } else if let Some(i) = tool.list_state.selected() {
                 if i >= new_items.len() {
                     tool.list_state.select(Some(new_items.len() - 1));
                 }
             }
        }
    }

    pub fn export_selected(&mut self) {
        let tool_idx = self.active_tool_idx;
        let paths_to_export: Vec<String> = {
            let tool = &self.tools[tool_idx];
            if !tool.selected_items.is_empty() {
                 let mut items = Vec::new();
                 for idx in &tool.selected_items {
                     if let Some(s) = tool.all_items.get(*idx) {
                          items.push(s.clone());
                     }
                 }
                 items
            } else {
                let items = tool.filtered_items();
                if let Some(i) = tool.list_state.selected() {
                    if i < items.len() {
                        vec![items[i].1.clone()]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
        };

        if !paths_to_export.is_empty() {
             match fs::write("czkawka_export.txt", paths_to_export.join("\n")) {
                 Ok(_) => self.status_message = format!("Exported {} items to czkawka_export.txt", paths_to_export.len()),
                 Err(e) => self.status_message = format!("Failed to export: {}", e),
             }
        } else {
             self.status_message = "Nothing to export".to_string();
        }
    }
}
