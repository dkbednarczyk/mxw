pub mod battery;
pub mod dpi;
pub mod firmware;
pub mod profile;

use anyhow::{Result, anyhow};
use hidapi::HidDevice;
use std::thread;

use crate::util::IO_DELAY;

/// Number of times to poll for a response before giving up.
const MAX_READ_ATTEMPTS: usize = 5;

pub fn read(device: &HidDevice, length: u8, command: u8, sub: u8, arg: u8) -> Result<[u8; 65]> {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = length;
    bfr[5] = sub;
    bfr[6] = command;
    bfr[7] = arg;

    device.send_feature_report(&bfr)?;

    let mut resp = [0u8; 65];
    for _ in 0..MAX_READ_ATTEMPTS {
        thread::sleep(IO_DELAY);

        device.get_feature_report(&mut resp)?;

        if resp[6] == command {
            return Ok(resp);
        }
    }

    Err(anyhow!("no response to read command {command:#04X}"))
}
