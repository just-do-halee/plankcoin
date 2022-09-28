mod config;
mod logger;
mod opt;

use crate::common::*;

pub struct Settings {
    pub from_opt: opt::Opt,
    pub from_config: config::PlankcoinConfig,
}
impl Settings {
    pub fn as_wallet_path(&self) -> &Path {
        &self.from_config.account.wallet
    }
}

#[inline]
#[once(panic)]
pub fn try_build() -> AnyResult<Settings> {
    let opt = opt::parse(); // parse command line arguments

    logger::build(&opt)?; // initialize logger

    info!("Parsing configuration file {}", opt.config.display());
    let config = config::try_parse(&opt)?; // parse config file
    info!("Configuartion file parsed successfully");

    Ok(Settings {
        from_opt: opt,
        from_config: config,
    })
}
