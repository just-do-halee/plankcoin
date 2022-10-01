use super::*;

mod wallet;

use wallet::*;

#[inline]
#[once(panic)]
pub fn run(settings: init::Settings) -> AnyResult<()> {
    let term = Term::stdout();

    let wallet = Wallet::read_from_terminal(&term, settings.as_wallet_path())?;

    Ok(())
}
