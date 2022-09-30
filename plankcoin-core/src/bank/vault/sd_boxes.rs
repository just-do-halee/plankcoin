use super::*;

pub use sd_box::SdBox;

#[derive(Debug, Default)]
pub struct SdBoxes {
    boxes: Vec<SdBox>,
}

impl SdBoxes {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
    pub fn to_sdbr_hash(&self) -> Hash {
        Hash::default()
    }
}

mod sd_box;
