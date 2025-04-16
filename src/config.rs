//! # Greenlight Configuration
//!
//! This module defines the configuration schema for Greenlight, a boot-time validation
//! tool designed for DPU and bootc-based systems. It supports YAML configuration
//! for system-level checks and logging behavior.

use crate::checks::CheckKind;
use serde::Deserialize;
use std::path::PathBuf;

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
    pub include: Vec<CheckKind>,
    pub exclude: Vec<CheckKind>,
    pub required: Vec<CheckKind>,
    pub wanted: Vec<CheckKind>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_parse_bootc_config_defaults() {
        let yaml = r#"
            system:
              deployment: bootc
              arch: aarch64
        "#;

        let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
        match config.system {
            System::Bootc { arch, target } => {
                assert_eq!(arch, SystemArchitecture::AARCH64);
                assert_eq!(target, Target::Edge);
            }
            other => panic!("unexpected variant: {:?}", other),
        }
    }

    #[test]
    fn test_parse_ostree_config_defaults() {
        let yaml = r#"
            system:
              deployment: ostree
              arch: aarch64
        "#;

        let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
        match config.system {
            System::Ostree { arch, target } => {
                assert_eq!(arch, SystemArchitecture::AARCH64);
                assert_eq!(target, Target::Edge);
            }
            other => panic!("unexpected variant: {:?}", other),
        }
    }

    #[test]
    fn test_parse_bootc_with_target() {
        let yaml = r#"
            system:
              deployment: bootc
              arch: x86
              target: dpu
        "#;

        let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
        match config.system {
            System::Bootc { arch, target } => {
                assert_eq!(arch, SystemArchitecture::X86);
                assert_eq!(target, Target::DPU);
            }
            _ => panic!("Unexpected system type variant — this should not happen"),
        }
    }

    #[test]
    fn test_parse_logging_basic() {
        let yaml = r#"
            system:
              deployment: bootc
              arch: x86
              target: edge
            logging:
              kind: basic
              level: debug
        "#;

        let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
        match config.logging {
            Logging::Basic { level, output } => {
                assert_eq!(level, LogLevel::Debug);
                assert_eq!(output, PathBuf::from("/var/log/greenlight.log"));
            }
            _ => panic!("Expected basic logging"),
        }
    }

    #[test]
    fn test_parse_check_kinds() {
        let yaml = r#"
            system:
              deployment: bootc
              target: edge
              arch: aarch64
            checks:
              include:
                - rootfs_readonly
                - microshift_installed
              exclude:
                - swap_disabled
              required:
                - rootfs_readonly
              wanted:
                - microshift_installed
        "#;

        let config: Config = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
        assert_eq!(
            config.checks.include,
            vec![CheckKind::RootFsReadonly, CheckKind::MicroShiftInstalled]
        );
        assert_eq!(config.checks.exclude, vec![CheckKind::SwapDisabled]);
        assert_eq!(config.checks.required, vec![CheckKind::RootFsReadonly]);
        assert_eq!(config.checks.wanted, vec![CheckKind::MicroShiftInstalled]);
    }

    #[test]
    fn test_invalid_check_kind() {
        let yaml = r#"
            system:
              deployment: bootc
              arch: aarch64
              target: edge
            checks:
              include:
                - invalid_check
        "#;

        let result: Result<Config, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err(), "Invalid check kind should fail");
    }
}
