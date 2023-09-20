use crate::error::Error;
use crate::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct MojangResponseItem {
    id: String,
}

pub async fn get_uuid_from_mojang(nickname: &str) -> Result<Option<String>> {
    let request_body = format!(r#"["{}"]"#, nickname);
    let response = reqwest::Client::new()
        .post("https://api.mojang.com/profiles/minecraft")
        .body(request_body)
        .send()
        .await?
        .text()
        .await?;

    if response.contains("error") {
        return Err(Error::TooManyRequests);
    }

    let response: Vec<MojangResponseItem> = serde_json::from_str(&response)?;
    if let Some(first_item) = response.get(0) {
        Ok(Some((*first_item).id.clone()))
    } else {
        Ok(None)
    }
}
