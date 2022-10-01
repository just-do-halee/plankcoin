use super::*;

pub use enums::*;
pub use errors::*;
pub use vaults::*;

pub struct Bank {
    owner: Keypair,
    kind: BankKind,
    vaults: Vaults,
}

impl Default for Bank {
    #[inline]
    fn default() -> Self {
        Self {
            owner: Keypair::generate(&mut OsRng),
            vaults: Vaults::new_with_genesis(),
            kind: BankKind::Big,
        }
    }
}

impl Bank {
    #[inline]
    pub fn owner(&self) -> PublicKey {
        self.owner.public
    }
    #[inline]
    pub fn kind(&self) -> BankKind {
        self.kind
    }
    #[inline]
    pub fn vaults(&self) -> &Vaults {
        &self.vaults
    }

    #[inline]
    pub fn new(owner: Keypair, kind: BankKind) -> Self {
        Self {
            owner,
            kind,
            ..Default::default()
        }
    }

    /// Validate the vaults in the bank.
    pub fn audit(&self) -> Result<(), AuditError> {
        match self.kind {
            BankKind::Big => self.audit_for_big(),
            // BankKind::Medium => self.audit_for_medium(),
            // BankKind::Small => self.audit_for_small(),
            _ => todo!(),
        }
    }

    // ---------------------------------------------------------------------------------------------

    fn audit_for_big(&self) -> Result<(), AuditError> {
        // previous vault info hash
        // and previous locked time
        // to keep track of the whole vaults
        let mut pvi_hash;
        let mut p_locked_time;

        {
            // 1. `genesis vault must be the first vault`
            let genesis = self.vaults.first();
            if !genesis.is_genesis() {
                return Err(AuditError::GenesisVaultNotFound);
            }

            // initial previous vault info hash
            // and initial previous locked time
            // must be the equal to the genesis vault's
            pvi_hash = genesis.info.to_hash();
            p_locked_time = genesis.info.sup_level().locked_time;
        }

        // iterate through the rest of the vaults
        self.vaults.iter().skip(1).try_for_each(|vault| {
            // 2. `the vault must be locked`
            if !vault.is_locked() {
                return Err(AuditError::UnlockedVaultFound);
            }

            // 3. `the vault's pvi_hash must be the same as the actual previous vault's hash`
            if vault.info.top_level().pvi_hash != pvi_hash {
                return Err(AuditError::PviHashMismatch);
            }

            let locked_time = vault.info.sup_level().locked_time;

            // 4. `the vault's locked_time must be more than the actual previous vault's locked_time`
            if locked_time > p_locked_time {
                return Err(AuditError::LockedTimeNotIncreasing);
            }

            let hash = vault.info.to_hash();

            // 5. `the vault's hash must be less than the target hash`
            if hash >= vault.info.to_target_hash() {
                return Err(AuditError::VaultHashNotLessThanTargetHash);
            }

            // 6. `the vault's sdboxes_mkr_hash must be the same as the actual sdboxes_mkr_hash`
            if vault.info.state_level().sdboxes_mkr_hash != vault.sdboxes().to_mkr_hash() {
                return Err(AuditError::SdbrHashMismatch);
            }

            // update the previous pvi_hash and locked_time
            p_locked_time = locked_time;
            pvi_hash = hash;
            Ok(())
        })
    }
}

mod enums;
mod errors;
mod vaults;
