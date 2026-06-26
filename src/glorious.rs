pub const VENDOR_ID: u16 = 0x258A;
pub const INTERFACE: i32 = 0x02;

pub const WIRELESS_PRODUCT_IDS: [u16; 7] = [0x2013, 0x2018, 0x2022, 0x2023, 0x2025, 0x2034, 0x2027];
pub const WIRED_PRODUCT_IDS: [u16; 5] = [0x2011, 0x2012, 0x2015, 0x2024, 0x2031];

pub fn is_glorious_product(product_id: u16) -> bool {
    WIRELESS_PRODUCT_IDS.contains(&product_id) || WIRED_PRODUCT_IDS.contains(&product_id)
}

pub fn is_wired(product_id: u16) -> bool {
    WIRED_PRODUCT_IDS.contains(&product_id)
}
