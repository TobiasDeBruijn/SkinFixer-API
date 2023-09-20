use crate::Result;
use sqlx::{FromRow, MySql, Pool};

pub struct Player {
    pub uuid: String,
    pub exp: i64,
}

#[derive(FromRow)]
struct _Player {
    uuid: String,
    exp: i64,
}

pub async fn get_player_by_nickname(pool: &Pool<MySql>, nickname: &str) -> Result<Option<Player>> {
    let row: Option<_Player> =
        sqlx::query_as("SELECT uuid,exp FROM player_cache WHERE nickname = ?")
            .bind(&nickname)
            .fetch_optional(pool)
            .await?;

    Ok(row.map(|row| Player {
        uuid: row.uuid,
        exp: row.exp,
    }))
}

pub async fn delete_player(pool: &Pool<MySql>, uuid: &str) -> Result<()> {
    sqlx::query("DELETE FROM player_cache WHERE uuid = ?")
        .bind(uuid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert_player(conn: &Pool<MySql>, nickname: &str, uuid: &str) -> Result<()> {
    //Insert the newly found UUID into the database
    //TTL is 60 days
    let exp = (time::OffsetDateTime::now_utc() + time::Duration::days(60)).unix_timestamp();
    sqlx::query("INSERT INTO player_cache (uuid, nickname, exp) VALUES (?, ?, ?)")
        .bind(uuid)
        .bind(nickname)
        .bind(exp)
        .execute(conn)
        .await?;

    Ok(())
}
