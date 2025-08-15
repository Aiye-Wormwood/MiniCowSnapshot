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

// 每次请求都可调用，选择后端
fn select_fs(backend: &str) -> Arc<RwLock<Box<dyn FileSystem>>> {
    match backend {
        "disk" => Arc::new(RwLock::new(Box::new(disk_fs::DiskFS::new(PathBuf::from("./data"))))),
        "netdisk" => Arc::new(RwLock::new(Box::new(net_disk_fs::NetDiskFS::new()))),
        _ => Arc::new(RwLock::new(Box::new(mini_fs::MiniFS::new()))),
    }
}

// handler里通过Query参数获取backend
fn get_backend(query: &web::Query<std::collections::HashMap<String, String>>) -> &str {
    query.get("backend").map(|s| s.as_str()).unwrap_or("memory")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let snapshots = Arc::new(RwLock::new(SnapshotManager::new()));

    // upsert_file
    let upsert_file = {
        move |path: web::Path<String>, body: String, query: web::Query<std::collections::HashMap<String, String>>| {
            let backend = get_backend(&query);
            let fs = select_fs(backend);
            async move {
                fs.write().unwrap().upsert_file(path.into_inner(), body);
                HttpResponse::Ok().body("File updated")
            }
        }
    };

    // get_file
    let get_file = {
        move |path: web::Path<String>, query: web::Query<std::collections::HashMap<String, String>>| {
            let backend = get_backend(&query);
            let fs = select_fs(backend);
            async move {
                match fs.read().unwrap().get_file(&path.into_inner()) {
                    Some(content) => HttpResponse::Ok().body(content.to_string()),
                    None => HttpResponse::NotFound().body("File not found"),
                }
            }
        }
    };

    // create_snapshot
    let create_snapshot = {
        let snapshots = snapshots.clone();
        move |query: web::Query<std::collections::HashMap<String, String>>| {
            let backend = get_backend(&query);
            let fs = select_fs(backend);
            let snapshots = snapshots.clone();
            async move {
                let files = fs.read().unwrap().clone_files();
                let mut snaps = snapshots.write().unwrap();
                let snap_id = snaps.create_snapshot(&files);
                HttpResponse::Ok().body(snap_id)
            }
        }
    };

    // restore_snapshot
    let restore_snapshot = {
        let snapshots = snapshots.clone();
        move |path: web::Path<String>, query: web::Query<std::collections::HashMap<String, String>>| {
            let backend = get_backend(&query);
            let fs = select_fs(backend);
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