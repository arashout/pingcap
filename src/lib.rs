pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore{
        KvStore{}
    }
    pub fn get(&self, key: String) -> Option<String>{
        Some(key.to_owned())
    }
    pub fn remove(&self, key: String){

    }
    pub fn set(&mut self, key: String, value: String){

    }
}
