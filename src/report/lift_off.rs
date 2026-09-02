use crate::report;
use anyhow::{Result, anyhow};
use hidapi::HidDevice;

pub fn get(device: &HidDevice, value_only: bool) -> Result<()> {
    let resp = report::read(device, 0x01, 0x87, 0x01, 0x00)?;

    if resp[7] > 1 {
        return Err(anyhow!(
            "device reported an implausible lift-off distance (wire value {})",
            resp[7]
        ));
    }

    // Wire value is 0-based: 0x00 is 1 mm, 0x01 is 2 mm.
    let mm = resp[7] + 1;

    if value_only {
        println!("{mm}");
    } else {
        println!("Lift-off distance: {mm} mm");
    }

    Ok(())
}
