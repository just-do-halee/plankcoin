use super::*;

pub use info::*;
pub use sdboxes::*;

#[derive(Debug, Default)]
pub struct Vault {
    pub info: VaultInfo,
    sdboxes: Sdboxes, // = safe deposit boxes
}

impl Vault {
    #[inline]
    pub fn sdboxes(&self) -> &Sdboxes {
        &self.sdboxes
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
    pub fn new(level: u64, pvi_hash: Hash, sdboxes: Sdboxes, owner: Hash) -> Self {
        Self {
            info: VaultInfo::new(level, pvi_hash, sdboxes.to_mkr_hash(), owner),
            sdboxes,
        }
    }
}

mod info;
mod sdboxes;
