use crate::args::{Binding, Button, DPIFn, KeyKind, KeyboardFn, MediaFn, MouseFn};
use anyhow::{anyhow, Result};
use colored::Colorize;
use hidapi::HidDevice;
use std::{thread, time::Duration};

pub fn set(device: &HidDevice, profile: u8, button: Button, binding: Binding) -> Result<()> {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x09;
    bfr[5] = 0x03;
    bfr[7] = profile;
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
        Binding::Key { kind } => set_key(&mut bfr[10..], kind),
        Binding::Mouse(mouse_fn) => set_mouse(&mut bfr[10..], mouse_fn),
        Binding::Keyboard(keyboard_fn) => set_keyboard(&mut bfr[10..], keyboard_fn),
        Binding::Media(media_fn) => set_media(&mut bfr[10..], media_fn),
        Binding::DPI(dpi_fn) => set_dpi(&mut bfr[10..], dpi_fn),
        Binding::None => (),
    }

    device.send_feature_report(&bfr)?;
    set_and_check(device, &bfr)
}

fn set_and_check(device: &HidDevice, bfr: &[u8]) -> Result<()> {
    let mut waiting = false;

    for _ in 0..3 {
        thread::sleep(Duration::from_millis(50));

        if waiting {
            continue;
        }

        let mut read = [0u8; 65];
        device.get_feature_report(&mut read)?;
        thread::sleep(Duration::from_millis(50));

        match read[0] {
            0xA2 => device.send_feature_report(bfr)?,
            0xA0 => (),
            0xA4 => waiting = true,
            _ => return Ok(()),
        }
    }

    eprintln!("{}: failed to bind key", "error".bold().red());

    Err(anyhow!(
        "feature report did not return new bind after 3 retries"
    ))
}

fn set_key(bfr: &mut [u8], kind: KeyKind) {
    bfr[0] = 0x04;
    bfr[1] = 0x02;

    let (key, modifier) = match kind {
        KeyKind::ScanCode { key, modifier }
        | KeyKind::Code { key, modifier }
        | KeyKind::KeyCode { key, modifier } => (key, modifier),
    };

    if let Some(value) = modifier {
        bfr[2] = value.modifier.unwrap_or(0);
    }

    if let Some(value) = key.modifier {
        bfr[2] |= value;
    } else {
        bfr[3] = key.scan_code;
    }
}

const fn set_mouse(bfr: &mut [u8], mouse_fn: MouseFn) {
    let id = match mouse_fn {
        MouseFn::Left => 1,
        MouseFn::Scroll => 3,
        MouseFn::Right => 2,
        MouseFn::Forward => 5,
        MouseFn::Back => 4,
        MouseFn::ScrollUp => 16,
        MouseFn::ScrollDown => 17,
        MouseFn::ProfileCycleUp => 24,
        MouseFn::ProfileCycleDown => 25,
        MouseFn::BatteryStatus => 12,
    };

    bfr[0] = 0x01;
    bfr[1] = 0x01;
    bfr[2] = id;

    if id == 12 {
        bfr[0] = 12;
        bfr[2] = 1;
    } else if id == 24 {
        bfr[0] = 8;
        bfr[2] = 4;
    } else if id == 25 {
        bfr[0] = 8;
        bfr[2] = 3;
    }
}

const fn set_keyboard(bfr: &mut [u8], keyboard_fn: KeyboardFn) {
    bfr[0] = 0x05;
    bfr[1] = 0x02;
    bfr[2] = match keyboard_fn {
        KeyboardFn::ProfileCycleUp => 1,
        KeyboardFn::ProfileCycleDown => 2,
        KeyboardFn::LayerCycleUp => 3,
        KeyboardFn::LayerCycleDown => 4,
    };
    bfr[3] = 0x0F;
}

const fn set_dpi(bfr: &mut [u8], dpi_fn: DPIFn) {
    bfr[0] = 0x07;
    bfr[1] = 0x01;
    bfr[2] = match dpi_fn {
        DPIFn::StageUp => 1,
        DPIFn::StageDown => 2,
        DPIFn::CycleUp => 6,
        DPIFn::CycleDown => 7,
    };
}

const fn set_media(bfr: &mut [u8], media_fn: MediaFn) {
    let (b1, b2) = match media_fn {
        MediaFn::Player => (1, 131),
        MediaFn::PlayPause => (0, 205),
        MediaFn::Next => (0, 181),
        MediaFn::Previous => (0, 182),
        MediaFn::Stop => (0, 183),
        MediaFn::Mute => (0, 226),
        MediaFn::VolumeUp => (0, 233),
        MediaFn::VolumeDown => (0, 234),
    };

    bfr[0] = 0x05;
    bfr[1] = 0x02;
    bfr[2] = b1;
    bfr[3] = b2;
}
