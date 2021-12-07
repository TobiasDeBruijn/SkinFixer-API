pub mod url;
pub mod uuid;

use serde::{Deserialize, Serialize};
use paperclip::actix::Apiv2Schema;

#[derive(Deserialize, Debug)]
pub struct MineskinResponse {
    pub data:   Option<MineskinSkinData>,

    #[serde(flatten)]
    pub error:  Option<MineskinError>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MineskinError {
    pub error:          String,
    pub error_code:     Option<String>,
    pub next_request:   Option<i64>,
    pub delay:          Option<i64>
}

#[derive(Deserialize, Debug)]
pub struct MineskinSkinData {
    pub texture: MineskinTextureInfo
}

#[derive(Deserialize, Debug)]
pub struct MineskinTextureInfo {
    pub value:      String,
    pub signature:  String
}

#[derive(Serialize, Apiv2Schema)]
pub struct UserResponse {
    pub value:      String,
    pub signature:  String,
}