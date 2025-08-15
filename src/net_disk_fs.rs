use std::sync::Arc;
use crate::filesystem::FileSystem;

pub struct NetDiskFS;

impl NetDiskFS {
    pub fn new() -> Self {
        Self
    }
}

impl FileSystem for NetDiskFS {
    fn upsert_file(&mut self, name: String, content: String) {
        println!("NetDisk: upserting file: {} [stub]", name);
    }

    fn get_file(&self, name: &str) -> Option<Arc<String>> {
        println!("NetDisk: getting file: {} [stub]", name);
        None
    }

    fn clone_files(&self) -> Box<dyn FileSystem> {
        Box::new(NetDiskFS::new())
    }

    fn set_files(&mut self, _other: &dyn FileSystem) {}

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}