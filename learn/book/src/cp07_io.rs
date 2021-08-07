use std::fs::File;
use std::io::{Error, ErrorKind, Read};

#[test]
#[allow(unused_variables)]
fn test_error() {
    let mut f = File::open("poem.md").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("poem.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let mut s = String::new();
    f.read_to_string(&mut s).unwrap_or_else(|error| {
        println!("{}", error.to_string());
        return 0;
    });

    read_username_from_file().unwrap();
}

fn read_username_from_file() -> Result<String, Error> {
    let mut s = String::new();
    File::open("poem.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
