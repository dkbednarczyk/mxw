#![warn(clippy::all, clippy::nursery)]

pub mod args;
pub mod config;
pub mod glorious;
pub mod report;
pub mod util;

use anyhow::{anyhow, Result};
use args::{Args, Config, Kind, Report};
use clap::Parser;
use hidapi::HidApi;

fn main() -> Result<()> {
    let args = Args::parse();

    let hid_api = HidApi::new()?;

    let device_info = hid_api
        .device_list()
        .filter(|d| {
            d.vendor_id() == glorious::VENDOR_ID
                && glorious::is_glorious_product(d.product_id())
                && d.interface_number() == glorious::INTERFACE
        })
        // A Model O Wireless plugged in via cable enumerates both the wired
        // (0x2011) and wireless (0x2022) interfaces; the wired one is preferred
        // and has the lower product id.
        .min_by_key(|d| d.product_id())
        .ok_or_else(|| anyhow!("no matching device found"))?;

    let wired = glorious::is_wired(device_info.product_id());

    let device = device_info.open_device(&hid_api)?;

    match args.kind {
        // mxw report
        Kind::Report(report) => match report {
            // mow report battery
            Report::Battery { hide_status } => report::battery::get(&device, wired, hide_status),

            // mow report firmware
            Report::Firmware => report::firmware::get(&device),

            // mxw report profile
            Report::Profile => report::profile::get(&device),

            // mxw report dpi
            Report::DPI {
                profile,
                all,
                dpi,
                stage,
            } => report::dpi::get(&device, profile, all, dpi, stage),
        },

        // mxw config
        Kind::Config(config) => {
            util::status::check_sleep(&device)?;

            match config {
                // mow config bind ...
                Config::Bind {
                    profile,
                    button,
                    binding,
                } => config::bind::set(&device, profile, button, binding),

                // mxw config scroll <DIRECTION>
                Config::Scroll { direction } => config::scroll::set(&device, direction),

                // mxw config profile <ID>
                Config::Profile { id } => config::profile::set(&device, id),

                // mxw config sleep <MINUTES> [SECONDS]
                Config::Sleep { minutes, seconds } => config::sleep::set(&device, minutes, seconds),

                // mxw config led-brightness <WIRED> [WIRELESS]
                Config::LEDBrightness { wired, wireless } => {
                    config::led_brightness::set(&device, wired, wireless)
                }

                // mxw config led-effect <EFFECT> ...
                Config::LEDEffect { profile, effect } => {
                    config::led_effect::set(&device, profile, effect)
                }

                // mxw config polling-rate <MS>
                Config::PollingRate { ms } => config::polling_rate::set(&device, ms),

                // mxw config lift-off <MM>
                Config::LiftOff { mm } => config::lift_off::set(&device, mm),

                // mxw config debounce <MS>
                Config::Debounce { profile, ms } => config::debounce::set(&device, profile, ms),

                // mxw config dpi-stage <ID>
                Config::DPIStage { profile, id } => config::dpi_stage::set(&device, profile, id),

                // mxw config dpi-stages <STAGES>...
                Config::DPIStages {
                    profile,
                    uniform,
                    stages,
                } => config::dpi_stages::set(&device, profile, stages, uniform),

                // mxw config dpi-colors <COLORS>...
                Config::DPIColors { profile, colors } => {
                    config::dpi_colors::set(&device, profile, colors)
                }
            }
        }
    }
}
