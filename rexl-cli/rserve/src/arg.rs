use rexl::argparse::FromArgs;
use std::process;

#[derive(Debug, FromArgs)]
pub struct Config {
    #[arg_parser(name = "a,address")]
    pub bind_address: String,
    #[arg_parser(name = "p,path")]
    pub base_path: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let parsed = Self::from_args(args);
        match parsed {
            Ok(mut config) => {
                if config.bind_address.is_empty() {
                    config.bind_address = "0.0.0.0:5000".to_string();
                }
                if config.base_path.is_empty() {
                    config.base_path = "/**".to_string();
                }
                config
            }
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
        }
    }
}
