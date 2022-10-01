use super::*;

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
