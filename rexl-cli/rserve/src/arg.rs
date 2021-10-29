use rexl::cli::{ArgParser, ArgParserError};
use std::process;

#[derive(Debug)]
pub struct Config {
    pub bind_address: String,
    pub base_path:    String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum ArgType {
    BindAddress,
    BasePath,
}

impl Config {
    pub fn from_args(args: Vec<String>) -> Config {
        let mut bind_address = "0.0.0.0:5000".to_string();
        let mut base_path = "/**".to_string();
        let mut parser = Self::arg_parser();
        match parser.parse(args) {
            Ok(_) => {
                if let Some(v) = parser.get_string(ArgType::BindAddress) {
                    bind_address = v.clone();
                }
                if let Some(v) = parser.get_string(ArgType::BasePath) {
                    base_path = v.clone();
                }
            }
            Err(err) => {
                if err != ArgParserError::NoArgs {
                    eprintln!("{}", err);
                    process::exit(1);
                }
            }
        }
        Config {
            bind_address,
            base_path,
        }
    }

    fn arg_parser() -> ArgParser<ArgType> {
        let mut parser = ArgParser::new();
        parser
            .add(ArgType::BindAddress, vec!["a", "address", "bind-address"])
            .add(ArgType::BasePath, vec!["p", "path", "base-path"]);
        parser
    }
}
