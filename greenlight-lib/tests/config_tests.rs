use greenlight_lib::checks::check::Check;
use greenlight_lib::config::*;
use std::path::PathBuf;
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
        vec![Check::RootfsReadonly, Check::MicroshiftInstalled]
    );
    assert_eq!(config.checks.exclude, vec![Check::SwapDisabled]);
    assert_eq!(config.checks.required, vec![Check::RootfsReadonly]);
    assert_eq!(config.checks.wanted, vec![Check::MicroshiftInstalled]);
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
