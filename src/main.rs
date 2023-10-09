use crate::appdata::{AppData, Env};
use actix_cors::Cors;
use actix_route_config::Routable;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{web, App, HttpServer};
use noiseless_tracing_actix_web::NoiselessRootSpanBuilder;
use tracing::{debug, info};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub use crate::error::Result;

mod api;
mod appdata;
mod database;
mod empty;
mod error;
mod key_rotation;
mod routes;

#[actix_web::main()]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    install_tracing();

    info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    debug!("Reading environment");
    let env = Env::new()?;
    debug!("Creating AppData instance");
    let appdata = AppData::new(&env).await?;

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::<NoiselessRootSpanBuilder>::new())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::Data::new(appdata.clone()))
            .configure(routes::Router::configure)
    })
    .bind("[::]:8080")?
    .run()
    .await?;

    Ok(())
}

fn install_tracing() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{}=INFO", env!("CARGO_PKG_NAME")));
    }

    tracing_subscriber::registry()
        .with(layer().compact())
        .with(EnvFilter::from_default_env())
        .init();
}
