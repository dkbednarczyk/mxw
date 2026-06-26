use crate::util::color::Color;
use anyhow::Result;
use hidapi::HidDevice;

pub fn set(device: &HidDevice, profile: u8, colors: Vec<Color>) -> Result<()> {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x13;
    bfr[5] = 0x02;
    bfr[6] = 0x01;

    bfr[7] = profile;

    for (i, color) in colors.iter().enumerate() {
        let offset = 8 + (3 * i);

        bfr[offset] = color.red;
        bfr[offset + 1] = color.green;
        bfr[offset + 2] = color.blue;
    }

    device.send_feature_report(&bfr)?;

    Ok(())
}
