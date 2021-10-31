use std::collections::HashMap;

pub struct StringKV {

    m: HashMap<String, String>,

}

impl StringKV {
    pub fn new() -> StringKV {
        StringKV{
            m: HashMap::new()
        }
    }
   
    pub fn put(&mut self, k: String, v: String) {
        self.m.insert(k, v);
    } 
    pub fn get(&self, k: &str) -> &str {
        match self.m.get(k) {
            Some(v) => v,
            None => ""
        }
    }
}