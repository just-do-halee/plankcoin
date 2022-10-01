#![allow(dead_code)]

#[derive(PartialEq, Eq)]
pub enum WriteMode {
    CreateNew,
    Overwrite,
}
