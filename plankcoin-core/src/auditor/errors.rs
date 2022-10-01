use super::*;

/// Chain Validation Error
#[derive(Debug, Error)]
pub enum AuditError {
    #[error("Genesis vault is not found")]
    GenesisVaultNotFound,
    #[error("Unlocked vault is found")]
    UnlockedVaultFound,
    #[error("Pvi hash mismatch")]
    PviHashMismatch,
    #[error("Locked time is not increasing")]
    LockedTimeNotIncreasing,
    #[error("Vault hash is not less than target hash")]
    VaultHashNotLessThanTargetHash,
    #[error("Safe deposit boxes merkle root hash mismatch")]
    SdboxesMkrHashMismatch,
}
