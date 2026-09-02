use crate::config::debounce::MAX_DEBOUNCE_MS;
use crate::report;
use anyhow::{Result, anyhow};
use hidapi::HidDevice;

pub fn get(device: &HidDevice, profile: u8, value_only: bool) -> Result<()> {
    let resp = report::read(device, 0x02, 0x88, 0x00, profile)?;

    let ms = resp[8];

    if ms > MAX_DEBOUNCE_MS {
        return Err(anyhow!(
            "device reported an implausible debounce time ({ms} ms)"
        ));
    }

    if value_only {
        println!("{ms}");
    } else {
        println!("Debounce: {ms} ms (profile {profile})");
    }

    Ok(())
}
