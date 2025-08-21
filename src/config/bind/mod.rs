pub mod dpi;
pub mod key;
pub mod keyboard;
pub mod media;
pub mod mouse;

use crate::args::{Binding, Button};
use anyhow::{anyhow, Result};
use colored::Colorize;
use hidapi::HidDevice;
use std::{thread, time::Duration};

use super::DEFAULT_PROFILE;

pub fn set(
    device: &HidDevice,
    profile: Option<u8>,
    button: Button,
    binding: Binding,
) -> Result<()> {
    let mut bfr = [0u8; 65];
    let profile_id = profile.unwrap_or(DEFAULT_PROFILE);

    bfr[3] = 0x02;
    bfr[4] = 0x09;
    bfr[5] = 0x03;
    bfr[7] = profile_id;
    bfr[8] = match button {
        Button::Left => 1,
        Button::Scroll => 3,
        Button::Right => 2,
        Button::Forward => 5,
        Button::Back => 4,
        Button::DPIBtn => 20,
        Button::ScrollUp => 16,
        Button::ScrollDown => 17,
    };

    match binding {
        Binding::Key { kind } => key::set(&mut bfr[10..], kind),
        Binding::Mouse(mouse_fn) => mouse::set(&mut bfr[10..], mouse_fn),
        Binding::Keyboard(keyboard_fn) => keyboard::set(&mut bfr[10..], keyboard_fn),
        Binding::Media(media_fn) => media::set(&mut bfr[10..], media_fn),
        Binding::DPI(dpi_fn) => dpi::set(&mut bfr[10..], dpi_fn),
        Binding::None => (),

        _ => unimplemented!(),
    }

    device.send_feature_report(&bfr)?;
    set_and_check(device, &mut bfr, 0, false)
}

pub fn set_and_check(device: &HidDevice, _bfr: &mut [u8], depth: u8, waiting: bool) -> Result<()> {
    if depth >= 3 {
        println!("{}: failed to bind key", "error".bold().red());
        return Err(anyhow!(
            "feature report did not return new bind after 3 retries"
        ));
    }

    thread::sleep(Duration::from_millis(100));

    if waiting {
        set_and_check(device, _bfr, depth + 1, true)
    } else {
        let mut bfr = [0u8; 55];
        device.get_feature_report(&mut bfr)?;
        thread::sleep(Duration::from_millis(40));

        match bfr[0] {
            0xA2 => {
                device.send_feature_report(_bfr)?;
                set_and_check(device, _bfr, depth + 1, false)
            }
            0xA0 => set_and_check(device, _bfr, depth + 1, false),
            0xA4 => set_and_check(device, _bfr, depth + 1, true),

            _ => Ok(()),
        }
    }
}
