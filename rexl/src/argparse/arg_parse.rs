use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use crate::argparse::{ArgParserError, Argument};

/// I will not give the implement about CLI i18n since I believe less is more
#[derive(Debug, Default)]
pub struct ArgParser {
    // multi argparse-names to one key
    pub(crate) names_key_map: HashMap<String, String>,
    // key, argument
    pub(crate) key_argument_map: HashMap<String, Argument>,
    // key, value
    pub(crate) string_map: HashMap<String, String>,
    pub(crate) bool_map: HashMap<String, bool>,
    pub(crate) integer_map: HashMap<String, i64>,
    pub(crate) float_map: HashMap<String, f64>,
    // multiple values
    pub(crate) strings_map: HashMap<String, Vec<String>>,
    pub(crate) integers_map: HashMap<String, Vec<i64>>,
    pub(crate) floats_map: HashMap<String, Vec<f64>>,
    pub(crate) properties_map: HashMap<String, HashMap<String, String>>,
    // position values
    pub(crate) position_values: Vec<String>,
}

impl Display for ArgParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let arguments_str = self
            .key_argument_map
            .values()
            .cloned()
            .map(|arg| format!("  {}", arg.to_string()))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "ArgParser{{\n{}\n}}", arguments_str)
    }
}

pub trait ArgParserRunnable {
    fn run(self); // be owned
}

pub trait FromArgs: Sized {
    type Output;

    fn from_args(args: Vec<String>) -> Result<Self::Output, ArgParserError>;
}

pub trait RunWithArgs: Sized {
    fn run_with_args(args: Vec<String>) -> Result<(), ArgParserError>;
}
