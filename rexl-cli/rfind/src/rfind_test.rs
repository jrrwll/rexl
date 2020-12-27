use crate::*;
use crate::Options::*;

#[test]
fn test_arg_parser() {
    let args = vec![
            "-V",
            "--help",
            "--show", "size","content", "all",
            "--v",
            "--all",
            "--depth", "3",
            "--type", "f", "dir",
            "--name", "dong", "yan",
            "--name-pattern", ".*a.+",
            "-s", ">1MB,<=3G;>5T",
            "--access-time=>2020-12-12",
            "--modify-time", "<=2021-12;<>2021-12",
            "--change-time=!=2020-12-12",
            "--content", "awesome", "somehow",
            "--content-pattern", "how|owe",
        ].iter().map(|s|s.to_string()).collect();
    let mut parser = new_arg_parser();
    match parser.parse(args) {
        Ok(_) => {
            println!("bool version:\t{:?}", parser.get_bool(Version));
            println!("bool help:\t{:?}", parser.get_bool(Help));
            println!("bool verbose:\t{:?}", parser.get_bool(Verbose));
            println!("bool all:\t{:?}", parser.get_bool(All));

            println!("integer depth:\t{:?}", parser.get_integer(Depth));
            println!("strings type:\t{:?}", parser.get_strings(Kind));

            println!("strings name:\t{:?}", parser.get_strings(Name));
            println!("strings name-pattern:\t{:?}", parser.get_strings(NamePattern));
            println!("strings size:\t{:?}", parser.get_strings(Size));
            println!("strings access-time:\t{:?}", parser.get_strings(AccessTime));
            println!("strings modify-time:\t{:?}", parser.get_strings(ModifyTime));
            println!("strings change-time:\t{:?}", parser.get_strings(ChangeTime));
            println!("strings content:\t{:?}", parser.get_strings(Content));
            println!("strings content-pattern:\t{:?}", parser.get_strings(ContentPattern));

            println!("extra values:\t{:?}", parser.get_extra_values());
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}