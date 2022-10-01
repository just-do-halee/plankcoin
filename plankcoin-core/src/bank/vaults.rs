use super::*;

pub use vault::*;

#[derive(Debug)]
pub struct Vaults {
    vaults: Vec<Vault>,
}
impl Default for Vaults {
    fn default() -> Self {
        Self {
            vaults: vec![Vault::new_genesis()],
        }
    }
}

impl Vaults {
    #[inline]
    pub fn new_with_genesis() -> Self {
        Default::default()
    }
    #[inline]
    pub fn new(vaults: Vec<Vault>) -> Self {
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

mod vault;
