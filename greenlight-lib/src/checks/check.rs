use crate::errors::GreenlightError;
use serde::Deserialize;

use crate::checks::network::Interface;
use crate::checks::rootfs::is_rootfs_readonly;
use crate::checks::unit::{wait_for_unit, ActiveState};
use std::fs::read_to_string;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Check {
    RootfsReadonly,
    BootcStatusMatchesOsRelease,
    MicroshiftInstalled,
    Interfaces {
        interfaces: Vec<Interface>,
    },
    SwapDisabled,
    UnitState {
        unit: String,
        expected: ActiveState,
        #[serde(default)]
        timeout: Option<u64>, // in seconds
    },
}
impl Check {
    pub async fn run(&self) -> Result<bool, GreenlightError> {
        match self {
            Check::UnitState {
                unit: service,
                expected,
                timeout,
            } => {
                let result = wait_for_unit(service, expected.clone(), *timeout).await;
                match result {
                    Ok(true) => Ok(true),
                    Ok(false) => {
                        tracing::error!(
                            "Unit '{}' did not reach expected state '{:?}' within {:?} seconds",
                            service,
                            expected,
                            timeout.unwrap_or(0)
                        );
                        Ok(false)
                    }
                    Err(e) => {
                        tracing::error!("Failed to check unit state for '{}': {}", service, e);
                        Err(e)
                    }
                }
            }

            Check::RootfsReadonly => tokio::task::spawn_blocking(is_rootfs_readonly).await?,

            Check::SwapDisabled => tokio::task::spawn_blocking(is_swap_off).await?,

            _ => Err(GreenlightError::UnsupportedDeployment),
        }
    }
}

pub fn is_swap_off() -> Result<bool, GreenlightError> {
    let content = read_to_string("/proc/swaps")?;
    let mut lines = content.lines();
    lines.next();
    Ok(lines.next().is_none())
}
