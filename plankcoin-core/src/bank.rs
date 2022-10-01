use super::*;

pub mod sdboxes;
pub mod vaults;

use sdboxes::*;
use vaults::*;

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
        debug!(
            "The '{}' Bank was founded in '{}' by '{}'",
            kind,
            Utc::now(),
            owner.public.to_hex0x(),
        );
        Self {
            owner,
            kind,
            ..Default::default()
        }
    }
}
