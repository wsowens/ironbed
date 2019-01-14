use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config  = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("File1: {}", config.filename1);
    println!("File2: {}", config.filename2);
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
