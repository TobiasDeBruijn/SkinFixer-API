use actix_web::{HttpResponse, web, get};
use crate::AppData;
use serde::Serialize;
use rand::Rng;
use crate::endpoints::{MineskinResponse, UserResponse};

#[derive(Serialize)]
pub struct MineskinRequest<'a> {
    uuid:       &'a str,
    visibility: u8,
    name:       String
}

const MINESKIN_API: &str = "https://api.mineskin.org/generate/user";

#[get("/generate/uuid/{uuid}")]
pub async fn generate(web::Path(uuid): web::Path<String>, data: web::Data<AppData>) -> HttpResponse {
    let name: String = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(32).map(char::from).collect();

    let uuid = match base64::decode(uuid) {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::BadRequest().body(&format!("Invalid UUID: {:?}", e))
    };

    let uuid = match String::from_utf8(uuid) {
        Ok(uuid) => uuid,
        Err(e) => return HttpResponse::BadRequest().body(&format!("Invalid UUID: {:?}", e))
    };

    match crate::cache::get_uuid(&data, &uuid)  {
        Ok(Some((sig, val))) => {
            let resp = UserResponse {
                signature: sig,
                value: val
            };

            return HttpResponse::Ok().body(serde_json::to_string(&resp).unwrap());
        },
        Ok(None) => {},
        Err(e) => eprintln!("Failed to query skin cache: {:?}. Falling back to the MineSkin API", e)
    };

    let payload = MineskinRequest {
        uuid: &uuid,
        name,
        visibility: 0
    };

    let key = data.keys.get_key();
    let url = format!("{}?key={}", MINESKIN_API, key);

    let request = match reqwest::blocking::Client::new()
        .post(&url)
        .header("User-Agent", "SkinFixer-API")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload).unwrap())
        .send() {

        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to request skin by UUID from Mineskin: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let response = match request.text() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to deserialize Mineskin response: {:?}", e);
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
        if let Some(error_code) = &error.error_code {
            eprintln!("MineSkinError: {:?}", &error);
            return HttpResponse::BadRequest().body(error_code);
        }

        if error.next_request.is_some() {
            return HttpResponse::TooManyRequests().finish();
        }

        eprintln!("MineSkinError: {:?}", &error);
        return HttpResponse::InternalServerError().body("MineSkin Error");
    }

    if response_ser.data.is_none() {
        eprintln!("MineSkinError: '{:?}': '{:?}'", &response_ser, &response);
        return HttpResponse::InternalServerError().body(&response);
    }

    let mdata = response_ser.data.unwrap();

    match crate::cache::set_uuid(&data, &uuid, &mdata.texture.signature, &mdata.texture.value) {
        Ok(_) => {},
        Err(e) => eprintln!("Failed to insert skin into uuid cache: {:?}", e)
    }

    let user_response = UserResponse {
        value: mdata.texture.value,
        signature: mdata.texture.signature
    };

    HttpResponse::Ok().body(serde_json::to_string(&user_response).unwrap())
}
