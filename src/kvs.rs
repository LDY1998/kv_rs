use std::collections::HashMap;

pub struct KvStore {
    values: HashMap<String, String>,
}


impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            values: HashMap::new()
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.values.get(&key) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.values.remove(&key);
    }
}