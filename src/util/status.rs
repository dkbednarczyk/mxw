use anyhow::{Result, anyhow};
use hidapi::HidDevice;
use std::thread;

use crate::util::IO_DELAY;

pub fn get_buffer(device: &HidDevice) -> Result<[u8; 65]> {
    let mut to_send = [0u8; 65];

    to_send[3] = 0x02;
    to_send[4] = 0x02;
    to_send[6] = 0x83;

    device.send_feature_report(&to_send)?;

    thread::sleep(IO_DELAY);

    let mut resp = [0u8; 65];

    device.get_feature_report(&mut resp)?;

    Ok(resp)
}

/// Device state from the `resp[1]` field of a `0x83` status report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    Awake,    // 0xA1
    Asleep,   // 0xA4
    WakingUp, // 0xA0
    Unknown(u8),
    /// The response was not a status report (`resp[6] != 0x83`),
    /// e.g. a stale one.
    Invalid,
}

impl DeviceStatus {
    const fn from_report(resp: &[u8; 65]) -> Self {
        if resp[6] != 0x83 {
            return Self::Invalid;
        }

        match resp[1] {
            0xA1 => Self::Awake,
            0xA4 => Self::Asleep,
            0xA0 => Self::WakingUp,
            other => Self::Unknown(other),
        }
    }
}

pub fn get(device: &HidDevice) -> Result<DeviceStatus> {
    let mut resp = get_buffer(device)?;

    device.get_feature_report(&mut resp)?;

    Ok(DeviceStatus::from_report(&resp))
}

pub fn check_sleep(device: &HidDevice) -> Result<()> {
    if get(device)? == DeviceStatus::Asleep {
        return Err(anyhow!("device is sleeping"));
    }

    Ok(())
}
