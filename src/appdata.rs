use std::sync::atomic::{AtomicUsize, Ordering};
use mysql::prelude::Queryable;

#[derive(Clone, Debug)]
pub struct AppData {
    pub keys: KeyRotation,
    pub pool: mysql::Pool
}

impl AppData {
    pub fn new(env: &Env) -> Result<Self, String> {
        let key_vec: Vec<String> = env.api_key.split(",").map(|c| c.to_string()).collect();
        let keys = KeyRotation::new(key_vec);

        let mysql_uri = format!("mysql://{}:{}@{}/{}", env.db_username, env.db_password, env.db_host, env.db_name);
        let pool = match mysql::Pool::new(mysql_uri) {
            Ok(p) => p,
            Err(e) => return Err(e.to_string())
        };

        Self::migrate(&pool)?;

        Ok(Self {
            keys,
            pool
        })
    }

    fn migrate(pool: &mysql::Pool) -> Result<(), String> {
        let mut conn = match pool.get_conn() {
            Ok(c) => c,
            Err(e) => return Err(e.to_string())
        };

        match conn.exec::<usize, &str, mysql::Params>("CREATE TABLE IF NOT EXISTS uuid_cache (`uuid` varchar(36) NOT NULL, `signature` TEXT NOT NULL, `value` TEXT NOT NULL, `exp` bigint(64) NOT NULL) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4", mysql::params::Params::Empty) {
            Ok(_) => {},
            Err(e) => return Err(e.to_string())
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Env {
    api_key:       String,
    db_host:        String,
    db_name:        String,
    db_username:    String,
    db_password:    String
}

impl Env {
    pub fn new() -> Result<Self, &'static str> {
        use std::env::var;

        let api_key = var("API_KEY");
        if api_key.is_err() {
            return Err("Environmental variabble 'API_KEY' is invalid or unset.");
        }

        let db_host = var("DB_HOST");
        if db_host.is_err() {
            return Err("Environmental variabble 'DB_HOST' is invalid or unset.");
        }

        let db_name = var("DB_NAME");
        if db_name.is_err() {
            return Err("Environmental variabble 'DB_NAME' is invalid or unset.");
        }

        let db_username = var("DB_USERNAME");
        if db_username.is_err() {
            return Err("Environmental variabble 'DB_USERNAME' is invalid or unset.");
        }

        let db_password = var("DB_PASSWORD");
        if db_password.is_err() {
            return Err("Environmental variabble 'DB_PASSWORD' is invalid or unset.");
        }

        Ok(Self {
            api_key: api_key.unwrap(),
            db_host: db_host.unwrap(),
            db_name: db_name.unwrap(),
            db_username: db_username.unwrap(),
            db_password: db_password.unwrap()
        })
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