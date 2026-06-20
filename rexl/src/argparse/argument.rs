use std::fmt;
use std::fmt::{Debug, Display, Formatter};

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

#[derive(PartialEq, Clone)]
pub struct Argument {
    pub key: String,
    pub names: Vec<String>,
    pub kind: ArgumentKind,
    pub multiple: bool,
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let key_str = format!("{:?}", self.key);
        let names_str = self.arg_names();
        let mut kind_str = self.kind.to_string();
        if self.multiple {
            kind_str = format!("{}[]", kind_str)
        }
        write!(f, "Argument<{}>({}: {})", kind_str, key_str.trim_matches('"'), names_str)
    }
}

impl Debug for Argument {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Argument {
    pub fn arg_names(&self) -> String {
        self.names
            .iter()
            .map(|name| {
                let prefix = if name.len() == 1 { "-" } else { "--" };
                format!("{}{}", prefix, name)
            })
            .collect::<Vec<String>>()
            .join("|")
    }

    pub fn check_kind(&self, passed: ArgumentKind) -> Result<(), ArgParserError> {
        let expect = self.kind;
        if expect != passed {
            Err(ArgParserError::MismatchedKind(MismatchedKindValue {
                argument: self.clone(),
                passed,
            }))
        } else {
            Ok(())
        }
    }

    pub fn parse_bool(&self, value: String) -> Result<bool, ArgParserError> {
        value.parse::<bool>().or_else(|e| {
            Err(ArgParserError::BoolParse(ParseErrorValue {
                argument: self.clone(),
                source: value,
                error: e.to_string(),
            }))
        })
    }

    pub fn parse_i64(&self, value: String) -> Result<i64, ArgParserError> {
        value.parse::<i64>().or_else(|e| {
            Err(ArgParserError::NumberParse(ParseErrorValue {
                argument: self.clone(),
                source: value,
                error: e.to_string(),
            }))
        })
    }

    pub fn parse_f64(&self, value: String) -> Result<f64, ArgParserError> {
        value.parse::<f64>().or_else(|e| {
            Err(ArgParserError::NumberParse(ParseErrorValue {
                argument: self.clone(),
                source: value,
                error: e.to_string(),
            }))
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum ArgParserError {
    // parse stage
    UnexpectedArg(String),
    MismatchedKind(MismatchedKindValue),
    MissingValue(Argument),
    BoolParse(ParseErrorValue),
    NumberParse(ParseErrorValue),
    ValueParse(ParseErrorValue),
    NotPropertiesPassed(Argument),
    // get stage
    NoArgPassed(Argument),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MismatchedKindValue {
    pub argument: Argument,
    pub passed: ArgumentKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseErrorValue {
    pub argument: Argument,
    pub source: String,
    pub error: String,
}

impl Display for ArgParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ArgParserError::UnexpectedArg(name) => {
                format!("unexpected argument `{}`", name)
            }
            ArgParserError::MismatchedKind(kind) => {
                let argument = &kind.argument;
                format!(
                    "mismatched kind for {} since expect {} but got {}",
                    argument.arg_names(),
                    argument.kind,
                    kind.passed
                )
            }
            ArgParserError::MissingValue(argument) => {
                format!("missing value for {}", argument.arg_names())
            }
            ArgParserError::BoolParse(e) => {
                format!(
                    "failed to parse bool arg {} for value `{}`, got error `{}`",
                    e.argument.arg_names(),
                    e.source,
                    e.error
                )
            }
            ArgParserError::NumberParse(e) => {
                format!(
                    "failed to parse number arg {} for value `{}`, got error `{}`",
                    e.argument.arg_names(),
                    e.source,
                    e.error
                )
            }
            ArgParserError::ValueParse(e) => {
                format!(
                    "failed to parse arg {} for value `{}`, got error `{}`",
                    e.argument.arg_names(),
                    e.source,
                    e.error
                )
            }
            ArgParserError::NotPropertiesPassed(argument) => {
                format!("invalid arg value passed for {}", argument.arg_names())
            }
            ArgParserError::NoArgPassed(argument) => {
                format!("required arg {} is missing", argument.arg_names())
            }
        };
        write!(f, "{}", msg)
    }
}
