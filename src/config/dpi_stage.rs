use crate::report;
use anyhow::{anyhow, Result};
use hidapi::HidDevice;

pub fn set(device: &HidDevice, profile: u8, id: u8) -> Result<()> {
    let count = report::dpi::count(device, profile)?;

    if id > count {
        return Err(anyhow!(
            "active DPI stage {id} is out of range: profile {profile} has {count} stage(s)"
        ));
    }

    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x02;
    bfr[5] = 0x01;
    bfr[6] = 0x02;

    bfr[7] = profile;
    bfr[8] = id;

    device.send_feature_report(&bfr)?;

    Ok(())
}
