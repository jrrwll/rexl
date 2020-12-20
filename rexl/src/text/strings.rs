
/// only a-zA-Z0-9_ is considered, 97-122, 65-90, 48-57, 95
pub fn is_variable_char(c: char) -> bool {
    is_first_variable_char(c) || c >= '0' && c <= '9'
}

pub fn is_first_variable_char(c: char) -> bool {
    c == '_' || (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}