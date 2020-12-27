use rexl::cli::*;
use std::process;

use crate::*;
use crate::Options::*;

pub fn new_arg_parser() -> ArgParser<Options> {
    let mut parser = ArgParser::new();
    parser.add_bool(Version, vec!["V", "version"])
        .add_bool(Help,  vec!["h", "help"])
        //.add_bool(show,  vec!["v", "verbose"])
        .add_bool(Verbose,  vec!["v", "verbose"])
        //
        .add_bool(All,  vec!["a", "all"])
        .add_integer(Depth,  vec!["d", "depth"])
        .add_multiple(Kind, vec!["t", "type"])
        .add_multiple(Name, vec!["n", "name"])
        .add_multiple(NamePattern, vec!["N", "name-pattern"])
        .add(Size, vec!["s", "size"])
        // .add(AccessTime, vec!["access-time"])
        // .add(ModifyTime, vec!["modify-time"])
        // .add(ChangeTime, vec!["change-time"])
        .add_multiple(Content, vec!["c", "content"])
        .add_multiple(ContentPattern, vec!["C", "content-pattern"]);
    parser
}

pub fn parse_args(context: &Context, args: Vec<String>) -> Main {
    let mut parser = new_arg_parser();
    match parser.parse(args) {
        Ok(_) => {
            if parser.get_bool(Help) {
                println!("{}", context.usage);
                process::exit(0);
            } else if parser.get_bool(Version) {
                println!("rfind {}", VERSION);
                process::exit(0);
            }
            convert_args(&parser, context)
        }
        Err(err) => {
            match err {
                ArgParserError::NoArgs => {
                    eprintln!("{}", context.format("arg.no-passed", Vec::new()));
                    process::exit(1);
                }
                ArgParserError::UnexpectedArg(arg) => {
                    eprintln!("{}", context.format("arg.unexpected", vec![arg]));
                    process::exit(2);
                }
                ArgParserError::MismatchedKind(value) => {
                    let MismatchedKindValue{argument, passed} = value;
                    eprintln!("{}", context.format(
                        "arg.mismatched-kind", vec![argument.key.to_string(), passed.to_string()]));
                    process::exit(3);
                }
                ArgParserError::MissingValue(argument) => {
                    eprintln!("{}", context.format("arg.missing-value", vec![argument.key.to_string()]));
                    process::exit(4);
                }
                ArgParserError::NumberParse(value) => {
                    let NumberParseValue{argument, source, error} = value;
                    eprintln!("{}", context.format(
                        "arg.number-parse-error", vec![source, argument.key.to_string(), error]));
                    process::exit(5);
                }
                ArgParserError::NoProperties(_) => {
                    panic!("never reach here")
                }
            }
        }
    }
}
