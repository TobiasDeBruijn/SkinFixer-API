use crate::error::Error;
use crate::key_rotation::KeyRotation;
use crate::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use tracing::{instrument, warn};

const MINESKIN_API_URL: &str = "https://api.mineskin.org/generate/url";
const MINESKIN_API_UUID: &str = "https://api.mineskin.org/generate/user";

fn random_name() -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

#[instrument(skip(rotation))]
pub async fn skin_by_uuid(rotation: &KeyRotation, uuid: &str) -> Result<MineskinTextureInfo> {
    let key = rotation.next_key();
    let data =
        make_mineskin_request(MINESKIN_API_UUID, &MineskinRequest::from_uuid(uuid), key).await?;

    Ok(data.texture)
}

#[instrument(skip(rotation))]
pub async fn skin_by_url(rotation: &KeyRotation, skin_url: &str) -> Result<MineskinTextureInfo> {
    let key = rotation.next_key();
    let data =
        make_mineskin_request(MINESKIN_API_URL, &MineskinRequest::from_url(skin_url), key).await?;
    Ok(data.texture)
}

#[instrument]
async fn make_mineskin_request<T: Serialize + ?Sized + Debug>(
    url: &str,
    payload: &T,
    key: &str,
) -> Result<MineskinSkinData> {
    let response: MineskinResponse = reqwest::Client::new()
        .post(url)
        .header("User-Agent", "skinfixer/v1.0")
        .bearer_auth(key)
        .json(payload)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    match response.data {
        Some(r) => Ok(r),
        None => {
            warn!("MineSkinError: {:?}", response.error);
            Err(Error::InternalServer)
        }
    }
}

#[derive(Serialize, Debug)]
struct MineskinRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>,
    name: String,
    visibility: u8,
}

impl<'a> MineskinRequest<'a> {
    fn from_uuid(uuid: &'a str) -> Self {
        Self {
            uuid: Some(uuid),
            url: None,
            name: random_name(),
            visibility: 0,
        }
    }

    fn from_url(url: &'a str) -> Self {
        Self {
            uuid: None,
            url: Some(url),
            name: random_name(),
            visibility: 0,
        }
    }
}

#[derive(Deserialize)]
struct MineskinResponse {
    pub data: Option<MineskinSkinData>,
    #[serde(flatten)]
    pub error: Option<MineskinError>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MineskinError {
    error_code: Option<String>,
    next_request: Option<i64>,
}

#[derive(Deserialize, Debug)]
struct MineskinSkinData {
    texture: MineskinTextureInfo,
}

#[derive(Deserialize, Debug)]
pub struct MineskinTextureInfo {
    pub value: String,
    pub signature: String,
}
