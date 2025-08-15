use actix_web::{web, App, HttpServer, HttpResponse};
use std::collections::HashMap;

// Handler types
type UpsertFile = fn(web::Path<String>, String, web::Query<HashMap<String, String>>) -> actix_web::dev::HttpResponseFuture;
type GetFile = fn(web::Path<String>, web::Query<HashMap<String, String>>) -> actix_web::dev::HttpResponseFuture;
type CreateSnapshot = fn(web::Query<HashMap<String, String>>) -> actix_web::dev::HttpResponseFuture;
type RestoreSnapshot = fn(web::Path<String>, web::Query<HashMap<String, String>>) -> actix_web::dev::HttpResponseFuture;

pub async fn run_rest_api(
    upsert_file: impl Fn(web::Path<String>, String, web::Query<HashMap<String, String>>) -> actix_web::dev::HttpResponseFuture + Clone + Send + 'static,
    get_file: impl Fn(web::Path<String>, web::Query<HashMap<String, String>>) -> actix_web::dev::HttpResponseFuture + Clone + Send + 'static,
    create_snapshot: impl Fn(web::Query<HashMap<String, String>>) -> actix_web::dev::HttpResponseFuture + Clone + Send + 'static,
    restore_snapshot: impl Fn(web::Path<String>, web::Query<HashMap<String, String>>) -> actix_web::dev::HttpResponseFuture + Clone + Send + 'static,
) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/file/{name}", web::put().to({
                let upsert_file = upsert_file.clone();
                move |path, body, query| upsert_file(path, body, query)
            }))
            .route("/file/{name}", web::get().to({
                let get_file = get_file.clone();
                move |path, query| get_file(path, query)
            }))
            .route("/snapshot", web::post().to({
                let create_snapshot = create_snapshot.clone();
                move |query| create_snapshot(query)
            }))
            .route("/snapshot/{id}/restore", web::post().to({
                let restore_snapshot = restore_snapshot.clone();
                move |path, query| restore_snapshot(path, query)
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}