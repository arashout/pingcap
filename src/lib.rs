#![feature(try_trait)]

use std::collections::HashMap;
use std::ops::Try;

pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
        }
    }
    pub fn get(&self, key: String) -> Result<Option<String>, std::option::NoneError> {
        self.map.get(&key).map(|v| Some(v.to_owned())).into_result()
    }
    pub fn remove(&mut self, key: String) -> Result<(), std::option::NoneError>{
        self.map.remove(&key).map(|v| ()).into_result()
    }
    pub fn set(&mut self, key: String, value: String) -> Result<(), std::option::NoneError> {
        self.map.insert(key, value).map(|v| ()).into_result()
    }
    pub fn open(){
        unimplemented!();
    }
}
