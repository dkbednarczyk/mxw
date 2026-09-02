use crate::report;
use anyhow::Result;
use hidapi::HidDevice;

pub fn get(device: &HidDevice, seconds_only: bool) -> Result<()> {
    let resp = report::read(device, 0x02, 0x87, 0x00, 0x00)?;

    let total = u16::from_be_bytes([resp[7], resp[8]]);

    if seconds_only {
        if total == 0xFFFF {
            println!("never");
        } else {
            println!("{total}");
        }

        return Ok(());
    }

    if total == 0xFFFF {
        println!("Sleep delay: never");
    } else {
        println!("Sleep delay: {}", describe(total));
    }

    Ok(())
}

/// Human description of a sleep delay in total seconds, omitting zero parts.
fn describe(total: u16) -> String {
    let minutes = total / 60;
    let seconds = total % 60;

    let minute_part = match minutes {
        0 => None,
        1 => Some("1 minute".to_string()),
        _ => Some(format!("{minutes} minutes")),
    };

    let second_part = match seconds {
        0 => None,
        1 => Some("1 second".to_string()),
        _ => Some(format!("{seconds} seconds")),
    };

    match (minute_part, second_part) {
        (Some(m), Some(s)) => format!("{m} {s}"),
        (Some(m), None) => m,
        (None, Some(s)) => s,
        // A zero delay is only reachable via a raw device value; the `config`
        // command maps it to `never` instead.
        (None, None) => "0 seconds".to_string(),
    }
}
