pub(crate) const VENDOR_ID: u16 = 0x258A;
pub(crate) const INTERFACE: i32 = 0x02;

/// All known product ids with their link type: `true` is wired.
const MODELS: [(u16, bool); 12] = [
    (0x2011, true),
    (0x2012, true),
    (0x2015, true),
    (0x2024, true),
    (0x2031, true),
    (0x2013, false),
    (0x2018, false),
    (0x2022, false),
    (0x2023, false),
    (0x2025, false),
    (0x2034, false),
    (0x2027, false),
];

pub(crate) fn is_glorious_product(product_id: u16) -> bool {
    MODELS.iter().any(|(id, _)| *id == product_id)
}

pub(crate) fn is_wired(product_id: u16) -> bool {
    MODELS.iter().any(|(id, wired)| *id == product_id && *wired)
}
