#[macro_use]
extern crate include_dir;
#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use actix::System;
use async_trait::async_trait;

use events::{Event, EventBus, EventHandler};

use crate::config::CONFIG;

mod assets;
mod config;
mod couchdb;
mod dashboard;
mod http;
mod logger;
mod oauth;
mod observability;
mod oci;
mod rbac;
mod routes;
mod server;
mod storage;
mod template;
mod tracing;
mod user;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    logger::init(&CONFIG);

    tracing::init(&CONFIG);

    couchdb::migrate(&CONFIG).await?;

    log::info!("Starting Enseada...");

    server::run(&CONFIG).await?;

    log::info!("Stopping Enseada...");

    Ok(())
}
