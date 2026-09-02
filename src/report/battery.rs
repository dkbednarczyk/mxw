use crate::util::status;
use anyhow::Result;
use colored::Colorize;
use hidapi::HidDevice;

pub fn get(device: &HidDevice, wired: bool, value_only: bool) -> Result<()> {
    let bfr_r = status::get_buffer(device)?;
    let status = status::DeviceStatus::from_report(&bfr_r);

    // A device plugged in from empty briefly reports 0 before the first real
    // reading, and readings above 100 are garbage; clamp both into the
    // plausible 1..=100 range.
    let percentage = bfr_r[8].clamp(1, 100);

    if value_only {
        match status {
            status::DeviceStatus::Awake => println!("{percentage}"),
            status::DeviceStatus::Asleep => println!("asleep"),
            status::DeviceStatus::WakingUp => println!("waking up"),
            _ => println!("unknown"),
        }

        return Ok(());
    }

    match (status, wired) {
        (status::DeviceStatus::Awake, false) => println!("Battery: {percentage}%"),
        (status::DeviceStatus::Awake, true) => {
            let charging_status = match percentage {
                0..=24 => format!(" ({})", "charging".red()),
                25..=74 => format!(" ({})", "charging".yellow()),
                75..=99 => format!(" ({})", "charging".bright_yellow()),
                100.. => format!(" ({})", "fully charged".green()),
            };

            println!("Battery: {percentage}%{charging_status}")
        }
        (status::DeviceStatus::Asleep, _) => println!("Battery: asleep"),
        (status::DeviceStatus::WakingUp, _) => println!("Battery: waking up"),
        (_, _) => {
            println!(
                "Battery: unknown [1:{:0>2X}, 6:{:0>2X}, 8:{:0>2X}] ({})",
                bfr_r[1],
                bfr_r[6],
                bfr_r[8],
                "unknown status".red(),
            );
        }
    }

    Ok(())
}
