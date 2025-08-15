use std::sync::Arc;

pub trait FileSystem: Send + Sync {
    fn upsert_file(&mut self, name: String, content: String);
    fn get_file(&self, name: &str) -> Option<Arc<String>>;
    fn clone_files(&self) -> Box<dyn FileSystem>;
    fn set_files(&mut self, other: &dyn FileSystem);
    fn as_any(&self) -> &dyn std::any::Any;
}