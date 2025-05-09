use crate::errors::GreenlightError;
use serde::Deserialize;
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq, Deserialize, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ActiveState {
    Active,
    Inactive,
    Failed,
    Activating,
    Deactivating,
    Unknown,
}

pub async fn get_unit_state(unit_name: &str) -> Result<ActiveState, GreenlightError> {
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg(unit_name)
        .output()
        .await
        .map_err(GreenlightError::Io)?;

    // systemctl returns non-zero for inactive, but that's not an error here
    let stdout = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_lowercase();

    let state = match stdout.as_str() {
        "active" => ActiveState::Active,
        "inactive" => ActiveState::Inactive,
        "failed" => ActiveState::Failed,
        "activating" => ActiveState::Activating,
        "deactivating" => ActiveState::Deactivating,
        "unknown" | "" => ActiveState::Unknown,
        _ => {
            tracing::warn!(
                "Unexpected state '{}' returned by systemctl for unit '{}'",
                stdout,
                unit_name
            );
            ActiveState::Unknown
        }
    };

    Ok(state)
}
