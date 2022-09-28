mod cli;
mod cmn;
mod init;

// --------------------

use crate::cmn::*;

fn main() -> AnyResult<()> {
    let settings = init::try_build()?;

    info!("--------------------------");
    info!("Starting {}...", to_capitalize(NODE_NAME));
    info!("Node Version: {NODE_VERSION}");
    info!("--------------------------");

    cli::run(settings)
}
