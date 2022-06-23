use paperclip::actix::{web, get, api_v2_operation};
use serde::Serialize;
use rand::Rng;
use crate::endpoints::generate::{MineskinResponse, UserResponse};
use crate::appdata::AppData;
use crate::Result;
use log::warn;
use crate::error::Error;

#[derive(Serialize)]
pub struct MineskinRequest {
    url:        String,
    visibility: u8,
    name:       String
}

const MINESKIN_API: &str = "https://api.mineskin.org/generate/url";

/// Generate skin data from a URL. The URL must be Base64-encoded and must be a direct link to the image file
#[get("/generate/url/{url}")]
#[api_v2_operation]
pub async fn generate(web::Path(url): web::Path<String>, data: web::Data<AppData>) -> Result<web::Json<UserResponse>> {
    let name: String = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(32).map(char::from).collect();

    let url = base64::decode(url)?;
    let url = String::from_utf8(url)?;

    let payload = MineskinRequest {
        url,
        name,
        visibility: 0
    };

    let key = data.keys.get_key();
    let url = format!("{}?key={}", MINESKIN_API, key);

    let response: MineskinResponse = reqwest::blocking::Client::new()
        .post(&url)
        .header("User-Agent", "SkinFixer-API")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload).unwrap())
        .send()?
        .json()?;

    if let Some(error) = response.error {
        if let Some(error_code) = &error.error_code {
            warn!("MineSkinError: {:?}", &error);
            return Err(Error::BadRequest(error_code.to_string()));
        }

        if error.next_request.is_some() {
            return Err(Error::TooManyRequests);
        }

        warn!("MineSkinError: {:?}", &error);
        return Err(Error::InternalServer);
    }

    let data = match response.data {
        Some(r) => r,
        None => {
            warn!("MineSkinError: {:?}", response);
            return Err(Error::InternalServer);
        }
    };

    let user_response = UserResponse {
        value: data.texture.value,
        signature: data.texture.signature
    };

    Ok(web::Json(user_response))
}
