use std::collections::HashMap;
use std::env;

pub struct EnvManager{
    vars: HashMap<String, String>,
}

impl EnvManager {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        for (key, value) in env::vars()  {
            vars.insert(key, value);
        }
        EnvManager { vars }
    }

    pub  fn get(&self, key: &str) -> Option<&String>{
        self.vars.get(key)
    }

    #[allow(dead_code)]   // TODO
    pub fn set(&mut self, key: &str, value: &str){
        self.vars.insert(key.to_string(), value.to_string());
        env::set_var(key, value);
    }

}