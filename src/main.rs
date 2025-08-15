mod filesystem;
mod mini_fs;
mod disk_fs;
mod net_disk_fs;
mod snapshot_manager;

use std::sync::{Arc, RwLock};
use std::path::PathBuf;
use actix_web::{web, App, HttpResponse, HttpServer};
use filesystem::FileSystem;
use snapshot_manager::SnapshotManager;

// Select backend by parameter
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

    HttpServer::new(move || {
        App::new()
            .route("/file/{name}", web::put().to(upsert_file.clone()))
            .route("/file/{name}", web::get().to(get_file.clone()))
            .route("/snapshot", web::post().to(create_snapshot.clone()))
            .route("/snapshot/{id}/restore", web::post().to(restore_snapshot.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}