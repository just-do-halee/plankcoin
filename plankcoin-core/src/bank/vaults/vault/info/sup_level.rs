use super::*;

/// Owner-dependent information
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupLevelInfo {
    pub owner: Hash,
    pub locked_time: u64,
    pub password: u64,
}
impl ToHash for SupLevelInfo {
    /// The conversion to hash is done by serializing the struct to bytes
    /// as a big-endian byte array.
    #[inline]
    fn to_hash<T: Digest>(&self, hasher: T) -> T {
        hasher
            .chain_update(self.owner.as_ref())
            .chain_update(self.locked_time.to_be_bytes())
            .chain_update(self.password.to_be_bytes())
    }
}

impl SupLevelInfo {
    #[inline]
    pub fn new(owner: Hash) -> Self {
        Self {
            owner,
            ..Default::default()
        }
    }
    #[inline]
    pub fn new_genesis() -> Self {
        Self {
            locked_time: VaultTime::now(),
            ..Default::default()
        }
    }
}
