use crate::errors::GreenlightError;
use futures::TryStreamExt;
use rtnetlink::{new_connection, packet_route::link::LinkAttribute};
use serde::Deserialize;
use tracing::{debug, error, info};

#[derive(Debug, Deserialize, Clone, PartialEq, Hash, Eq)]
#[serde(rename_all = "lowercase")]
pub enum InterfaceState {
    Up,
    Down,
    Absent,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Hash, Eq)]
pub struct Interface {
    pub name: String,
    #[serde(default)]
    pub state: Option<InterfaceState>,
    #[serde(default)]
    pub mtu: Option<u32>,
    #[serde(default)]
    pub promisc: Option<bool>,
}

impl Interface {
    pub async fn validate(&self) -> Result<bool, GreenlightError> {
        debug!("Validating interface: {:?}", self);

        let (conn, handle, _) = new_connection()?;
        tokio::spawn(conn);
        let mut links = handle.link().get().match_name(self.name.clone()).execute();

        if let Some(link) = links.try_next().await? {
            debug!("Interface '{}' found", self.name);

            if let Some(expected_state) = &self.state {
                if !self.match_oper_state(expected_state, &link.attributes) {
                    error!(
                        "❌ Interface '{}' state mismatch: expected {:?}, found different state",
                        self.name, expected_state
                    );
                    return Ok(false);
                } else {
                    debug!(
                        "✅ Interface '{}' state matches expected: {:?}",
                        self.name, expected_state
                    );
                }
            }

            if let Some(expected_mtu) = &self.mtu {
                let actual = link.attributes.iter().find_map(|attr| match attr {
                    LinkAttribute::Mtu(val) => Some(*val),
                    _ => None,
                });

                match actual {
                    Some(actual_mtu) if actual_mtu == *expected_mtu => {
                        debug!(
                            "✅ MTU matches for '{}': expected {}, got {}",
                            self.name, expected_mtu, actual_mtu
                        );
                    }
                    Some(actual_mtu) => {
                        error!(
                            "❌ MTU mismatch for '{}': expected {}, got {}",
                            self.name, expected_mtu, actual_mtu
                        );
                        return Ok(false);
                    }
                    None => {
                        error!("❌ MTU attribute not found for '{}'", self.name);
                        return Ok(false);
                    }
                }
            }

            if let Some(expected_promisc) = &self.promisc {
                let actual_promisc = link.attributes.iter().find_map(|attr| match attr {
                    LinkAttribute::Promiscuity(val) => Some(*val > 0),
                    _ => None,
                });
                match actual_promisc {
                    Some(is_promisc) if is_promisc == *expected_promisc => {
                        debug!(
                            "✅ Promiscuity matches for '{}': expected {}, got {}",
                            self.name, expected_promisc, is_promisc
                        );
                    }
                    Some(is_promisc) => {
                        error!(
                            "❌ Promiscuity mismatch for '{}': expected {}, got {}",
                            self.name, expected_promisc, is_promisc
                        );
                        return Ok(false);
                    }
                    None => {
                        error!("❌ Promiscuity attribute not found for '{}'", self.name);
                        return Ok(false);
                    }
                }
            }

            info!("✅ Interface '{}' passed all checks", self.name);
            Ok(true)
        } else {
            debug!("Interface '{}' not found", self.name);
            if self.state == Some(InterfaceState::Absent) {
                info!("✅ Interface '{}' is correctly absent", self.name);
                Ok(true)
            } else {
                error!(
                    "❌ Interface '{}' not found, but expected state was {:?}",
                    self.name, self.state
                );
                Ok(false)
            }
        }
    }

    fn match_oper_state(&self, expected: &InterfaceState, attrs: &[LinkAttribute]) -> bool {
        use rtnetlink::packet_route::link::State::*;
        use LinkAttribute::OperState;

        let actual = attrs.iter().find_map(|attr| match attr {
            OperState(state) => Some(state),
            _ => None,
        });

        match (expected, actual) {
            (InterfaceState::Up, Some(Up)) => true,
            (InterfaceState::Down, Some(Down | Dormant | LowerLayerDown)) => true,
            (InterfaceState::Absent, _) => false, // present when it shouldn't be
            _ => false,
        }
    }
}
