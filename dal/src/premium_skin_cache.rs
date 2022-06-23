use mysql::{params, Row};
use mysql::prelude::Queryable;
use crate::{Dal, PREMIUM_SKIN_CACHE_TTL, Result};

pub struct PremiumSkin {
    pub uuid: String,
    pub signature: String,
    pub value: String,
}

impl PremiumSkin {
    pub fn get(dal: Dal, uuid: String) -> Result<Option<Self>> {
        let mut conn = dal.get_conn()?;
        let row: Row = match conn.exec_first("SELECT signature, value, expires_at FROM premium_skin_cache WHERE uuid = :uuid", params! {
            "uuid" => &uuid
        })? {
            Some(x) => x,
            None => return Ok(None)
        };

        let expires_at: i64 = row.get("expires_at").unwrap();
        if time::OffsetDateTime::now_utc().unix_timestamp() > expires_at {
            Self::remove(dal, uuid)?;
            return Ok(None);
        }

        Ok(Some(Self {
            uuid,
            signature: row.get("signature").unwrap(),
            value: row.get("value").unwrap(),
        }))
    }

    pub fn remove(dal: Dal, uuid: String) -> Result<()> {
        let mut conn = dal.get_conn()?;
        conn.exec_drop("DELETE FROM premium_skin_cache WHERE uuid = :uuid", params! {
            "uuid" => &uuid
        })?;
        Ok(())
    }

    pub fn insert(&self, dal: Dal) -> Result<()> {
        let mut conn = dal.get_conn()?;
        conn.exec_drop("INSERT INTO premium_skin_cache (uuid, signature, value, expires_at) VALUES (:uuid, :signature, :value, :expires_at)", params! {
            "uuid" => &self.uuid,
            "signature" => &self.signature,
            "value" => &self.value,
            "expires_at" => time::OffsetDateTime::now_utc().unix_timestamp() + PREMIUM_SKIN_CACHE_TTL
        })?;
        Ok(())
    }
}