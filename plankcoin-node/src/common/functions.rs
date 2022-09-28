use crate::common::*;

use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub fn stdin_get_char(mut f: impl FnMut(char) -> bool) -> AnyResult<char> {
    // enable raw mode for getting single char without pressing enter
    debug!("Enabling raw mode");
    enable_raw_mode()?;

    // read a single char until the key code is not a char
    loop {
        debug!("Reading a single char\r");
        if let Event::Key(key) = read()? {
            if let KeyCode::Char(ch) = key.code {
                // filter the char
                debug!("Filtering a char: {}\r", ch);
                if f(ch) {
                    // disable raw mode
                    debug!("Disabling raw mode\r");
                    disable_raw_mode()?;
                    return Ok(ch);
                }
            } else {
                debug!("Key code: {:?}\r", key.code);
            }
        }
    }
}

pub fn is_y_or_n(c: char) -> bool {
    c == 'y' || c == 'n'
}

pub fn to_capitalize(s: &str) -> String {
    match s.len() {
        0 => String::new(),
        1 => s.to_uppercase(),
        _ => s[..1].to_uppercase() + &s[1..],
    }
}
