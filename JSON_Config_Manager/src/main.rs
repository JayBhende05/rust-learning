use std::fs;
use serde::Deserialize;
use std::collections::HashMap

#[derive(Deserialize,Debug)]
struct Config{
    app_name : String,
    port : u16,
    debug : bool
}


struct ConfigManager{
    data : HashMap<String, String>
}

impl ConfigManager {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get(self, key : &str) -> Option<&String> {
        self.data.get(key)
    }

    fn list(self){
         for (key, value) in &self.data {
            println!("{}: {}", key, value);
        }
    }
}

fn validate_json(config : &Config)-> Result<bool, String>{
    if config.app_name.is_empty(){
        return Err(String::from("The App Name is Empty"));
    }
    if config.port == 0 {
       return Err(String::from("Port number is Invalid"));
    }

    Ok(true)
}


fn main(){
    let content = fs::read_to_string("config.json")
                        .expect("error in getting a file");

    let config_content : Config = serde_json::from_str(&content)
                        .expect("error in getting a file ");

    let validated_json = validate_json(&config_content);

    
match validated_json {
    Ok(true) => {
        println!("This is a file content {}", content);
        println!("This is config structure content {:?}", config_content);
    }
    Ok(false) => {
        println!("Configuration validation failed");
    }
    Err(e) => {
        println!("Validation error: {}", e);
    }
}

   
    
}