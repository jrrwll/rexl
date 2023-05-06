#[derive(Debug, PartialEq, Clone)]
pub enum InterpolationError {
    InvalidChar(InvalidCharValue),
    InvalidString(InvalidStringValue),
    MissingVariable(String),
    MissingPositionalVariable(usize),
    NumberParse(NumberParseValue),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvalidCharValue {
    pub found: char,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvalidStringValue {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumberParseValue {
    pub offset: usize,
    pub source: String,
    pub error: String,
}

// private functions

#[inline]
pub(crate) fn invalid_char_err(found: char, offset: usize) -> InterpolationError {
    InterpolationError::InvalidChar(InvalidCharValue { found, offset })
}

#[inline]
pub(crate) fn invalid_string_err(start: usize, end: usize) -> InterpolationError {
    InterpolationError::InvalidString(InvalidStringValue { start, end })
}

#[inline]
pub(crate) fn number_parse_err(offset: usize, source: String, error: String) -> InterpolationError {
    InterpolationError::NumberParse(NumberParseValue {
        offset,
        source,
        error,
    })
}

// 1/8 UPPER_SIZE 1/4 LOWER_SIZE 1/2
const UPPER_SIZE: usize = 65536;
const LOWER_SIZE: usize = 256;

#[inline]
pub(crate) fn size_grow_up(n: usize) -> usize {
    if n > UPPER_SIZE {
        n + n >> 2
    } else if n > LOWER_SIZE {
        n + n >> 1
    } else {
        n + n
    }
}

#[inline]
pub(crate) fn add_default_value(
    result: &mut String, default_value: Option<&str>, variable: String,
) -> Result<(), InterpolationError> {
    if let Some(some_default_value) = default_value {
        result.push_str(some_default_value);
        Ok(())
    } else {
        Err(InterpolationError::MissingVariable(variable))
    }
}

#[inline]
pub(crate) fn add_default_value_positional(
    result: &mut String, default_value: Option<&str>, index: usize,
) -> Result<(), InterpolationError> {
    if let Some(some_default_value) = default_value {
        result.push_str(some_default_value);
        Ok(())
    } else {
        Err(InterpolationError::MissingPositionalVariable(index))
    }
}
