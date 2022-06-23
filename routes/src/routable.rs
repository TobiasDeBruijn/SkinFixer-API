use paperclip::actix::web::ServiceConfig;

pub trait Routable {
    fn configure(config: &mut ServiceConfig);
}