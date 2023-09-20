use crate::error::Error;
use crate::key_rotation::KeyRotation;
use crate::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::warn;

const MINESKIN_API_URL: &str = "https://api.mineskin.org/generate/url";
const MINESKIN_API_UUID: &str = "https://api.mineskin.org/generate/user";

fn random_name() -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub async fn skin_by_uuid(rotation: &KeyRotation, uuid: &str) -> Result<MineskinTextureInfo> {
    let key = rotation.get_key();
    let url = format!("{}?key={}", MINESKIN_API_UUID, key);

    let data = make_mineskin_request(&url, &MineskinRequest::from_uuid(uuid)).await?;

    Ok(data.texture)
}

pub async fn skin_by_url(rotation: &KeyRotation, skin_url: &str) -> Result<MineskinTextureInfo> {
    let key = rotation.get_key();
    let url = format!("{}?key={}", MINESKIN_API_URL, key);

    let data = make_mineskin_request(&url, &MineskinRequest::from_url(skin_url)).await?;
    Ok(data.texture)
}

async fn make_mineskin_request<T: Serialize + ?Sized>(
    url: &str,
    payload: &T,
) -> Result<MineskinSkinData> {
    let response: MineskinResponse = reqwest::Client::new()
        .post(url)
        .header("User-Agent", "SkinFixer-API")
        .header("Content-Type", "application/json")
        .json(payload)
        .send()
        .await?
        .json()
        .await?;

    check_mineskin_error(&response)?;

    match response.data {
        Some(r) => Ok(r),
        None => {
            warn!("MineSkinError: {:?}", response.error);

            Err(Error::InternalServer)
        }
    }
}

fn check_mineskin_error(response: &MineskinResponse) -> Result<()> {
    if let Some(error) = &response.error {
        if let Some(error_code) = &error.error_code {
            warn!("MineSkinError: {:?}", &error);
            return Err(Error::BadRequest(error_code.to_string()));
        }

        if error.next_request.is_some() {
            return Err(Error::TooManyRequests);
        }

        warn!("MineSkinError: {:?}", &error);
        Err(Error::InternalServer)
    } else {
        Ok(())
    }
}

#[derive(Serialize)]
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
