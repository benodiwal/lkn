use tun_tap::{Iface, Mode};
use crate::error;

pub fn create_nic() -> Result<Iface, error::Error> {
    let nic = Iface::new("lkn0", Mode::Tun)?;
    Ok(nic)
}
