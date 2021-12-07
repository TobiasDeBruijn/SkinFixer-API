use std::sync::atomic::{AtomicUsize, Ordering};
use serde::Deserialize;
use crate::Result;

mod migrations {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[derive(Clone, Debug)]
pub struct AppData {
    pub keys: KeyRotation,
    pub pool: mysql::Pool
}

impl AppData {
    pub fn new(env: &Env) -> Result<Self> {
        let key_vec: Vec<String> = env.api_key.split(",").map(|c| c.to_string()).collect();
        let keys = KeyRotation::new(key_vec);

        let opts = mysql::OptsBuilder::new()
            .ip_or_hostname(Some(&env.db_host))
            .db_name(Some(&env.db_name))
            .user(Some(&env.db_username))
            .pass(Some(&env.db_password));
        let pool = mysql::Pool::new(opts)?;

        let this = Self {
            keys,
            pool
        };
        Ok(this)
    }

    pub fn get_conn(&self) -> mysql::Result<mysql::PooledConn> {
        self.pool.get_conn()
    }

    pub fn migrate(&self) -> Result<()> {
        let mut conn = self.get_conn()?;
        migrations::migrations::runner().run(&mut conn)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Env {
    api_key:       String,
    db_host:        String,
    db_name:        String,
    db_username:    String,
    db_password:    String
}

impl Env {
    pub fn new() -> Result<Self> {
        Ok(envy::from_env()?)
    }
}

#[derive(Debug)]
pub struct KeyRotation {
    keys:   Vec<String>,
    index:  AtomicUsize,
}

impl Clone for KeyRotation {
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
            index: AtomicUsize::new(self.index.load(Ordering::SeqCst))
        }
    }
}

impl KeyRotation {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            keys,
            index: AtomicUsize::new(0)
        }
    }

    fn increment_index(&self) {
        let new_index = self.index.load(Ordering::SeqCst);
        if new_index > self.keys.len() {
            self.index.store(0, Ordering::SeqCst);
        }

        self.index.store(new_index, Ordering::SeqCst);
    }

    pub fn get_key(&self) -> &String {
        let index = self.index.load(Ordering::SeqCst);
        self.increment_index();

        let key = self.keys.get(index).unwrap();
        key
    }
}