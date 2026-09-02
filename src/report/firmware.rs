use crate::report;
use anyhow::Result;
use hidapi::HidDevice;

pub fn get(device: &HidDevice) -> Result<()> {
    let resp = report::read(device, 0x03, 0x81, 0x00, 0x00)?;

    println!(
        "Firmware version: {}.{}.{}.{}",
        resp[7], resp[8], resp[9], resp[10]
    );

    Ok(())
}
