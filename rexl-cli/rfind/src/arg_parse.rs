use crate::*;
use rexl::argparse::*;
use std::process;

impl Main {
    pub fn new(args: Vec<String>, context: &Context) -> Self {
        let parsed = Self::from_args(args);
        let Err(err) = parsed else {
            return parsed.unwrap();
        };

        match err {
            ArgParserError::UnexpectedArg(arg) => {
                eprintln!("{}", context.format("arg.unexpected", vec![arg]));
                process::exit(2);
            }
            ArgParserError::MismatchedKind(value) => {
                let MismatchedKindValue { argument, passed } = value;
                eprintln!(
                    "{}",
                    context.format(
                        "arg.mismatched-kind",
                        vec![argument.key.to_string(), passed.to_string()]
                    )
                );
                process::exit(3);
            }
            ArgParserError::MissingValue(argument) => {
                eprintln!(
                    "{}",
                    context.format("arg.missing-value", vec![argument.key.to_string()])
                );
                process::exit(4);
            }
            ArgParserError::BoolParse(value) => {
                Self::parse_error(value, "arg.bool-parse-error", context);
                process::exit(5);
            }
            ArgParserError::NumberParse(value) => {
                Self::parse_error(value, "arg.number-parse-error", context);
                process::exit(6);
            }
            ArgParserError::ValueParse(value) => {
                Self::parse_error(value, "arg.value-parse-error", context);
                process::exit(7);
            }
            _ => {
                panic!("assert never reach here")
            }
        }
    }

    fn parse_error(value: ParseErrorValue, context_key: &str, context: &Context) {
        let ParseErrorValue { argument, source, error } = value;
        eprintln!("{}", context.format(context_key, vec![source, argument.key.to_string(), error]));
    }
}
