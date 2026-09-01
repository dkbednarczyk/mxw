use crate::util::status;
use anyhow::Result;
use colored::Colorize;
use hidapi::HidDevice;

pub fn get(device: &HidDevice, wired: bool, hide_status: bool) -> Result<()> {
    let bfr_r = status::get_buffer(device)?;

    // A device plugged in from empty briefly reports 0 before the first real
    // reading, and readings above 100 are garbage; clamp both into the
    // plausible 1..=100 range.
    let percentage = bfr_r[8].clamp(1, 100);

    let status = status::get(device)?;

    match (status, wired) {
        (status::DeviceStatus::Awake, false) => println!("{percentage}%"),
        (status::DeviceStatus::Awake, true) => {
            let mut charging_status = String::new();
            if !hide_status {
                charging_status = match percentage {
                    0..=24 => format!(" ({})", "charging".red()),
                    25..=74 => format!(" ({})", "charging".yellow()),
                    75..=99 => format!(" ({})", "charging".bright_yellow()),
                    100.. => format!(" ({})", "fully charged".green()),
                }
            }

            println!("{percentage}%{charging_status}")
        }
        (status::DeviceStatus::Asleep, _) => println!("(asleep)"),
        (status::DeviceStatus::WakingUp, _) => print!("(waking up)"),
        (_, _) => {
            println!(
                "[1:{:0>2X}, 6:{:0>2X}, 8:{:0>2X}] ({})",
                bfr_r[1],
                bfr_r[6],
                bfr_r[8],
                "unknown status".red(),
            );
        }
    }

    Ok(())
}
