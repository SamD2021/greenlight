use crate::config::{System, Target};
use serde::Deserialize;

/// These checks are mapped as Enums so we can design the checks as fully valid states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Check {
    #[serde(rename = "rootfs_readonly")]
    RootFsReadonly,
    BootcStatusMatchesOsRelease,
    #[serde(rename = "microshift_installed")]
    MicroShiftInstalled,
    ExpectedInterfacePresent,
    SwapDisabled,
    SshdRunning,
}

impl Check {
    pub fn applies_to(&self, deployment: &System, target: &Target) -> bool {
        use Check::*;

        match self {
            RootFsReadonly => !matches!(deployment, System::Traditional { .. }), // Deployment level check, but not really useful in traditional deployments

            BootcStatusMatchesOsRelease => matches!(deployment, System::Bootc { .. }),

            MicroShiftInstalled | ExpectedInterfacePresent | SwapDisabled | SshdRunning => {
                matches!(target, Target::DPU)
            }
        }
    }
}
