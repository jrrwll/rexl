use rexl::cli::{ArgParser, ArgParserError};

#[test]
fn test_parse() {
    let args = "aux -n3 -o yaml -P6379 --rm=true -owide \
    -H x-opts=gzip -Ffilename=awesome.rb -Ffilemode=777 \
    -R svc ep ds -n5";
    let mut args: Vec<String> = args.split(" ").map(|s| s.to_string()).collect();
    args.extend(vec![
        "-H", "Accept: */*", "-H", "User-Agent: curl/7.54.0",
        "--", "nowarn", "noredirect"].iter().map(|s| s.to_string()));
    println!("args:\t{:?}", args);

    let mut parser = get_parser();
    match parser.parse_bsd(args) {
        Ok(_) => {
            println!("integer P:\t{:?}", parser.get_integer("P"));
            println!("bool rm:\t{:?}", parser.get_bool("rm"));
            println!("bool a:\t{:?}", parser.get_bool("a"));
            println!("bool u:\t{:?}", parser.get_bool("u"));
            println!("bool x:\t{:?}", parser.get_bool("x"));
            println!("strings o:\t{:?}", parser.get_strings("o"));
            println!("strings H:\t{:?}", parser.get_strings("H"));
            println!("strings R:\t{:?}", parser.get_strings("R"));
            println!("properties F:\t{:?}", parser.get_properties("F"));
            println!("extra values:\t{:?}", parser.get_extra_values());
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}

fn get_parser() -> ArgParser {
    let mut parser = ArgParser::new();
    parser.add_integer_multiple("n", vec!["n", "number"])
        .add_integer("P",  vec!["P", "port"])
        .add_bool("rm",  vec!["r", "rm", "remove"])
        .add_bool("a",  vec!["a"])
        .add_bool("u",  vec!["u"])
        .add_bool("x",  vec!["x"])
        .add_multiple("o", vec!["o", "output"])
        .add_multiple("H", vec!["H", "header"])
        .add_multiple("R", vec!["resource", "R"])
        .add_property("F", vec!["F"]);
    parser
}