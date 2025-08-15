use std::collections::HashMap;
use crate::filesystem::FileSystem;

pub struct SnapshotManager {
    snapshots: HashMap<String, Box<dyn FileSystem>>,
}

impl SnapshotManager {
    pub fn new() -> Self {
        Self { snapshots: HashMap::new() }
    }

    pub fn create_snapshot(&mut self, files: &Box<dyn FileSystem>) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.snapshots.insert(id.clone(), files.clone_files());
        id
    }

    pub fn get_snapshot(&self, id: &str) -> Option<&Box<dyn FileSystem>> {
        self.snapshots.get(id)
    }
}