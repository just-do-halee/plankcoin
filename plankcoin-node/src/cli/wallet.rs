use super::*;

pub use structs::Wallet;

#[inline]
pub fn read(mut term: &Term, path: &Path) -> AnyResult<Wallet> {
    info!("Finding wallet...");

    let path_display = path.display();

    // check if wallet exists
    if !path.exists() {
        writeln!(
            term,
            "The wallet file {} not found, do you want to create it? [y/n]",
            path_display
        )?;

        match term_get_char(term, is_y_or_n)? {
            'y' => {
                writeln!(term, "Creating wallet file {}", path_display)?;

                let passphrase = term_read_wallet_passphrase(term)?;

                Wallet::new(path).try_write(passphrase, WriteMode::CreateNew)?;

                info!("Wallet file created successfully");
            }
            'n' => {
                log_bail!("The root wallet does not exist");
            }
            _ => unreachable!(),
        }
    }
    info!("Wallet file {} found", path_display);

    let passphrase = term_read_wallet_passphrase(term)?;

    info!("Reading wallet file...");
    let wallet = Wallet::try_read(path, passphrase)?;

    Ok(wallet)
}

fn term_read_wallet_passphrase(mut term: &Term) -> io::Result<[u8; AES_KEY_SIZE]> {
    const PLEASE_ENTER_AGAIN: &str = "please enter again:\n🔒";

    write!(term, "Please enter your new passphrase:\n🔒")?;
    let input = term_read_password(term, |mut term, s| {
        if s.find(' ').is_some() {
            write!(term, "Passphrase has spaces, {PLEASE_ENTER_AGAIN}")?;
            return Ok(false);
        }

        let len = s.len();
        if len < WALLET_PASSPHRASE_MIN_SIZE {
            write!(term, "Passphrase is too short, {PLEASE_ENTER_AGAIN}")?;
            Ok(false)
        } else if len > AES_KEY_SIZE {
            write!(term, "Passphrase is too long, {PLEASE_ENTER_AGAIN}")?;
            Ok(false)
        } else {
            Ok(true)
        }
    })?;

    let mut passphrase = [0; AES_KEY_SIZE];
    passphrase[..input.len()].copy_from_slice(&input);

    Ok(passphrase)
}

use constants::*;
use enums::*;
use errors::*;
use libraries::*;

mod constants;
mod enums;
mod errors;
mod libraries;
mod structs;
