use std::collections::HashMap;
use std::fmt::Write;

/// only 0-9 is considered
#[inline]
pub fn is_number_char(c: char) -> bool {
    c >= '0' && c <= '9'
}

/// only a-zA-Z0-9_ is considered, 97-122, 65-90, 48-57, 95
#[inline]
pub fn is_variable_char(c: char) -> bool {
    is_first_variable_char(c) || is_number_char(c)
}

#[inline]
pub fn is_first_variable_char(c: char) -> bool {
    c == '_' || (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}

pub fn to_snake(s: &str) -> String {
    let mut buf = String::with_capacity(s.len() + 4);
    let mut prev = '_';
    for ch in s.chars() {
        if ch.is_uppercase() && prev != '_' {
            buf.push('_');
        }
        buf.push(ch.to_ascii_lowercase());
        prev = ch;
    }
    buf.split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

const UNITS: &[u8] = b"BKMGT";

pub fn to_size_str(n: usize) -> String {
    let mut buf = String::new();
    if n == 0 {
        buf.push_str("0B");
        return buf;
    }

    let mut value = n as f64;
    let mut exp = 0;
    while value >= 1024.0 && exp < 4 {
        value /= 1024.0;
        exp += 1;
    }

    let value = (value * 10.0).round() / 10.0;
    let int_part = value as u64;
    let frac = (value * 10.0) as u64 % 10;

    write!(&mut buf, ".{}", int_part).unwrap();
    if frac != 0 {
        write!(&mut buf, ".{}", frac).unwrap();
    }
    buf.push(UNITS[exp] as char);
    buf
}

pub fn translate(s: &str, tr_map: &HashMap<char, char>) -> String {
    s.chars()
        .map(|c| *tr_map.get(&c).unwrap_or(&c))
        .collect()
}

pub fn translate_ascii(s: &str, table: &[u8; 128]) -> String {
    s.bytes()
        .map(|b| if b < 128 { table[b as usize] } else { b })
        .map(|b| b as char)
        .collect()
}
