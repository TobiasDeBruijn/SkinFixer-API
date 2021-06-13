#[derive(Clone, Debug)]
pub struct AppData {
    pub api_key: Option<String>
}

impl AppData {
    pub fn new() -> Self {
        use std::env::var;

        let api_key = match var("API_KEY") {
            Ok(k) => Some(k),
            Err(_) => None
        };

        Self {
            api_key
        }
    }
}