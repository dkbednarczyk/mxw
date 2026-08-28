use crate::report;
use anyhow::Result;
use hidapi::HidDevice;

pub fn get(device: &HidDevice) -> Result<()> {
    let resp = report::read(device, 0x01, 0x85, 0x00, 0x00)?;

    println!("{}", resp[7]);

    Ok(())
}
