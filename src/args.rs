use crate::util::color::{self, Color};
use crate::util::key::{self, Key};
use clap::{value_parser, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    // From Cargo.toml
    author, version, about,

    // Don't need a subcommand when we have flags
    disable_help_subcommand = true,

    // Same reported version for subcommands
    propagate_version = true
)]
pub struct Args {
    #[clap(subcommand)]
    pub kind: Kind,
}

#[derive(Subcommand)]
pub enum Kind {
    /// Retrieve information about the device
    #[clap(subcommand)]
    Report(Report),

    /// Change the device's various settings
    #[clap(subcommand)]
    Config(Config),
}

#[derive(Subcommand)]
pub enum Report {
    /// Battery percentage (if available)
    Battery {
        #[arg(long, help = "Hide charging status")]
        hide_status: bool,
    },

    /// Device firmware version
    Firmware,

    /// Active profile id
    Profile,

    /// Active DPI stage and its resolution
    DPI {
        /// Profile id (1-3), defaults to the active profile
        #[arg(short, long, value_parser = value_parser!(u8).range(1..=3))]
        profile: Option<u8>,

        /// List every DPI stage, marking the active one
        #[arg(short, long, conflicts_with_all = ["dpi", "stage"])]
        all: bool,

        /// Print only the resolution, e.g. `1600`
        #[arg(short, long, conflicts_with = "stage")]
        dpi: bool,

        /// Print only the active stage number, e.g. `3`
        #[arg(short, long)]
        stage: bool,
    },
}

#[derive(Subcommand)]
pub enum Config {
    /// Active profile by id
    Profile {
        #[arg(value_parser = value_parser!(u8).range(1..=3))]
        id: u8,
    },

    /// LED Effect
    LEDEffect {
        /// Profile id (1-3)
        #[arg(
            short, long,
            default_value = "1",
            value_parser = value_parser!(u8).range(1..=3),
        )]
        profile: u8,

        #[clap(subcommand)]
        effect: Effect,
    },

    /// LED brightness value[s] (0-255)
    LEDBrightness {
        wired: u8,

        #[clap(default_value = "0")]
        wireless: u8,
    },

    /// Sleep delay in minutes [and seconds]
    Sleep {
        minutes: u8,

        #[clap(default_value = "0")]
        seconds: u8,
    },

    /// Active DPI stage by id
    DPIStage {
        /// Profile id (1-3)
        #[arg(
            short, long,
            default_value = "1",
            value_parser = value_parser!(u8).range(1..=3),
        )]
        profile: u8,

        #[arg(value_parser = value_parser!(u8).range(1..=6))]
        id: u8,
    },

    /// Set DPI stages (200-19000)
    DPIStages {
        /// Profile id (1-3)
        #[arg(
            short, long,
            default_value = "1",
            value_parser = value_parser!(u8).range(1..=3),
        )]
        profile: u8,

        /// Set every stage in the profile to this DPI, keeping the current stage count
        #[arg(
            short, long,
            value_name = "DPI",
            value_parser = value_parser!(u16).range(100..=19000),
            conflicts_with = "stage",
        )]
        uniform: Option<u16>,

        #[arg(
            name = "stage",
            num_args(1..=6),
            value_parser = value_parser!(u16).range(100..=19000),
            default_values(&["400", "800", "1600", "3200"]),
        )]
        stages: Vec<u16>,
    },

    /// Set DPI stage colors
    DPIColors {
        /// Profile id (1-3)
        #[arg(
            short, long,
            default_value = "1",
            value_parser = value_parser!(u8).range(1..=3),
        )]
        profile: u8,

        #[arg(
            name = "COLOR",
            num_args(1..=6),
            value_parser(color::parse_hex),
            default_values(&["FFFF00", "0000FF", "FF0000", "00FF00"]),
        )]
        colors: Vec<Color>,
    },

    /// Lift-off distance in mm
    LiftOff {
        #[arg(value_parser(1..=2))]
        mm: u8,
    },

    /// Polling rate in ms
    PollingRate {
        #[arg(value_parser = parse_polling_rate)]
        ms: u8,
    },

    /// Debounce in ms (0-16)
    Debounce {
        /// Profile id (1-3)
        #[arg(
            short, long,
            default_value = "1",
            value_parser = value_parser!(u8).range(1..=3),
        )]
        profile: u8,

        #[clap(value_parser = value_parser!(u8).range(0..=16))]
        ms: u8,
    },

    /// Key binding
    Bind {
        /// Profile id (1-3)
        #[arg(
            short, long,
            default_value = "1",
            value_parser = value_parser!(u8).range(1..=3),
        )]
        profile: u8,

        /// Mouse button
        #[arg(value_enum)]
        button: Button,

        #[clap(subcommand)]
        binding: Binding,
    },

    /// Scroll inversion
    Scroll {
        #[arg(value_enum)]
        direction: ScrollDirection,
    },
}

fn parse_polling_rate(value: &str) -> Result<u8, &'static str> {
    match value {
        "1" | "2" | "4" | "8" => value.parse().map_err(|_| "invalid polling rate"),
        _ => Err("polling rate must be one of 1, 2, 4, 8"),
    }
}

#[derive(Clone, ValueEnum)]
pub enum ScrollDirection {
    Default,
    Invert,
}

#[derive(Subcommand)]
pub enum Effect {
    /// Name says it all
    Glorious {
        /// Effect rate, 0-100
        #[arg(
            short, long,
            default_value = "40",
            value_parser = value_parser!(u8).range(0..=100),
        )]
        rate: u8,
    },

    /// Cycle through all colors
    Cycle {
        /// Effect rate, 0-100
        #[arg(
            short, long,
            default_value = "40",
            value_parser = value_parser!(u8).range(0..=100),
        )]
        rate: u8,
    },

    /// Pulse on/off through given colors
    Pulse {
        /// Effect rate, 0-100
        #[arg(
            short, long,
            default_value = "40",
            value_parser = value_parser!(u8).range(0..=100),
        )]
        rate: u8,

        /// From 2 to 6 colors in hex format
        #[arg(
            required = true,
            num_args(2..=6),
            value_parser(color::parse_hex),
        )]
        colors: Vec<Color>,
    },

    /// Solid color
    Solid {
        /// Color in hex format
        #[arg(value_parser(color::parse_hex))]
        color: Color,
    },

    /// Pulse on/off one color
    PulseOne {
        /// Effect rate, 0-100
        #[arg(
            short, long,
            default_value = "40",
            value_parser = value_parser!(u8).range(0..=100),
        )]
        rate: u8,

        #[arg(value_parser(color::parse_hex))]
        color: Color,
    },

    /// Glorious, but colors don't "move"
    Tail {
        /// Effect rate, 0-100
        #[arg(
            short, long,
            default_value = "40",
            value_parser = value_parser!(u8).range(0..=100),
        )]
        rate: u8,
    },

    /// Strobe-like effect
    Rave {
        /// Effect rate, 0-100
        #[arg(
            short, long,
            default_value = "40",
            value_parser = value_parser!(u8).range(0..=100),
        )]
        rate: u8,

        /// 1 or 2 colors in hex format
        #[arg(
            required = true,
            num_args(1..=2),
            value_parser(color::parse_hex),
        )]
        colors: Vec<Color>,
    },

    /// Glorious, but more circus
    Wave {
        /// Effect rate, 0-100
        #[arg(
            short, long,
            default_value = "40",
            value_parser = value_parser!(u8).range(0..=100),
        )]
        rate: u8,
    },

    /// No effect, LED off
    Off,
}

#[derive(Clone, ValueEnum)]

pub enum Button {
    Left,
    Right,
    Scroll,
    Forward,
    Back,
    DPIBtn,
    ScrollUp,
    ScrollDown,
}

#[derive(Subcommand)]
pub enum Binding {
    /// Single key
    Key {
        #[clap(subcommand)]
        kind: KeyKind,
    },

    /// Keyboard function
    #[clap(subcommand)]
    Keyboard(KeyboardFn),

    /// Mouse function
    #[clap(subcommand)]
    Mouse(MouseFn),

    /// DPI modifier
    #[clap(subcommand)]
    DPI(DPIFn),

    /// Multimedia
    #[clap(subcommand)]
    Media(MediaFn),

    /// Do nothing
    None,
}

#[derive(Subcommand)]
pub enum KeyKind {
    /// Hardware scan code
    ScanCode {
        #[arg(value_parser(key::parse_scan_code))]
        key: Key,

        /// Optional modifier
        #[arg(short, long, value_parser(key::parse_scan_code))]
        modifier: Option<Key>,
    },

    /// JS-style KeyCode
    KeyCode {
        #[arg(value_parser(key::parse_key_code))]
        key: Key,

        /// Optional modifier
        #[arg(short, long, value_parser(key::parse_key_code))]
        modifier: Option<Key>,
    },

    /// JS-style Code
    Code {
        #[arg(value_parser(key::parse_code))]
        key: Key,

        /// Optional modifier
        #[arg(short, long, value_parser(key::parse_code))]
        modifier: Option<Key>,
    },
}

#[derive(Subcommand)]
pub enum MouseFn {
    Left,
    Right,
    Scroll,
    Forward,
    Back,
    ScrollUp,
    ScrollDown,
    ProfileCycleUp,
    ProfileCycleDown,
    BatteryStatus,
}

#[derive(Subcommand)]
pub enum KeyboardFn {
    ProfileCycleUp,
    ProfileCycleDown,
    LayerCycleUp,
    LayerCycleDown,
}

#[derive(Subcommand)]
pub enum DPIFn {
    StageUp,
    StageDown,
    CycleUp,
    CycleDown,
}

#[derive(Subcommand)]
pub enum MediaFn {
    Player,
    PlayPause,
    Next,
    Previous,
    Stop,
    Mute,
    VolumeUp,
    VolumeDown,
}
