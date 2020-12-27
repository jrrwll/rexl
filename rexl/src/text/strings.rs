
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