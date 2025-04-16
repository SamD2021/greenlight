use crate::config::{System, Target};
use serde::Deserialize;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckKind {
    #[serde(rename = "rootfs_readonly")]
    RootFsReadonly,
    BootcStatusMatchesOsRelease,
    #[serde(rename = "microshift_installed")]
    MicroShiftInstalled,
    ExpectedInterfacePresent,
    SwapDisabled,
    SshdRunning,
}

impl CheckKind {
    pub fn applies_to(&self, deployment: &System, target: &Target) -> bool {
        use CheckKind::*;

        match self {
            RootFsReadonly => true, // universally useful

            BootcStatusMatchesOsRelease => matches!(deployment, System::Bootc { .. }),

            MicroShiftInstalled | ExpectedInterfacePresent | SwapDisabled | SshdRunning => {
                matches!(target, Target::DPU)
            }
        }
    }
}
