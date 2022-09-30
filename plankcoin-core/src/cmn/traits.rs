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
