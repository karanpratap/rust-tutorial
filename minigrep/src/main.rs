use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(-1);
    });

    // Read file contents
    if let Err(err) = minigrep::run(config) {
        eprintln!("Error reading file: {}", err);
        process::exit(-2);
    };
}
