use super::*;

pub trait ToHex: AsRef<[u8]> {
    #[inline]
    fn to_hex(&self) -> String {
        to_hex_string(self.as_ref(), HexMode::Lower)
    }
    #[inline]
    fn to_upper_hex(&self) -> String {
        to_hex_string(self.as_ref(), HexMode::Upper)
    }
    #[inline]
    fn to_hex0x(&self) -> String {
        to_hex_string(self.as_ref(), HexMode::Lower0x)
    }
    #[inline]
    fn to_upper_hex0x(&self) -> String {
        to_hex_string(self.as_ref(), HexMode::Upper0x)
    }
}

// implementations for common types
impl<T: AsRef<[u8]>> ToHex for T {}
