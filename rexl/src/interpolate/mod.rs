pub use self::brace_interpolate::*;
pub use self::dollar_interpolate::*;

mod brace_interpolate;
mod dollar_interpolate;

#[derive(Debug, PartialEq, Clone)]
pub enum InterpolationError {
    InvalidChar(InvalidCharValue),
    MissingVariable(String),
    MissingPositionalVariable(usize),
    NumberParse(NumberParseValue),
    Unexpected(UnexpectedValue),
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvalidCharValue {
    pub found: char,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumberParseValue {
    pub offset: usize,
    pub source: String,
    pub error: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnexpectedValue {
    pub template: String,
    pub offset: usize,
    pub code: String,
}

/// private functions
#[inline]
fn invalid_char_err(found: char, offset: usize)
                    -> InterpolationError {
    InterpolationError::InvalidChar(InvalidCharValue { found, offset })
}

#[inline]
fn number_parse_err(offset: usize, source: String, error: String)
                    -> InterpolationError {
    InterpolationError::NumberParse(NumberParseValue { offset, source, error })
}

#[inline]
fn unexpected_err<S1, S2>(s: S1, offset: usize, code: S2) -> InterpolationError
    where S1: ToString, S2: ToString {
    InterpolationError::Unexpected(UnexpectedValue {
        template: s.to_string(), offset, code: code.to_string()  })
}

// 1/8 UPPER_SIZE 1/4 LOWER_SIZE 1/2
const UPPER_SIZE: usize = 65536;
const LOWER_SIZE: usize = 256;

#[inline]
fn size_grow_up(n: usize) -> usize {
    if n > UPPER_SIZE {
        n + n >> 2
    } else if n > LOWER_SIZE {
        n + n >> 1
    } else {
        n + n
    }
}

#[inline]
fn add_default_value(result: &mut String, default_value: Option<&str>, variable: String)
    -> Result<(), InterpolationError> {
    if let Some(some_default_value) = default_value {
        result.push_str(some_default_value);
        Ok(())
    } else {
        Err(InterpolationError::MissingVariable(variable))
    }
}

#[inline]
fn add_default_value_positional(result: &mut String, default_value: Option<&str>, index: usize)
                     -> Result<(), InterpolationError> {
    if let Some(some_default_value) = default_value {
        result.push_str(some_default_value);
        Ok(())
    } else {
        Err(InterpolationError::MissingPositionalVariable(index))
    }
}