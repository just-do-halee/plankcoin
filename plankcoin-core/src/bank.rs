use super::*;

pub use errors::*;
pub use vault::*;

pub struct Bank {
    vaults: Vec<Vault>,
}

impl Default for Bank {
    #[inline]
    fn default() -> Self {
        Self {
            vaults: vec![Vault::new_genesis()],
        }
    }
}

impl Bank {
    #[inline]
    pub fn latest_vault(&self) -> &Vault {
        self.vaults.last().unwrap()
    }
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Validate the vaults in the bank.
    pub fn audit(&self) -> Result<(), AuditError> {
        // 1. `genesis vault must be the first vault`
        if !self.vaults[0].is_genesis() {
            return Err(AuditError::GenesisVaultNotFound);
        }

        // first pvi_hash must be the same as the genesis vault's hash
        let mut pvi_hash = self.vaults[0].info.to_hash();
        // first locked_time must be the same as the genesis vault's locked_time
        let mut p_locked_time = self.vaults[0].info.to_locked_time();

        // iterate through the rest of the vaults
        self.vaults.iter().skip(1).try_for_each(|vault| {
            // 2. `the vault must be locked`
            if !vault.is_locked() {
                return Err(AuditError::UnlockedVaultFound);
            }

            // 3. `the vault's pvi_hash must be the same as the actual previous vault's hash`
            if vault.info.to_pvi_hash() != pvi_hash {
                return Err(AuditError::PviHashMismatch);
            }

            let locked_time = vault.info.to_locked_time();

            // 4. `the vault's locked_time must be more than the actual previous vault's locked_time`
            if locked_time > p_locked_time {
                return Err(AuditError::LockedTimeNotIncreasing);
            }

            let hash = vault.info.to_hash();

            // 5. `the vault's hash must be less than the target hash`
            if hash >= vault.info.to_target_hash() {
                return Err(AuditError::VaultHashNotLessThanTargetHash);
            }

            // 6. `the vault's sdbr_hash must be the same as the actual sdbr_hash`
            if vault.info.to_sdbr_hash() != vault.sd_boxes().to_sdbr_hash() {
                return Err(AuditError::SdbrHashMismatch);
            }

            // update the previous pvi_hash and locked_time
            p_locked_time = locked_time;
            pvi_hash = hash;
            Ok(())
        })
    }
}

mod errors;
mod vault;
