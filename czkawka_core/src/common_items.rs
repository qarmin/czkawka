use std::path::Path;

#[cfg(not(target_family = "unix"))]
use crate::common::normalize_windows_path;
use crate::common::regex_check;
use crate::common_messages::Messages;

#[cfg(target_family = "unix")]
pub const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["/proc", "/dev", "/sys", "/run", "/snap"];
#[cfg(not(target_family = "unix"))]
pub const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["C:\\Windows"];

#[cfg(target_family = "unix")]
pub const DEFAULT_EXCLUDED_ITEMS: &str = "*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,*/snap/*,/home/*/.cache/*";
#[cfg(not(target_family = "unix"))]
pub const DEFAULT_EXCLUDED_ITEMS: &str = "*\\.git\\*,*\\node_modules\\*,*\\lost+found\\*,*:\\windows\\*,*:\\$RECYCLE.BIN\\*,*:\\$SysReset\\*,*:\\System Volume Information\\*,*:\\OneDriveTemp\\*,*:\\hiberfil.sys,*:\\pagefile.sys,*:\\swapfile.sys";

#[derive(Debug, Clone, Default)]
pub struct ExcludedItems {
    expressions: Vec<String>,
    connected_expressions: Vec<SingleExcludedItem>,
}

#[derive(Debug, Clone, Default)]
pub struct SingleExcludedItem {
    pub expression: String,
    pub expression_splits: Vec<String>,
    pub unique_extensions_splits: Vec<String>,
}

impl ExcludedItems {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_from(excluded_items: Vec<String>) -> Self {
        let mut s = Self::new();
        s.set_excluded_items(excluded_items);
        s
    }

    pub fn set_excluded_items(&mut self, excluded_items: Vec<String>) -> Messages {
        let mut warnings: Vec<String> = Vec::new();
        if excluded_items.is_empty() {
            return Messages::new();
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

        for checked_expression in &checked_expressions {
            let item = new_excluded_item(checked_expression);
            self.expressions.push(item.expression.clone());
            self.connected_expressions.push(item);
        }
        Messages {
            messages: vec![],
            warnings,
            errors: vec![],
        }
    }

    pub fn get_excluded_items(&self) -> &Vec<String> {
        &self.expressions
    }
    pub fn is_excluded(&self, path: &Path) -> bool {
        if self.connected_expressions.is_empty() {
            return false;
        }
        #[cfg(target_family = "windows")]
        let path = normalize_windows_path(path);

        let path_str = path.to_string_lossy();

        for expression in &self.connected_expressions {
            if regex_check(expression, &path_str) {
                return true;
            }
        }
        false
    }
}

pub fn new_excluded_item(expression: &str) -> SingleExcludedItem {
    let expression = expression.trim().to_string();
    let expression_splits: Vec<String> = expression.split('*').filter_map(|e| if e.is_empty() { None } else { Some(e.to_string()) }).collect();
    let mut unique_extensions_splits = expression_splits.clone();
    unique_extensions_splits.sort();
    unique_extensions_splits.dedup();
    unique_extensions_splits.sort_by_key(|b| std::cmp::Reverse(b.len()));
    SingleExcludedItem {
        expression,
        expression_splits,
        unique_extensions_splits,
    }
}
