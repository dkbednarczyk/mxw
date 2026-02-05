use crate::util::color::Color;
use anyhow::Result;
use hidapi::HidDevice;

use super::DEFAULT_PROFILE;

pub fn set(device: &HidDevice, profile: Option<u8>, colors: Vec<Color>) -> Result<()> {
    let mut bfr = [0u8; 65];

    let profile_id = profile.unwrap_or(DEFAULT_PROFILE);

    bfr[3] = 0x02;
    bfr[4] = 0x13;
    bfr[5] = 0x02;
    bfr[6] = 0x01;

    bfr[7] = profile_id;

    for (i, color) in colors.iter().enumerate() {
        let offset = 8 + (3 * i);

        bfr[offset] = color.red;
        bfr[offset + 1] = color.green;
        bfr[offset + 2] = color.blue;
    }

    device.send_feature_report(&bfr)?;

    Ok(())
}
