pub mod url;
pub mod uuid;

use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(
            web::scope("/generate")
                .route("/url/{url}", web::get().to(url::generate))
                .route("/uuid/{uuid}", web::get().to(uuid::generate)),
        );
    }
}
