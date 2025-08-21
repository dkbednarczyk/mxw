use anyhow::Result;
use hidapi::HidDevice;

use super::DEFAULT_PROFILE;

pub fn set(device: &HidDevice, profile: Option<u8>, stages: Vec<u16>) -> Result<()> {
    let mut bfr = [0u8; 65];

    let profile_id = profile.unwrap_or(DEFAULT_PROFILE);

    bfr[3] = 0x02;
    bfr[4] = 0x12;
    bfr[5] = 0x01;
    bfr[6] = 0x01;

    bfr[7] = profile_id;
    bfr[8] = stages.len() as u8;

    for i in 0..stages.len() {
        let [first, second] = stages[i].to_be_bytes();

        bfr[9 + (4 * i)] = first;
        bfr[9 + (4 * i) + 1] = second;
        bfr[9 + (4 * i) + 2] = first;
        bfr[9 + (4 * i) + 3] = second;
    }

    device.send_feature_report(&bfr)?;

    Ok(())
}
