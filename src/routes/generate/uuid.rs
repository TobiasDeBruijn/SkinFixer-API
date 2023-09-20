use crate::api::skin_by_uuid;
use crate::appdata::AppData;
use crate::Result;
use actix_web::web;
use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Path {
    uuid: String,
}

#[derive(Serialize)]
pub struct Response {
    value: String,
    signature: String,
}

/// Generate skin data for the player with the provided UUID. Note that the UUID must be Base64-encoded
pub async fn generate(
    path: web::Path<Path>,
    data: web::Data<AppData>,
) -> Result<web::Json<Response>> {
    let uuid = base64::prelude::BASE64_STANDARD.decode(&path.uuid)?;
    let uuid = String::from_utf8(uuid)?;

    // Check the cache
    match crate::database::get_uuid(&data, &uuid).await? {
        Some((sig, val)) => {
            let resp = Response {
                signature: sig,
                value: val,
            };

            return Ok(web::Json(resp));
        }
        None => {}
    };

    let skin_data = skin_by_uuid(&data.keys, &path.uuid).await?;
    crate::database::set_uuid(&data, &uuid, &skin_data.signature, &skin_data.value).await?;

    let user_response = Response {
        value: skin_data.value,
        signature: skin_data.signature,
    };

    Ok(web::Json(user_response))
}
