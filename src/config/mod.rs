pub mod data;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use aes_gcm::aead::OsRng;
use aes_gcm::aead::rand_core::RngCore;
use data::{ConfigError, Hosts};
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub hosts: Hosts,
    pub port: u16,
    pub signature: SigningKey,
    pub pgp_public: String,
    pub pgp_private: String,
}

impl Config {
    pub fn read_from(path: &str) -> Result<Self, ConfigError> {
        let json = fs::read_to_string(path).map_err(|_| ConfigError::IOError)?;

        serde_json::from_str(&json).map_err(|_| ConfigError::ParseError)
    }

    pub fn setup(path: &str) -> Result<Self, ConfigError> {
        if !Path::new(path).exists() {
            let config = Self {
                hosts: Hosts(Vec::new()),
                port: ((f64::from(OsRng.next_u32()) / f64::from(u32::MAX)) * (65535.0 - 8000.0))
                    as u16
                    + 8000,
                signature: SigningKey::generate(&mut OsRng),
                pgp_public: String::new(),
                pgp_private: String::new(),
            };

            config.write_to(path)?;
            panic!("First run! Please setup pgp keys at {path}.");
        }

        Self::read_from(path)
    }

    pub fn write_to(&self, path: &str) -> Result<(), ConfigError> {
        let json = serde_json::to_string_pretty(&self).map_err(|_| ConfigError::ParseError)?;
        let mut file = File::create(path).map_err(|_| ConfigError::IOError)?;

        file.write_all(json.as_bytes())
            .map_err(|_| ConfigError::IOError)
    }

    pub fn compute_identifier(&self) -> String {
        // TODO base64 of signature + public pgp key
        String::new()
    }
}
