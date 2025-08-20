use strum_macros::EnumIter;

pub const VENDOR_ID: u16 = 0x258A;
pub const INTERFACE: i32 = 0x02;

#[derive(Debug, EnumIter)]
pub enum Device {
    ModelO = 0x2011,
    ModelD = 0x2012,
    ModelOMinus = 0x2013,
    ModelDMinus = 0x2025,
    SeriesOnePro = 0x2018,
    WiredModelO = 0x2022,
    WiredModelD = 0x2023,
    WiredModelOMinus = 0x2024,
    WiredSeriesOnePro = 0x2031,
    ModelD2Pro = 0x2034,
}
