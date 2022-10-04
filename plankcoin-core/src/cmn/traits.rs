use super::*;

pub trait ToBytes<const N: usize> {
    fn to_bytes(&self) -> [u8; N];
}

impl<const N: usize> ToBytes<N> for Uint {
    #[inline]
    fn to_bytes(&self) -> [u8; N] {
        let mut bytes = [0u8; N];
        bytes.iter_mut().enumerate().for_each(|(i, byte)| {
            *byte = self.byte(N - 1 - i);
        });
        bytes
    }
}

impl<T: AsRef<[u8]>> ToHex for T {}

pub trait ToHash {
    fn to_hash<T: Digest>(&self, hasher: T) -> T;
}

pub trait ToHex: AsRef<[u8]> {
    #[inline]
    fn to_hex(&self) -> String {
        self.to_hex_with_mode(HexMode::Lower)
    }
    #[inline]
    fn to_upper_hex(&self) -> String {
        self.to_hex_with_mode(HexMode::Upper)
    }
    #[inline]
    fn to_hex0x(&self) -> String {
        self.to_hex_with_mode(HexMode::Lower0x)
    }
    #[inline]
    fn to_upper_hex0x(&self) -> String {
        self.to_hex_with_mode(HexMode::Upper0x)
    }
    #[inline]
    fn to_hex_with_mode(&self, mode: HexMode) -> String {
        use fmt::Write;

        trace!("Converting bytes to hex string, mode: {}", mode);
        let mut hex = String::with_capacity(self.as_ref().len() * 2);

        if mode == HexMode::Lower0x || mode == HexMode::Upper0x {
            hex.write_str("0x").unwrap();
        }

        for byte in self.as_ref() {
            match mode {
                HexMode::Lower | HexMode::Lower0x => write!(&mut hex, "{:02x}", byte).unwrap(),
                HexMode::Upper | HexMode::Upper0x => write!(&mut hex, "{:02X}", byte).unwrap(),
            }
        }
        trace!("Hex string: {}", hex);
        hex
    }
}

// implementations for common types
impl ToBytes<4> for u32 {
    #[inline]
    fn to_bytes(&self) -> [u8; 4] {
        self.to_be_bytes()
    }
}
impl ToBytes<4> for i32 {
    #[inline]
    fn to_bytes(&self) -> [u8; 4] {
        self.to_be_bytes()
    }
}
impl ToBytes<8> for u64 {
    #[inline]
    fn to_bytes(&self) -> [u8; 8] {
        self.to_be_bytes()
    }
}
impl ToBytes<8> for i64 {
    #[inline]
    fn to_bytes(&self) -> [u8; 8] {
        self.to_be_bytes()
    }
}
impl ToBytes<18> for SocketAddr {
    #[inline]
    fn to_bytes(&self) -> [u8; 18] {
        let mut bytes = [0u8; 18];
        bytes[16..].copy_from_slice(&self.port().to_be_bytes());
        match self {
            SocketAddr::V4(addr) => {
                bytes[..4].copy_from_slice(&addr.ip().octets());
            }
            SocketAddr::V6(addr) => {
                bytes[..16].copy_from_slice(&addr.ip().octets());
            }
        }
        bytes
    }
}
