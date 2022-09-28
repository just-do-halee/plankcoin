mod wallet;

use crate::{common::*, init::Settings};

#[inline]
#[once(panic)]
pub fn run(settings: Settings) -> AnyResult<()> {
    let wallet = wallet::read(settings.as_wallet_path())?;

    Ok(())
}
