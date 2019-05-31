extern crate mergebg;
use mergebg::bedgraph::BedGraphIterator;


fn main() {
    let readers: Vec<BedGraphIterator> = std::env::args().skip(1).map(
        | x | BedGraphIterator::new(&x).unwrap()
    ).collect();
    mergebg::bedgraph::union(readers, "null");
}

/*


#[derive(Debug)]
struct BedGraphReader {
    filename: String,
    reader: BufReader<File>,
    // should this be lazily evaluated?
    last_line: Option<BedGraphLine>,
    lineno: u32,
    new_chrom: bool,
}

impl BedGraphReader {
    fn new(fname: &str) -> Result<BedGraphReader, String> {
        //consider removing... this information is in the reader
        match File::open(fname) {
            Err(x) => Err(x.to_string()),
            Ok(handle) =>
                 Ok( BedGraphReader{filename: fname.to_string(), reader: BufReader::new(handle), last_line: None, lineno: 0, new_chrom: true}   )
          }
    }

    //consider adding a flag, "done", when they hit the end of file
    fn read_line(&mut self) {
        //put a new buffer here
        let mut temp = String::new();
        match self.reader.read_line(&mut temp) {
            Err(err) => {eprintln!("{}", err.to_string()); self.last_line = None;},
            Ok(bytes) => {
                match bytes {
                    0 => {
                        self.last_line = None;
                        eprintln!("Epic");
                    } _ => {
                        self.lineno += 1;
                        let new_line = BedGraphLine::new(&temp).unwrap();
                        if let Some(ref line) = self.last_line {
                            self.new_chrom = line.coords.chrom != line.coords.chrom;
                        }
                        self.last_line = Some(new_line);
                    }
                }
            }
        }
    }

    fn read_line_until(&mut self, chrom: &ChromPos, stop: u32) {

    }
}

/*TODO: create a separate iterator that returns references */

struct BedGraphUnion {
    members: Vec<BedGraphIterator>,
    //the position of the last truncation
    last_stop: Option<(String, u32)>,
}

impl BedGraphUnion {
    //TODO: should users be prevented from 
    fn new(filenames: &[String]) -> Result<BedGraphUnion, &str> {
        if filenames.len() < 2 {
            Err(&"Not enough files to create a union.")
        } else {
            Ok(BedGraphUnion{members: filenames.iter().map(|filename| BedGraphReader::new(filename).unwrap()).collect(), last_stop: None})
        }
    }
}

impl Iterator for BedGraphUnion {
    type Item = Vec<Option<BedGraphLine>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut all_none = true;
        let mut lines: Vec<Option<BedGraphLine>> = Vec::with_capacity(self.members.len());
        //include a "new chrom field" that requires checking of strings
        //let mut min_start_pos: Option<(String, u32)> = None;
        //let mut min_stop_pos:  Option<(String, u32)> = None;
        for bg in &mut self.members {
            let ref bg_option = bg.last_line;
            if let Some(_) = bg_option {
                all_none = false;
            }
            //lines.push();
        }
        if all_none {
            None
        } else {
            Some(lines)
        }
    }
}

enum UnionResult {
    
    None,
}

fn total_lines(bg: &mut BedGraphReader) -> u32 {
    bg.read_line();
    while let Some(_) = bg.last_line {
        bg.read_line();
    }
    bg.lineno
}

fn union(bedgraphs: BedGraphUnion) {
    //TODO: consider refining this into an iterator
    for round in bedgraphs {
        println!("{:?}", round);
    }
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn chrom_pos() {
        let mut bedgraph = BedGraphReader::new("test/unionbedg/1+2+3.bg").unwrap();
        assert_eq!(bedgraph.last_line, None);
        bedgraph.read_line();
        assert_eq!(bedgraph.last_line.unwrap().coords, ChromPos{chrom: "chr1".to_string(), start: 900, stop: 1000})
    }

    #[test]
    fn get_bg_data() {
        let mut bedgraph = BedGraphReader::new("test/unionbedg/1+2+3.bg").unwrap();
        assert_eq!(bedgraph.last_line, None);
        bedgraph.read_line();
        assert_eq!(bedgraph.last_line.unwrap().data, Some(String::from("0\t60\t0")));
    }

    #[test]
    fn test_truncate() {
        let mut line = BedGraphLine::new("chr1 900 1600 37").unwrap();
        line.truncate(950);
        assert_eq!(line, BedGraphLine::new("chr1 950 1600 37").unwrap());
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
        let mut bgs: Vec<BedGraphReader> = filenames.iter().map(|fname| BedGraphReader::new(fname).unwrap()).collect();
        assert_eq!(bgs[0].last_line, None);
        assert_eq!(bgs[1].last_line, None);
        let mut all_none = true;
        for bg in &mut bgs {
            bg.read_line();
            match bg.last_line {
                Some(_) => { all_none = false }
                _ => continue
            }
        }
        assert_eq!(all_none, false);
        assert_coords(&bgs[0], ChromPos{chrom: "chr1".to_string(), start: 1000, stop: 1500});
        assert_coords(&bgs[1], ChromPos{chrom: "chr1".to_string(), start: 900, stop: 1600});
        for bg in &mut bgs {
            bg.read_line();
            match bg.last_line {
                Some(_) => { all_none = false }
                _ => continue
            }
        }
        assert_coords(&bgs[0], ChromPos{chrom: "chr1".to_string(), start: 2000, stop: 2100});
        assert_coords(&bgs[1], ChromPos{chrom: "chr1".to_string(), start: 1700, stop: 2050});
        assert_data(&bgs[0], Some(String::from("20")));
        assert_data(&bgs[1], Some(String::from("50")));
        bgs[0].read_line();
        bgs[1].read_line();
        assert_eq!(bgs[0].last_line, None);
        assert_eq!(bgs[1].last_line, None);
    }
    /*
    #[test]
    fn union_uneven_loop() {
        let filenames = vec!["test/unionbedg/1.bg", "test/unionbedg/2.bg", "test/unionbedg/long.bg"];
        let mut bgs: Vec<BedGraphReader> = filenames.iter().map(|fname| BedGraphReader::new(fname).unwrap()).collect();
        assert_eq!(bgs[0].last_line, None);
        assert_eq!(bgs[1].last_line, None);
        assert_eq!(bgs[2].last_line, None);
        let mut all_none = true;
        for bg in &mut bgs {
            bg.read_line();
            match bg.last_line {
                Some(_) => { all_none = false }
                _ => continue
            }
        }
        assert_eq!(all_none, false);
        assert_coords(&bgs[0], ChromPos{chrom: "chr1".to_string(), start: 1000, stop: 1500});
        assert_coords(&bgs[1], ChromPos{chrom: "chr1".to_string(), start: 900, stop: 1600});
        assert_coords(&bgs[2], ChromPos{chrom: "chr1".to_string(), start: 1000, stop: 2000});
        while !all_none {
            for bg in &mut bgs {
                bg.read_line();
                match bg.last_line {
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
        bgs[0].read_line();
        bgs[1].read_line();
        bgs[2].read_line();
        assert_eq!(bgs[0].last_line, None);
        assert_eq!(bgs[1].last_line, None);
        assert_eq!(bgs[2].last_line, None);
    } */
}
*/