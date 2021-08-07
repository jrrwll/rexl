use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn load_properties<P: AsRef<Path>>(path: P) -> Result<HashMap<String, String>, Error> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    let mut properties = HashMap::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        // EOF
        if bytes_read == 0 {
            break;
        }
        if line.is_empty() || line.starts_with('#') || line.starts_with('=') {
            continue;
        }
        load_properties_per_line(&line, &mut properties);
        // do not accumulate data
        line.clear();
    }
    Ok(properties)
}

pub fn load_properties_from_str(content: &str) -> HashMap<String, String> {
    let sep = if cfg!(windows) { "\r\n" } else { "\n" };
    let lines = content.split(sep).map(|s| s.to_string()).collect();
    load_properties_from_vec(&lines)
}

pub fn load_properties_from_vec(lines: &Vec<String>) -> HashMap<String, String> {
    let mut properties = HashMap::new();
    for line in lines.iter() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        load_properties_per_line(&line, &mut properties);
    }
    properties
}

fn load_properties_per_line(line: &String, properties: &mut HashMap<String, String>) {
    // ind is never 0 since we skip it
    if let Some(ind) = line.find('=') {
        let line_size = line.len();
        if ind == line_size - 1 {
            properties.insert((&line[..ind]).to_string(), "".to_string());
        } else {
            properties.insert((&line[..ind]).to_string(), (&line[ind + 1..]).to_string());
        }
    } else {
        properties.insert(line.clone(), "".to_string());
    }
}
