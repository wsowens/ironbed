use std::env;
use std::process;
use std::error::Error;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config  = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("File1: {}", config.filename1);
    println!("File2: {}", config.filename2);
    
    if let Err(e) = run(config) {
        println!("Application Error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents1 = fs::read_to_string(config.filename1)?;
    let contents2 = fs::read_to_string(config.filename2)?;

    println!("File1 data:\n{}", contents1);
    println!("File2 data:\n{}", contents2);
    Ok(())
}

struct Config {
    filename1: String,
    filename2: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let filename1 = args[1].clone();
        let filename2 = args[2].clone();
    
        Ok(Config {filename1, filename2})
    }
}
