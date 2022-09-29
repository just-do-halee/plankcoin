use super::*;

pub struct Bank {
    vaults: Vec<Vault>,
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
}

impl Default for Bank {
    #[inline]
    fn default() -> Self {
        Self {
            vaults: vec![Vault::new_genesis()],
        }
    }
}

use vault::*;
mod vault;
