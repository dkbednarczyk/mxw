use anyhow::Result;
use hidapi::HidDevice;

/// Maximum lift-off distance in mm; the device only offers 1 mm and 2 mm.
pub const MAX_LIFT_OFF_MM: u8 = 2;

pub fn set(device: &HidDevice, mm: u8) -> Result<()> {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x01;
    bfr[5] = 0x01;
    bfr[6] = 0x07;

    // The wire value is 0-based: 1 mm is encoded as 0x00, 2 mm as 0x01.
    bfr[7] = mm - 1;

    device.send_feature_report(&bfr)?;

    Ok(())
}
