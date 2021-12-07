use std::process::exit;
use crate::appdata::{AppData, Env};
use actix_web::{HttpServer, App};
use actix_cors::Cors;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::middleware::normalize::TrailingSlash;
use log::{info, error, debug};
use paperclip::actix::OpenApiExt;
pub use crate::error::{Result, Error};

mod error;
mod endpoints;
mod appdata;
mod cache;

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{}=INFO", env!("CARGO_PKG_NAME")));
    }
    env_logger::init();
    info!("Starting {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    debug!("Reading environment");
    let env = match Env::new() {
        Ok(e) => e,
        Err(e) => {
            error!("Failed to read environment: {:?}", e);
            exit(1);
        }
    };

    debug!("Creating AppData instance");
    let appdata = match AppData::new(&env) {
        Ok(a) => a,
        Err(e) => {
            error!("Failed to create AppData instance: {:?}", e);
            exit(1);
        }
    };

    debug!("Running migrations");
    match appdata.migrate() {
        Ok(_) => {},
        Err(e) => {
            error!("Failed to run migrations: {:?}", e);
            exit(1);
        }
    }

    HttpServer::new(move ||
        App::new()
            .wrap_api()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .data(appdata.clone())
            .service(crate::endpoints::generate::url::generate)
            .service(crate::endpoints::generate::uuid::generate)
            .service(crate::endpoints::up::up)
            .service(crate::endpoints::player::get_by_name)
            .with_json_spec_at("/spec")
            .build()
    ).bind("[::]:8080")?.run().await
}
