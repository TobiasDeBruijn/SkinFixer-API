use paperclip::actix::{web, get, api_v2_operation, Apiv2Schema};
use mysql::prelude::Queryable;
use mysql::{Row, params};
use crate::appdata::AppData;
use serde::{Serialize, Deserialize};
use crate::Result;
use crate::error::Error;

#[derive(Serialize, Apiv2Schema)]
pub struct Response {
    uuid: Option<String>
}

/// Resolve a playername to a UUID
#[get("/player/{name}")]
#[api_v2_operation]
pub async fn get_by_name(web::Path(nickname): web::Path<String>, data: web::Data<AppData>) -> Result<web::Json<Response>> {
    let mut conn = data.get_conn()?;

    let row: Option<Row> = conn.exec_first("SELECT uuid,exp FROM player_cache WHERE nickname = :nickname", params! {
       "nickname" => &nickname
    })?;

    let uuid = if let Some(row) = row {
        let uuid: String = row.get("uuid").unwrap();
        let exp: i64 = row.get("exp").unwrap();
        let now = time::OffsetDateTime::now_utc().unix_timestamp();

        // cached UUID is expired, remove it from the database and ask Mojang for a new UUID, and then insert that
        if now >= exp {
            conn.exec_drop("DELETE FROM player_cache WHERE uuid = :uuid", params!(uuid))?;

            let uuid = match get_uuid_from_mojang(&nickname)? {
                Some(uuid) => uuid,
                None => return Err(Error::NotFound(serde_json::to_string(&Response { uuid: None })?)),
            };

            insert_uuid_into_database(&mut conn, &nickname, &uuid)?;
            uuid
        } else { uuid }
    } else {
        let uuid = match get_uuid_from_mojang(&nickname)? {
            Some(uuid) => uuid,
            None => return Err(Error::NotFound("No UUID was found for that nickname".to_string())),
        };

        insert_uuid_into_database(&mut conn, &nickname, &uuid)?;
        uuid
    };


    Ok(web::Json(Response { uuid: Some(uuid) }))
}

fn insert_uuid_into_database(conn: &mut mysql::PooledConn, nickname: &str, uuid: &str) -> Result<()>{
    //Insert the newly found UUID into the database
    //TTL is 60 days
    let exp = (time::OffsetDateTime::now_utc() + time::Duration::days(60)).unix_timestamp();
    conn.exec_drop("INSERT INTO player_cache (uuid, nickname, exp) VALUES (:uuid, :nickname, :exp)", params!(uuid, nickname, exp))?;
    Ok(())
}

#[derive(Deserialize)]
struct MojangResponseItem {
    id: String,
}

fn get_uuid_from_mojang(nickname: &str) -> Result<Option<String>> {
    let request_body = format!(r#"["{}"]"#, nickname);
    let response = reqwest::blocking::Client::new()
        .post("https://api.mojang.com/profiles/minecraft")
        .body(request_body)
        .send()?
        .text()?;

    if response.contains("error") {
        return Err(Error::TooManyRequests)
    }

    let response: Vec<MojangResponseItem> = serde_json::from_str(&response)?;
    if let Some(first_item) = response.get(0) {
        Ok(Some((*first_item).id.clone()))
    } else {
        Ok(None)
    }
}
