use ratatui::widgets::ListState;
use std::fs;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Search,
}

pub struct App {
    pub input_mode: InputMode,
    pub filter: String,
    pub all_items: Vec<String>,
    pub list_state: ListState,
    pub status_message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input_mode: InputMode::Normal,
            filter: String::new(),
            all_items: Vec::new(),
            list_state: ListState::default(),
            status_message: String::new(),
        }
    }
}

impl App {
    pub fn next(&mut self) {
        let items = self.filtered_items();
        if items.is_empty() {
            self.list_state.select(None);
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let items = self.filtered_items();
        if items.is_empty() {
            self.list_state.select(None);
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn filtered_items(&self) -> Vec<&String> {
        if self.filter.is_empty() {
            self.all_items.iter().collect()
        } else {
            self.all_items
                .iter()
                .filter(|s| s.to_lowercase().contains(&self.filter.to_lowercase()))
                .collect()
        }
    }

    pub fn on_filter_change(&mut self) {
        self.list_state.select(Some(0));
        let items = self.filtered_items();
        if items.is_empty() {
             self.list_state.select(None);
        }
    }

    pub fn delete_selected(&mut self) {
        let path = {
            let items = self.filtered_items();
            if let Some(i) = self.list_state.selected() {
                if i < items.len() {
                    Some(items[i].clone())
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(path) = path {
            match fs::remove_file(&path) {
                Ok(_) => {
                    self.status_message = format!("Deleted: {}", path);
                    // find index in all_items and remove it
                    if let Some(idx) = self.all_items.iter().position(|x| x == &path) {
                        self.all_items.remove(idx);
                    }

                    // adjust selection
                    let new_items = self.filtered_items();
                    if new_items.is_empty() {
                        self.list_state.select(None);
                    } else if let Some(i) = self.list_state.selected() {
                        if i >= new_items.len() {
                            self.list_state.select(Some(new_items.len() - 1));
                        }
                    }
                },
                Err(e) => {
                    self.status_message = format!("Failed to delete {}: {}", path, e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation() {
        let mut app = App::default();
        app.all_items = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        assert_eq!(app.list_state.selected(), None);

        app.next();
        assert_eq!(app.list_state.selected(), Some(0));

        app.next();
        assert_eq!(app.list_state.selected(), Some(1));

        app.previous();
        assert_eq!(app.list_state.selected(), Some(0));

        app.previous();
        assert_eq!(app.list_state.selected(), Some(2)); // wraps around
    }

    #[test]
    fn test_filtering() {
        let mut app = App::default();
        app.all_items = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];

        app.filter = "a".to_string();
        assert_eq!(app.filtered_items().len(), 2);

        app.filter = "b".to_string();
        assert_eq!(app.filtered_items().len(), 1);

        app.filter = "z".to_string();
        assert_eq!(app.filtered_items().len(), 0);
    }
}
