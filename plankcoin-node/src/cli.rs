mod wallet;

use crate::{common::*, init::Settings};

pub fn run(settings: Settings) -> AnyResult<()> {
    let wallet = wallet::read(settings.as_wallet_path())?;

    Ok(())
}
