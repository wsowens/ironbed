use std::env;
use std::process;

extern crate mergebg;
use mergebg::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config  = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("File1: {}", config.filename1);
    println!("File2: {}", config.filename2);
    
    if let Err(e) = mergebg::run(config) {
        println!("Application Error: {}", e);

        process::exit(1);
    }
}
