use crate::util::status;
use anyhow::Result;
use hidapi::HidDevice;

use super::DEFAULT_PROFILE;

pub fn set(device: &HidDevice, profile: Option<u8>, id: u8) -> Result<()> {
    status::check_sleep(device)?;

    let mut bfr = [0u8; 65];

    let profile_id = profile.unwrap_or(DEFAULT_PROFILE);

    bfr[3] = 0x02;
    bfr[4] = 0x02;
    bfr[5] = 0x01;
    bfr[6] = 0x02;

    bfr[7] = profile_id;
    bfr[8] = id;

    device.send_feature_report(&bfr)?;

    Ok(())
}
