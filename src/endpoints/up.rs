use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn up() -> HttpResponse {
    HttpResponse::Ok().finish()
}