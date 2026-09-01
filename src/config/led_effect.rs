use crate::args::Effect;
use crate::util::color::Color;
use anyhow::Result;
use hidapi::HidDevice;

/// Colors are written in 3-byte slots from byte 12, padded with black.
const fn black() -> Color {
    Color {
        red: 0,
        green: 0,
        blue: 0,
    }
}

fn write_colors(bfr: &mut [u8], colors: &[Color], slots: usize) {
    for (i, color) in colors
        .iter()
        .chain(std::iter::repeat(&black()))
        .take(slots)
        .enumerate()
    {
        let offset = 12 + (3 * i);

        bfr[offset] = color.red;
        bfr[offset + 1] = color.green;
        bfr[offset + 2] = color.blue;
    }
}

pub fn set(device: &HidDevice, profile: u8, effect: Effect) -> Result<()> {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[5] = 0x02;
    bfr[7] = profile;
    bfr[8] = 0xFF;

    match effect {
        Effect::Glorious { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x01;
            bfr[11] = rate_default(rate);
        }

        Effect::Cycle { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x02;
            bfr[11] = rate_default(rate);
            bfr[12] = 0xFF;
        }

        Effect::Pulse { rate, colors } => {
            bfr[4] = (colors.len() as u8) * 3 + 5;
            bfr[9] = 0x03;
            bfr[11] = rate_default(rate);

            write_colors(&mut bfr, &colors, 6);
        }

        Effect::Solid { color } => {
            bfr[4] = 0x08;
            bfr[9] = 0x04;

            write_colors(&mut bfr, &[color], 1);
        }

        Effect::PulseOne { rate, color } => {
            bfr[4] = 0x08;
            bfr[9] = 0x05;
            bfr[11] = rate_default(rate);

            write_colors(&mut bfr, &[color], 1);
        }

        Effect::Tail { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x06;
            bfr[11] = rate_default(rate);
        }

        Effect::Rave { rate, colors } => {
            bfr[4] = (colors.len() as u8) * 3 + 5;
            bfr[9] = 0x07;
            bfr[11] = rate_rave_wave(rate);

            write_colors(&mut bfr, &colors, 2);
        }

        Effect::Wave { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x08;
            bfr[11] = rate_rave_wave(rate);
        }

        Effect::Off => {
            bfr[4] = 0x05;
            bfr[9] = 0x00;
        }
    }

    device.send_feature_report(&bfr)?;

    Ok(())
}

// Both formulas are taken verbatim from the original reverse engineering of
// Glorious Core. The firmware's meaning of these constants is not documented;
// they are only known to be the required wire values.
const RATE_BASE: u8 = 105;

/// Wire rate for every animated effect except Rave and Wave:
/// `(RATE_BASE - rate) / 5`.
const fn rate_default(rate: u8) -> u8 {
    (RATE_BASE - rate) / 5
}

/// Wire rate for the Rave and Wave effects: `(RATE_BASE - rate) * 2`.
const fn rate_rave_wave(rate: u8) -> u8 {
    (RATE_BASE - rate) * 2
}
