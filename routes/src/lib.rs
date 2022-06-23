mod error;
mod routes;
mod empty;
mod routable;
mod appdata;

use actix_web::{App, HttpServer};
use paperclip::actix::{OpenApiExt, web};
use dal::Dal;

pub use crate::error::Error;
pub use crate::appdata::Config;

pub(crate) use crate::error::*;
#[allow(unused)] // Will be used in the future
pub(crate) use crate::empty::*;
pub(crate) use crate::routable::*;
pub(crate) use crate::appdata::*;

/// Start the web server
pub async fn start(dal: Dal, config: Config) -> std::io::Result<()> {
    let appdata = AppData::new(dal, config);

    HttpServer::new(move || {
        App::new()
            .wrap_api()
            .app_data(web::Data::new(appdata.clone()))
            .with_json_spec_at("/spec")
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(actix_cors::Cors::permissive())
            .configure(routes::Router::configure)
            .build()
    })
    .bind("[::]:8080")?
    .run()
    .await
}