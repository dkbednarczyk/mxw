use super::DEFAULT_PROFILE;
use anyhow::Result;
use hidapi::HidDevice;

pub fn set(device: &HidDevice, profile: Option<u8>, ms: u8) -> Result<()> {
    let mut bfr = [0u8; 65];

    let profile_id = profile.unwrap_or(DEFAULT_PROFILE);

    bfr[3] = 0x02;
    bfr[4] = 0x01;
    bfr[6] = 0x08;

    bfr[7] = profile_id;
    bfr[8] = ms;

    device.send_feature_report(&bfr)?;

    Ok(())
}
