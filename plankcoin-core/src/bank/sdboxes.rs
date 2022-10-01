use super::*;

pub mod sdbox;

use sdbox::Sdbox;

#[derive(Debug, Default)]
pub struct Sdboxes {
    boxes: Vec<Sdbox>,
}

impl Sdboxes {
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
    pub fn to_mkr_hash(&self) -> Hash {
        Hash::default()
    }
}
