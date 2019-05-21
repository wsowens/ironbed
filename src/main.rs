use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bedf = BedGraph::new(&args[1]);
    match bedf {
        Ok(x) => run(x),
        Err(msg) => eprintln!("{}", msg),
    }
}

struct BedGraph
{
    // add in trait based generics?
    filename: String,
    reader: BufReader<File>,
    lineno: u32,
}

impl BedGraph {
    fn new(fname: &String) -> Result<BedGraph, String> {
        let filename = fname.clone();
        match File::open(&fname) {
            Err(x) => Err(x.to_string()),
            Ok(handle) =>
                 Ok( BedGraph{filename, reader: BufReader::new(handle), lineno: 0}   )
          }
    }
}

impl Iterator for BedGraph {
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

fn run(mut bedf: BedGraph) {
    for _ in &mut bedf {

    }
    println!("Total length: {}", &bedf.lineno);

}
