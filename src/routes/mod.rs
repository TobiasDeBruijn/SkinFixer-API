use actix_route_config::Routable;
use actix_web::web;
use actix_web::web::ServiceConfig;

pub mod generate;
pub mod health;
pub mod player;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config
            .route("/player/{nickname}", web::get().to(player::get_by_name))
            .route("/health", web::get().to(health::up));
    }
}
