use super::*;

pub use sd_boxes::*;
pub use vault_info::*;

#[derive(Debug, Default)]
pub struct Vault {
    pub info: VaultInfo,
    sd_boxes: SdBoxes, // = safe deposit boxes
}

impl Vault {
    #[inline]
    pub fn sd_boxes(&self) -> &SdBoxes {
        &self.sd_boxes
    }

    #[inline]
    pub fn is_genesis(&self) -> bool {
        self.info.is_genesis()
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        self.info.is_locked()
    }

    #[inline]
    pub fn lock(&self) {
        self.info.lock()
    }

    #[inline]
    pub fn new_genesis() -> Self {
        Self {
            info: VaultInfo::new_genesis(),
            ..Default::default()
        }
    }
    #[inline]
    pub fn new(level: u64, pvi_hash: Hash, sd_boxes: SdBoxes, owner: Hash) -> Self {
        Self {
            info: VaultInfo::new(level, pvi_hash, sd_boxes.to_sdbr_hash(), owner),
            sd_boxes,
        }
    }
}

mod sd_boxes;
mod vault_info;
