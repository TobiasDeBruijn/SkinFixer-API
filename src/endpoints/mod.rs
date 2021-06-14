pub mod url;
pub mod up;
pub mod uuid;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MineskinResponse {
    pub data:   Option<MineskinSkinData>,

    #[serde(flatten)]
    pub error:  Option<MineskinError>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MineskinError {
    pub error:          String,
    pub error_code:     Option<String>,
    pub next_request:   Option<i64>,
    pub delay:          Option<i64>
}

#[derive(Deserialize)]
pub struct MineskinSkinData {
    pub texture: MineskinTextureInfo
}

#[derive(Deserialize)]
pub struct MineskinTextureInfo {
    pub value:      String,
    pub signature:  String
}

#[derive(Serialize)]
pub struct UserResponse {
    pub value:      String,
    pub signature:  String,
}