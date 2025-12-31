use strum_macros::EnumIter;

pub const VENDOR_ID: u16 = 0x258A;
pub const INTERFACE: i32 = 0x02;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Device {
    ModelOMinus = 0x2013,
    SeriesOnePro = 0x2018,
    ModelO = 0x2022,
    ModelD = 0x2023,
    ModelDMinus = 0x2025,
    ModelD2Pro = 0x2034,
    ModelOPro = 0x2027,

    WiredModelO = 0x2011,
    WiredModelD = 0x2012,
    WiredModelOPro = 0x2015,
    WiredModelOMinus = 0x2024,
    WiredSeriesOnePro = 0x2031,
}

pub const fn is_wired(dev: Device) -> bool {
    matches!(
        dev,
        Device::WiredModelO
            | Device::WiredModelD
            | Device::WiredModelOMinus
            | Device::WiredSeriesOnePro
            | Device::WiredModelOPro
    )
}
