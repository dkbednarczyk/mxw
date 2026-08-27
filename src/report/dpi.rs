use crate::report;
use anyhow::{anyhow, Result};
use hidapi::HidDevice;

pub fn get(device: &HidDevice, profile: Option<u8>, all: bool) -> Result<()> {
    // Mirror of `config::profile::set` (0x05): active profile id at index 7.
    let profile = match profile {
        Some(p) => p,
        None => report::read(device, 0x01, 0x85, 0x00, 0x00)?[7],
    };

    // Mirror of `config::dpi_stage::set` (0x02): active stage id at index 8.
    let active = report::read(device, 0x02, 0x82, 0x01, profile)?[8];

    // Mirror of `config::dpi_stages::set` (0x01): stage count at index 8, then
    // one big-endian u16 per stage repeated for the X and Y axes (4 bytes each).
    let stages = report::read(device, 0x12, 0x81, 0x01, profile)?;
    let count = stages[8];

    let dpi = |stage: u8| -> u16 {
        let offset = 9 + 4 * (stage as usize - 1);
        u16::from_be_bytes([stages[offset], stages[offset + 1]])
    };

    if !(1..=count).contains(&active) {
        return Err(anyhow!("device reported invalid active DPI stage {active}"));
    }

    if all {
        for stage in 1..=count {
            let marker = if stage == active { "* " } else { "  " };
            println!("{marker}{stage}: {} DPI", dpi(stage));
        }

        return Ok(());
    }

    println!("{} DPI (stage {active} of {count})", dpi(active));

    Ok(())
}
