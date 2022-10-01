use super::*;

/// State-dependent information
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateLevelInfo {
    pub state_mkr_hash: Hash,
    /// safe deposit boxes merkle root hash
    pub sdboxes_mkr_hash: Hash,
}
impl ToHash for StateLevelInfo {
    /// The conversion to hash is done by serializing the struct to bytes
    /// as a big-endian byte array.
    #[inline]
    fn to_hash<T: Digest>(&self, hasher: T) -> T {
        hasher
            .chain_update(self.state_mkr_hash.as_ref())
            .chain_update(self.sdboxes_mkr_hash.as_ref())
    }
}
