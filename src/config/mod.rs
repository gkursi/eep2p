pub mod hosts;
pub mod id;

use hosts::Hosts;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use aes_gcm::aead::OsRng;
use ed25519_dalek::SigningKey;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    hosts: Hosts,
    signature: SigningKey,
    pgp_public: String,
    pgp_private: String
}

impl Config {
    pub fn read_from(path: &str) -> Self {
        let json = fs::read_to_string(path).unwrap();
        serde_json::from_str(&json).unwrap()
    }

    pub fn setup(path: &str) -> Self {
        if !Path::new(path).exists() {
            let config = Self {
                hosts: Hosts(Vec::new()),
                signature: SigningKey::generate(&mut OsRng),
                pgp_public: String::new(),
                pgp_private: String::new(),
            };

            config.write_to(path);
            panic!("First run! Please setup pgp keys at {path}.");
        }
            
        Self::read_from(path)
    }

    pub fn write_to(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn compute_identifier(&self) -> String {
        // TODO base64 of signature + public pgp key
        "".to_string()
    }
}
