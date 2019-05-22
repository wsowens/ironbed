use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bedf = BedGraphReader::new(&args[1]);
}

struct BedGraphReader {
    // add in trait based generics?
    filename: String,
    reader: BufReader<File>,
    lineno: u32,
}

impl BedGraphReader {
    fn new(fname: &String) -> Result<BedGraphReader, String> {
        let filename = fname.clone();
        match File::open(&fname) {
            Err(x) => Err(x.to_string()),
            Ok(handle) =>
                 Ok( BedGraphReader{filename, reader: BufReader::new(handle), lineno: 0}   )
          }
    }
}

impl Iterator for BedGraphReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut temp = String::new();
        match self.reader.read_line(&mut temp) {
            Err(err) => {eprintln!("{}", err.to_string()); None},
            Ok(bytes) => {
                match bytes {
                    0 => None,
                    _ => {
                        self.lineno += 1;
                        Some(temp)
                    }
                }
            }
        }
    }
}

fn total_lines(mut bg: BedGraphReader) -> u32 {
    for _ in &mut bg {

    }
    bg.lineno
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_count() {
        //create a bedgraph reader for a test file with 9 lines
        let filename = String::from("test/unionbedg/1+2+3.bg");
        let mut bedgraph = BedGraphReader::new(&filename).unwrap();
        //check the number of lines
        assert_eq!(total_lines(bedgraph), 9);
    }
}