use crate::common::*;

use super::opt::Opt;

/// This is for simple logging.
///
/// * We may want to use a more advanced logger in the future.
#[once(panic)]
pub fn build(opt: &Opt) -> Result<(), log::SetLoggerError> {
    pretty_env_logger::formatted_builder()
        .filter_level(match opt.verbose {
            _ if opt.quiet => log::LevelFilter::Error,
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .try_init()
}
