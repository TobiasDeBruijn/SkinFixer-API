use crate::appdata::AppData;
use mysql::prelude::Queryable;
use mysql::{Row, Params, params};

pub fn set_uuid(appdata: &AppData, uuid: &str, signature: &str, value: &str) -> Result<(), String> {
    let mut conn = match appdata.pool.get_conn() {
        Ok(c) => c,
        Err(e) => return Err(e.to_string())
    };

    let exp = chrono::Utc::now().timestamp() + (10 * 24 * 60 * 60);
    match conn.exec::<usize, &str, Params>("INSERT INTO uuid_cache (uuid, signature, value, exp) VALUES (:uuid, :signature, :value, :exp)", params! {
        "uuid" => uuid,
        "signature" => signature,
        "value" => value,
        "exp" => exp
    }) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

pub fn get_uuid(appdata: &AppData, uuid: &str) -> Result<Option<(String, String)>, String> {
    let mut conn = match appdata.pool.get_conn() {
        Ok(c) => c,
        Err(e) => return Err(e.to_string())
    };

    let query = match conn.exec::<Row, &str, Params>("SELECT signature,value,exp FROM uuid_cache WHERE uuid = :uuid", params! {
        "uuid" => uuid
    }) {
        Ok(q) => q,
        Err(e) => return Err(e.to_string())
    };


    let mut sig = None;
    let mut val = None;
    for row in query {
        let signature = row.get::<String, &str>("signature");
        let value = row.get::<String, &str>("value");
        let exp = row.get::<i64, &str>("exp");

        if let (Some(signature), Some(value), Some(exp)) = (signature, value, exp) {
            let now = chrono::Utc::now().timestamp();
            if now > exp {
                match conn.exec::<Row, &str, Params>("DELETE FROM uuid_cache WHERE uuid = :uuid AND signature = :signature", params! {
                    "uuid" => uuid,
                    "signature" => signature
                }) {
                    Ok(_) => continue,
                    Err(e) => return Err(e.to_string())
                }
            }

            sig = Some(signature);
            val = Some(value);
        }
    }

    Ok(match (sig, val) {
        (Some(s), Some(v)) => {
            Some((s, v))
        },
        _ => None
    })
}