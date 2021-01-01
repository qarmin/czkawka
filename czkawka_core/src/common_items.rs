use crate::common::Common;
use crate::common_messages::Messages;
use std::path::Path;
use std::time::SystemTime;

#[derive(Default)]
pub struct ExcludedItems {
    pub items: Vec<String>,
}

impl ExcludedItems {
    pub fn new() -> Self {
        Default::default()
    }
    /// Setting excluded items which needs to contains * wildcard
    /// Are a lot of slower than absolute path, so it should be used to heavy
    pub fn set_excluded_items(&mut self, mut excluded_items: String, text_messages: &mut Messages) {
        let start_time: SystemTime = SystemTime::now();

        if excluded_items.is_empty() {
            return;
        }

        excluded_items = excluded_items.replace("\"", "");
        let expressions: Vec<String> = excluded_items.split(',').map(String::from).collect();
        let mut checked_expressions: Vec<String> = Vec::new();

        for expression in expressions {
            let expression: String = expression.trim().to_string();

            if expression.is_empty() {
                continue;
            }
            if expression == "DEFAULT" {
                if cfg!(target_family = "unix") {
                    checked_expressions.push("*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,*/snap/*,/home/*/.cache/*".to_string());
                }
                if cfg!(target_family = "windows") {
                    checked_expressions.push("*/.git/*,*/node_modules/*,*/lost+found/*,*:/windows/*".to_string());
                }
                continue;
            }
            if !expression.contains('*') {
                text_messages.warnings.push("Excluded Items Warning: Wildcard * is required in expression, ignoring ".to_string() + expression.as_str());
                continue;
            }

            checked_expressions.push(expression);
        }
        self.items = checked_expressions;
        Common::print_time(start_time, SystemTime::now(), "set_excluded_items".to_string());
    }

    /// Checks whether a specified path is excluded from searching
    pub fn is_excluded(&self, path: impl AsRef<Path>) -> bool {
        #[cfg(target_family = "windows")]
        let path = Common::normalize_windows_path(path);

        for expression in &self.items {
            if Common::regex_check(expression, &path) {
                return true;
            }
        }
        false
    }
}
