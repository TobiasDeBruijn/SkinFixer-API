use crate::{client, Error, Result};
use serde::Deserialize;

pub struct Mojang;

impl Mojang {
    pub async fn resolve_nickname_to_uuid<S: AsRef<str>>(nickname: S) -> Result<Option<String>> {
        let response = client()
            .post("https://api.mojang.com/profiles/minecraft")
            .json(&vec![nickname.as_ref()])
            .send()
            .await?;

        match response.status().as_u16() {
            404 => return Ok(None),
            409 => return Err(Error::TooManyRequests("Mojang")),
            _ => {}
        }

        response.error_for_status_ref()?;

        #[derive(Deserialize)]
        struct Response {
            id: String,
        }

        let response: Response = response.json().await?;
        Ok(Some(response.id))
    }
}
