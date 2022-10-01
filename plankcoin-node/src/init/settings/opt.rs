use super::*;

use clap::{AppSettings, ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = PROGRAM_NAME)]
#[clap(version = NODE_VERSION, author = NODE_AUTHOR)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Opt {
    /// Verbose mode (-v, -vv(default), -vvv, etc.)
    #[clap(short, long, default_value_t = 2, action = ArgAction::Count)]
    pub verbose: u8,

    /// Quiet mode
    #[clap(short, long, action)]
    pub quiet: bool,

    /// Config file path
    #[clap(short, long, default_value = DEFAULT_CONFIG_PATH)]
    pub config: PathBuf,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

impl Opt {
    /// Parse command line arguments
    #[inline]
    pub fn get() -> Opt {
        Opt::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Start node
    Start {
        /// Start as a miner
        #[clap(short, long)]
        miner: bool,

        /// Start as a validator
        #[clap(short, long)]
        validator: bool,

        /// Start as a full node
        #[clap(short, long)]
        full: bool,
    },
}
