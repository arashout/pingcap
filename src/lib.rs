use std::collections::HashMap;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).map(|v| v.to_owned())
    }
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
}
