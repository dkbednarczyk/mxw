use anyhow::Result;
use hidapi::HidDevice;

pub fn set(device: &HidDevice, profile: u8, stages: Vec<u16>) -> Result<()> {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x12;
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
