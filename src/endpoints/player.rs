use actix_web::{web, get, HttpResponse};
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};
use crate::appdata::AppData;
use serde::{Serialize, Deserialize};
use std::net::Shutdown::Read;

#[derive(Serialize)]
struct Response {
    uuid: Option<String>
}

#[get("/player/{name}")]
pub async fn get_by_name(web::Path(nickname): web::Path<String>, data: web::Data<AppData>) -> HttpResponse {
    let mut conn = match data.pool.get_conn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to create MySQL connection: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to communicate with database");
        }
    };

    let rows = match conn.exec::<Row, &str, Params>("SELECT uuid,exp FROM player_cache WHERE nickname = :nickname", params! {
       "nickname" => &nickname
    }) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to query MySQL database for player UUID by nickname: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to query database");
        }
    };

    let uuid = if rows.is_empty() {
        let uuid = match get_uuid_from_mojang(&nickname) {
            Ok(Some(uuid)) => uuid,
            Ok(None) => return HttpResponse::NotFound().body("No UUID was found with that nickname"),
            Err(e) => {
                eprintln!("Failed to query Mojang API for UUID by nickname: {}", &e);
                return HttpResponse::InternalServerError().body(&format!("Mojang API returned an error: {}", &e));
            }

        };

        match insert_uuid_into_database(&mut conn, &nickname, &uuid) {
            Ok(_) => {},
            Err(e) => {
                //This isn't a fatal error for the caller, so we wont return an error
                eprintln!("{}", e)
            }
        };

        uuid
    } else {
        // Safe to unwrap because we know the Vec contains at least 1 Row, due to the is_empty() check above.
        let row = rows.get(0).unwrap();

        // It's safe to unwrap as the schema requires the values to be NOT NULL
        let uuid: String = row.get("uuid").unwrap();
        let exp: i64 = row.get("exp").unwrap();

        let now = chrono::Utc::now().timestamp();
        // cached UUID is expired, remove it from the database and ask Mojang for a new UUID, and then insert that
        if now >= exp {
            match conn.exec::<usize, &str, Params>("DELETE FROM player_cache WHERE uuid = :uuid", params! {
                "uuid" => &uuid
            }) {
                Ok(_) => {},
                Err(e) => {
                    //This isn't a fatal error for the caller, so we wont return an error
                    eprintln!("{:?}", e)
                }
            }

            let uuid = match get_uuid_from_mojang(&nickname) {
                Ok(Some(uuid)) => uuid,
                Ok(None) => return HttpResponse::NotFound().body(serde_json::to_string(&Response { uuid: None }).unwrap()),
                Err(e) => {
                    eprintln!("Failed to query Mojang API for UUID by nickname: {}", &e);
                    return HttpResponse::InternalServerError().body(&format!("Mojang API returned an error: {}", &e));
                }
            };

            match insert_uuid_into_database(&mut conn, &nickname, &uuid) {
                Ok(_) => {},
                Err(e) => {
                    //This isn't a fatal error for the caller, so we wont return an error
                    eprintln!("{}", e)
                }
            };

            uuid
        } else { uuid }
    };

    HttpResponse::Ok().body(serde_json::to_string(&Response { uuid: Some(uuid) }).unwrap())
}

fn insert_uuid_into_database(conn: &mut mysql::PooledConn, nickname: &str, uuid: &str) -> Result<(), String>{
    //Insert the newly found UUID into the database
    //TTL is 60 days
    let exp = chrono::Utc::now().timestamp() + (60 * 24 * 60 * 60);
    match conn.exec::<usize, &str, Params>("INSERT INTO player_cache (uuid, nickname, exp) VALUES (:uuid, :nickname, :exp)", params! {
            "uuid" => &uuid,
            "nickname" => &nickname,
            "exp" => &exp
        }) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to insert UUID into player_cache: {:?}", e))
    }
}

#[derive(Deserialize)]
struct MojangResponseItem {
    id: String,
}

fn get_uuid_from_mojang(nickname: &str) -> Result<Option<String>, String> {
    let request_body = format!(r#"["{}"]"#, nickname);
    let response = match reqwest::blocking::Client::new()
        .post("https://api.mojang.com/profiles/minecraft")
        .body(request_body)
        .send() {

        Ok(r) => r,
        Err(e) => return Err(e.to_string())
    };

    let response_body = match response.text() {
        Ok(rb) => rb,
        Err(e) => return Err(e.to_string())
    };

    if response_body.contains("error") {
        return Err("Too many requests".to_string());
    }

    let response_body: Vec<MojangResponseItem> = match serde_json::from_str(&response_body) {
        Ok(rb) => rb,
        Err(e) => {
            eprintln!("Original response body: {}", &response_body);
            return Err(format!("Failed to deserialize response: {:?}", e));
        }
    };

    if let Some(first_item) = response_body.get(0) {
        Ok(Some((*first_item).id.clone()))
    } else {
        Ok(None)
    }
}
