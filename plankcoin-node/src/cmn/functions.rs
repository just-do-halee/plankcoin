use crate::cmn::*;

pub fn term_read_password(
    term: &Term,
    mut f: impl FnMut(&Term, &str) -> io::Result<bool>,
) -> io::Result<Vec<u8>> {
    loop {
        trace!("Reading password from terminal securely");
        let s = term.read_secure_line()?;
        trace!("Filtering password");
        if f(term, &s)? {
            return Ok(s.into_bytes());
        }
    }
}

pub fn term_get_char(term: &Term, mut f: impl FnMut(char) -> bool) -> io::Result<char> {
    loop {
        trace!("Reading a single char");
        let ch = term.read_char()?;
        trace!("Filtering a char: {}", ch);
        if f(ch) {
            return Ok(ch);
        }
    }
}

#[inline]
pub fn is_y_or_n(c: char) -> bool {
    c == 'y' || c == 'n'
}

#[inline]
pub fn to_capitalize(s: &str) -> String {
    match s.len() {
        0 => String::new(),
        1 => s.to_uppercase(),
        _ => s[..1].to_uppercase() + &s[1..],
    }
}

// --------------------------------------------------

#[inline]
pub fn to_hex_string(s: &[u8], mode: HexMode) -> String {
    use fmt::Write;
    trace!("Converting bytes to hex string, mode: {}", mode);
    let mut hex = String::with_capacity(s.len() * 2);

    if mode == HexMode::Lower0x || mode == HexMode::Upper0x {
        hex.write_str("0x").unwrap();
    }

    for byte in s {
        match mode {
            HexMode::Lower | HexMode::Lower0x => write!(&mut hex, "{:02x}", byte).unwrap(),
            HexMode::Upper | HexMode::Upper0x => write!(&mut hex, "{:02X}", byte).unwrap(),
        }
    }
    trace!("Hex string: {}", hex);
    hex
}
