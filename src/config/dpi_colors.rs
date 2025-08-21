use crate::util::color::Color;
use crate::util::status;
use anyhow::Result;
use hidapi::HidDevice;

use super::DEFAULT_PROFILE;

pub fn set(device: &HidDevice, profile: Option<u8>, colors: Vec<Color>) -> Result<()> {
    status::check_sleep(device)?;

    let mut bfr = [0u8; 65];

    let profile_id = profile.unwrap_or(DEFAULT_PROFILE);

    bfr[3] = 0x02;
    bfr[4] = 0x13;
    bfr[5] = 0x02;
    bfr[6] = 0x01;

    bfr[7] = profile_id;

    for i in 0..colors.len() {
        bfr[8 + 3 * i] = colors[i].red;
        bfr[8 + 3 * i + 1] = colors[i].green;
        bfr[8 + 3 * i + 2] = colors[i].blue;
    }

    device.send_feature_report(&bfr)?;

    Ok(())
}
