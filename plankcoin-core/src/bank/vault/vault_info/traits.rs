use super::*;

pub trait HasVaultInfo {
    fn to_version(&self) -> i64;
    fn to_locked_time(&self) -> u64;
    fn to_password(&self) -> u64;
    fn to_level(&self) -> u64;
    fn to_owner(&self) -> Hash;
    fn to_pvi_hash(&self) -> Hash;
    fn to_sdbr_hash(&self) -> Hash;
}
