use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use dal::Dal;

pub type WebData = paperclip::actix::web::Data<AppData>;

#[derive(Clone, Debug)]
pub struct AppData {
    pub dal: Dal,
    pub config: Config,
    key_index: Arc<AtomicUsize>,
}

impl AppData {
    pub fn new(dal: Dal, config: Config) -> Self {
        Self {
            dal,
            config,
            key_index: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn get_key(&self) -> String {
        let index = self.key_index.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
            let mut new_index = x + 1;
            if new_index > self.config.mineskin_keys.len() {
                new_index = 0;
            }

            Some(new_index)
        }).unwrap_or_else(|x| x);

        let key = self.config.mineskin_keys.get(index).unwrap_or_else(|| panic!("Missing mineskin key at index {index}"));
        key.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub mineskin_keys: Vec<String>,
}