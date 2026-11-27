use std::path::{Path, PathBuf};

use crate::common::regex_check;
use crate::flc;
use crate::helpers::messages::Messages;

#[cfg(target_family = "unix")]
pub const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["/proc", "/dev", "/sys", "/snap"];
#[cfg(not(target_family = "unix"))]
pub const DEFAULT_EXCLUDED_DIRECTORIES: &[&str] = &["C:\\Windows"];

#[cfg(all(target_family = "unix", target_os = "macos"))]
pub const DEFAULT_EXCLUDED_ITEMS: &str = "*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,/Users/*/Library/Caches/*";

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
pub const DEFAULT_EXCLUDED_ITEMS: &str = "*/.git/*,*/node_modules/*,*/lost+found/*,*/Trash/*,*/.Trash-*/*,*/snap/*,/home/*/.cache/*,/home/*/.var/app/,/home/*/.*";

#[cfg(not(target_family = "unix"))]
pub const DEFAULT_EXCLUDED_ITEMS: &str = "*\\.git\\*,*\\node_modules\\*,*\\lost+found\\*,*:\\windows\\*,*:\\$RECYCLE.BIN\\*,*:\\$SysReset\\*,*:\\System Volume Information\\*,*:\\OneDriveTemp\\*,*:\\hiberfil.sys,*:\\pagefile.sys,*:\\swapfile.sys,*:\\Users\\*\\AppData";

#[cfg(all(target_family = "unix", target_os = "macos"))]
pub const TRASH_EXCLUDED_ITEMS: &str = "*/Trash/*,*/.Trash-*/*";
#[cfg(all(target_family = "unix", not(target_os = "macos")))]
pub const TRASH_EXCLUDED_ITEMS: &str = "*/Trash/*,*/.Trash-*/*";
#[cfg(not(target_family = "unix"))]
pub const TRASH_EXCLUDED_ITEMS: &str = "*:\\$RECYCLE.BIN\\*";

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
                push_preset_items(&mut checked_expressions, DEFAULT_EXCLUDED_ITEMS);
                continue;
            }
            if expression == "$TRASH" {
                push_preset_items(&mut checked_expressions, TRASH_EXCLUDED_ITEMS);
                continue;
            }
            if !expression.contains('*') {
                warnings.push(flc!("core_excluded_items_wildcard_required", expression = expression));
                continue;
            }

            // On Windows the scanned path is lowercased before matching (see `normalize_windows_path`),
            // so the expression must be lowercased too, otherwise patterns with uppercase letters never match.
            #[cfg(target_family = "windows")]
            let expression = expression.to_ascii_lowercase();

            checked_expressions.push(expression);
        }

        for checked_expression in &checked_expressions {
            let item = new_excluded_item(checked_expression);
            self.expressions.push(item.expression.clone());
            self.connected_expressions.push(item);
        }
        Messages {
            critical: None,
            messages: Vec::new(),
            warnings,
            errors: Vec::new(),
        }
    }

    pub(crate) fn get_excluded_items(&self) -> &Vec<String> {
        &self.expressions
    }
    /// `root` must already be normalized; returns (kept, lifted) or `None` when no pattern matches the root
    fn split_by_root(&self, root: &Path) -> Option<(Self, Self)> {
        let root_str = root.to_string_lossy();
        // The trailing separator lets patterns like `*/.git/*` match the root itself
        let root_with_separator = format!("{root_str}{}", std::path::MAIN_SEPARATOR);
        let (lifted, kept): (Vec<_>, Vec<_>) = self
            .connected_expressions
            .iter()
            .cloned()
            .partition(|item| regex_check(item, &root_str) || regex_check(item, &root_with_separator));
        if lifted.is_empty() {
            return None;
        }
        Some((Self::from_connected(kept), Self::from_connected(lifted)))
    }

    fn from_connected(connected_expressions: Vec<SingleExcludedItem>) -> Self {
        Self {
            expressions: connected_expressions.iter().map(|item| item.expression.clone()).collect(),
            connected_expressions,
        }
    }

    fn matches_normalized(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.connected_expressions.iter().any(|expression| regex_check(expression, &path_str))
    }

    pub(crate) fn is_excluded(&self, path: &Path) -> bool {
        if self.connected_expressions.is_empty() {
            return false;
        }
        #[cfg(target_family = "windows")]
        let path = crate::common::normalize_windows_path(path);
        #[cfg(target_family = "windows")]
        let path = path.as_path();

        self.matches_normalized(path)
    }
}

const NEUTRAL_ROOT_NAME: &str = "_root_";

#[derive(Debug, Clone)]
struct RootScope {
    root: PathBuf,
    kept: ExcludedItems,
    lifted: ExcludedItems,
}

impl RootScope {
    fn is_excluded(&self, path: &Path) -> bool {
        if self.kept.matches_normalized(path) {
            return true;
        }
        let relative = path.strip_prefix(&self.root).unwrap_or(path);
        let neutral_path = Path::new(std::path::MAIN_SEPARATOR_STR).join(NEUTRAL_ROOT_NAME).join(relative);
        self.lifted.matches_normalized(&neutral_path)
    }
}

/// Like `ExcludedItems`, but a pattern matching an included root does not exclude the root itself,
/// so a hidden folder added by the user is not skipped by e.g. `/home/*/.*`.
/// Inside that root such a pattern is still checked on the path below the root, so `/x/.A/.b` stays excluded.
#[derive(Debug, Clone, Default)]
pub struct ScopedExcludedItems {
    default_items: ExcludedItems,
    // Normalized roots, longest first, so the most specific scope wins
    scopes: Vec<RootScope>,
}

impl ScopedExcludedItems {
    pub(crate) fn new(default_items: ExcludedItems, included_roots: &[PathBuf]) -> Self {
        let mut scopes: Vec<RootScope> = included_roots
            .iter()
            .filter_map(|root| {
                #[cfg(target_family = "windows")]
                let root = crate::common::normalize_windows_path(root);
                #[cfg(not(target_family = "windows"))]
                let root = root.clone();
                default_items.split_by_root(&root).map(|(kept, lifted)| RootScope { root, kept, lifted })
            })
            .collect();
        scopes.sort_by_key(|scope| std::cmp::Reverse(scope.root.components().count()));
        Self { default_items, scopes }
    }

    pub(crate) fn is_excluded(&self, path: &Path) -> bool {
        if self.default_items.connected_expressions.is_empty() {
            return false;
        }
        #[cfg(target_family = "windows")]
        let path = crate::common::normalize_windows_path(path);
        #[cfg(target_family = "windows")]
        let path = path.as_path();

        match self.scopes.iter().find(|scope| path.starts_with(&scope.root)) {
            Some(scope) => scope.is_excluded(path),
            None => self.default_items.matches_normalized(path),
        }
    }
}

fn push_preset_items(checked_expressions: &mut Vec<String>, preset: &str) {
    for item in preset.split(',') {
        let item = item.trim();
        if !item.is_empty() {
            #[cfg(not(target_family = "windows"))]
            checked_expressions.push(item.to_string());
            // On Windows, scanned paths are lowercased before matching, so patterns
            // must be lowercased too - including those from the preset sets.
            #[cfg(target_family = "windows")]
            checked_expressions.push(item.to_ascii_lowercase());
        }
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
        let expected = DEFAULT_EXCLUDED_ITEMS.split(',').filter(|s| !s.trim().is_empty()).count();
        assert_eq!(items.expressions.len(), expected);
        assert!(items.expressions.iter().any(|e| e.contains(".git")));
        assert!(items.expressions.iter().any(|e| e.contains("node_modules")));
    }

    #[test]
    fn test_set_excluded_items_with_trash() {
        let mut items = ExcludedItems::new();
        let msgs = items.set_excluded_items(vec!["$TRASH".to_string()]);
        assert!(msgs.warnings.is_empty());
        let expected = TRASH_EXCLUDED_ITEMS.split(',').filter(|s| !s.trim().is_empty()).count();
        assert_eq!(items.expressions.len(), expected);
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
        items.set_excluded_items(vec!["*/.git/*".to_string(), "*/node_modules/*".to_string(), "/home/*/.*".to_string()]);

        assert!(items.is_excluded(Path::new("/home/user/.git/config")));
        assert!(items.is_excluded(Path::new("/home/user/.abscd/config")));
        assert!(items.is_excluded(Path::new("/project/node_modules/package.json")));
        assert!(!items.is_excluded(Path::new("/home/user/file.txt")));

        // Empty items - nothing excluded
        let items_empty = ExcludedItems::new();
        assert!(!items_empty.is_excluded(Path::new("/any/path")));
    }

    #[test]
    fn test_included_hidden_directory_is_not_excluded_by_default_pattern() {
        let items = ExcludedItems::new_from(vec!["*/.git/*".to_string(), "/home/*/.*".to_string()]);
        let hidden_root = PathBuf::from("/home/user/X/.A");
        let scoped = ScopedExcludedItems::new(items, &[hidden_root, PathBuf::from("/home/user/X/B")]);

        assert!(!scoped.is_excluded(Path::new("/home/user/X/.A/file.txt")));
        assert!(!scoped.is_excluded(Path::new("/home/user/X/.A/sub/file.txt")));
        assert!(scoped.is_excluded(Path::new("/home/user/X/.A/.git/config")));
        assert!(scoped.is_excluded(Path::new("/home/user/X/B/.hidden/file.txt")));
        assert!(scoped.is_excluded(Path::new("/home/user/X/.other/file.txt")));
        assert!(!scoped.is_excluded(Path::new("/home/user/X/B/file.txt")));
    }

    #[cfg(unix)]
    #[test]
    fn test_scoped_without_matching_roots_behaves_like_plain_items() {
        let patterns = vec!["*/.*".to_string(), "*/target/*".to_string()];
        let plain = ExcludedItems::new_from(patterns.clone());
        let scoped = ScopedExcludedItems::new(ExcludedItems::new_from(patterns), &[PathBuf::from("/home/user/normal")]);

        for path in ["/home/user/normal/a.txt", "/home/user/normal/.h/a.txt", "/home/user/other/target/a", "/home/user/.cfg/a"] {
            assert_eq!(scoped.is_excluded(Path::new(path)), plain.is_excluded(Path::new(path)), "{path}");
        }
    }

    #[cfg(unix)]
    #[test]
    fn test_scoped_empty_patterns_and_empty_roots() {
        let scoped = ScopedExcludedItems::new(ExcludedItems::new(), &[PathBuf::from("/a/.b")]);
        assert!(!scoped.is_excluded(Path::new("/a/.b/c")));

        let scoped = ScopedExcludedItems::new(ExcludedItems::new_from(vec!["*/.*".to_string()]), &[]);
        assert!(scoped.is_excluded(Path::new("/a/.b/c")));
    }

    #[cfg(unix)]
    #[test]
    fn test_scope_does_not_leak_to_sibling_with_common_name_prefix() {
        let items = ExcludedItems::new_from(vec!["*/.*".to_string()]);
        let scoped = ScopedExcludedItems::new(items, &[PathBuf::from("/x/.A")]);

        assert!(!scoped.is_excluded(Path::new("/x/.A/f")));
        assert!(scoped.is_excluded(Path::new("/x/.AB/f")));
    }

    #[cfg(unix)]
    #[test]
    fn test_nested_roots_most_specific_scope_wins() {
        let items = ExcludedItems::new_from(vec!["*/.outer*".to_string(), "*/.inner*".to_string(), "*/.x/*".to_string()]);
        let scoped = ScopedExcludedItems::new(items, &[PathBuf::from("/r/.outer"), PathBuf::from("/r/.outer/.inner")]);

        assert!(!scoped.is_excluded(Path::new("/r/.outer/a")));
        assert!(scoped.is_excluded(Path::new("/r/.outer/.x/a")));
        assert!(!scoped.is_excluded(Path::new("/r/.outer/.inner/a")));
        assert!(scoped.is_excluded(Path::new("/r/other/.inner/a")));
    }

    #[cfg(unix)]
    #[test]
    fn test_pattern_with_trailing_wildcard_is_lifted_for_root() {
        let items = ExcludedItems::new_from(vec!["*/.git/*".to_string()]);
        let scoped = ScopedExcludedItems::new(items, &[PathBuf::from("/x/.git")]);

        assert!(!scoped.is_excluded(Path::new("/x/.git/config")));
        assert!(scoped.is_excluded(Path::new("/y/.git/config")));
    }

    #[cfg(unix)]
    #[test]
    fn test_root_matching_several_patterns_lifts_all_of_them() {
        let items = ExcludedItems::new_from(vec!["*/.*".to_string(), "*/.A*".to_string(), "*/zzz/*".to_string()]);
        let scoped = ScopedExcludedItems::new(items, &[PathBuf::from("/x/.A")]);

        assert!(!scoped.is_excluded(Path::new("/x/.A/f")));
        assert!(!scoped.is_excluded(Path::new("/x/.A/sub/f")));
        assert!(scoped.is_excluded(Path::new("/x/.A/zzz/f")));
    }

    #[cfg(unix)]
    #[test]
    fn test_hidden_subdirs_inside_included_hidden_root_stay_excluded() {
        let scoped = ScopedExcludedItems::new(ExcludedItems::new_from(vec!["*/.*".to_string()]), &[PathBuf::from("/x/.A")]);

        assert!(!scoped.is_excluded(Path::new("/x/.A")));
        assert!(!scoped.is_excluded(Path::new("/x/.A/sub/f.txt")));
        assert!(!scoped.is_excluded(Path::new("/x/.A/file.tar.gz")));
        assert!(scoped.is_excluded(Path::new("/x/.A/.b/f")));
        assert!(scoped.is_excluded(Path::new("/x/.A/sub/.b/f")));
        assert!(scoped.is_excluded(Path::new("/x/.A/.hidden_file")));
    }

    #[cfg(target_family = "windows")]
    #[test]
    fn test_included_hidden_directory_windows_case_insensitive() {
        let items = ExcludedItems::new_from(vec!["*\\.GIT\\*".to_string(), "*\\.Hidden*".to_string()]);
        let scoped = ScopedExcludedItems::new(items, &[PathBuf::from("C:\\Users\\Me\\.Hidden")]);

        assert!(!scoped.is_excluded(Path::new("C:\\Users\\Me\\.HIDDEN\\sub\\file.txt")));
        assert!(scoped.is_excluded(Path::new("C:\\Users\\Me\\.hidden\\.git\\config")));
        assert!(scoped.is_excluded(Path::new("C:\\Users\\Other\\.hidden\\file.txt")));
    }

    #[cfg(target_family = "windows")]
    #[test]
    fn test_is_excluded_windows_case_insensitive() {
        // Regression test for issue #1957: on Windows the scanned path is lowercased before
        // matching, so an uppercase pattern like `*\TEST\*` must still exclude `...\test\...`.
        let items = ExcludedItems::new_from(vec!["*\\TEST\\*".to_string()]);

        // Uppercase pattern matches the same-cased and differently-cased path segments.
        assert!(items.is_excluded(Path::new("C:\\NOT_TEST\\TEST\\file.txt")));
        assert!(items.is_excluded(Path::new("C:\\Some\\test\\file.txt")));
        assert!(items.is_excluded(Path::new("C:\\Some\\TeSt\\file.txt")));

        // A path without a `\TEST\` segment must not be excluded.
        assert!(!items.is_excluded(Path::new("C:\\NOT_TEST\\TEST2\\file.txt")));
        assert!(!items.is_excluded(Path::new("C:\\root\\file.txt")));
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
