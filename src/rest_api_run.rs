use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::{Arc, RwLock};

// Handler types
type UpsertFile = actix_web::dev::fn_service_factory<web::Path<String>, String, HttpResponse>;
type GetFile = actix_web::dev::fn_service_factory<web::Path<String>, HttpResponse>;
type CreateSnapshot = actix_web::dev::fn_service_factory<(), HttpResponse>;
type RestoreSnapshot = actix_web::dev::fn_service_factory<web::Path<String>, HttpResponse>;

pub async fn run_rest_api(
    upsert_file: impl Fn(web::Path<String>, String) -> actix_web::dev::HttpResponseFuture + Clone + Send + 'static,
    get_file: impl Fn(web::Path<String>) -> actix_web::dev::HttpResponseFuture + Clone + Send + 'static,
    create_snapshot: impl Fn() -> actix_web::dev::HttpResponseFuture + Clone + Send + 'static,
    restore_snapshot: impl Fn(web::Path<String>) -> actix_web::dev::HttpResponseFuture + Clone + Send + 'static,
) -> std::io::Result<()> {
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