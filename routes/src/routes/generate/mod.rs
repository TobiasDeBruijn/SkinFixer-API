use paperclip::actix::web;
use paperclip::actix::web::ServiceConfig;
use crate::Routable;

mod url;

pub struct Router;

impl Routable for Router {
    fn configure(config: &mut ServiceConfig) {
        config.service(web::scope("/generate")
            .route("/url/{url}", web::get().to(url::url))
        );
    }
}