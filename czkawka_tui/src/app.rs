use czkawka_core::common::model::ToolType as CoreToolType;
use ratatui::widgets::{ListState, TableState};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum InputMode {
    Normal,
    Search,
    Select,
    DirPicker,
    ConfirmAction,
    ActionMenu,
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

impl From<ToolType> for CoreToolType {
    fn from(t: ToolType) -> Self {
        match t {
            ToolType::Duplicate => CoreToolType::Duplicate,
            ToolType::EmptyFolders => CoreToolType::EmptyFolders,
            ToolType::BigFiles => CoreToolType::BigFile,
            ToolType::EmptyFiles => CoreToolType::EmptyFiles,
            ToolType::Temporary => CoreToolType::TemporaryFiles,
            ToolType::SimilarImages => CoreToolType::SimilarImages,
            ToolType::SameMusic => CoreToolType::SameMusic,
            ToolType::InvalidSymlinks => CoreToolType::InvalidSymlinks,
            ToolType::BrokenFiles => CoreToolType::BrokenFiles,
            ToolType::SimilarVideos => CoreToolType::SimilarVideos,
            ToolType::BadExtensions => CoreToolType::BadExtensions,
            ToolType::BadNames => CoreToolType::BadNames,
            ToolType::VideoOptimizer => CoreToolType::VideoOptimizer,
        }
    }
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

#[derive(Clone, Debug)]
pub struct TuiItem {
    pub path: String,
    pub size: u64,
    pub modified_date: u64,
}

impl std::fmt::Display for TuiItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

#[derive(Clone, Debug)]
pub struct TuiGroup {
    pub items: Vec<TuiItem>,
}

#[derive(Clone, Debug)]
pub enum ViewItem<'a> {
    Header(usize, &'a TuiGroup),
    Item(usize, usize, &'a TuiItem),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SortOrder {
    None,
    Size,
    Path,
    ModifiedDate,
    Name,
}

pub struct ToolState {
    pub tool_type: ToolType,
    pub filter: String,
    pub groups: Vec<TuiGroup>,
    pub table_state: TableState,
    pub selected_items: std::collections::HashSet<(usize, usize)>, // (group_idx, item_idx)
    pub sort_order: SortOrder,
}

impl ToolState {
    pub fn new(tool_type: ToolType) -> Self {
        Self {
            tool_type,
            filter: String::new(),
            groups: Vec::new(),
            table_state: TableState::default(),
            selected_items: std::collections::HashSet::new(),
            sort_order: SortOrder::None,
        }
    }

    pub fn filtered_items(&self) -> Vec<ViewItem<'_>> {
        let mut results = Vec::new();
        for (g_idx, group) in self.groups.iter().enumerate() {
            let mut has_items = false;
            for (i_idx, item) in group.items.iter().enumerate() {
                if self.filter.is_empty() || item.path.to_lowercase().contains(&self.filter.to_lowercase()) {
                    if !has_items {
                        results.push(ViewItem::Header(g_idx, group));
                        has_items = true;
                    }
                    results.push(ViewItem::Item(g_idx, i_idx, item));
                }
            }
        }
        results
    }

    pub fn apply_sort(&mut self) {
        let order = self.sort_order;
        for group in &mut self.groups {
            group.items.sort_by(|a, b| match order {
                SortOrder::None => std::cmp::Ordering::Equal,
                SortOrder::Size => b.size.cmp(&a.size),
                SortOrder::Path => a.path.cmp(&b.path),
                SortOrder::ModifiedDate => b.modified_date.cmp(&a.modified_date),
                SortOrder::Name => {
                    let a_name = std::path::Path::new(&a.path).file_name().unwrap_or_default();
                    let b_name = std::path::Path::new(&b.path).file_name().unwrap_or_default();
                    a_name.cmp(b_name)
                }
            });
        }

        self.groups.sort_by(|a, b| {
            if a.items.is_empty() {
                return std::cmp::Ordering::Greater;
            }
            if b.items.is_empty() {
                return std::cmp::Ordering::Less;
            }
            let item_a = &a.items[0];
            let item_b = &b.items[0];
            match order {
                SortOrder::None => std::cmp::Ordering::Equal,
                SortOrder::Size => item_b.size.cmp(&item_a.size),
                SortOrder::Path => item_a.path.cmp(&item_b.path),
                SortOrder::ModifiedDate => item_b.modified_date.cmp(&item_a.modified_date),
                SortOrder::Name => {
                    let a_name = std::path::Path::new(&item_a.path).file_name().unwrap_or_default();
                    let b_name = std::path::Path::new(&item_b.path).file_name().unwrap_or_default();
                    a_name.cmp(b_name)
                }
            }
        });
    }

    pub fn cycle_sort_order(&mut self) {
        self.sort_order = match self.sort_order {
            SortOrder::None => SortOrder::Size,
            SortOrder::Size => SortOrder::Path,
            SortOrder::Path => SortOrder::ModifiedDate,
            SortOrder::ModifiedDate => SortOrder::Name,
            SortOrder::Name => SortOrder::None,
        };
        self.apply_sort();
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PendingAction {
    None,
    Delete,
    Symlink,
    Hardlink,
}

pub struct App {
    pub input_mode: InputMode,
    pub active_tool_idx: usize,
    pub tools: Vec<ToolState>,
    pub status_message: String,
    pub scan_dir: String,

    // DirPicker state
    pub dir_picker_current_path: std::path::PathBuf,
    pub dir_picker_items: Vec<std::path::PathBuf>,
    pub dir_picker_list_state: ListState,
    pub selected_directories: Vec<String>,
    pub reference_directories: Vec<String>,

    // Action State
    pub pending_action: PendingAction,
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
            dir_picker_current_path: std::path::PathBuf::from("."),
            dir_picker_items: Vec::new(),
            dir_picker_list_state: ListState::default(),
            selected_directories: Vec::new(),
            reference_directories: Vec::new(),
            pending_action: PendingAction::None,
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

    pub fn open_dir_picker(&mut self) {
        self.input_mode = InputMode::DirPicker;
        self.refresh_dir_picker();
    }

    pub fn refresh_dir_picker(&mut self) {
        self.dir_picker_items.clear();
        if let Some(parent) = self.dir_picker_current_path.parent() {
            self.dir_picker_items.push(parent.to_path_buf());
        }
        if let Ok(entries) = std::fs::read_dir(&self.dir_picker_current_path) {
            let mut dirs: Vec<_> = entries.filter_map(|e| e.ok()).filter(|e| e.path().is_dir()).map(|e| e.path()).collect();
            dirs.sort();
            self.dir_picker_items.extend(dirs);
        }
        self.dir_picker_list_state.select(Some(0));
    }

    pub fn dir_picker_next(&mut self) {
        if self.dir_picker_items.is_empty() {
            return;
        }
        let i = match self.dir_picker_list_state.selected() {
            Some(i) => {
                if i >= self.dir_picker_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.dir_picker_list_state.select(Some(i));
    }

    pub fn dir_picker_previous(&mut self) {
        if self.dir_picker_items.is_empty() {
            return;
        }
        let i = match self.dir_picker_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.dir_picker_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.dir_picker_list_state.select(Some(i));
    }

    pub fn dir_picker_enter(&mut self) {
        if let Some(i) = self.dir_picker_list_state.selected()
            && let Some(path) = self.dir_picker_items.get(i)
        {
            self.dir_picker_current_path = path.clone();
            self.refresh_dir_picker();
        }
    }

    pub fn dir_picker_add_included(&mut self) {
        if let Some(i) = self.dir_picker_list_state.selected()
            && let Some(path) = self.dir_picker_items.get(i)
        {
            let p = path.to_string_lossy().to_string();
            if !self.selected_directories.contains(&p) {
                self.selected_directories.push(p);
            }
        }
    }

    pub fn dir_picker_add_reference(&mut self) {
        if let Some(i) = self.dir_picker_list_state.selected()
            && let Some(path) = self.dir_picker_items.get(i)
        {
            let p = path.to_string_lossy().to_string();
            if !self.reference_directories.contains(&p) {
                self.reference_directories.push(p);
            }
        }
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
            tool.table_state.select(None);
            return;
        }

        let i = match tool.table_state.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        tool.table_state.select(Some(i));
    }

    pub fn page_down(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        let items = tool.filtered_items();
        if items.is_empty() {
            tool.table_state.select(None);
            return;
        }

        let i = match tool.table_state.selected() {
            Some(i) => {
                if i + 10 >= items.len() - 1 {
                    items.len() - 1
                } else {
                    i + 10
                }
            }
            None => 0,
        };
        tool.table_state.select(Some(i));
    }

    pub fn page_up(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        let items = tool.filtered_items();
        if items.is_empty() {
            tool.table_state.select(None);
            return;
        }

        let i = match tool.table_state.selected() {
            Some(i) => {
                i.saturating_sub(10)
            }
            None => 0,
        };
        tool.table_state.select(Some(i));
    }

    pub fn home(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        let items = tool.filtered_items();
        if items.is_empty() {
            tool.table_state.select(None);
            return;
        }
        tool.table_state.select(Some(0));
    }

    pub fn end(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        let items = tool.filtered_items();
        if items.is_empty() {
            tool.table_state.select(None);
            return;
        }
        tool.table_state.select(Some(items.len() - 1));
    }

    pub fn previous(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        let items = tool.filtered_items();
        if items.is_empty() {
            tool.table_state.select(None);
            return;
        }

        let i = match tool.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        tool.table_state.select(Some(i));
    }

    pub fn on_filter_change(&mut self) {
        let tool_idx = self.active_tool_idx;
        let tool = &mut self.tools[tool_idx];
        tool.table_state.select(Some(0));
        let items = tool.filtered_items();
        if items.is_empty() {
            tool.table_state.select(None);
        }
    }

    pub fn toggle_selection(&mut self) {
        let tool_idx = self.active_tool_idx;
        let mut to_toggle = None;
        {
            let tool = &self.tools[tool_idx];
            if let Some(i) = tool.table_state.selected() {
                let items = tool.filtered_items();
                if i < items.len()
                    && let ViewItem::Item(g_idx, i_idx, _) = items[i] {
                        to_toggle = Some((g_idx, i_idx));
                    }
            }
        }
        if let Some((g_idx, i_idx)) = to_toggle {
            let tool = &mut self.tools[tool_idx];
            if tool.selected_items.contains(&(g_idx, i_idx)) {
                tool.selected_items.remove(&(g_idx, i_idx));
            } else {
                tool.selected_items.insert((g_idx, i_idx));
            }
        }
    }

    pub fn select_all(&mut self) {
        let mut to_insert = Vec::new();
        {
            let tool = self.active_tool();
            let items = tool.filtered_items();
            for item in items {
                if let ViewItem::Item(g_idx, i_idx, _) = item {
                    to_insert.push((g_idx, i_idx));
                }
            }
        }
        let tool = self.active_tool_mut();
        for idx in to_insert {
            tool.selected_items.insert(idx);
        }
    }

    pub fn deselect_all(&mut self) {
        let tool = self.active_tool_mut();
        tool.selected_items.clear();
    }

    pub fn invert_selection(&mut self) {
        let mut to_toggle = Vec::new();
        {
            let tool = self.active_tool();
            let items = tool.filtered_items();
            for item in items {
                if let ViewItem::Item(g_idx, i_idx, _) = item {
                    to_toggle.push((g_idx, i_idx));
                }
            }
        }
        let tool = self.active_tool_mut();
        for idx in to_toggle {
            if tool.selected_items.contains(&idx) {
                tool.selected_items.remove(&idx);
            } else {
                tool.selected_items.insert(idx);
            }
        }
    }

    pub fn select_all_except_biggest(&mut self) {
        let tool = self.active_tool_mut();
        tool.selected_items.clear();
        for (g_idx, group) in tool.groups.iter().enumerate() {
            if group.items.is_empty() {
                continue;
            }
            let mut max_size = group.items[0].size;
            let mut max_idx = 0;
            for (i_idx, item) in group.items.iter().enumerate() {
                if item.size > max_size {
                    max_size = item.size;
                    max_idx = i_idx;
                }
            }
            for i_idx in 0..group.items.len() {
                if i_idx != max_idx {
                    tool.selected_items.insert((g_idx, i_idx));
                }
            }
        }
    }

    pub fn select_all_except_smallest(&mut self) {
        let tool = self.active_tool_mut();
        tool.selected_items.clear();
        for (g_idx, group) in tool.groups.iter().enumerate() {
            if group.items.is_empty() {
                continue;
            }
            let mut min_size = group.items[0].size;
            let mut min_idx = 0;
            for (i_idx, item) in group.items.iter().enumerate() {
                if item.size < min_size {
                    min_size = item.size;
                    min_idx = i_idx;
                }
            }
            for i_idx in 0..group.items.len() {
                if i_idx != min_idx {
                    tool.selected_items.insert((g_idx, i_idx));
                }
            }
        }
    }

    pub fn select_all_except_newest(&mut self) {
        let tool = self.active_tool_mut();
        tool.selected_items.clear();
        for (g_idx, group) in tool.groups.iter().enumerate() {
            if group.items.is_empty() {
                continue;
            }
            let mut max_date = group.items[0].modified_date;
            let mut max_idx = 0;
            for (i_idx, item) in group.items.iter().enumerate() {
                if item.modified_date > max_date {
                    max_date = item.modified_date;
                    max_idx = i_idx;
                }
            }
            for i_idx in 0..group.items.len() {
                if i_idx != max_idx {
                    tool.selected_items.insert((g_idx, i_idx));
                }
            }
        }
    }

    pub fn select_all_except_oldest(&mut self) {
        let tool = self.active_tool_mut();
        tool.selected_items.clear();
        for (g_idx, group) in tool.groups.iter().enumerate() {
            if group.items.is_empty() {
                continue;
            }
            let mut min_date = group.items[0].modified_date;
            let mut min_idx = 0;
            for (i_idx, item) in group.items.iter().enumerate() {
                if item.modified_date < min_date {
                    min_date = item.modified_date;
                    min_idx = i_idx;
                }
            }
            for i_idx in 0..group.items.len() {
                if i_idx != min_idx {
                    tool.selected_items.insert((g_idx, i_idx));
                }
            }
        }
    }

    pub fn execute_action(&mut self) {
        let action = self.pending_action;
        let mut success_count = 0;
        let mut fail_count = 0;

        {
            let tool = self.active_tool_mut();
            if tool.selected_items.is_empty() {
                self.status_message = "No items selected".to_string();
                return;
            }

            let mut paths_to_act_on = Vec::new();
            for (g_idx, i_idx) in &tool.selected_items {
                if let Some(group) = tool.groups.get(*g_idx)
                    && let Some(item) = group.items.get(*i_idx)
                {
                    paths_to_act_on.push(((*g_idx, *i_idx), std::path::PathBuf::from(&item.path)));
                }
            }

            for ((g_idx, _i_idx), path) in paths_to_act_on {
                let res = match action {
                    PendingAction::Delete => czkawka_core::common::fs_ops::remove_single_file(&path, false).map_err(|e| e.to_string()),
                    PendingAction::Symlink | PendingAction::Hardlink => {
                        let original = tool.groups[g_idx].items.iter().find(|i| {
                            let pos = tool.groups[g_idx].items.iter().position(|x| x.path == i.path).unwrap();
                            !tool.selected_items.contains(&(g_idx, pos))
                        });
                        if let Some(orig) = original {
                            if action == PendingAction::Symlink {
                                czkawka_core::common::fs_ops::remove_single_file(&path, false).unwrap_or_default();
                                czkawka_core::common::fs_ops::make_file_symlink(std::path::PathBuf::from(&orig.path), &path).map_err(|e| e.to_string())
                            } else {
                                czkawka_core::common::fs_ops::remove_single_file(&path, false).unwrap_or_default();
                                czkawka_core::common::fs_ops::make_hard_link(std::path::PathBuf::from(&orig.path), &path).map_err(|e| e.to_string())
                            }
                        } else {
                            Err("No original item to link to".to_string())
                        }
                    }
                    _ => Ok(()),
                };

                if res.is_ok() {
                    success_count += 1;
                } else {
                    fail_count += 1;
                }
            }
            tool.selected_items.clear();
        }

        self.status_message = format!("Action {:?}: {} succeeded, {} failed", action, success_count, fail_count);
        self.pending_action = PendingAction::None;
    }

    pub fn export_selected(&mut self) {
        let tool_idx = self.active_tool_idx;
        let paths_to_export: Vec<String> = {
            let tool = &self.tools[tool_idx];
            if !tool.selected_items.is_empty() {
                let mut items = Vec::new();
                for (g_idx, i_idx) in &tool.selected_items {
                    if let Some(group) = tool.groups.get(*g_idx)
                        && let Some(item) = group.items.get(*i_idx)
                    {
                        items.push(item.path.clone());
                    }
                }
                items
            } else {
                let items = tool.filtered_items();
                if let Some(i) = tool.table_state.selected() {
                    if i < items.len() {
                        if let ViewItem::Item(_, _, item) = &items[i] { vec![item.path.clone()] } else { vec![] }
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
        };

        if !paths_to_export.is_empty() {
            match std::fs::write("czkawka_export.txt", paths_to_export.join("\n")) {
                Ok(_) => self.status_message = format!("Exported {} items to czkawka_export.txt", paths_to_export.len()),
                Err(e) => self.status_message = format!("Failed to export: {}", e),
            }
        } else {
            self.status_message = "Nothing to export".to_string();
        }
    }
}
