use crate::errors::GreenlightError;
use systemd_zbus::{ActiveState, ManagerProxyBlocking, UnitProxyBlocking};
use zbus::blocking::Connection;

fn get_unit(service_name: &str) -> Result<UnitProxyBlocking, GreenlightError> {
    let connection = Connection::system()?;
    let proxy = ManagerProxyBlocking::new(&connection)?;
    let unit_path = proxy.get_unit(service_name)?;
    let unit = UnitProxyBlocking::builder(&connection)
        .path(unit_path)?
        .build()?;
    Ok(unit)
}
pub fn is_service_running(service_name: &str) -> Result<bool, GreenlightError> {
    let unit = get_unit(service_name)?;
    let active_state = unit.active_state()?;
    Ok(active_state == ActiveState::Active)
}

pub fn is_sshd_running() -> Result<bool, GreenlightError> {
    let running = is_service_running("sshd.service")?;
    Ok(running)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_is_sshd_running() {
        assert!(is_sshd_running().unwrap());
    }
}
