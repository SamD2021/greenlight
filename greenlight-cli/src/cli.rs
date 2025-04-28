use std::path::PathBuf;

use clap::Parser;
use greenlight_lib::{
    checks::Check,
    config::{SystemArchitecture, Target},
};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub include_checks: Option<Vec<Check>>,
    #[arg(short, long, value_enum)]
    pub exclude_checks: Option<Vec<Check>>,

    #[arg(short = 'f', long, value_name = "FILE")]
    pub config_path: Option<PathBuf>,
    #[arg(short, long, value_enum)]
    pub deployment: Option<Deployment>,
    #[arg(short, long, value_enum)]
    pub target: Option<Target>,
    #[arg(short, long, value_enum)]
    pub arch: Option<SystemArchitecture>,
}

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum Deployment {
    Bootc,
    Ostree,
    Traditional,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_rootfs_readonly() {
        let args = Args::parse_from(["greenlight", "--include-checks", "rootfs_readonly"]);
        assert_eq!(args.include_checks, Some(vec![Check::RootfsReadonly]));
    }

    #[test]
    fn test_check_multiple() {
        let args = Args::parse_from([
            "greenlight",
            "--include-checks",
            "rootfs_readonly",
            "--include-checks",
            "sshd_running",
        ]);
        assert_eq!(
            args.include_checks,
            Some(vec![Check::RootfsReadonly, Check::SshdRunning])
        );
    }

    #[test]
    fn test_check_microshift_installed() {
        let args = Args::parse_from(["greenlight", "--include-checks", "microshift_installed"]);
        assert_eq!(args.include_checks, Some(vec![Check::MicroshiftInstalled]));
    }

    #[test]
    fn test_check_sshd_running() {
        let args = Args::parse_from(["greenlight", "--include-checks", "sshd_running"]);
        assert_eq!(args.include_checks, Some(vec![Check::SshdRunning]));
    }

    #[test]
    fn test_check_swap_disabled() {
        let args = Args::parse_from(["greenlight", "--include-checks", "swap_disabled"]);
        assert_eq!(args.include_checks, Some(vec![Check::SwapDisabled]));
    }

    #[test]
    fn test_check_expected_interface_present() {
        let args = Args::parse_from([
            "greenlight",
            "--include-checks",
            "expected_interface_present",
        ]);
        assert_eq!(
            args.include_checks,
            Some(vec![Check::ExpectedInterfacePresent])
        );
    }

    #[test]
    fn test_check_bootc_status_matches_os_release() {
        let args = Args::parse_from([
            "greenlight",
            "--include-checks",
            "bootc_status_matches_os_release",
        ]);
        assert_eq!(
            args.include_checks,
            Some(vec![Check::BootcStatusMatchesOsRelease])
        );
    }
}
