use std::path::Path;

use crate::common::Common;

#[cfg(target_family = "unix")]
pub const DEFAULT_EXCLUDED_ITEMS: &str = "*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,*/snap/*,/home/*/.cache/*";
#[cfg(not(target_family = "unix"))]
pub const DEFAULT_EXCLUDED_ITEMS: &str = "*\\.git\\*,*\\node_modules\\*,*\\lost+found\\*,*:\\windows\\*,*:\\$RECYCLE.BIN\\*,*:\\$SysReset\\*,*:\\System Volume Information\\*,*:\\OneDriveTemp\\*,*:\\hiberfil.sys,*:\\pagefile.sys,*:\\swapfile.sys";

#[derive(Debug, Clone, Default)]
pub struct ExcludedItems {
    pub items: Vec<String>,
}

impl ExcludedItems {
    pub fn new() -> Self {
        Default::default()
    }
    /// Setting excluded items which needs to contains * wildcard
    /// Are a lot of slower than absolute path, so it should be used to heavy
    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) -> (Vec<String>, Vec<String>, Vec<String>) {
        let messages: Vec<String> = Vec::new();
        let mut warnings: Vec<String> = Vec::new();
        let errors: Vec<String> = Vec::new();
        if excluded_items.is_empty() {
            return (messages, warnings, errors);
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
                checked_expressions.push(DEFAULT_EXCLUDED_ITEMS.to_string());
                continue;
            }
            if !expression.contains('*') {
                warnings.push("Excluded Items Warning: Wildcard * is required in expression, ignoring ".to_string() + expression.as_str());
                continue;
            }

            checked_expressions.push(expression);
        }
        self.items = checked_expressions;
        (messages, warnings, errors)
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
