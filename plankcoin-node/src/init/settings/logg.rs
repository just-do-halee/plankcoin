use super::*;

pub struct Logger;

impl Logger {
    /// This is for simple logging to stderr
    ///
    /// * We may want to use a more advanced logger in the future.
    #[inline]
    pub fn build(opt: &Opt) -> Result<(), log::SetLoggerError> {
        if opt.quiet {
            // no logging
            return Ok(());
        }
        pretty_env_logger::formatted_builder()
            .filter_level(match opt.verbose {
                1 => log::LevelFilter::Warn,
                2 => log::LevelFilter::Info,
                3 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            })
            .try_init()
    }
}
