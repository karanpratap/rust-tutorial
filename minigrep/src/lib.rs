use std::{fs, env};
use std::error::Error;

pub struct Config {
    pub search_string: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments, needed at least 2");
        }

        Ok(Self { 
            search_string: args[1].clone(), 
            file_path: args[2].clone(),
            ignore_case: if args.len() > 3 && args[3].starts_with("i") {
                true
            } else {
                env::var("IGNORE_CASE").is_ok()
            }
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
    let mut result: Vec<&'a str> = vec![];
    for line in contents.lines() {
        if line.contains(search_string) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(search_string: &str, contents: &'a str) -> Vec<&'a str> {
    let query = search_string.to_lowercase();
    let mut result: Vec<&'a str> = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}