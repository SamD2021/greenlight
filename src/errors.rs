use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GreenlightError {
    #[error("Failed to parse configuration: {0}")]
    ConfigParse(#[from] serde_yaml::Error),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Check failed: {0}")]
    CheckFailed(String),

    #[error("Unsupported system deployment")]
    UnsupportedDeployment,

    #[error("Unknown error: {0}")]
    Other(String),
}
