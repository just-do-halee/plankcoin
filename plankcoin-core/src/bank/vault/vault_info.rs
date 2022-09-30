use super::*;

#[derive(Debug)]
pub struct VaultInfo {
    version: i64,
    locked_time: Cell<u64>,
    password: Cell<u64>,
    level: u64,
    owner: Hash,
    pvi_hash: Hash,  // = previous vault information hash
    sdbr_hash: Hash, // = safe deposit boxes merkle root hash
    locked: Cell<bool>,
}

impl Default for VaultInfo {
    #[inline]
    fn default() -> Self {
        Self {
            version: VAULT_VERSION,
            locked_time: Cell::new(0),
            password: Cell::new(0),
            level: 0,
            owner: Hash::zero(),
            sdbr_hash: Hash::zero(),
            pvi_hash: Hash::zero(),
            locked: Cell::new(false),
        }
    }
}

impl VaultInfo {
    /// For calculating the target hash of the vault.
    ///
    /// ```
    /// # use plankcoin_core::{bank::VaultInfo, primitives::Uint};
    /// # let LEVEL_MASK = VaultInfo::LEVEL_MASK;
    /// # let level = 0;
    /// let target_shr_n_bits = (level & LEVEL_MASK) as u8;
    /// let target_head = level & !LEVEL_MASK;
    /// let target_hash = Uint::from(target_head) >> target_shr_n_bits;
    /// ```
    pub const LEVEL_MASK: u64 = 0x00000000000000FF;
    #[inline]
    pub fn to_target_hash(&self) -> Hash {
        debug!("level: {:#066b}", self.level);
        let target_shr_n_bits = (self.level & Self::LEVEL_MASK) as u8;
        debug!(
            "target_shr_n_bits: ({:#066b} & {:#066b}) = {:#010b}",
            self.level,
            Self::LEVEL_MASK,
            target_shr_n_bits
        );
        let target_head = self.level & !Self::LEVEL_MASK;
        debug!(
            "target_head: ({:#066b} & {:#066b}) = {:#066b}",
            self.level,
            !Self::LEVEL_MASK,
            target_head
        );
        let mut target_head = Uint::from(target_head);
        let leading_zeros = target_head.leading_zeros();
        debug!(
            "target_head(0x): {:#065x} <<= {} bits",
            target_head, leading_zeros
        );
        target_head <<= leading_zeros; // pull the head to the head
        debug!(
            "target_head(0x): {:#065x} >>= {} bits",
            target_head, target_shr_n_bits
        );
        target_head >>= target_shr_n_bits; // shift the head to the right
        debug!("target_hash(0x): {:#065x}", target_head);
        Hash::from(target_head.to_bytes())
    }
    #[inline]
    pub fn get_password(&self) -> u64 {
        self.password.get()
    }
    #[inline]
    pub fn set_password(&self, password: u64) {
        debug!("set password: {}", password);
        if self.locked.get() {
            debug!("the vault unlocked");
            self.locked.set(false);
            self.locked_time.set(0);
        }
        self.password.set(password)
    }
    #[inline]
    pub fn is_genesis(&self) -> bool {
        self.owner.is_zero() && self.pvi_hash.is_zero()
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        self.locked.get()
    }

    #[inline]
    fn _is_locked(&self) -> bool {
        let hash = self.to_hash();
        let target_hash = self.to_target_hash();
        debug!(
            "Hash(0x): {:#065x} < Target(0x): {:#065x}",
            hash, target_hash
        );
        hash < target_hash
    }

    /// Lock the vault.
    /// # Panic
    /// Panics if the vault is locked and the validation fails.
    #[inline]
    pub fn lock(&self) {
        if self.locked.get() && !self._is_locked() {
            debug!("Vault locking failed.");
            if self.is_genesis() {
                let msg = "The vault is a genesis vault and is already locked.";
                debug!("{msg}");
                panic!("{msg}");
            }
            panic!("The vault is locked and the validation fails.")
        }
        debug!("Vault locking started.");
        while !self._is_locked() {
            let new_password = self.password.get() + 1;
            debug!("Trying to lock the vault... password: {}", new_password);
            self.password.set(new_password);
        }
        let now = VaultTime::now();
        self.locked_time.set(now);
        self.locked.set(true);
        debug!("Vault is locked! Time: {}", now);
    }

    #[inline]
    pub fn new_genesis() -> Self {
        Self {
            level: GENESIS_VAULT_LEVEL,
            locked_time: Cell::new(VaultTime::now()),
            locked: Cell::new(true),
            ..Default::default()
        }
    }

    #[inline]
    pub fn new(level: u64, pvi_hash: Hash, sdbr_hash: Hash, owner: Hash) -> Self {
        Self {
            level,
            owner,
            pvi_hash,
            sdbr_hash,
            ..Default::default()
        }
    }

    #[inline]
    pub fn to_hash(&self) -> Hash {
        Hash::from_slice(
            &Sha3_256::new()
                .chain_update(&self.version.to_be_bytes())
                .chain_update(&self.locked_time.get().to_be_bytes())
                .chain_update(&self.password.get().to_be_bytes())
                .chain_update(&self.level.to_be_bytes())
                .chain_update(&self.owner)
                .chain_update(&self.pvi_hash)
                .chain_update(&self.sdbr_hash)
                .finalize(),
        )
    }
}
