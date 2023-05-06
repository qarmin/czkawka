use std::path::Path;

use crate::common::Common;
use crate::common_messages::Messages;

#[derive(Clone, Default)]
pub struct ExcludedItems {
    pub items: Vec<String>,
}

impl ExcludedItems {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }
    /// Setting excluded items which needs to contains * wildcard
    /// Are a lot of slower than absolute path, so it should be used to heavy
    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>, text_messages: &mut Messages) {
        if excluded_items.is_empty() {
            return;
        }

        let expressions: Vec<String> = excluded_items;
        let mut checked_expressions: Vec<String> = Vec::new();

        for expression in expressions {
            let expression: String = expression.trim().to_string();

            if expression.is_empty() {
                continue;
            }

            #[cfg(target_family = "windows")]
            let expression = expression.replace("/", "\\");

            if expression == "DEFAULT" {
                if cfg!(target_family = "unix") {
                    checked_expressions.push("*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,*/snap/*,/home/*/.cache/*".to_string());
                }
                if cfg!(target_family = "windows") {
                    checked_expressions.push("*\\.git\\*,*\\node_modules\\*,*\\lost+found\\*,*:\\windows\\*".to_string());
                }
                continue;
            }
            if !expression.contains('*') {
                text_messages
                    .warnings
                    .push("Excluded Items Warning: Wildcard * is required in expression, ignoring ".to_string() + expression.as_str());
                continue;
            }

            checked_expressions.push(expression);
        }
        self.items = checked_expressions;
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
