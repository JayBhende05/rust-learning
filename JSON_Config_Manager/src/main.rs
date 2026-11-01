
use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    app_name: String,
    port: u16,
    debug: bool,
}

struct ConfigManager {
    data: HashMap<String, String>,
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

   
    fn get(&self, key: &str) -> Result<&String, String> {
        match self.data.get(key) {
            Some(value) => Ok(value),
            None => Err(format!("Configuration '{}' not found", key)),
        }
    }

   
    fn list(&self) {
        for (key, value) in &self.data {
            println!("{}: {}", key, value);
        }
    }

   
    fn remove(&mut self, key: &str) -> Result<String, String> {
        match self.data.remove(key) {
            Some(value) => Ok(value),
            None => Err(format!("Configuration '{}' not found", key)),
        }
    }

   
    fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}


fn validate_config(config: &Config) -> Result<(), String> {
    if config.app_name.trim().is_empty() {
        return Err(String::from("App name cannot be empty"));
    }

    if config.port == 0 {
        return Err(String::from("Port number cannot be zero"));
    }

    Ok(())
}


fn read_config() -> Result<String, std::io::Error> {
    let contents = fs::read_to_string("config.json")?;

    Ok(contents)
}

fn main() {

    let contents = match read_config() {
        Ok(value) => value,
        Err(error) => {
            println!("Failed to read config: {}", error);
            return;
        }
    };


    let config: Config = match serde_json::from_str(&contents) {
        Ok(value) => value,
        Err(error) => {
            println!("Invalid JSON: {}", error);
            return;
        }
    };


    if let Err(error) = validate_config(&config) {
        println!("Validation error: {}", error);
        return;
    }

    println!("Configuration is valid!");

   
    let mut manager = ConfigManager::new();


    manager.set("app_name".to_string(), config.app_name);
    manager.set("port".to_string(), config.port.to_string());
    manager.set("debug".to_string(), config.debug.to_string());


    match manager.get("app_name") {
        Ok(value) => println!("\nApp Name: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    match manager.get("database_url") {
        Ok(value) => println!("Database: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    match manager.remove("debug") {
        Ok(value) => println!("\nRemoved debug: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    println!("\nRemaining configurations:");
    manager.list();
}
