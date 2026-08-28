use crate::config::{dpi_stage, MAX_DPI_STAGES};
use crate::report;
use anyhow::{anyhow, Result};
use hidapi::HidDevice;

pub fn set(device: &HidDevice, profile: u8, stages: Vec<u16>, uniform: Option<u16>) -> Result<()> {
    let stages = match uniform {
        // Fill every stage the profile currently has with the same value.
        // Reading the count back is unavoidable here: "all stages" is defined
        // by the device's current configuration, not by anything on the CLI.
        Some(dpi) => vec![dpi; report::dpi::count(device, profile)? as usize],
        None => stages,
    };

    if stages.is_empty() || stages.len() > MAX_DPI_STAGES as usize {
        return Err(anyhow!(
            "must provide between 1 and {MAX_DPI_STAGES} DPI stages"
        ));
    }

    // Reset the active stage to the first one before rewriting the stage list.
    // Mirrors the Glorious Core sequence; shrinking the list while a higher
    // stage is active can leave the mouse unresponsive until it is replugged.
    // Use the raw write: stage 1 is always valid, so there is no need to read
    // the current stage count back from the device first.
    dpi_stage::write(device, profile, 1)?;

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
