#![allow(dead_code)]

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HexMode {
    Lower,
    Upper,
    Lower0x,
    Upper0x,
}

impl fmt::Display for HexMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
