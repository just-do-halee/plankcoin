use super::*;

#[derive(Debug, Default)]
pub struct Vault {
    pub info: VaultInfo,
    pub sd_boxes: Vec<SdBox>, // = safe deposit boxes
}

impl Vault {
    #[inline]
    pub fn new_genesis() -> Self {
        Default::default()
    }
    #[inline]
    pub fn new(pvi_hash: [u8; HASH_SIZE], sd_boxes: Vec<SdBox>) -> Self {
        Self {
            info: VaultInfo::new(pvi_hash),
            sd_boxes,
        }
    }

    #[inline]
    pub fn is_mined(&self) -> bool {
        todo!()
    }

    pub fn mine(&mut self, level: u64) {
        todo!()
    }
}

use sd_box::*;
use vault_info::*;

mod sd_box;
mod vault_info;
