use super::*;

use crate::bank::{vaults::vault::Vault, Bank};

pub mod errors;

use errors::*;

#[derive(Clone, Copy)]
pub struct Auditor<'a>(&'a Bank);

impl<'a> Auditor<'a> {
    #[inline]
    pub fn new(bank: &'a Bank) -> Self {
        Self(bank)
    }

    #[inline]
    pub fn get_genesis_vault(&self) -> Option<&Vault> {
        trace!("Auditor is looking for genesis vault");
        // 1. `genesis vault must be the first vault`
        // if there is no first vault, then it panics
        // because the bank must have at least one vault
        let genesis = self.0.vaults().first();

        // if self.0.kind() == BankKind::Small {
        //     let genesis_pvi_hash = genesis.info.top_level().pvi_hash;
        //     if genesis_pvi_hash.is_zero() {
        //         return None;
        //     }
        // } else if !genesis.is_genesis() {
        //     return None;
        // }

        if !genesis.is_genesis() {
            return None;
        }

        Some(genesis)
    }

    /// Audit the current vault.
    ///
    /// This is not a full audit, but a partial audit.\
    /// It only checks the current vault. not the previous vaults.
    /// # Returns
    ///
    /// * Ok((current_vault_hash([`Hash`]) , current_vault_locked_time([`u64`]))) if the audit is successful.
    ///
    /// * Err([`AuditError`]) if the audit is failed.
    pub fn audit_with_vault(
        &self,
        previous_vault: &Vault,
        current_vault: &Vault,
    ) -> Result<(Hash, u64), AuditError> {
        trace!("Auditor is auditing the current vault");

        self.unchecked_audit_with_vault(
            previous_vault.info.to_hash(),
            previous_vault.info.sup_level().locked_time,
            current_vault,
        )
    }

    /// Audit the current vault.
    ///
    /// Unchecked version of [`Self::audit_with_vault`].
    /// # Returns
    ///
    /// * Ok((current_vault_hash([`Hash`]) , current_vault_locked_time([`u64`]))) if the audit is successful.
    ///
    /// * Err([`AuditError`]) if the audit is failed.
    #[inline]
    pub fn unchecked_audit_with_vault(
        &self,
        previous_vault_hash: Hash,
        previous_vault_locked_time: u64,
        current_vault: &Vault,
    ) -> Result<(Hash, u64), AuditError> {
        let kind = self.0.kind();

        // 1. `the vault must be locked`
        if !current_vault.is_locked() {
            return Err(AuditError::UnlockedVaultFound);
        }

        // 2. `the vault's pvi_hash must be the same as the actual previous vault's hash`
        if current_vault.info.top_level().pvi_hash != previous_vault_hash {
            return Err(AuditError::PviHashMismatch);
        }

        let current_vault_locked_time = current_vault.info.sup_level().locked_time;

        // 3. `the vault's locked_time must be more than the actual previous vault's locked_time`
        if current_vault_locked_time > previous_vault_locked_time {
            return Err(AuditError::LockedTimeNotIncreasing);
        }

        let current_vault_hash = current_vault.info.to_hash();

        // 4. `the vault's hash must be less than the target hash`
        if current_vault_hash >= current_vault.info.to_target_hash() {
            return Err(AuditError::VaultHashNotLessThanTargetHash);
        }

        // * big bank only has the full body of safe deposit boxes
        if kind == BankKind::Big {
            // 5. `the vault's sdboxes_mkr_hash must be the same as the actual sdboxes_mkr_hash`
            if current_vault.info.state_level().sdboxes_mkr_hash
                != current_vault.sdboxes().to_mkr_hash()
            {
                return Err(AuditError::SdboxesMkrHashMismatch);
            }
        }

        Ok((current_vault_hash, current_vault_locked_time))
    }

    /// Audit the whole bank.
    #[inline]
    pub fn audit(&self) -> Result<(), AuditError> {
        let genesis_vault = self
            .get_genesis_vault()
            .ok_or(AuditError::GenesisVaultNotFound)?;

        // previous vault info hash
        // and previous locked time
        // to keep track of the whole vaults
        let mut previous_vault_hash;
        let mut previous_vault_locked_time;

        // initial previous vault info hash
        // and initial previous locked time
        // must be the equal to the genesis vault's
        previous_vault_hash = genesis_vault.info.to_hash();
        previous_vault_locked_time = genesis_vault.info.sup_level().locked_time;

        debug!(
            "Auditor is auditing the {}'s bank.",
            self.0.owner().to_hex0x()
        );
        // iterate through the rest of the vaults
        let result =
            self.0
                .vaults()
                .iter()
                .skip(1)
                .enumerate()
                .try_for_each(|(i, current_vault)| {
                    trace!("Auditor is auditing the {}th vault", i);

                    (previous_vault_hash, previous_vault_locked_time) = self
                        .unchecked_audit_with_vault(
                            previous_vault_hash,
                            previous_vault_locked_time,
                            current_vault,
                        )?;
                    Ok(())
                });
        debug!(
            "Auditor has finished auditing the {}'s bank.",
            self.0.owner().to_hex0x()
        );
        result
    }
}
