use rand::Rng;
use crate::{client, Error, Result};
use serde::{Deserialize, Serialize};

pub struct Mineskin;

pub struct Skin {
    pub signature: String,
    pub value: String,
}

#[derive(Deserialize)]
struct MineskinResponse {
    data: Option<ResponseData>,
    #[serde(flatten)]
    error: Option<ResponseError>
}

#[derive(Deserialize)]
struct ResponseError {
    error: String,
    error_code: Option<String>,
    next_request: Option<i64>,
}

#[derive(Deserialize)]
struct ResponseData {
    texture: Texture,
}

#[derive(Deserialize)]
struct Texture {
    value: String,
    signature: String,
}

impl Mineskin {
    pub async fn generate_by_url<S: AsRef<str>>(url: S, key: &str) -> Result<Skin> {
        #[derive(Serialize)]
        struct Request<'a> {
            url: &'a str,
            name: String,
            visibility: u8,
        }

        let response = client()
            .post("https://api.mineskin.org/generate/url")
            .query(&[("key", key)])
            .json(&Request {
                name: rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(32).map(char::from).collect(),
                url: url.as_ref(),
                visibility: 0,
            })
            .send()
            .await?;

        if response.status().as_u16() == 409 {
            return Err(Error::TooManyRequests("Mineskin"));
        }

        response.error_for_status_ref()?;

        let response: MineskinResponse = response.json().await?;
        if let Some(error) = response.error {
            if let Some(error_code) = error.error_code {
                return Err(Error::Upstream("Mineskin", error_code));
            }

            if error.next_request.is_some() {
                return Err(Error::TooManyRequests("Mineskin"));
            }

            return Err(Error::Upstream("Mineskin", error.error));
        }

        if let Some(data) = response.data {
            Ok(Skin {
                value: data.texture.value,
                signature: data.texture.signature,
            })
        } else {
            Err(Error::Upstream("Mineskin", "API did not return data".to_string()))
        }
    }
}
