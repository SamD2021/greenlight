use greenlight_lib::checks::check::Check;
use greenlight_lib::config::*;
use std::path::PathBuf;

#[test]
fn test_parse_bootc_config_defaults() {
    let toml = r#"
        [system]
        deployment = "bootc"
        arch = "aarch64"
    "#;

    let config: Config = toml::from_str(toml).expect("Failed to parse TOML");
    match config.system {
        System::Bootc { arch, target } => {
            assert_eq!(arch, SystemArchitecture::AARCH64);
            assert_eq!(target, Target::Edge); // default
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn test_parse_ostree_config_defaults() {
    let toml = r#"
        [system]
        deployment = "ostree"
        arch = "aarch64"
    "#;

    let config: Config = toml::from_str(toml).expect("Failed to parse TOML");
    match config.system {
        System::Ostree { arch, target } => {
            assert_eq!(arch, SystemArchitecture::AARCH64);
            assert_eq!(target, Target::Edge); // default
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn test_parse_bootc_with_target() {
    let toml = r#"
        [system]
        deployment = "bootc"
        arch = "x86"
        target = "dpu"
    "#;

    let config: Config = toml::from_str(toml).expect("Failed to parse TOML");
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
    let toml = r#"
        [system]
        deployment = "bootc"
        arch = "x86"
        target = "edge"

        [logging]
        kind = "basic"
        level = "debug"
    "#;

    let config: Config = toml::from_str(toml).expect("Failed to parse TOML");
    match config.logging {
        Logging::Basic { level, output } => {
            assert_eq!(level, LogLevel::Debug);
            assert_eq!(output, PathBuf::from("/var/log/greenlight.log")); // default
        }
        _ => panic!("Expected basic logging"),
    }
}

#[test]
fn test_parse_check_kinds() {
    let toml = r#"
        [system]
        deployment = "bootc"
        target = "edge"
        arch = "aarch64"

        [[required.checks]]
        type = "rootfs_readonly"

        [[required.checks]]
        type = "microshift_installed"

        [[wanted.checks]]
        type = "swap_disabled"
    "#;

    let config: Config = toml::from_str(toml).expect("Failed to parse TOML");

    assert_eq!(
        config.required.checks,
        vec![Check::RootfsReadonly, Check::MicroshiftInstalled]
    );
    assert_eq!(config.wanted.checks, vec![Check::SwapDisabled]);
}

#[test]
fn test_invalid_check_kind() {
    let toml = r#"
        [system]
        deployment = "bootc"
        arch = "aarch64"
        target = "edge"

        [[required.checks]]
        type = "invalid_check"
    "#;

    let result: Result<Config, _> = toml::from_str(toml);
    assert!(result.is_err(), "Invalid check kind should fail");
}
