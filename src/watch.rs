//! Detect log file changes via size and mtime polling.

use std::path::Path;
use std::time::SystemTime;

#[allow(dead_code)]
pub struct Watcher {
    path: std::path::PathBuf,
    last_size: u64,
    last_mtime: Option<SystemTime>,
}

impl Watcher {
    #[allow(dead_code)]
    pub fn new(path: &Path) -> std::io::Result<Self> {
        let meta = std::fs::metadata(path)?;
        Ok(Self {
            path: path.to_path_buf(),
            last_size: meta.len(),
            last_mtime: meta.modified().ok(),
        })
    }

    #[allow(dead_code)]
    pub fn check(&mut self) -> std::io::Result<WatchEvent> {
        let meta = std::fs::metadata(&self.path)?;
        let new_size = meta.len();
        let new_mtime = meta.modified().ok();
        let event = if new_size < self.last_size {
            WatchEvent::Truncated
        } else if new_size > self.last_size {
            WatchEvent::Grew(new_size - self.last_size)
        } else if new_mtime != self.last_mtime {
            WatchEvent::Touched
        } else {
            WatchEvent::NoChange
        };
        self.last_size = new_size;
        self.last_mtime = new_mtime;
        Ok(event)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum WatchEvent {
    NoChange,
    Touched,
    Grew(u64),
    Truncated,
}
