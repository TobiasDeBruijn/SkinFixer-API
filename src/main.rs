use crate::appdata::AppData;
use actix_web::{HttpServer, App};
use actix_cors::Cors;

mod endpoints;
mod appdata;

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    println!("Starting SkinFixer API Server");

    HttpServer::new(move|| {
        let cors = Cors::permissive();
        let appdata = AppData::new();

        App::new()
            .wrap(cors)
            .data(appdata)
            .service(crate::endpoints::url::generate)
            .service(crate::endpoints::uuid::generate)
            .service(crate::endpoints::up::up)

    }).bind("0.0.0.0:8080")?.run().await
}
