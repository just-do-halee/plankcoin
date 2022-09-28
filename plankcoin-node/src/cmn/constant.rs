use lazy_static::lazy_static;

pub const PROGRAM_NAME: &str = "Plankcoin Node";

pub const NODE_NAME: &str = env!("CARGO_PKG_NAME");
pub const NODE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NODE_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

lazy_static! {
    pub static ref UPPERCASE_NODE_NAME: String = NODE_NAME.to_uppercase().replace('-', "_");
    pub static ref UPPERCASE_PROGRAM_NAME: String = PROGRAM_NAME.to_uppercase().replace('-', "_");
}

pub const DEFAULT_WALLET_PATH: &str = "./plank.wallet";
pub const DEFAULT_CONFIG_PATH: &str = "./Plankcoin.main.toml";
