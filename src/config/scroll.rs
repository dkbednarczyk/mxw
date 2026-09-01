use super::bind;
use crate::args::{Binding, Button, MouseFn, ScrollDirection};
use crate::config::MAX_PROFILES;
use anyhow::Result;
use hidapi::HidDevice;

pub fn set(device: &HidDevice, direction: ScrollDirection) -> Result<()> {
    let (up, down) = match direction {
        // Up => Up, Down => Down
        ScrollDirection::Default => (MouseFn::ScrollUp, MouseFn::ScrollDown),

        // Up => Down, Down => Up
        ScrollDirection::Invert => (MouseFn::ScrollDown, MouseFn::ScrollUp),
    };

    for i in 1..=MAX_PROFILES {
        bind::set(device, i, Button::ScrollUp, Binding::Mouse(up))?;
        bind::set(device, i, Button::ScrollDown, Binding::Mouse(down))?;
    }

    Ok(())
}
