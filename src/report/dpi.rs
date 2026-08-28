use crate::report;
use anyhow::{anyhow, Result};
use hidapi::HidDevice;

pub fn get(
    device: &HidDevice,
    profile: Option<u8>,
    all: bool,
    dpi_only: bool,
    stage_only: bool,
) -> Result<()> {
    let profile = match profile {
        Some(p) => p,
        None => report::read(device, 0x01, 0x85, 0x00, 0x00)?[7],
    };

    let active = report::read(device, 0x02, 0x82, 0x01, profile)?[8];

    if stage_only {
        println!("{active}");
        return Ok(());
    }

    let stages = report::read(device, 0x12, 0x81, 0x01, profile)?;
    let count = stages[8];

    if !(1..=13).contains(&count) {
        return Err(anyhow!(
            "device reported an implausible DPI stage count ({count})"
        ));
    }

    let dpi = |stage: u8| -> u16 {
        let offset = 9 + 4 * (stage as usize - 1);
        u16::from_be_bytes([stages[offset], stages[offset + 1]])
    };

    if !(1..=count).contains(&active) {
        return Err(anyhow!("device reported invalid active DPI stage {active}"));
    }

    if dpi_only {
        println!("{}", dpi(active));
        return Ok(());
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
