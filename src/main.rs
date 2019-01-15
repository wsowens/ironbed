use std::env;
use std::process;

extern crate mergebg;
use mergebg::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config  = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    
    if let Err(e) = mergebg::run(config) {
        eprintln!("Application Error: {}", e);

        process::exit(1);
    }
}
