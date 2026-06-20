use crate::*;
use rexl::argparse::FromArgs;

#[test]
fn test_arg_parser() {
    let args = vec![
        "-V",
        "--help",
        "--show",
        "size",
        "content",
        "all",
        "--v",
        "--all",
        "--depth",
        "3",
        "--type",
        "f",
        "dir",
        "--name",
        "dong",
        "yan",
        "--name-pattern",
        ".*a.+",
        "-s",
        ">1M,<=3G;>5T",
        "--access-time=>2020-12-12",
        "--modify-time",
        "<=2021-12;<>2021-12",
        "--change-time=!=2020-12-12",
        "--content",
        "awesome",
        "somehow",
        "--content-pattern",
        "how|owe",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let pared = Main::from_args(args);
    match pared {
        Ok(main) => {
            println!("{}", serde_json::to_string_pretty(&main).expect("failed to serialize"));
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
