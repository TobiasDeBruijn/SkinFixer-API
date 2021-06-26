use crate::appdata::{AppData, Env};
use actix_web::{HttpServer, App};
use actix_cors::Cors;

mod endpoints;
mod appdata;
mod cache;

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    println!("Starting SkinFixer API Server");

    let env = match Env::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to start SkinFixer API Server: {}", e);
            std::process::exit(1);
        }
    };

    let appdata = match AppData::new(&env) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to start SkinFixer API Server: {}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .data(appdata.clone())
            .service(crate::endpoints::url::generate)
            .service(crate::endpoints::uuid::generate)
            .service(crate::endpoints::up::up)

    }).bind("0.0.0.0:8080")?.run().await
}
