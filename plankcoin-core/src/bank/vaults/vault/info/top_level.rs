use super::*;

/// System-dependent information
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TopLevelInfo {
    pub version: i64,
    /// previous vault information hash
    pub pvi_hash: Hash,
    pub level: u64,
}

impl Default for TopLevelInfo {
    #[inline]
    fn default() -> Self {
        Self {
            version: VAULT_VERSION,
            level: GENESIS_VAULT_LEVEL,
            pvi_hash: Hash::zero(),
        }
    }
}
impl ToHash for TopLevelInfo {
    /// The conversion to hash is done by serializing the struct to bytes
    /// as a big-endian byte array.
    #[inline]
    fn to_hash<T: Digest>(&self, hasher: T) -> T {
        hasher
            .chain_update(self.version.to_be_bytes())
            .chain_update(self.level.to_be_bytes())
            .chain_update(self.pvi_hash.as_ref())
    }
}

impl TopLevelInfo {
    #[inline]
    pub fn new(level: u64, pvi_hash: Hash) -> Self {
        Self {
            version: VAULT_VERSION,
            level,
            pvi_hash,
        }
    }
}
