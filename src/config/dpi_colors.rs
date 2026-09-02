use crate::report;
use crate::util::color::Color;
use anyhow::{Result, anyhow};
use hidapi::HidDevice;

pub fn set(device: &HidDevice, profile: u8, colors: Vec<Color>) -> Result<()> {
    let count = report::dpi::count(device, profile)? as usize;

    if colors.len() != count {
        return Err(anyhow!(
            "profile {profile} has {count} DPI stage(s); provide exactly {count} color(s), got {}",
            colors.len()
        ));
    }

    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    // Payload length: 1 byte of profile, then 3 bytes (RGB) per stage color
    bfr[4] = 1 + (colors.len() * 3) as u8;
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
