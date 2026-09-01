use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 {
            return Err("color hex must be of length 6");
        }

        let parsed = u32::from_str_radix(s, 16).map_err(|_| "could not parse color hex")?;

        let [_, red, green, blue] = parsed.to_be_bytes();

        Ok(Self { red, green, blue })
    }
}
