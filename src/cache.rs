//! Memoize expensive analysis results by content fingerprint.

use std::collections::HashMap;
use std::sync::Mutex;

#[allow(dead_code)]
pub struct Cache<T: Clone> {
    inner: Mutex<HashMap<u64, T>>,
    max_size: usize,
}

impl<T: Clone> Cache<T> {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            max_size,
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, key: u64) -> Option<T> {
        self.inner.lock().ok()?.get(&key).cloned()
    }

    #[allow(dead_code)]
    pub fn put(&self, key: u64, value: T) {
        if let Ok(mut map) = self.inner.lock() {
            if map.len() >= self.max_size {
                if let Some(&k) = map.keys().next() {
                    map.remove(&k);
                }
            }
            map.insert(key, value);
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.inner.lock().map(|m| m.len()).unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
