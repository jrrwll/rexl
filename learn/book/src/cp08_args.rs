use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

const CONTENTS: &'static str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

#[test]
#[should_panic]
#[allow(unused_variables)]
fn test() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        panic!("Problem parsing arguments");
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        panic!("Application error");
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args /*args: &[String]*/) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

fn run(config: Config) -> Result<(), Error> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    //    let mut results = Vec::new();
    //
    //    for line in contents.lines() {
    //        if line.contains(query) {
    //            results.push(line);
    //        }
    //    }
    //    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[test]
fn test_search() {
    let query = "duct";
    assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));

    let query = "rUsT";
    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, CONTENTS)
    );
}
