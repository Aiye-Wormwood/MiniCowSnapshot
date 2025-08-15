use std::fs;
use std::sync::Arc;
use std::path::PathBuf;
use crate::filesystem::FileSystem;

pub struct DiskFS {
    dir: PathBuf,
}

impl DiskFS {
    pub fn new(dir: PathBuf) -> Self {
        fs::create_dir_all(&dir).unwrap();
        Self { dir }
    }
}

impl FileSystem for DiskFS {
    fn upsert_file(&mut self, name: String, content: String) {
        let path = self.dir.join(&name);
        fs::write(path, content).unwrap();
    }

    fn get_file(&self, name: &str) -> Option<Arc<String>> {
        let path = self.dir.join(name);
        match fs::read_to_string(path) {
            Ok(data) => Some(Arc::new(data)),
            Err(_) => None,
        }
    }

    fn clone_files(&self) -> Box<dyn FileSystem> {
        Box::new(DiskFS { dir: self.dir.clone() })
    }

    fn set_files(&mut self, _other: &dyn FileSystem) {
        // For disk, this method can be extended for file copying if needed.
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}