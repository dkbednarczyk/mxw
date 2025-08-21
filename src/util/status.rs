use anyhow::{anyhow, Result};
use hidapi::HidDevice;
use std::{thread, time::Duration};

pub fn get_buffer(device: &HidDevice) -> Result<[u8; 65]> {
    let mut to_send = [0u8; 65];

    to_send[3] = 0x02;
    to_send[4] = 0x02;
    to_send[6] = 0x83;

    device.send_feature_report(&to_send)?;

    thread::sleep(Duration::from_millis(50));

    let mut resp = [0u8; 65];

    device.get_feature_report(&mut resp)?;

    Ok(resp)
}

pub fn get(device: &HidDevice) -> Result<usize> {
    let mut resp = get_buffer(device)?;

    device.get_feature_report(&mut resp)?;

    let status = [0xA1, 0xA4, 0xA2, 0xA0, 0xA3]
        .iter()
        .position(|&s| s == resp[1])
        .ok_or_else(|| anyhow!("failed to get status"))?;

    if resp[6] != 0x83 {
        return Ok(2);
    }

    Ok(status)
}

pub fn check_sleep(device: &HidDevice) -> Result<()> {
    if get(device)? == 1 {
        return Err(anyhow!("device is sleeping"));
    }

    Ok(())
}
