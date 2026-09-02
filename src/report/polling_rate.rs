use crate::report;
use anyhow::{Result, anyhow};
use hidapi::HidDevice;

pub fn get(device: &HidDevice, ms_only: bool, hz_only: bool) -> Result<()> {
    let resp = report::read(device, 0x01, 0x80, 0x01, 0x00)?;

    let ms = resp[7];

    if ms == 0 {
        return Err(anyhow!(
            "device reported an implausible polling rate ({ms})"
        ));
    }

    if ms_only {
        println!("{ms}");
        return Ok(());
    }

    let hz = 1000 / u16::from(ms);

    if hz_only {
        println!("{hz}");
        return Ok(());
    }

    println!("Polling rate: {ms} ms ({hz} Hz)");

    Ok(())
}
