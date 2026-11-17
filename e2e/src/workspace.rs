use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Private temp directory of one scenario, removed on drop
pub struct Workspace {
    root: PathBuf,
}

impl Workspace {
    pub fn new() -> Result<Self, String> {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("czkawka_e2e_{}_{id}", std::process::id()));
        fs::create_dir_all(&root).map_err(|e| format!("cannot create {}: {e}", root.display()))?;
        Ok(Self { root })
    }

    /// Fake HOME, so the app never touches real settings
    pub fn home(&self) -> PathBuf {
        self.root.join("home")
    }

    pub fn config_file(&self) -> PathBuf {
        self.home().join(".config/krokiet/config_general.json")
    }

    pub fn data_dir(&self) -> PathBuf {
        self.root.join("data")
    }

    pub fn log_file(&self) -> PathBuf {
        self.root.join("krokiet.log")
    }

    pub fn read_log(&self) -> Result<String, String> {
        let path = self.log_file();
        fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))
    }

    pub fn write_file(&self, relative: &str, content: &[u8]) -> Result<PathBuf, String> {
        let path = self.data_dir().join(relative);
        let parent = path.parent().ok_or("file has no parent")?;
        fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        fs::write(&path, content).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        Ok(path)
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).ok();
    }
}
