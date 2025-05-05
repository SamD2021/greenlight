use crate::errors::GreenlightError;
use systemd_zbus::{ActiveState, ManagerProxyBlocking, UnitProxyBlocking};
use zbus::blocking::Connection;

fn get_unit(unit_name: &str) -> Result<UnitProxyBlocking, GreenlightError> {
    let connection = Connection::system()?;
    let proxy = ManagerProxyBlocking::new(&connection)?;
    let unit_path = proxy.get_unit(unit_name)?;
    let unit = UnitProxyBlocking::builder(&connection)
        .path(unit_path)?
        .build()?;
    Ok(unit)
}
pub fn get_unit_state(unit_name: &str) -> Result<ActiveState, GreenlightError> {
    let unit = get_unit(unit_name)?;
    let active_state = unit.active_state()?;
    Ok(active_state)
}
