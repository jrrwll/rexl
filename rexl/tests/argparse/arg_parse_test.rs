use rexl::argparse::ArgParser;

#[test]
fn test_arg_parser() {
    let args = vec![
        "-n3",
        "-o",
        "yaml",
        "-P6379",
        "--rm=false",
        "-owide",
        "-H",
        "x-opts=gzip",
        "-Ffilename=awesome.rb",
        "-Ffilemode=777",
        "-R",
        "svc",
        "ep",
        "ds",
        "-n5",
        "-H",
        "Accept: */*",
        "-H",
        "User-Agent: curl/7.54.0",
        "--",
        "nowarn",
        "noredirect",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    println!("{:?}", &args);

    let mut parser = get_parser();
    match parser.parse(args) {
        Ok(_) => {
            println!("integer P:\t{:?}", parser.get_integer("P"));
            println!("bool rm:\t{:?}", parser.get_bool("rm"));
            println!("strings o:\t{:?}", parser.get_strings("o"));
            println!("strings H:\t{:?}", parser.get_strings("H"));
            println!("strings R:\t{:?}", parser.get_strings("R"));
            println!("properties F:\t{:?}", parser.get_properties("F"));
            println!("position values:\t{:?}", parser.get_position_values());
            println!("position value1:\t{:?}", &parser.get_position_values()[0]);

            assert_eq!(Some(6379), parser.get_integer("P"));
            assert_eq!(false, parser.get_bool("rm"));
            assert_eq!(&vec!["yaml".to_string()], parser.get_strings("o").as_ref());
            assert_eq!(
                &vec![
                    "x-opts=gzip".to_string(),
                    "Accept: */*".to_string(),
                    "User-Agent: curl/7.54.0".to_string()
                ],
                parser.get_strings("H").as_ref()
            );
            assert_eq!(
                &vec!["svc".to_string(), "ep".to_string(), "ds".to_string()],
                parser.get_strings("R").as_ref()
            );

            let props = parser.get_properties("F");
            assert_eq!("awesome.rb", props["filename"]);
            assert_eq!("777", props["filemode"]);

            assert_eq!(
                &vec!["nowarn".to_string(), "noredirect".to_string()],
                parser.get_position_values()
            );
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}

fn get_parser() -> ArgParser {
    let mut parser = ArgParser::default();
    parser
        .add_integer_multiple("n", vec!["n", "number"])
        .add_integer("P", vec!["P", "port"])
        .add_bool("rm", vec!["r", "rm", "remove"])
        .add_multiple("o", vec!["o", "output"])
        .add_multiple("H", vec!["H", "header"])
        .add_multiple("R", vec!["resource", "R"])
        .add_property("F", vec!["F"]);
    parser
}
