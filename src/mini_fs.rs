use std::collections::HashMap;
use std::sync::Arc;
use crate::filesystem::FileSystem;

pub struct MiniFS {
    files: HashMap<String, Arc<String>>,
}

impl MiniFS {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }
}

impl FileSystem for MiniFS {
    fn upsert_file(&mut self, name: String, content: String) {
        self.files.insert(name, Arc::new(content));
    }

    fn get_file(&self, name: &str) -> Option<Arc<String>> {
        self.files.get(name).cloned()
    }

    fn clone_files(&self) -> Box<dyn FileSystem> {
        Box::new(Self { files: self.files.clone() })
    }

    fn set_files(&mut self, other: &dyn FileSystem) {
        if let Some(mini) = other.as_any().downcast_ref::<MiniFS>() {
            self.files = mini.files.clone();
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}