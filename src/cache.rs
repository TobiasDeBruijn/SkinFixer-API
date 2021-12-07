use crate::appdata::AppData;
use mysql::prelude::Queryable;
use mysql::{Row, params};
use crate::Result;

pub fn set_uuid(appdata: &AppData, uuid: &str, signature: &str, value: &str) -> Result<()> {
    let mut conn = appdata.get_conn()?;
    let exp = (time::OffsetDateTime::now_utc() + time::Duration::days(60)).unix_timestamp();
    conn.exec_drop("INSERT INTO uuid_cache (uuid, signature, value, exp) VALUES (:uuid, :signature, :value, :exp)", params!(uuid, signature, value, exp))?;

    Ok(())
}

pub fn get_uuid(appdata: &AppData, uuid: &str) -> Result<Option<(String, String)>> {
    let mut conn = appdata.get_conn()?;
    let query: Vec<Row> = conn.exec("SELECT signature,value,exp FROM uuid_cache WHERE uuid = :uuid", params!(uuid))?;

    let mut sig = None;
    let mut val = None;
    for row in query {
        let signature = row.get::<String, &str>("signature");
        let value = row.get::<String, &str>("value");
        let exp = row.get::<i64, &str>("exp");

        if let (Some(signature), Some(value), Some(exp)) = (signature, value, exp) {
            let now = time::OffsetDateTime::now_utc().unix_timestamp();
            if now > exp {
                conn.exec_drop("DELETE FROM uuid_cache WHERE uuid = :uuid AND signature = :signature", params! {
                    "uuid" => &uuid,
                    "signature" => &signature
                })?;
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