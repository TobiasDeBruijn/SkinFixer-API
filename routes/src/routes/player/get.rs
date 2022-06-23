use serde::{Serialize, Deserialize};
use paperclip::actix::{Apiv2Schema, api_v2_operation, web};
use tracing::instrument;
use dal::NamedPlayer;
use crate::{Error, WebData, WebResult};

#[derive(Serialize, Apiv2Schema)]
pub struct Response {
    uuid: String,
}

#[derive(Debug, Deserialize, Apiv2Schema)]
pub struct Path {
    name: String,
}

#[instrument]
#[api_v2_operation]
pub async fn get(data: WebData, path: web::Path<Path>) -> WebResult<web::Json<Response>> {
    let cached_player = NamedPlayer::get(data.dal.clone(), path.name.clone())?;

    let uuid = if let Some(player) = cached_player {
        player.uuid
    } else {
        let player = api::Mojang::resolve_nickname_to_uuid(&path.name).await?.ok_or(Error::NotFound("Could not resolve nickname"))?;
        NamedPlayer {
            nickname: path.name.to_string(),
            uuid: player.clone()
        }.insert(data.dal.clone())?;

        player
    };

    Ok(web::Json(Response {
        uuid
    }))
}