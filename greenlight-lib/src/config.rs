//! # Greenlight Configuration
//!
//! This module defines the configuration schema for Greenlight, a boot-time validation
//! tool designed for DPU and bootc-based systems. It supports YAML configuration
//! for system-level checks and logging behavior.

use crate::{checks::Check, errors::GreenlightError};
use serde::Deserialize;
use serde_yaml::from_str;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

/// Top-level Greenlight configuration.
#[derive(Deserialize, Debug)]
pub struct Config {
    /// Required
    pub system: System,

    /// Optional
    #[serde(default)]
    pub logging: Logging,

    /// Optional
    #[serde(default)]
    pub checks: Checks,
}
impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, GreenlightError> {
        let content = std::fs::read_to_string(path).map_err(GreenlightError::Io)?;
        Self::from_str(&content)
    }
}

impl FromStr for Config {
    type Err = GreenlightError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str(s).map_err(GreenlightError::ConfigParse)
    }
}

/// System-related configuration flags.
/// We can use serde's macros to tag this enum with the key "deployment" so when parsed it will
/// take we get for example (deployment: bootc) and we can safeguard the config at the type
/// level.
/// Now we can make sure "Image Mode" deployments only has access to its own configuration
#[derive(Debug, Deserialize)]
#[serde(tag = "deployment", rename_all = "snake_case")]
pub enum System {
    Bootc {
        arch: SystemArchitecture,
        #[serde(default)]
        target: Target,
    },
    Ostree {
        arch: SystemArchitecture,
        #[serde(default)]
        target: Target,
    },
    Traditional {
        arch: SystemArchitecture,
        #[serde(default)]
        target: Target,
    },
}

/// System-related configuration flags.
#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Target {
    DPU,
    Automotive,
    #[default]
    Edge,
}

/// System
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SystemArchitecture {
    X86,
    AARCH64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Logging {
    Basic {
        level: LogLevel,
        #[serde(default = "default_path")]
        output: PathBuf,
    },
    Advanced {
        target: String,
        format: String,
    },
}

impl Default for Logging {
    fn default() -> Self {
        Logging::Basic {
            level: LogLevel::Info,
            output: default_path(),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Checks {
    pub include: Vec<Check>,
    pub exclude: Vec<Check>,
    pub required: Vec<Check>,
    pub wanted: Vec<Check>,
}

fn default_path() -> PathBuf {
    PathBuf::from("/var/log/greenlight.log")
}

#[derive(Debug, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}
