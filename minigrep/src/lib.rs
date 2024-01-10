use std::{fs, env};
use std::error::Error;

pub struct Config {
    pub search_string: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();    // ignore program name and get to the real args

        let search_string = match args.next() {
            Some(query) => query,
            None => return Err("No search string")
        };

        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("File path not specified")
        };

        let ignore_case = match args.next() {
            Some(ic) => {
                if ic.starts_with("i") { true }
                else { false }
            }
            None => env::var("IGNORE_CASE").is_ok()
        };

        Ok(Self { 
            search_string, 
            file_path,
            ignore_case
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.search_string, &contents)
    } else {
        search(&config.search_string, &contents)
    };

    for line in result {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(search_string: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(&search_string))
        .collect()
}

pub fn search_case_insensitive<'a>(search_string: &str, contents: &'a str) -> Vec<&'a str> {
    let query = search_string.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}