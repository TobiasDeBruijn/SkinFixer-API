use paperclip::actix::web;
use paperclip::actix::web::ServiceConfig;
use crate::Routable;

mod get;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/player")
            .route("/{name}", web::get().to(get::get))
        );
    }
}