use rexl::argparse::{ArgParserError, FromArgs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[path = "../../rexl-cli/rfind/src/util/mod.rs"]
mod common;

use common::RegexStr;

#[derive(Debug, Serialize, Deserialize, FromArgs)]
#[arg_parser(first_char)]
struct App {
    // value
    user: String,
    password: Option<String>,
    version: bool,
    following: Option<bool>,
    #[arg_parser(name = "")]
    num_u8: Option<u8>,
    #[arg_parser(name = "")]
    num_i16: i16,
    #[arg_parser(name = "")]
    num_i64: i64,
    #[arg_parser(name = "")]
    num_isize: isize,
    #[arg_parser(name = "")]
    num_usize: Option<usize>,
    #[arg_parser(name = "")]
    num_f32: f32,
    #[arg_parser(name = "")]
    num_f64: Option<f64>,

    // multiple
    #[arg_parser(name = "F,fg")]
    flag: Vec<String>,
    #[arg_parser(name = "")]
    flag_i128: Vec<i128>,
    #[arg_parser(name = "")]
    flag_f64: Vec<f64>,

    // properties
    #[arg_parser(name = "D")]
    properties: HashMap<String, String>,

    // position
    #[arg_parser(positions)]
    urls: Vec<String>,
    #[arg_parser(position = 0)]
    url: String,
    #[arg_parser(position = 0)]
    url_option: Option<String>,
    #[arg_parser(position = 1)]
    position1: u16,
    #[arg_parser(position = 2)]
    position2: Option<f32>,
    #[arg_parser(position = 3)]
    position3: Option<isize>,

    // from_str
    #[arg_parser(from_str)]
    regex: RegexStr,
    #[arg_parser(name = "", from_str)]
    regex_option: Option<common::RegexStr>,
    #[arg_parser(name = "", from_str)]
    regex_multiple: Vec<RegexStr>,
}

#[test]
fn test_app() {
    let args: Vec<String> = vec![
        // position
        "http://exmaple.com",
        "123",
        "3.14",
        // value
        "-u",
        "a:b",
        "-v",
        "--following",
        "--num-u8",
        "12",
        "--num-isize",
        "999",
        "--num-f32",
        "1.414",
        // multiple
        "-F",
        "a",
        "b",
        "--flag-i128",
        "3",
        "2",
        "1",
        "-r",
        "http://exmaple[.](com|net)",
        "-Dk1=v1",
        "-Dk2=v2",
    ]
    .into_iter()
    .map(|s| s.into())
    .collect();

    let app = App::from_args(args).expect("failed to parse args");
    println!("{:?}", &app);
    println!("{}", serde_json::to_string_pretty(&app).expect("failed to serialize"));
}

// cargo expand --test argparse
#[test]
fn test_app_expand() {
    let app = new_app();
    assert!(app.is_err(), "Expected error, got: {:?}", app);
    if app.is_ok() {
        println!("{:?}", &app.unwrap());
    } else {
        println!("{:?}", &app.unwrap_err());
    }
}

fn new_app() -> Result<App, ArgParserError> {
    let parser = rexl::argparse::ArgParser::default();
    let app = App {
        user: parser.get_string_or_default("user"),
        password: parser.get_string("password"),
        version: parser.get_bool("version"),
        following: parser.get_bool_option("following"),
        num_u8: parser.get_integer("num_u8").map(|v| v as u8),
        num_i16: parser.get_integer_or_default("num_i16") as i16,
        num_i64: parser.get_integer_or_default("num_i64"),
        num_isize: parser.get_integer_or_default("num_isize") as isize,
        num_usize: parser
            .get_integer("num_usize")
            .map(|v| v as usize),
        num_f32: parser.get_integer_or_default("num_f32") as f32,
        num_f64: parser.get_integer("num_f64").map(|v| v as f64),
        flag: parser.get_strings("flag").as_ref().clone(),
        flag_i128: parser
            .get_integers("flag_i128")
            .as_ref()
            .clone()
            .iter()
            .map(|v| v.clone() as i128)
            .collect::<Vec<_>>(),
        flag_f64: parser.get_floats("flag_f64").as_ref().clone(),
        properties: parser
            .get_properties("properties")
            .as_ref()
            .clone(),
        urls: parser.get_position_values().clone(),
        url: parser
            .get_position_values()
            .get(0)
            .map_or_else(|| String::default(), |v| v.clone()),
        url_option: parser
            .get_position_values()
            .get(0)
            .map(|v| v.clone()),
        position1: parser
            .get_position_values()
            .get(1)
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or_default(),
        position2: parser
            .get_position_values()
            .get(2)
            .and_then(|v| v.parse::<f32>().ok()),
        position3: parser
            .get_position_values()
            .get(3)
            .and_then(|v| v.parse::<isize>().ok()),
        regex: parser
            .get_from_str_or_err("regex", |v| <RegexStr as std::str::FromStr>::from_str(v))?,
        regex_option: parser.get_from_str("regex_option", |v| {
            <common::RegexStr as std::str::FromStr>::from_str(v)
        })?,
        regex_multiple: parser.get_multiple_from_str("regex_multiple", |v| {
            <RegexStr as std::str::FromStr>::from_str(v)
        })?,
    };
    Ok(app)
}
