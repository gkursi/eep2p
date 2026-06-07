use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use aes_gcm::aead::OsRng;
use aes_gcm::aead::rand_core::RngCore;
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};

use super::data::hosts::Hosts;
use super::error::ConfigError;
use crate::config::data::pgp::PgpKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub hosts: Hosts,
    pub port: u16,
    pub signature: SigningKey,
    pub pgp: PgpKey,
}

impl Config {
    pub fn read(path: &str) -> Result<Self, ConfigError> {
        let json = fs::read_to_string(path).map_err(|_| ConfigError::IOError)?;
        serde_json::from_str(&json).map_err(|_| ConfigError::ParseError)
    }

    pub fn read_or_create(path: &str) -> Result<Self, ConfigError> {
        if Path::new(path).exists() {
            return Self::read(path);
        }

        let config = Self {
            hosts: Hosts(Vec::new()),
            signature: SigningKey::generate(&mut OsRng),
            pgp: PgpKey::empty(),
            port: (OsRng.next_u32() % (65535 - 8000)) as u16 + 8000,
        };

        config.write(path)?;
        Err(ConfigError::PartialConfigError(path.to_string()))
    }

    pub fn write(&self, path: &str) -> Result<(), ConfigError> {
        let json = serde_json::to_string_pretty(&self).map_err(|_| ConfigError::ParseError)?;
        let mut file = File::create(path).map_err(|_| ConfigError::IOError)?;

        file.write_all(json.as_bytes())
            .map_err(|_| ConfigError::IOError)
    }

    pub fn compute_identifier(&self) -> String {
        // todo base64 of signature + public pgp key
        String::new()
    }
}
