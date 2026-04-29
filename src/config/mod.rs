pub mod hosts;
pub mod id;

use hosts::Hosts;
use id::Identifier;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Config {
    hosts: Hosts,
    id: Identifier,
}

impl Config {
    pub fn read_from(path: &str) -> Self {
        let json = fs::read_to_string(path).unwrap();
        serde_json::from_str(json).unwrap();
    }

    pub fn setup(path: &str) -> Self {
        let config = if !Path::new(path).exists() {
                
        }

        config.write_to(path);
        config
    }

    pub fn write_to(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        let mut file = File::create(path);
        file.write_all(json.as_bytes()).unwrap();
    }
}
