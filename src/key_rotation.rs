use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct KeyRotation {
    keys: Vec<String>,
    index: Arc<AtomicUsize>,
}

impl Clone for KeyRotation {
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
            index: self.index.clone(),
        }
    }
}

impl KeyRotation {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            keys,
            index: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn get_index(&self) -> usize {
        self.index.load(Ordering::SeqCst)
    }

    fn set_index(&self, new_index: usize) {
        self.index.store(new_index, Ordering::SeqCst);
    }

    fn increment_index(&self) {
        let new_index = self.get_index() + 1;

        if new_index >= self.keys.len() {
            self.set_index(0);
        } else {
            self.set_index(new_index);
        }
    }

    pub fn next_key(&self) -> &String {
        let index = self.get_index();
        self.increment_index();

        self.keys
            .get(index)
            .unwrap_or_else(|| self.keys.first().unwrap())
    }
}
