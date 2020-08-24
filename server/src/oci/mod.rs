use std::sync::Arc;

use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware::DefaultHeaders;
use actix_web::web::{self, ServiceConfig};
use actix_web::{get, FromRequest, ResponseError};
use actix_web::{HttpRequest, HttpResponse};
use http::StatusCode;
use serde::Deserialize;

use oci::header;
use oci::service::{BlobService, ManifestService, RepoService, UploadService};

use crate::config::CONFIG;
use crate::http::extractor::session::TokenSession;
use crate::http::guard::subdomain;
use crate::storage;

mod api;
mod blob;
mod error;
mod manifest;
mod tag;
mod upload;

pub type Result<T> = std::result::Result<T, error::ErrorResponse>;

pub fn mount(cfg: &mut ServiceConfig) {
    let couch = &crate::couchdb::SINGLETON;
    let db = Arc::new(couch.database(crate::couchdb::name::OCI, true));
    let provider = Arc::new(storage::new_provider().expect("docker storage provider"));

    let repo = RepoService::new(db.clone());
    cfg.data(repo);

    let repo = UploadService::new(db.clone(), provider.clone());
    cfg.data(repo);

    let blob = BlobService::new(db.clone(), provider.clone());
    cfg.data(blob);

    let manifest = ManifestService::new(db);
    cfg.data(manifest);

    cfg.service(api::list_repos);
    cfg.service(api::create_repo);
    cfg.service(api::get_repo);
    cfg.service(api::delete_repo);

    let sub = CONFIG.oci().subdomain();
    cfg.service(
        web::scope("/v2")
            .guard(subdomain(sub))
            .wrap(DefaultHeaders::new().header(header::DISTRIBUTION_API_VERSION, "registry/2.0"))
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::UNAUTHORIZED, error::handle_unauthorized_request),
            )
            .app_data(actix_web::web::Bytes::configure(|cfg| {
                cfg.limit(1_073_741_824) // Set max file size to 1 Gib
            }))
            .service(root)
            // Tags
            .service(tag::list)
            // Manifests
            .service(
                web::resource("/{group}/{name}/manifests/{reference}")
                    .route(web::get().to(manifest::get))
                    .route(web::head().to(manifest::get))
                    .route(web::put().to(manifest::put))
                    .route(web::delete().to(manifest::delete)),
            )
            // Blobs
            .service(blob::get)
            .service(blob::head)
            .service(blob::delete)
            // Uploads
            .service(upload::start)
            .service(upload::get)
            .service(upload::push)
            .service(upload::complete)
            .service(upload::delete),
    );
}

#[derive(Debug, Deserialize)]
pub struct RepoPath {
    group: String,
    name: String,
}

#[get("/")]
pub async fn root(session: TokenSession) -> HttpResponse {
    log::debug!("{:?}", session);
    HttpResponse::Ok().finish()
}
