use actix_web::{HttpResponse, web, get};
use crate::AppData;
use serde::Serialize;
use rand::Rng;
use crate::endpoints::{MineskinResponse, UserResponse};

#[derive(Serialize)]
pub struct MineskinRequest {
    url:        String,
    visibility: u8,
    name:       String
}

const MINESKIN_API: &str = "https://api.mineskin.org/generate/url";

#[get("/generate/url/{url}")]
pub async fn generate(web::Path(url): web::Path<String>, data: web::Data<AppData>) -> HttpResponse {
    let name: String = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(32).map(char::from).collect();

    let url = match base64::decode(url) {
        Ok(u) => u,
        Err(e) => return HttpResponse::BadRequest().body(&format!("Invalid URL: {:?}", e))
    };

    let url = match String::from_utf8(url) {
        Ok(u) => u,
        Err(e) => return HttpResponse::BadRequest().body(&format!("Invalid URL: {:?}", e))
    };

    let payload = MineskinRequest {
        url,
        name,
        visibility: 0
    };

    let url = match &data.keys {
        Some(kr) => {
            let k = kr.get_key();
            format!("{}?key={}", MINESKIN_API, k.as_str())
        },
        None => MINESKIN_API.to_string()
    };

    let request = match reqwest::blocking::Client::new()
        .post(&url)
        .header("User-Agent", "SkinFixer-API")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload).unwrap())
        .send() {

        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to request skin by URL from Mineskin: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let response = match request.text() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to convert Mineskin response: {:?}", e);
            return HttpResponse::InternalServerError().finish()
        }
    };

    let response_ser: MineskinResponse = match serde_json::from_str(&response) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to deserialize Mineskin response: {:?}", e);
            return HttpResponse::InternalServerError().finish()
        }
    };

    if let Some(error) = response_ser.error {
        if let Some(error_code) = error.error_code {
            return HttpResponse::BadRequest().body(error_code);
        }

        if error.next_request.is_some() {
            return HttpResponse::TooManyRequests().finish();
        }

        return HttpResponse::InternalServerError().body("MineSkin Error");
    }

    if response_ser.data.is_none() {
        return HttpResponse::InternalServerError().body(&response);
    }

    let data = response_ser.data.unwrap();
    let user_response = UserResponse {
        value: data.texture.value,
        signature: data.texture.signature
    };


    HttpResponse::Ok().body(serde_json::to_string(&user_response).unwrap())
}
