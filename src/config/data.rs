use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hosts(pub Vec<String>);

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("input/output error")]
    IOError,
    #[error("failed to parse config file")]
    ParseError,
}
