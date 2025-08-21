use crate::util::status;
use anyhow::Result;
use colored::Colorize;
use hidapi::HidDevice;

pub fn get(device: &HidDevice, wired: bool) -> Result<()> {
    let status = status::get(device)?;
    let bfr_r = status::get_buffer(device)?;

    let mut percentage = bfr_r[8];
    if percentage == 0 {
        percentage = 1;
    }

    match (status, wired) {
        (0, false) => println!("{percentage}%"),
        (0, true) => {
            let charging_status = match percentage {
                0..=24 => "charging".red(),
                25..=74 => "charging".yellow(),
                75..=99 => "charging".bright_yellow(),
                100.. => "fully charged".green(),
            };
            println!("{percentage}% ({charging_status})")
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
