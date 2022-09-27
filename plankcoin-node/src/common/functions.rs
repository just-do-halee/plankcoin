pub fn to_capitalize(s: &str) -> String {
    match s.len() {
        0 => String::new(),
        1 => s.to_uppercase(),
        _ => s[..1].to_uppercase() + &s[1..],
    }
}
