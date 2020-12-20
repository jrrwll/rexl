use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter, Debug};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ArgumentKind {
    String,
    Bool,
    Integer,
    Float,
    Property,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Argument {
    pub names: Vec<String>,
    pub kind: ArgumentKind,
    pub multiple: bool,
}

impl Argument {

    pub fn check_kind(&self, expect: ArgumentKind) -> Result<(), ArgParserError> {
        let got = self.kind;
        if got != expect {
            Err(ArgParserError::MismatchedKind(MismatchedKindValue{
                argument: self.clone(), expect
            }))
        } else {
            Ok(())
        }
    }

    pub fn parse_i64(&self, value: String) -> Result<i64, ArgParserError> {
        value.parse::<i64>().or_else(|e| {
            Err(ArgParserError::NumberParse(NumberParseValue{
                argument: self.clone(),
                source: value,
                error: e.to_string()
            }))
        })
    }

    pub fn parse_f64(&self, value: String) -> Result<f64, ArgParserError> {
        value.parse::<f64>().or_else(|e| {
            Err(ArgParserError::NumberParse(NumberParseValue{
                argument: self.clone(),
                source: value,
                error: e.to_string()
            }))
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum ArgParserError {
    NoArgs,
    UnexpectedArg(String),
    // argument, expect
    MismatchedKind(MismatchedKindValue),
    MissingValue(Argument),
    NumberParse(NumberParseValue),
    NoProperties(Argument),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MismatchedKindValue {
    pub argument: Argument,
    pub expect: ArgumentKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumberParseValue {
    pub argument: Argument,
    pub source: String,
    pub error: String,
}

impl Display for ArgParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ArgParserError {}
