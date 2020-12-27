use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter, Debug};
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ArgumentKind {
    String,
    Bool,
    Integer,
    Float,
    Property,
}

impl Display for ArgumentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Argument<K: Hash + Eq + Debug + Clone> {
    pub key: K,
    pub names: Vec<String>,
    pub kind: ArgumentKind,
    pub multiple: bool,
}

impl<K: Hash + Eq + Debug + Clone> Argument<K> {

    pub fn check_kind(&self, passed: ArgumentKind) -> Result<(), ArgParserError<K>> {
        let expect = self.kind;
        if expect != passed {
            Err(ArgParserError::MismatchedKind(MismatchedKindValue{
                argument: self.clone(), passed
            }))
        } else {
            Ok(())
        }
    }

    pub fn parse_bool(&self, value: String) -> Result<bool, ArgParserError<K>> {
        value.parse::<bool>().or_else(|e| {
            Err(ArgParserError::NumberParse(NumberParseValue{
                argument: self.clone(),
                source: value,
                error: e.to_string()
            }))
        })
    }

    pub fn parse_i64(&self, value: String) -> Result<i64, ArgParserError<K>> {
        value.parse::<i64>().or_else(|e| {
            Err(ArgParserError::NumberParse(NumberParseValue{
                argument: self.clone(),
                source: value,
                error: e.to_string()
            }))
        })
    }

    pub fn parse_f64(&self, value: String) -> Result<f64, ArgParserError<K>> {
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
pub enum ArgParserError<K: Hash + Eq + Debug + Clone> {
    NoArgs,
    UnexpectedArg(String),
    // argument, expect
    MismatchedKind(MismatchedKindValue<K>),
    MissingValue(Argument<K>),
    NumberParse(NumberParseValue<K>),
    NoProperties(Argument<K>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MismatchedKindValue<K: Hash + Eq + Debug + Clone> {
    pub argument: Argument<K>,
    pub passed: ArgumentKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumberParseValue<K: Hash + Eq + Debug + Clone> {
    pub argument: Argument<K>,
    pub source: String,
    pub error: String,
}

impl<K: Hash + Eq + Debug + Clone> Display for ArgParserError<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<K: Hash + Eq + Debug + Clone> Error for ArgParserError<K> {}
