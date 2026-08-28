use crate::config::{dpi_stage, MAX_DPI_STAGES};
use anyhow::{anyhow, Result};
use hidapi::HidDevice;

pub fn set(device: &HidDevice, profile: u8, stages: Vec<u16>) -> Result<()> {
    if stages.is_empty() || stages.len() > MAX_DPI_STAGES as usize {
        return Err(anyhow!(
            "must provide between 1 and {MAX_DPI_STAGES} DPI stages"
        ));
    }

    // Set active stage to the first one before writing stages
    // This follows how Glorious Core software works (at least from my testing)
    dpi_stage::set(device, profile, 1)?;

    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    // Payload length: 2 bytes of profile + count, then 4 bytes per stage
    bfr[4] = 2 + (stages.len() * 4) as u8;
    bfr[5] = 0x01;
    bfr[6] = 0x01;

    bfr[7] = profile;
    bfr[8] = stages.len() as u8;

    for (i, stage) in stages.iter().enumerate() {
        let [first, second] = stage.to_be_bytes();
        let offset = 9 + (4 * i);

        bfr[offset] = first;
        bfr[offset + 1] = second;
        bfr[offset + 2] = first;
        bfr[offset + 3] = second;
    }

    device.send_feature_report(&bfr)?;

    Ok(())
}
