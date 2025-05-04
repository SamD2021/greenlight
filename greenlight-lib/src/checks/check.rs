use crate::{
    config::{System, Target},
    errors::GreenlightError,
};
use clap::ValueEnum;
use serde::Deserialize;
use std::str::FromStr;

use crate::checks::rootfs::is_rootfs_readonly;

use super::services::is_sshd_running;

/// These checks are mapped as Enums so we can design the checks as fully valid states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")] // Ensures Clap matches Serde strings
pub enum Check {
    RootfsReadonly,
    BootcStatusMatchesOsRelease,
    MicroshiftInstalled,
    ExpectedInterfacePresent,
    SwapDisabled,
    SshdRunning,
}

impl Check {
    pub fn applies_to(&self, deployment: &System, target: &Target) -> bool {
        use Check::*;

        match self {
            RootfsReadonly => !matches!(deployment, System::Traditional { .. }), // Deployment level check, but not really useful in traditional deployments

            BootcStatusMatchesOsRelease => matches!(deployment, System::Bootc { .. }),

            MicroshiftInstalled | ExpectedInterfacePresent | SwapDisabled | SshdRunning => {
                matches!(target, Target::DPU)
            }
        }
    }
    pub fn run(&self) -> Result<bool, GreenlightError> {
        match self {
            Check::RootfsReadonly => Ok(is_rootfs_readonly()?),
            Check::SshdRunning => Ok(is_sshd_running()?),
            _ => Err(GreenlightError::UnsupportedDeployment),
        }
    }
}

impl FromStr for Check {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rootfs_readonly" => Ok(Check::RootfsReadonly),
            "bootc_status_matches_os_release" => Ok(Check::BootcStatusMatchesOsRelease),
            "microshift_installed" => Ok(Check::MicroshiftInstalled),
            "expected_interface_present" => Ok(Check::ExpectedInterfacePresent),
            "swap_disabled" => Ok(Check::SwapDisabled),
            "sshd_running" => Ok(Check::SshdRunning),
            _ => Err(format!("Unknown check kind: {}", s)),
        }
    }
}
