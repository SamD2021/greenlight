use crate::{
    config::{System, Target},
    errors::GreenlightError,
};
use serde::Deserialize;
use systemd_zbus::ActiveState;

use crate::checks::rootfs::is_rootfs_readonly;
use crate::checks::unit::get_unit_state;
use std::fs::read_to_string;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Check {
    RootfsReadonly,
    BootcStatusMatchesOsRelease,
    MicroshiftInstalled,
    ExpectedInterfacePresent,
    SwapDisabled,
    UnitState {
        unit: String,
        expected: ExpectedActiveState,
    },
}
impl Check {
    pub fn applies_to(&self, deployment: &System, target: &Target) -> bool {
        use Check::*;
        match self {
            RootfsReadonly => !matches!(deployment, System::Traditional { .. }),
            BootcStatusMatchesOsRelease => matches!(deployment, System::Bootc { .. }),
            MicroshiftInstalled | ExpectedInterfacePresent | SwapDisabled | UnitState { .. } => {
                matches!(target, Target::DPU)
            }
        }
    }

    pub fn run(&self) -> Result<bool, GreenlightError> {
        match self {
            Check::RootfsReadonly => Ok(is_rootfs_readonly()?),

            Check::UnitState {
                unit: service,
                expected,
            } => {
                let actual = get_unit_state(service)?; // returns ActiveState
                Ok(actual == expected.clone().into())
            }
            Check::SwapDisabled => Ok(is_swap_off()?),

            _ => Err(GreenlightError::UnsupportedDeployment),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ExpectedActiveState {
    Active,
    Inactive,
    Failed,
    Activating,
    Deactivating,
}

impl From<ExpectedActiveState> for ActiveState {
    fn from(e: ExpectedActiveState) -> Self {
        match e {
            ExpectedActiveState::Active => ActiveState::Active,
            ExpectedActiveState::Inactive => ActiveState::Inactive,
            ExpectedActiveState::Failed => ActiveState::Failed,
            ExpectedActiveState::Activating => ActiveState::Activating,
            ExpectedActiveState::Deactivating => ActiveState::Deactivating,
        }
    }
}

pub fn is_swap_off() -> Result<bool, GreenlightError> {
    let content = read_to_string("/proc/swaps")?;
    let mut lines = content.lines();
    lines.next();
    Ok(lines.next().is_none())
}
