use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct KeyRotation {
    keys: Vec<String>,
    index: AtomicUsize,
}

impl Clone for KeyRotation {
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
            index: AtomicUsize::new(self.index.load(Ordering::SeqCst)),
        }
    }
}

impl KeyRotation {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            keys,
            index: AtomicUsize::new(0),
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
