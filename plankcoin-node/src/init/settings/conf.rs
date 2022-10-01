use super::*;

use config::{self, ConfigError, Environment, File};

#[derive(Debug, Deserialize)]
pub struct Account {
    pub wallet: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub account: Account,
}

impl Config {
    /// Parse configuration file (.toml, .yaml, .json, etc.)
    #[inline]
    pub fn try_parse(opt: &Opt) -> Result<Config, ConfigError> {
        config::Config::builder()
            .add_source(File::with_name(&opt.config.to_string_lossy()))
            .add_source(Environment::with_prefix(&UPPERCASE_PROGRAM_NAME))
            .set_override("account.wallet", DEFAULT_WALLET_PATH)?
            .build()?
            .try_deserialize()
    }
}
