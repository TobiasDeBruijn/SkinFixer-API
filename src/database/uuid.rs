use crate::appdata::AppData;
use crate::Result;
use sqlx::FromRow;

#[derive(FromRow)]
struct _Uuid {
    signature: String,
    value: String,
    exp: i64,
}

pub async fn set_uuid(appdata: &AppData, uuid: &str, signature: &str, value: &str) -> Result<()> {
    let exp = (time::OffsetDateTime::now_utc() + time::Duration::days(60)).unix_timestamp();
    sqlx::query("INSERT INTO uuid_cache (uuid, signature, value, exp) VALUES (?, ?, ?, ?)")
        .bind(uuid)
        .bind(signature)
        .bind(value)
        .bind(exp)
        .execute(&appdata.pool)
        .await?;

    Ok(())
}

pub async fn get_uuid(appdata: &AppData, uuid: &str) -> Result<Option<(String, String)>> {
    let query: Vec<_Uuid> =
        sqlx::query_as("SELECT signature, value, exp FROM uuid_cache WHERE uuid = ?")
            .bind(uuid)
            .fetch_all(&appdata.pool)
            .await?;

    let mut sig = None;
    let mut val = None;
    for row in query {
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        if now > row.exp {
            sqlx::query("DELETE FROM uuid_cache WHERE uuid = ? AND signature = ?")
                .bind(&uuid)
                .bind(&row.signature)
                .execute(&appdata.pool)
                .await?;
        }

        sig = Some(row.signature);
        val = Some(row.value);
    }

    Ok(match (sig, val) {
        (Some(s), Some(v)) => Some((s, v)),
        _ => None,
    })
}
