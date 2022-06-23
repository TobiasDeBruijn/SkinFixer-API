use paperclip::actix::web;
use paperclip::actix::web::ServiceConfig;
use crate::Routable;

mod generate;
mod player;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/")
            .configure(generate::Router::configure)

        );
    }
}