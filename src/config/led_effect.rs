use crate::args::Effect;
use crate::util::status;
use anyhow::{anyhow, Result};
use hidapi::HidDevice;

use super::DEFAULT_PROFILE;

pub fn set(device: &HidDevice, profile: Option<u8>, effect: Effect) -> Result<()> {
    status::check_sleep(device)?;

    let mut bfr = [0u8; 65];

    let profile_id = profile.unwrap_or(DEFAULT_PROFILE);

    bfr[3] = 0x02;
    bfr[5] = 0x02;
    bfr[7] = profile_id;
    bfr[8] = 0xFF;

    match effect {
        Effect::Glorious { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x01;
            bfr[11] = rate_check(rate, 1)?;
        }

        Effect::Cycle { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x02;
            bfr[11] = rate_check(rate, 2)?;
            bfr[12] = 0xFF;
        }

        Effect::Pulse { rate, colors } => {
            bfr[4] = (colors.len() as u8) * 3 + 5;
            bfr[9] = 0x03;
            bfr[11] = rate_check(rate, 3)?;

            for i in 0..6 {
                let offset = 12 + (3 * i);

                if i >= colors.len() {
                    bfr[offset] = 0x00;
                    bfr[offset + 1] = 0x00;
                    bfr[offset + 2] = 0x00;
                } else {
                    bfr[offset] = colors[i].red;
                    bfr[offset + 1] = colors[i].green;
                    bfr[offset + 2] = colors[i].blue;
                }
            }
        }

        Effect::Solid { color } => {
            bfr[4] = 0x08;
            bfr[9] = 0x04;

            bfr[12] = color.red;
            bfr[12 + 1] = color.green;
            bfr[12 + 2] = color.blue;
        }

        Effect::PulseOne { rate, color } => {
            bfr[4] = 0x08;
            bfr[9] = 0x05;
            bfr[11] = rate_check(rate, 5)?;

            bfr[12] = color.red;
            bfr[12 + 1] = color.green;
            bfr[12 + 2] = color.blue;
        }

        Effect::Tail { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x06;
            bfr[11] = rate_check(rate, 6)?;
        }

        Effect::Rave { rate, colors } => {
            bfr[4] = (colors.len() as u8) * 3 + 5;
            bfr[9] = 0x07;
            bfr[11] = rate_check(rate, 7)?;

            for i in 0..2 {
                let offset = 12 + (3 * i);

                if i >= colors.len() {
                    bfr[offset] = 0x00;
                    bfr[offset + 1] = 0x00;
                    bfr[offset + 2] = 0x00;
                } else {
                    bfr[offset] = colors[i].red;
                    bfr[offset + 1] = colors[i].green;
                    bfr[offset + 2] = colors[i].blue;
                }
            }
        }

        Effect::Wave { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x08;
            bfr[11] = rate_check(rate, 8)?;
        }

        Effect::Off => {
            bfr[4] = 0x05;
            bfr[9] = 0x00;
        }
    }

    device.send_feature_report(&bfr)?;

    Ok(())
}

const RATE_DEFAULT: u8 = 40;

fn rate_check(rate: Option<u8>, effect_id: u8) -> Result<u8> {
    let rate_unwrapped = rate.unwrap_or(RATE_DEFAULT);

    let rate_checked = match rate_unwrapped {
        0..=100 => rate_unwrapped,
        _ => return Err(anyhow!("rate must be in the range of 0-100")),
    };

    match effect_id {
        7 | 8 => Ok((105 - rate_checked) * 2),
        _ => Ok((105 - rate_checked) / 5),
    }
}
