use std::ops::Deref;
use mysql::{OptsBuilder, Pool};
use thiserror::Error;

mod playername_cache;
mod premium_skin_cache;

pub use playername_cache::*;
pub use premium_skin_cache::*;

pub(crate) type Result<T> = std::result::Result<T, Error>;
/// After how many seconds does a cached skin expire
pub(crate) const PREMIUM_SKIN_CACHE_TTL: i64 = 2_630_000; // 1 month
/// After how many seconds does a cached playername expire
pub(crate) const PLAYERNAME_CACHE_TTL: i64 = 2_630_000; // 1 month

/// Possible errors from the DAL
#[derive(Debug, Error)]
pub enum Error {
    /// Mysql error
    #[error("{0}")]
    Mysql(#[from] mysql::Error),
    /// Migrations error
    #[error("{0}")]
    Refinery(#[from] refinery::Error)
}

/// Embedded migrations
mod migrations {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

/// Mysql credentials
#[derive(Debug, Clone)]
pub struct Credentials {
    /// Mysql host
    pub host: String,
    /// Mysql database
    pub database: String,
    /// Mysql username
    pub username: String,
    /// Mysql password
    pub password: String,
}

/// The Dal
#[derive(Debug, Clone)]
pub struct Dal(Pool);

impl Deref for Dal {
    type Target = Pool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Dal {
    /// Create a new Dal
    ///
    /// # Errors
    ///
    /// If applying migrations fails, or if creating the MySQL connection fails
    pub fn new(credentials: Credentials) -> Result<Self> {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(credentials.host))
            .db_name(Some(credentials.database))
            .user(Some(credentials.username))
            .pass(Some(credentials.password));
        let pool = Pool::new(opts)?;

        let mut conn = pool.get_conn()?;
        migrations::migrations::runner()
            .set_migration_table_name("__skinfixer_api_migrations")
            .run(&mut conn)?;

        Ok(Self(pool))
    }
}

