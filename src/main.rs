use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match BedGraphUnion::new(&args[1..]) {
        Err(msg) => eprintln!("{}", msg),
        Ok(bg_union) => {
            println!("{:?}", bg_union.members);
        }
    }
    //union(bgs);
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
    data: Option<String>,
    lineno: u32,
}

impl BedGraphReader {
    fn new(fname: &String) -> Result<BedGraphReader, String> {
        //consider removing... this information is in the reader
        let filename = fname.clone();
        match File::open(&fname) {
            Err(x) => Err(x.to_string()),
            Ok(handle) =>
                 Ok( BedGraphReader{filename, reader: BufReader::new(handle), coords: None, data: None, lineno: 0}   )
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
                        let (coords, data) = extract_coords(&temp).unwrap();
                        self.coords = Some(coords);
                        self.data = Some(data);
                        Some(temp)
                    }
                }
            }
        }
    }
}

//TODO: consider abstacting this into a generic "Union" iterator
struct BedGraphUnion {
    members: Vec<BedGraphReader>,
}

impl BedGraphUnion {
    //TODO: should users be prevented from 
    fn new(filenames: &[String]) -> Result<BedGraphUnion, &str> {
        if filenames.len() < 2 {
            Err(&"Not enough files to create a union.")
        } else {
            Ok(BedGraphUnion{members: filenames.iter().map(|filename| BedGraphReader::new(filename).unwrap()).collect()})
        }
    }

    /*
    fn min_start(&self) -> u32 {
        let min: u32 = 
    }
    
    fn min_stop(&self) -> u32 {

    }
    */
}

/*
impl Iterator for BedGraphUnion {
    fn next(&mut self) -> Option<Vec<String>> {

    }
}
*/
fn extract_coords(line: &String) -> Result<(ChromPos, String), String> {
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

        if cols.len() > 3 {
            Ok((ChromPos{chrom, start, stop}, cols[3..].join("\t")))
        } else {
            Ok((ChromPos{chrom, start, stop}, "".to_string()))
        }
    }
}

fn total_lines(mut bg: BedGraphReader) -> u32 {
    for _ in &mut bg {
        //iterate over the entire BedGraph
    }
    bg.lineno
}

fn all_none<T>(items: Vec<Option<T>>) -> bool {
    for i in items {
        match i {
            None => return false,
            _ => continue
        }
    }
    true
}

fn union(mut bedgraphs: Vec<&mut BedGraphReader>) {
    //TODO: consider refining this into an iterator
    loop {
        let mut all_none = true;
        for bg in &mut bedgraphs {
            match bg.next() {
                Some(_) => { all_none = false }
                _ => continue
            }
        }
        if all_none {
            break
        }
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

    #[test]
    fn get_bg_data() {
        let mut bedgraph = BedGraphReader::new(&"test/unionbedg/1+2+3.bg".to_string()).unwrap();
        assert_eq!(bedgraph.data, None);
        bedgraph.next();
        assert_eq!(bedgraph.data, Some(String::from("0\t60\t0")));
    }

    #[test]
    fn union_loop() {
        let filenames = vec!["test/unionbedg/1.bg", "test/unionbedg/2.bg"];
        let mut bgs: Vec<BedGraphReader> = filenames.iter().map(|fname| BedGraphReader::new(&fname.to_string()).unwrap()).collect();
        assert_eq!(bgs[0].coords, None);
        assert_eq!(bgs[1].coords, None);
        let mut all_none = true;
        for bg in &mut bgs {
            match bg.next() {
                Some(_) => { all_none = false }
                _ => continue
            }
        }
        assert_eq!(all_none, false);
        assert_eq!(bgs[0].coords, Some(ChromPos{chrom: "chr1".to_string(), start: 1000, stop: 1500}));
        assert_eq!(bgs[1].coords, Some(ChromPos{chrom: "chr1".to_string(), start: 900, stop: 1600}));
        while !all_none {
            all_none = true;
            for bg in &mut bgs {
                match bg.next() {
                    Some(_) => { all_none = false }
                    _ => continue
                }
            }
        }
        assert_eq!(bgs[0].coords, Some(ChromPos{chrom: "chr1".to_string(), start: 2000, stop: 2100}));
        assert_eq!(bgs[1].coords, Some(ChromPos{chrom: "chr1".to_string(), start: 1700, stop: 2050}));
        assert_eq!(bgs[0].data, Some(String::from("20")));
        assert_eq!(bgs[1].data, Some(String::from("50")));
        assert_eq!(bgs[0].next(), None);
        assert_eq!(bgs[1].next(), None);
    }
}