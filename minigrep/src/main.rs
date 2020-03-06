use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // Old version
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| { // we care about return value
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    
    let config = Config::new(env::args()).unwrap_or_else(|err| { // env::args return an iterator. 
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) { // we don't care about return value
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


