use std::path::Path;

#[cfg(not(target_family = "unix"))]
use crate::common::normalize_windows_path;
use crate::common::regex_check;
use crate::helpers::messages::Messages;

#[cfg(target_family = "unix")]
pub const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["/proc", "/dev", "/sys", "/snap"];
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

    pub(crate) fn set_excluded_items(&mut self, excluded_items: Vec<String>) -> Messages {
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

    pub(crate) fn get_excluded_items(&self) -> &Vec<String> {
        &self.expressions
    }
    pub(crate) fn is_excluded(&self, path: &Path) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excluded_items_new_and_basic_operations() {
        let items = ExcludedItems::new();
        assert!(items.expressions.is_empty());
        assert!(items.connected_expressions.is_empty());

        let items = ExcludedItems::new_from(vec!["*/.git/*".to_string(), "*/node_modules/*".to_string()]);
        assert_eq!(items.expressions.len(), 2);
        assert_eq!(items.get_excluded_items().len(), 2);
    }

    #[test]
    fn test_set_excluded_items_with_default() {
        let mut items = ExcludedItems::new();
        let msgs = items.set_excluded_items(vec!["DEFAULT".to_string()]);
        assert!(msgs.warnings.is_empty());
        assert_eq!(items.expressions.len(), 1);
        assert!(items.expressions[0].contains(".git") || items.expressions[0].contains("node_modules"));
    }

    #[test]
    fn test_set_excluded_items_warnings() {
        let mut items = ExcludedItems::new();
        let msgs = items.set_excluded_items(vec!["no_wildcard".to_string(), "  ".to_string()]);
        assert_eq!(msgs.warnings.len(), 1);
        assert!(msgs.warnings[0].contains("Wildcard * is required"));
        assert!(items.expressions.is_empty());
    }

    #[test]
    fn test_is_excluded() {
        let mut items = ExcludedItems::new();
        items.set_excluded_items(vec!["*/.git/*".to_string(), "*/node_modules/*".to_string()]);

        assert!(items.is_excluded(Path::new("/home/user/.git/config")));
        assert!(items.is_excluded(Path::new("/project/node_modules/package.json")));
        assert!(!items.is_excluded(Path::new("/home/user/file.txt")));

        // Empty items - nothing excluded
        let items_empty = ExcludedItems::new();
        assert!(!items_empty.is_excluded(Path::new("/any/path")));
    }

    #[test]
    fn test_new_excluded_item() {
        let item = new_excluded_item("  */test/*.txt  ");
        assert_eq!(item.expression, "*/test/*.txt");
        assert_eq!(item.expression_splits, vec!["/test/", ".txt"]);
        assert_eq!(item.unique_extensions_splits.len(), 2);

        let item2 = new_excluded_item("*abc*def*abc*");
        assert_eq!(item2.expression_splits, vec!["abc", "def", "abc"]);
        // unique_extensions_splits should be deduplicated and sorted by length
        assert_eq!(item2.unique_extensions_splits, vec!["abc", "def"]);
    }
}
