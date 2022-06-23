use crate::{WebData, WebResult};
use paperclip::actix::{Apiv2Schema, api_v2_operation, web};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Deserialize, Apiv2Schema)]
pub struct Path {
    url: String
}

#[derive(Debug, Serialize, Apiv2Schema)]
pub struct Response {
    value: String,
    signature: String,
}

#[instrument]
#[api_v2_operation]
pub async fn url(data: WebData, path: web::Path<Path>) -> WebResult<web::Json<Response>> {
    let real_url = String::from_utf8(base64::decode(&path.url)?)?;
    let skin = api::Mineskin::generate_by_url(&real_url, &data.get_key()).await?;

    Ok(web::Json(Response {
        signature: skin.signature,
        value: skin.value
    }))
}
