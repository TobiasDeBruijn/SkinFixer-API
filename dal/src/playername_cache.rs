use mysql::{params, Row};
use mysql::prelude::Queryable;
use crate::{Dal, PLAYERNAME_CACHE_TTL, Result};

pub struct NamedPlayer {
    pub nickname: String,
    pub uuid: String,
}

impl NamedPlayer {
    pub fn get(dal: Dal, nickname: String) -> Result<Option<Self>> {
        let mut conn = dal.get_conn()?;
        let row: Row = match conn.exec_first("SELECT uuid,expires_at FROM premium_playername_cache WHERE nickname = :nickname", params! {
            "nickname" => &nickname
        })? {
            Some(x) => x,
            None => return Ok(None)
        };

        let expires_at: i64 = row.get("expires_at").unwrap();
        if time::OffsetDateTime::now_utc().unix_timestamp() > expires_at {
            Self::delete(dal, nickname)?;
            return Ok(None)
        }

        Ok(Some(Self {
            nickname,
            uuid: row.get("uuid").unwrap()
        }))
    }

    pub fn delete(dal: Dal, nickname: String) -> Result<()> {
        let mut conn = dal.get_conn()?;
        conn.exec_drop("DELETE FROM premium_playername_cache WHERE nickname = :nickname", params! {
            "nickname" => &nickname
        })?;
        Ok(())
    }

    pub fn insert(&self, dal: Dal) -> Result<()> {
        let mut conn = dal.get_conn()?;
        conn.exec_drop("INSERT INTO premium_playername_cache (nickname, uuid, expires_at) VALUES (:nickname, :uuid, :expires_at)", params! {
            "nickname" => &self.nickname,
            "uuid" => &self.uuid,
            "expires_at" => time::OffsetDateTime::now_utc().unix_timestamp() + PLAYERNAME_CACHE_TTL
        })?;
        Ok(())
    }
}
