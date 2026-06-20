pub fn from_backslash(s: &str) -> String {
    from_escape(s, '\\')
}

pub fn from_escape(s: &str, escape: char) -> String {
    if s.is_empty() {
        return s.to_string();
    }
    let size = s.len();
    let mut result = String::with_capacity(size);

    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == escape {
            if let Some(next) = chars.next() {
                result.push(next);
            } else {
                // found the unmatched escape in the end of the string
            }
        } else {
            result.push(c);
        }
    }
    result
}
