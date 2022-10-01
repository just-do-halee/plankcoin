use super::*;

pub mod vault;

use vault::*;

#[derive(Debug)]
pub struct Vaults {
    vaults: Vec<Vault>,
}
impl Default for Vaults {
    #[inline]
    fn default() -> Self {
        Self {
            vaults: vec![Vault::new_genesis()],
        }
    }
}

impl Vaults {
    #[inline]
    pub fn new_with_genesis() -> Self {
        trace!("Vaults were created with genesis vault");
        Default::default()
    }
    #[inline]
    pub fn new(vaults: Vec<Vault>) -> Self {
        trace!("Vaults were created with {} vaults", vaults.len());
        Self { vaults }
    }
    /// # Panic
    /// Panics if there is no genesis vault.
    #[inline]
    pub fn first(&self) -> &Vault {
        &self.vaults[0]
    }
    /// # Panic
    /// Panics if there is no genesis vault.
    #[inline]
    pub fn last(&self) -> &Vault {
        &self.vaults[self.vaults.len() - 1]
    }
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Vault> {
        self.vaults.iter()
    }
}
