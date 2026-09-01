use crate::util::IO_DELAY;
use anyhow::Result;
use hidapi::HidDevice;
use std::thread;

pub fn set(device: &HidDevice, wired: u8, wireless: u8) -> Result<()> {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x02;
    bfr[5] = 0x02;
    bfr[6] = 0x02;
    bfr[7] = 0x01;

    bfr[8] = wired;

    device.send_feature_report(&bfr)?;

    thread::sleep(IO_DELAY);

    bfr[7] = 0x00;
    bfr[8] = wireless;

    device.send_feature_report(&bfr)?;

    Ok(())
}
