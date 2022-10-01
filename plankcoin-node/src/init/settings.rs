use super::*;

mod conf;
mod logg;
mod opt;

use conf::*;
use logg::*;
use opt::*;

pub struct Settings {
    pub from_option: Opt,
    pub from_config: Config,
}

impl Settings {
    #[inline]
    pub fn as_wallet_path(&self) -> &Path {
        &self.from_config.account.wallet
    }
}

#[inline]
#[once(panic)]
pub fn try_build() -> AnyResult<Settings> {
    // parse command line arguments
    let option = Opt::get();

    // initialize logger
    Logger::build(&option)?;

    info!("Parsing configuration file {}", option.config.display());

    let config = Config::try_parse(&option)?; // parse config file

    info!("Configuration file parsed successfully");

    Ok(Settings {
        from_option: option,
        from_config: config,
    })
}
