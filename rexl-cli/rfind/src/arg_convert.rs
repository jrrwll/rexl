use crate::Options::*;
use crate::*;
use regex::Regex;
use rexl::cli::ArgParser;
use std::process;
use std::str::FromStr;

pub fn convert_args<'a>(parser: &ArgParser<Options>, context: &'a Context) -> Main<'a> {
    let verbose = parser.get_bool(Verbose);

    let depth = parser.get_integer_or_else(Depth, || 255) as usize;

    let kind = parse_kind(parser.get_strings_or_empty(Kind)).unwrap_or_else(|err| {
        eprintln!("{}", context.format("type.unknown", vec![err.to_string()]));
        process::exit(6)
    });

    let name = parser.get_strings_or_empty(Name);

    let name_pattern = parser
        .get_strings_or_empty(NamePattern)
        .iter()
        .map(|s| match Regex::from_str(s) {
            Ok(v) => v,
            Err(err) => {
                eprintln!(
                    "{}",
                    context.format(
                        "name-pattern.unknown-format",
                        vec![s.to_string(), err.to_string()]
                    )
                );
                process::exit(7);
            }
        })
        .collect();

    let size = if let Some(s) = parser.get_string(Size) {
        SizeOption::parse(s)
            .iter()
            .map(|opt| {
                if let Some(su) = opt {
                    *su
                } else {
                    eprintln!(
                        "{}",
                        context.format("size.unknown-format", vec![s.to_string()])
                    );
                    process::exit(8);
                }
            })
            .collect::<Vec<(SizeOption, u64)>>()
    } else {
        Vec::new()
    };

    let content = parser.get_strings_or_empty(Content);

    let content_pattern = parser
        .get_strings_or_empty(ContentPattern)
        .iter()
        .map(|s| match Regex::from_str(s) {
            Ok(v) => v,
            Err(err) => {
                eprintln!(
                    "{}",
                    context.format(
                        "content-pattern.unknown-format",
                        vec![s.to_string(), err.to_string()]
                    )
                );
                process::exit(9);
            }
        })
        .collect();

    let path = parser.get_extra_values().clone();
    if path.is_empty() {
        eprintln!("{}", context.format("path.empty", Vec::new()));
        process::exit(10);
    }

    Main {
        context,
        verbose,
        depth,
        kind,
        name,
        name_pattern,
        size,
        content,
        content_pattern,
        path,
    }
}
