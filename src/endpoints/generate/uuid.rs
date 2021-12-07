use paperclip::actix::{api_v2_operation, web, get};
use serde::Serialize;
use rand::Rng;
use crate::endpoints::generate::{MineskinResponse, UserResponse};
use crate::appdata::AppData;
use crate::{Error, Result};
use log::warn;

#[derive(Serialize)]
pub struct MineskinRequest<'a> {
    uuid:       &'a str,
    visibility: u8,
    name:       String
}

const MINESKIN_API: &str = "https://api.mineskin.org/generate/user";

/// Generate skin data for the player with the provided UUID. Note that the UUID must be Base64-encoded
#[get("/generate/uuid/{uuid}")]
#[api_v2_operation]
pub async fn generate(web::Path(uuid): web::Path<String>, data: web::Data<AppData>) -> Result<web::Json<UserResponse>> {
    let name: String = rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(32).map(char::from).collect();

    let uuid = base64::decode(uuid)?;
    let uuid = String::from_utf8(uuid)?;

    // Check the cache
    match crate::cache::get_uuid(&data, &uuid)?  {
        Some((sig, val)) => {
            let resp = UserResponse {
                signature: sig,
                value: val
            };

            return Ok(web::Json(resp))
        },
        None => {},
    };

    let payload = MineskinRequest {
        uuid: &uuid,
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
            return Err(Error::TooManyRequests)
        }

        warn!("MineSkinError: {:?}", &error);
        return Err(Error::InternalServer)
    }

    let mineskin_data = match response.data {
        Some(d) => d,
        None => {
            warn!("MineSkinError: {:?}", &response);
            return Err(Error::InternalServer)
        }
    };

    crate::cache::set_uuid(&data, &uuid, &mineskin_data.texture.signature, &mineskin_data.texture.value)?;

    let user_response = UserResponse {
        value: mineskin_data.texture.value,
        signature: mineskin_data.texture.signature
    };

    Ok(web::Json(user_response))
}
