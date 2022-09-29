mod wallet;

use crate::{cmn::*, init::Settings};

#[inline]
#[once(panic)]
pub fn run(settings: Settings) -> AnyResult<()> {
    let term = Term::stdout();

    let wallet = wallet::read(&term, settings.as_wallet_path())?;

    Ok(())
}
