use crate::common::*;

use config::{Config, ConfigError, Environment, File};

use super::opt::Opt;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub wallet: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct PlankcoinConfig {
    pub account: Account,
}

/// Parse configuration file (.toml, .yaml, .json, etc.)
#[inline]
#[once(panic)]
pub fn try_parse(opt: &Opt) -> Result<PlankcoinConfig, ConfigError> {
    Config::builder()
        .add_source(File::with_name(&opt.config.to_string_lossy()))
        .add_source(Environment::with_prefix(&UPPERCASE_PROGRAM_NAME))
        .set_override("account.wallet", DEFAULT_WALLET_PATH)?
        .build()?
        .try_deserialize()
}
