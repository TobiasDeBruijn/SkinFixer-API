use crate::api::get_uuid_from_mojang;
use crate::appdata::AppData;
use crate::database::{delete_player, get_player_by_nickname, insert_player};
use crate::error::Error;
use crate::Result;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Response {
    uuid: Option<String>,
}

#[derive(Deserialize)]
pub struct Path {
    nickname: String,
}

/// Resolve a playername to a UUID
pub async fn get_by_name(
    path: web::Path<Path>,
    data: web::Data<AppData>,
) -> Result<web::Json<Response>> {
    let uuid = if let Some(row) = get_player_by_nickname(&data.pool, &path.nickname).await? {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        // cached UUID is expired, remove it from the database and ask Mojang for a new UUID, and then insert that
        if now >= row.exp {
            delete_player(&data.pool, &row.uuid).await?;

            let uuid = match get_uuid_from_mojang(&path.nickname).await? {
                Some(uuid) => uuid,
                None => {
                    return Err(Error::NotFound(serde_json::to_string(&Response {
                        uuid: None,
                    })?))
                }
            };

            insert_player(&data.pool, &path.nickname, &uuid).await?;
            uuid
        } else {
            row.uuid
        }
    } else {
        let uuid = match get_uuid_from_mojang(&path.nickname).await? {
            Some(uuid) => uuid,
            None => {
                return Err(Error::NotFound(
                    "No UUID was found for that nickname".to_string(),
                ))
            }
        };

        insert_player(&data.pool, &path.nickname, &uuid).await?;
        uuid
    };

    Ok(web::Json(Response { uuid: Some(uuid) }))
}
