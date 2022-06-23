use dal::Dal;

pub type WebData = paperclip::actix::web::Data<AppData>;

#[derive(Clone, Debug)]
pub struct AppData {
    pub dal: Dal,
    pub config: Config,
}

#[derive(Clone, Debug)]
pub struct Config {

}