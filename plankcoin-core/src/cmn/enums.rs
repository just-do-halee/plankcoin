use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BankKind {
    /// Syncing entire vaults from other peers.
    Big,
    /// Syncing only the info of vaults from other peers.
    Medium,
    // /// Syncing only the state of vaults from other peers.
    // Small,
}

impl fmt::Display for BankKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
