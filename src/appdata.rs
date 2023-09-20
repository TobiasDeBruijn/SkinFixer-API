use crate::key_rotation::KeyRotation;
use serde::Deserialize;
use sqlx::migrate::Migrator;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::sqlx_macros::migrate;
use sqlx::{MySql, Pool};

#[derive(Clone, Debug)]
pub struct AppData {
    pub keys: KeyRotation,
    pub pool: Pool<MySql>,
}

const MIGRATOR: Migrator = migrate!("./migrations");

impl AppData {
    pub async fn new(env: &Env) -> color_eyre::Result<Self> {
        let key_vec: Vec<String> = env.api_key.split(",").map(|c| c.to_string()).collect();
        let keys = KeyRotation::new(key_vec);

        let opts = MySqlConnectOptions::new()
            .username(&env.db_username)
            .password(&env.db_password)
            .host(&env.db_host)
            .database(&env.db_name);
        let pool = sqlx::MySqlPool::connect_with(opts).await?;

        MIGRATOR.run(&mut pool.acquire().await?).await?;

        let this = Self { keys, pool };
        Ok(this)
    }
}

#[derive(Debug, Deserialize)]
pub struct Env {
    api_key: String,
    db_host: String,
    db_name: String,
    db_username: String,
    db_password: String,
}

impl Env {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(envy::from_env()?)
    }
}
