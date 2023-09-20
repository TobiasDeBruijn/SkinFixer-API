use crate::api::skin_by_url;
use crate::appdata::AppData;
use crate::Result;
use actix_web::web;
use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Path {
    url: String,
}

#[derive(Serialize)]
pub struct Response {
    value: String,
    signature: String,
}

/// Generate skin data from a URL. The URL must be Base64-encoded and must be a direct link to the image file
pub async fn generate(
    path: web::Path<Path>,
    data: web::Data<AppData>,
) -> Result<web::Json<Response>> {
    let url = base64::prelude::BASE64_STANDARD.decode(&path.url)?;
    let url = String::from_utf8(url)?;

    let texture_data = skin_by_url(&data.keys, &url).await?;
    let user_response = Response {
        value: texture_data.value,
        signature: texture_data.signature,
    };

    Ok(web::Json(user_response))
}
