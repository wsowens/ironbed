use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents1 = fs::read_to_string(config.filename1)?;
    let contents2 = fs::read_to_string(config.filename2)?;

    println!("File1 data:\n{}", contents1);
    println!("File2 data:\n{}", contents2);
    Ok(())
}

pub struct Config {
    pub filename1: String,
    pub filename2: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let filename1 = args[1].clone();
        let filename2 = args[2].clone();
    
        Ok(Config {filename1, filename2})
    }
}
