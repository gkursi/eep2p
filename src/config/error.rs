use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("input/output error")]
    IOError,
    #[error("failed to parse config file")]
    ParseError,
    #[error("first run, set up keys at `{0}`")]
    PartialConfigError(String),
}
