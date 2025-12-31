use crate::util::status;
use anyhow::Result;
use colored::Colorize;
use hidapi::HidDevice;

pub fn get(device: &HidDevice, wired: bool, hide_status: bool) -> Result<()> {
    let bfr_r = status::get_buffer(device)?;

    let mut percentage = bfr_r[8];
    if percentage == 0 {
        percentage = 1;
    }

    let status = status::get(device)?;

    match (status, wired) {
        (0, false) => println!("{percentage}%"),
        (0, true) => {
            let mut charging_status = "".to_string();
            if !hide_status {
                charging_status = match percentage {
                    0..=24 => format!("({})", "charging".red()),
                    25..=74 => format!("({})", "charging".yellow()),
                    75..=99 => format!("({})", "charging".bright_yellow()),
                    100.. => format!("({})", "fully charged".green()),
                }
            }

            println!("{percentage}% {charging_status}")
        }
        (1, _) => println!("(asleep)"),
        (3, _) => print!("(waking up)"),
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
