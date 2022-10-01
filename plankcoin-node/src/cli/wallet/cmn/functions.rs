use super::*;

pub fn term_read_wallet_passphrase(mut term: &Term) -> io::Result<[u8; AES_KEY_SIZE]> {
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
