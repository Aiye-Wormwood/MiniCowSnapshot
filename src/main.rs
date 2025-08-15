mod filesystem;
mod mini_fs;
mod disk_fs;
mod net_disk_fs;
mod snapshot_manager;
mod rest_api_run;

use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use actix_web::{web, HttpResponse};
use filesystem::FileSystem;
use snapshot_manager::SnapshotManager;

fn select_fs(backend: &str) -> Arc<RwLock<Box<dyn FileSystem>>> {
    match backend {
        "disk" => Arc::new(RwLock::new(Box::new(disk_fs::DiskFS::new(PathBuf::from("./data"))))),
        "netdisk" => Arc::new(RwLock::new(Box::new(net_disk_fs::NetDiskFS::new()))),
        _ => Arc::new(RwLock::new(Box::new(mini_fs::MiniFS::new()))),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let backend = std::env::var("FS_BACKEND").unwrap_or_else(|_| "memory".to_string());
    let fs = select_fs(&backend);
    let snapshots = Arc::new(RwLock::new(SnapshotManager::new()));

    let upsert_file = {
        let fs = fs.clone();
        move |path: web::Path<String>, body: String| {
            let fs = fs.clone();
            async move {
                fs.write().unwrap().upsert_file(path.into_inner(), body);
                HttpResponse::Ok().body("File updated")
            }
        }
    };

    let get_file = {
        let fs = fs.clone();
        move |path: web::Path<String>| {
            let fs = fs.clone();
            async move {
                match fs.read().unwrap().get_file(&path.into_inner()) {
                    Some(content) => HttpResponse::Ok().body(content.to_string()),
                    None => HttpResponse::NotFound().body("File not found"),
                }
            }
        }
    };

    let create_snapshot = {
        let fs = fs.clone();
        let snapshots = snapshots.clone();
        move || {
            let fs = fs.clone();
            let snapshots = snapshots.clone();
            async move {
                let files = fs.read().unwrap().clone_files();
                let mut snaps = snapshots.write().unwrap();
                let snap_id = snaps.create_snapshot(&files);
                HttpResponse::Ok().body(snap_id)
            }
        }
    };

    let restore_snapshot = {
        let fs = fs.clone();
        let snapshots = snapshots.clone();
        move |path: web::Path<String>| {
            let fs = fs.clone();
            let snapshots = snapshots.clone();
            async move {
                let snap_id = path.into_inner();
                let snaps = snapshots.read().unwrap();
                if let Some(files) = snaps.get_snapshot(&snap_id) {
                    fs.write().unwrap().set_files(&**files);
                    HttpResponse::Ok().body("Snapshot restored")
                } else {
                    HttpResponse::NotFound().body("Snapshot not found")
                }
            }
        }
    };

    rest_api_run::run_rest_api(
        upsert_file,
        get_file,
        create_snapshot,
        restore_snapshot,
    ).await
}