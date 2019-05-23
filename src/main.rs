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

#[derive(Debug, PartialEq, Eq)]
struct BedGraphLine {
    coords: ChromPos,
    data: Option<String>,
}

impl BedGraphLine {
    fn new(input_str: &String) -> Result<BedGraphLine, String> {
        //TODO: avoid iterating over the entire line by using enumerate
        let cols: Vec<&str> = input_str.split_whitespace().collect();
        if cols.len() < 3 {
            Err(format!("Invalid number of columns [{}] in line:\n{}", cols.len(), input_str))
        } else {
            //use lifetimes to make this work, instead of copying the string
            let chrom = cols[0].to_string();
            let start: u32 = cols[1].parse().unwrap();
            let stop:  u32 = cols[2].parse().unwrap();
            if cols.len() > 3 {
                Ok( BedGraphLine{coords: ChromPos{chrom, start, stop}, data: Some(cols[3..].join("\t")) } )
            } else {
                Ok( BedGraphLine{coords: ChromPos{chrom, start, stop}, data: None} )
            }
        }
    }
}

#[derive(Debug)]
struct BedGraphReader {
    filename: String,
    reader: BufReader<File>,
    // should this be lazily evaluated?
    last_line: Option<BedGraphLine>,
    lineno: u32,
}

impl BedGraphReader {
    fn new(fname: &String) -> Result<BedGraphReader, String> {
        //consider removing... this information is in the reader
        let filename = fname.clone();
        match File::open(&fname) {
            Err(x) => Err(x.to_string()),
            Ok(handle) =>
                 Ok( BedGraphReader{filename, reader: BufReader::new(handle), last_line: None, lineno: 0}   )
          }
    }
}

impl Iterator for BedGraphReader {
    //TODO: make this more efficient with references / lifetimes
    type Item = BedGraphLine;

    fn next(&mut self) -> Option<Self::Item> {
        let mut temp = String::new();
        match self.reader.read_line(&mut temp) {
            Err(err) => {eprintln!("{}", err.to_string()); None},
            Ok(bytes) => {
                match bytes {
                    0 => None,
                    _ => {
                        self.lineno += 1;
                        self.last_line = Some(BedGraphLine::new(&temp).unwrap());
                        //HORRIBLY INEFFICIENT, USE REFERENCES AND LIFETIMES
                        Some(BedGraphLine::new(&temp).unwrap())
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
}

impl Iterator for BedGraphUnion {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}


fn total_lines(mut bg: BedGraphReader) -> u32 {
    for _ in &mut bg {
        //iterate over the entire BedGraph
    }
    bg.lineno
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
        assert_eq!(bedgraph.last_line, None);
        bedgraph.next();
        assert_eq!(bedgraph.last_line.unwrap().coords, ChromPos{chrom: "chr1".to_string(), start: 900, stop: 1000})
    }

    #[test]
    fn get_bg_data() {
        let mut bedgraph = BedGraphReader::new(&"test/unionbedg/1+2+3.bg".to_string()).unwrap();
        assert_eq!(bedgraph.last_line, None);
        bedgraph.next();
        assert_eq!(bedgraph.last_line.unwrap().data, Some(String::from("0\t60\t0")));
    }

    fn assert_coords(bg: &BedGraphReader, coords: ChromPos) {
        if let Some(ref bg_line) = bg.last_line {
            assert_eq!(bg_line.coords, coords);
        } else {
            panic!(format!("Last line is None: {:?}", bg));
        }
    }

    fn assert_data(bg: &BedGraphReader, data: Option<String>) {
        if let Some(ref bg_line) = bg.last_line {
            assert_eq!(bg_line.data, data);
        } else {
            panic!(format!("Last line is None: {:?}", bg));
        }
    }


    #[test]
    fn union_loop() {
        let filenames = vec!["test/unionbedg/1.bg", "test/unionbedg/2.bg"];
        let mut bgs: Vec<BedGraphReader> = filenames.iter().map(|fname| BedGraphReader::new(&fname.to_string()).unwrap()).collect();
        assert_eq!(bgs[0].last_line, None);
        assert_eq!(bgs[1].last_line, None);
        let mut all_none = true;
        for bg in &mut bgs {
            match bg.next() {
                Some(_) => { all_none = false }
                _ => continue
            }
        }
        assert_eq!(all_none, false);
        assert_coords(&bgs[0], ChromPos{chrom: "chr1".to_string(), start: 1000, stop: 1500});
        assert_coords(&bgs[1], ChromPos{chrom: "chr1".to_string(), start: 900, stop: 1600});
        while !all_none {
            all_none = true;
            for bg in &mut bgs {
                match bg.next() {
                    Some(_) => { all_none = false }
                    _ => continue
                }
            }
        }
        //assert_coords(&bgs[0], ChromPos{chrom: "chr1".to_string(), start: 2000, stop: 2100});
        //assert_coords(&bgs[1], ChromPos{chrom: "chr1".to_string(), start: 1700, stop: 2050});
        //assert_data(&bgs[0], Some(String::from("20")));
        //assert_data(&bgs[1], Some(String::from("50")));
        //assert_eq!(bgs[0].next(), None);
        //assert_eq!(bgs[1].next(), None);
    }

    #[test]
    fn union_uneven_loop() {
        let filenames = vec!["test/unionbedg/1.bg", "test/unionbedg/2.bg", "test/unionbedg/long.bg"];
        let mut bgs: Vec<BedGraphReader> = filenames.iter().map(|fname| BedGraphReader::new(&fname.to_string()).unwrap()).collect();
        assert_eq!(bgs[0].last_line, None);
        assert_eq!(bgs[1].last_line, None);
        assert_eq!(bgs[2].last_line, None);
        let mut all_none = true;
        for bg in &mut bgs {
            match bg.next() {
                Some(_) => { all_none = false }
                _ => continue
            }
        }
        assert_eq!(all_none, false);
        assert_coords(&bgs[0], ChromPos{chrom: "chr1".to_string(), start: 1000, stop: 1500});
        assert_coords(&bgs[1], ChromPos{chrom: "chr1".to_string(), start: 900, stop: 1600});
        assert_coords(&bgs[2], ChromPos{chrom: "chr1".to_string(), start: 1000, stop: 2000});
        while !all_none {
            all_none = true;
            for bg in &mut bgs {
                match bg.next() {
                    Some(_) => { all_none = false }
                    _ => continue
                }
            }
        }
        assert_coords(&bgs[0], ChromPos{chrom: "chr1".to_string(), start: 2000, stop: 2100});
        assert_coords(&bgs[1], ChromPos{chrom: "chr1".to_string(), start: 1700, stop: 2050});
        assert_coords(&bgs[2], ChromPos{chrom: "chr6".to_string(), start: 5000, stop: 7533});
        assert_data(&bgs[0], Some(String::from("20")));
        assert_data(&bgs[1], Some(String::from("50")));
        assert_data(&bgs[2], Some(String::from("43")));
        assert_eq!(bgs[0].next(), None);
        assert_eq!(bgs[1].next(), None);
        assert_eq!(bgs[2].next(), None);
    }
}