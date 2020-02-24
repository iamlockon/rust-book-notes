use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| { // we care about return value
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) { // we don't care about return value
        println!("Application error: {}", e);
        process::exit(1);
    }
}


