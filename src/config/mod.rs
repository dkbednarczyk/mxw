/// Maximum number of profiles supported by the software
pub const MAX_PROFILES: u8 = 3;

/// Maximum number of DPI stages supported by the software
pub const MAX_DPI_STAGES: u8 = 6;

/// Payload length of a DPI stage list report: 2 bytes of profile + count,
/// then 4 bytes per stage.
pub const fn dpi_stages_payload_len(stage_count: usize) -> u8 {
    2 + (stage_count * 4) as u8
}

pub mod bind;
pub mod debounce;
pub mod dpi_colors;
pub mod dpi_stage;
pub mod dpi_stages;
pub mod led_brightness;
pub mod led_effect;
pub mod lift_off;
pub mod polling_rate;
pub mod profile;
pub mod scroll;
pub mod sleep;
