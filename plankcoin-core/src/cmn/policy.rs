/// Plankcoin magic bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Magic([u8; 4]);

impl Magic {
    pub const SIZE: usize = 4;

    pub const MAINNET: Self = Self([0x27, 0x7E, 0x94, 0xA7]); // Planck constant
    pub const TESTNET: Self = Self([0x11, 0xDE, 0x78, 0x4A]); // Speed of light
    pub const DEVNET: Self = Self([0x52, 0x4B, 0x0D, 0xA0]); // Boltzmann constant

    pub fn from_bytes(bytes: [u8; 4]) -> Magic {
        Magic(bytes)
    }

    pub fn to_bytes(self) -> [u8; 4] {
        self.0
    }
}
