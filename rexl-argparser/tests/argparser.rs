use std::collections::HashMap;
use std::default::Default;
use rexl_argparser::ArgParser;

#[derive(Debug, Default, ArgParser)]
#[arg_parser(all_props = true, first_char = true)]
struct Curl {
    #[arg_parser_position(0)]
    url: String,
    data: Option<String>,
    fail: bool,
    #[arg_parser_value("O")]
    remote_name: bool,
    user_name: Option<String>,
    #[arg_parser_value("D")]
    properties: HashMap<String, String>,
    version: bool,
}

#[test]
fn test_curl() {
    let args: Vec<String> = vec![
        "http://exmaple.com", "-d", "{}",
        "-f", "--remote-name", "-u", "a:b",
        "-Dk1=v1", "-Dk2=v2"].into_iter().map(|s| s.into()).collect();

    let curl = Curl::parse_from_arg(args);
    println!("{:?}", curl);
    println!("{:?}", Curl::default())
}
