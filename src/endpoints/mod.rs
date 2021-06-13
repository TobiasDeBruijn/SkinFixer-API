pub mod url;
pub mod up;
pub mod uuid;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MineskinResponse {
    pub data: MineskinSkinData
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