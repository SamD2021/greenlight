#[cfg(test)]
mod tests {
    use greenlight::{config::Config, errors::GreenlightError};

    // use super::*;

    #[test]
    fn test_config_parse_success() {
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

        let result: Result<Config, GreenlightError> =
            serde_yaml::from_str(yaml).map_err(GreenlightError::from);
        assert!(
            result.is_ok(),
            "Expected config to parse successfully, got error: {:?}",
            result
        );
    }

    #[test]
    fn test_config_parse_fail() {
        let invalid_yaml = r#"
            system:
              deployment: bootc
              arch: "???"
        "#;

        let result: Result<Config, GreenlightError> =
            serde_yaml::from_str(invalid_yaml).map_err(GreenlightError::from);
        assert!(matches!(result, Err(GreenlightError::ConfigParse(_))));
    }

    #[test]
    fn test_check_failed_error() {
        let error = GreenlightError::CheckFailed("RootFS is not readonly".to_string());
        assert_eq!(error.to_string(), "Check failed: RootFS is not readonly");
    }

    #[test]
    fn test_other_error() {
        let error = GreenlightError::Other("unexpected failure".into());
        assert_eq!(error.to_string(), "Unknown error: unexpected failure");
    }

    #[test]
    fn test_unsupported_deployment() {
        let error = GreenlightError::UnsupportedDeployment;
        assert_eq!(error.to_string(), "Unsupported system deployment");
    }
}
