use crate::report;
use anyhow::{Result, anyhow};
use hidapi::HidDevice;

pub fn set(device: &HidDevice, profile: u8, id: u8) -> Result<()> {
    let count = report::dpi::count(device, profile)?;

    if id > count {
        return Err(anyhow!(
            "active DPI stage {id} is out of range: profile {profile} has {count} stage(s)"
        ));
    }

    write(device, profile, id)
}

/// Write the active-stage packet without checking `id` against the device's
/// current stage count. Callers that already know `id` is in range (e.g.
/// `dpi_stages::set` resetting to stage 1) use this to skip the extra read.
pub(crate) fn write(device: &HidDevice, profile: u8, id: u8) -> Result<()> {
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
