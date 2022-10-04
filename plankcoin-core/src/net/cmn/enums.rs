use super::*;

bitflags! {
    pub struct ProtocolVersion: u32 {
        #[allow(clippy::unusual_byte_groupings)]
        /// Major _ Minor _ Patch
        const V1_0_0 = 0b00000001_0000000000000000_00000000;
    }
}

impl ProtocolVersion {
    pub const SIZE: usize = u32::BITS as usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// [Network::Mainnet] is the main network. It is used for real transactions.
    Mainnet,
    /// [Network::Testnet] is the test network. It is used for testing network features.
    Testnet,
    /// [Network::Devnet] is the development network. It is used for development and testing.
    Devnet,
}
impl fmt::Display for Network {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<Network> for Magic {
    #[inline]
    fn from(network: Network) -> Self {
        match network {
            Network::Mainnet => Magic::MAINNET,
            Network::Testnet => Magic::TESTNET,
            Network::Devnet => Magic::DEVNET,
        }
    }
}

bitflags! {
    /// This is a bitflag that represents the capabilities of a node.
    pub struct ServiceFlags: u64 {
        /// [`ServiceFlags::NONE`] means the node does not provide any service.\
        /// That means the node can only be used for bank synchronization.
        const NONE = 0;

        /// [`ServiceFlags::STATE`] means the node is capable of serving the state only.
        const REQ_STATE = 1 << 1;
        /// [`ServiceFlags::STATE`] means the node is capable of serving the state only.
        const RES_STATE = 1 << 2;
        /// [`ServiceFlags::VAULTINFO`] means the node is capable of serving the vault info only.
        const REQ_VAULTINFO = 1 << 3;
        /// [`ServiceFlags::VAULTINFO`] means the node is capable of serving the vault info only.
        const RES_VAULTINFO = 1 << 4;
        /// [`ServiceFlags::SDBOX`] means the node is capable of serving the safe deposit boxes only.
        const REQ_SDBOX = 1 << 5;
        /// [`ServiceFlags::SDBOX`] means the node is capable of serving the safe deposit boxes only.
        const RES_SDBOX = 1 << 6;

        /// [`ServiceFlags::REQ_SMALL_BANK`] means the node is capable of serving the small bank.\
        /// That means the node that has the state can only be set to this flag.
        const REQ_SMALL_BANK = Self::REQ_STATE.bits;
        /// [`ServiceFlags::RES_SMALL_BANK`] means the node is capable of serving the small bank.\
        /// That means the node that has the state can only be set to this flag.
        const RES_SMALL_BANK = Self::RES_STATE.bits;
        /// [`ServiceFlags::REQ_MEDIUM_BANK`] means the node is capable of serving the medium bank.\
        /// That means the node that has the state and vault info can only be set to this flag.
        const REQ_MEDIUM_BANK = Self::REQ_SMALL_BANK.bits | Self::REQ_VAULTINFO.bits;
        /// [`ServiceFlags::RES_MEDIUM_BANK`] means the node is capable of serving the medium bank.\
        /// That means the node that has the state and vault info can only be set to this flag.
        const RES_MEDIUM_BANK = Self::RES_SMALL_BANK.bits | Self::RES_VAULTINFO.bits;
        /// [`ServiceFlags::REQ_BIG_BANK`] means the node is capable of serving the big bank.\
        /// That means the node that has the big bank(state, vault info, sd boxes) only can be set to this flag.
        const REQ_BIG_BANK = Self::REQ_MEDIUM_BANK.bits | Self::REQ_SDBOX.bits;
        /// [`ServiceFlags::RES_BIG_BANK`] means the node is capable of serving the big bank.\
        /// That means the node that has the big bank(state, vault info, sd boxes) only can be set to this flag.
        const RES_BIG_BANK = Self::RES_MEDIUM_BANK.bits | Self::RES_SDBOX.bits;
    }
}

impl ServiceFlags {
    pub const SIZE: usize = u64::BITS as usize;

    #[inline]
    pub fn from_u64(n: u64) -> Self {
        ServiceFlags::from_bits_truncate(n)
    }
}
