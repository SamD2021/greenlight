use std::io;
use thiserror::Error;
use zbus::Error as ZbusError;

#[derive(Error, Debug)]
pub enum GreenlightError {
    #[error("Failed to parse configuration: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("DBus error: {0}")]
    Dbus(#[from] ZbusError),

    #[error("Netlink error: {0}")]
    Netlink(#[from] rtnetlink::Error),

    #[error("Check failed: {0}")]
    CheckFailed(String),

    #[error("Unsupported system deployment")]
    UnsupportedDeployment,

    #[error("Unknown error: {0}")]
    Other(String),
}
