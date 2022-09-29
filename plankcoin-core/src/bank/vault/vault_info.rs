use super::*;

#[derive(Debug)]
pub struct VaultInfo {
    pub version: i64,
    pub locked_time: u64,
    pub password: u64,
    pub level: u64,
    pub pvi_hash: [u8; HASH_SIZE], // = previous vault information hash
}

impl Default for VaultInfo {
    #[inline]
    fn default() -> Self {
        Self {
            version: VAULT_VERSION,
            locked_time: VaultTime::now(),
            password: 0,
            level: 0,
            pvi_hash: [0; HASH_SIZE],
        }
    }
}

impl VaultInfo {
    #[inline]
    pub fn new(pvi_hash: [u8; HASH_SIZE]) -> Self {
        Self {
            pvi_hash,
            locked_time: 0,
            ..Default::default()
        }
    }
    #[inline]
    pub fn to_hash(&self) -> [u8; HASH_SIZE] {
        Sha3_256::new()
            .chain_update(&self.version.to_be_bytes())
            .chain_update(&self.locked_time.to_be_bytes())
            .chain_update(&self.password.to_be_bytes())
            .chain_update(&self.level.to_be_bytes())
            .chain_update(&self.pvi_hash)
            .finalize()
            .into()
    }
}
