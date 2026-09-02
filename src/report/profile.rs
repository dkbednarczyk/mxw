use crate::report;
use anyhow::Result;
use hidapi::HidDevice;

pub fn get(device: &HidDevice, value_only: bool) -> Result<()> {
    let resp = report::read(device, 0x01, 0x85, 0x00, 0x00)?;

    if value_only {
        println!("{}", resp[7]);
    } else {
        println!("Active profile: {}", resp[7]);
    }

    Ok(())
}
