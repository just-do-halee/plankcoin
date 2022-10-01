use super::*;

mod state_level;
mod sup_level;
mod top_level;

pub use state_level::*;
pub use sup_level::*;
pub use top_level::*;

#[derive(Debug, Default)]
pub struct VaultInfo {
    top: TopLevelInfo,
    /// For the lock/unlock of the vault info.
    sup: RefCell<SupLevelInfo>,
    state: StateLevelInfo,
    /// For the lock/unlock of the vault info.
    locked: Cell<bool>,
}

impl VaultInfo {
    #[inline]
    pub fn top_level(&self) -> &TopLevelInfo {
        &self.top
    }
    #[inline]
    pub fn sup_level(&self) -> Ref<SupLevelInfo> {
        self.sup.borrow()
    }
    #[inline]
    pub fn state_level(&self) -> &StateLevelInfo {
        &self.state
    }

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
        let TopLevelInfo { level, .. } = self.top_level();

        debug!("level: {:#066b}", level);
        let target_shr_n_bits = (level & Self::LEVEL_MASK) as u8;
        debug!(
            "target_shr_n_bits: ({:#066b} & {:#066b}) = {:#010b}",
            level,
            Self::LEVEL_MASK,
            target_shr_n_bits
        );
        let target_head = level & !Self::LEVEL_MASK;
        debug!(
            "target_head: ({:#066b} & {:#066b}) = {:#066b}",
            level,
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
    pub fn is_genesis(&self) -> bool {
        self.sup_level().owner.is_zero() && self.top.pvi_hash.is_zero()
    }

    #[inline]
    pub fn is_locked(&self) -> bool {
        self.locked.get()
    }

    /// Actual calculation of the hash of the target.
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
        if self.is_locked() && !self._is_locked() {
            debug!("Vault locking failed.");
            if self.is_genesis() {
                let msg = "The vault is a genesis vault and is already locked.";
                debug!("{msg}");
                panic!("{msg}");
            }
            panic!("The vault is locked and the validation fails.")
        }
        debug!("Vault locking started.");
        let mut sup = self.sup.borrow_mut();
        // until the vault is locked,
        // changing the password and finding the target hash
        while !self._is_locked() {
            let new_password = sup.password + 1;
            debug!("Trying to lock the vault... password: {}", new_password);
            sup.password = new_password;
        }
        // set the vault locked time
        let now = VaultTime::now();
        sup.locked_time = now;
        // set the locked flag
        self.locked.set(true);
        debug!("Vault is locked! Time: {}", now);
    }

    #[inline]
    pub fn new_genesis() -> Self {
        Self {
            sup: RefCell::new(SupLevelInfo::new_genesis()),
            locked: Cell::new(true),
            ..Default::default()
        }
    }

    #[inline]
    pub fn new(
        level: u64,
        pvi_hash: Hash,
        state_mkr_hash: Hash,
        sdboxes_mkr_hash: Hash,
        owner: Hash,
    ) -> Self {
        Self {
            top: TopLevelInfo::new(level, pvi_hash),
            sup: RefCell::new(SupLevelInfo::new(owner)),
            state: StateLevelInfo::new(state_mkr_hash, sdboxes_mkr_hash),
            ..Default::default()
        }
    }

    #[inline]
    pub fn to_hash(&self) -> Hash {
        let mut hasher = Sha3_256::new();
        hasher = self.top.to_hash(hasher);
        hasher = self.sup_level().to_hash(hasher);
        hasher = self.state_level().to_hash(hasher);
        Hash::from_slice(&hasher.finalize())
    }
}
