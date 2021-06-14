use crate::appdata::{AppData, KeyRotation};
use actix_web::{HttpServer, App};
use actix_cors::Cors;

mod endpoints;
mod appdata;

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    println!("Starting SkinFixer API Server");

    let kr = match std::env::var("API_KEY") {
        Ok(k) => {
            let k: Vec<String> = k.split(",").map(|c| c.to_string()).collect();

            let kr = KeyRotation::new(k);
            Some(kr)
        },
        Err(_) => None
    };

    HttpServer::new(move|| {
        let cors = Cors::permissive();
        let appdata = AppData::new(kr.clone());

        App::new()
            .wrap(cors)
            .data(appdata)
            .service(crate::endpoints::url::generate)
            .service(crate::endpoints::uuid::generate)
            .service(crate::endpoints::up::up)

    }).bind("0.0.0.0:8080")?.run().await
}
