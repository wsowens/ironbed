use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bgs: Vec<BedGraphReader> = args[1..].iter().map(|filename| BedGraphReader::new(filename).unwrap()).collect();
    union(bgs);
}

#[derive(Debug, PartialEq, Eq)]
struct ChromPos {
    chrom: String,
    start: u32,
    stop: u32,
}

impl fmt::Display for ChromPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.chrom, self.start, self.stop)
    }
}

#[derive(Debug)]
struct BedGraphReader {
    filename: String,
    reader: BufReader<File>,
    // should this be lazily evaluated?
    coords: Option<ChromPos>,
    lineno: u32,
}

impl BedGraphReader {
    fn new(fname: &String) -> Result<BedGraphReader, String> {
        //consider removing... this information is in the reader
        let filename = fname.clone();
        match File::open(&fname) {
            Err(x) => Err(x.to_string()),
            Ok(handle) =>
                 Ok( BedGraphReader{filename, reader: BufReader::new(handle), coords: None, lineno: 0}   )
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
                        self.coords = Some(extract_coords(&temp).unwrap());
                        Some(temp)
                    }
                }
            }
        }
    }
}

fn extract_coords(line: &String) -> Result<ChromPos, String> {
    //TODO: avoid iterating over the entire line by using enumerate
    let cols:Vec<&str> = line.split_whitespace().collect();
    if cols.len() < 3 {
        Err(format!("Invalid number of columns [{}] in line:\n{}", cols.len(), line))
    } else {
        //use lifetimes to make this work, instead of copying the string
        let chrom = cols[0].to_string();
        //TODO: provide a more useful error message for these
        let start: u32 = cols[1].parse().unwrap();
        let stop:  u32 = cols[2].parse().unwrap();
        Ok(ChromPos{chrom, start, stop})
    }
}

fn total_lines(mut bg: BedGraphReader) -> u32 {
    for _ in &mut bg {
        //iterate over the entire BedGraph
    }
    bg.lineno
}

fn union(bedgraphs: Vec<BedGraphReader>) {
    //TODO: consider refining this into an iterator
    for mut bg in bedgraphs {
        bg.next();
    }
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

    #[test]
    fn chrom_pos() {
        let mut bedgraph = BedGraphReader::new(&"test/unionbedg/1+2+3.bg".to_string()).unwrap();
        assert_eq!(bedgraph.coords, None);
        bedgraph.next();
        assert_eq!(bedgraph.coords, Some(ChromPos{chrom: "chr1".to_string(), start: 900, stop: 1000}))
    }
}