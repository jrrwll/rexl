use std::fs::*;

use crate::*;
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Main<'a> {
    pub context:         &'a Context,
    pub verbose:         bool,
    // filter
    pub depth:           usize,
    // bits: 1file,2dir,3link
    pub kind:            u8,
    pub name:            Vec<String>,
    pub name_pattern:    Vec<Regex>,
    pub size:            Vec<(SizeOption, u64)>,
    // pub access_time: Vec<TimeOption>,
    // pub modify_time: Vec<TimeOption>,
    // pub change_time: Vec<TimeOption>,
    pub content:         Vec<String>,
    pub content_pattern: Vec<Regex>,
    pub path:            Vec<String>,
}

impl<'a> Main<'a> {
    pub fn run(&mut self) {
        let depth = self.depth;
        for path in self.path.iter() {
            match self.search(Path::new(path), depth) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("error: {}", err);
                    return
                }
            }
        }
    }

    fn search(&self, path: &Path, depth: usize) -> Result<(), String> {
        let meta = metadata(path).map_err(|err| err.to_string())?;
        if meta.is_file() {
            // kind
            if !self.include_file() {
                if self.verbose {
                    println!("skipping file by type filtering {:?}", path)
                }
                return Ok(())
            }

            let file_size = meta.len();
            let file_name = path
                .file_name()
                .ok_or(format!("cannot read the file name from path {:?}", path))?;
            let file_name = file_name.to_str().ok_or(format!(
                "cannot convert os_string to string for path {:?}",
                path
            ))?;

            // size
            if !self.size.is_empty() {
                let mut matched = false;
                for (so, size) in self.size.iter() {
                    if so.matched(*size, file_size) {
                        matched = true;
                        break
                    }
                }
                if !matched {
                    if self.verbose {
                        println!(
                            "skipping file by size({:?}) filtering {:?}",
                            file_size, path
                        )
                    }
                    return Ok(())
                }
            }
            // name
            if !self.name.is_empty() {
                let mut matched = false;
                for name in self.name.iter() {
                    if file_name.contains(name) {
                        matched = true;
                        break
                    }
                }
                if !matched {
                    if self.verbose {
                        println!("skipping file by name filtering {:?}", path)
                    }
                    return Ok(())
                }
            }
            // name_pattern
            if !self.name_pattern.is_empty() {
                let mut matched = false;
                for name_pattern in self.name_pattern.iter() {
                    if name_pattern.find(file_name).is_some() {
                        matched = true;
                        break
                    }
                }
                if !matched {
                    if self.verbose {
                        println!("skipping file by name-pattern filtering {:?}", path)
                    }
                    return Ok(())
                }
            }

            self.search_file(path)?;
        } else if meta.is_dir() {
            // kind
            if !self.include_dir() {
                if self.verbose {
                    println!("skipping dir by type filtering {:?}", path)
                }
                return Ok(())
            }
            // depth
            if depth <= 1 {
                return Ok(())
            }

            self.search_dir(path, depth - 1)?;
        }
        Ok(())
    }

    fn search_file(&self, path: &Path) -> Result<(), String> {
        let file = File::open(path).map_err(|err| err.to_string())?;
        let reader = BufReader::new(file);

        if self.content.is_empty() && self.content_pattern.is_empty() {
            println!("found: {:?}", path);
            return Ok(())
        }

        let mut matched = false;
        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    // content
                    if !self.content.is_empty() {
                        for content in self.content.iter() {
                            if line.contains(content) {
                                matched = true;
                                break
                            }
                        }
                    }
                }
                Err(err) => {
                    if self.verbose {
                        eprintln!(
                            "error while reading {:?}, error is: {:?}",
                            path,
                            err.to_string()
                        );
                    }
                    return Ok(())
                }
            }
        }
        if !matched {
            if self.verbose {
                println!("skipping file by content filtering {:?}", path)
            }
            return Ok(())
        }
        println!("found: {:?}", path);
        Ok(())
    }

    fn search_dir(&self, path: &Path, depth: usize) -> Result<(), String> {
        let rd = read_dir(path).map_err(|err| err.to_string())?;
        for p in rd {
            let entry = p.map_err(|err| err.to_string())?;
            self.search(entry.path().as_path(), depth)?;
        }
        Ok(())
    }

    fn include_file(&self) -> bool {
        self.kind & 0b0000_0001 != 0
    }

    fn include_dir(&self) -> bool {
        self.kind & 0b0000_0010 != 0
    }
}
