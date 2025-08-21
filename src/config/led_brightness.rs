use crate::util::status;
use anyhow::Result;
use hidapi::HidDevice;
use std::{thread, time::Duration};

pub fn set(device: &HidDevice, wired: u8, wireless: Option<u8>) -> Result<()> {
    status::check_sleep(device)?;

    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x02;
    bfr[5] = 0x02;
    bfr[6] = 0x02;
    bfr[7] = 0x01;

    bfr[8] = wired;

    device.send_feature_report(&bfr)?;

    thread::sleep(Duration::from_millis(30));

    bfr[7] = 0x00;
    bfr[8] = wireless.unwrap_or(wired);

    device.send_feature_report(&bfr)?;

    Ok(())
}
