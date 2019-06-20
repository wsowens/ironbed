mod chrom_geo {
    use std::fmt;

    //this will have to all be manually implemented
    //when we have a "new_chrom" flag
    #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
    pub struct ChromPos {
        pub chrom: String,
        pub index: u32
    }

    //TODO: replace String with str
    #[derive(Debug, PartialEq, Eq)]
    pub struct ChromSeg {
        pub chrom: String,
        pub start: u32,
        pub stop:  u32
    }

    impl ChromSeg {
        pub fn start_pos(&self) -> ChromPos {
            ChromPos{chrom: self.chrom.clone(), index: self.start}
        }
        
        pub fn stop_pos(&self) -> ChromPos {
            ChromPos{chrom: self.chrom.clone(), index: self.stop}
        }
    }
    
    impl fmt::Display for ChromSeg {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}\t{}\t{}", self.chrom, self.start, self.stop)
        }
    }

    #[cfg(test)]
    mod chrom_geo_tests {
        use super::*;

        #[test]
        fn start_stop() {
            let seg = ChromSeg{chrom: "chr3".to_string(), start: 100, stop: 250};
            assert_eq!(seg.start_pos(), ChromPos{chrom: "chr3".to_string(), index: 100});
            assert_eq!(seg.stop_pos(), ChromPos{chrom: "chr3".to_string(), index: 250});
        }

        #[test]
        fn sort() {
            let mut chrom_vec = vec![ChromPos{chrom: "chr4".to_string(), index: 1000},
                                 ChromPos{chrom: "chr1".to_string(), index: 5000},
                                 ChromPos{chrom: "chr15".to_string(), index: 100},
                                 ChromPos{chrom: "chr1".to_string(), index: 3000}];
            let sorted = vec![ChromPos{chrom: "chr1".to_string(), index: 3000},
                              ChromPos{chrom: "chr1".to_string(), index: 5000},
                              ChromPos{chrom: "chr15".to_string(), index: 100},
                              ChromPos{chrom: "chr4".to_string(), index: 1000}];
            chrom_vec.sort_unstable();
            assert_eq!(chrom_vec, sorted);
        }

        #[test]
        fn min_max() {
            let chrom_vec = vec![ChromPos{chrom: "chr4".to_string(), index: 1000},
                                 ChromPos{chrom: "chr1".to_string(), index: 5000},
                                 ChromPos{chrom: "chr15".to_string(), index: 100},
                                 ChromPos{chrom: "chr1".to_string(), index: 3000}];
            let min = chrom_vec.iter().min().unwrap();
            let max = chrom_vec.iter().max().unwrap();
            assert_eq!(*min, ChromPos{chrom: "chr1".to_string(), index: 3000});
            assert_eq!(*max, ChromPos{chrom: "chr4".to_string(), index: 1000});
        }
    }
}


pub mod bedgraph {
    use super::chrom_geo;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Debug, PartialEq, Eq)]
    pub struct BgLine {
        coords: chrom_geo::ChromSeg,
        //TODO: turn the data into a str that references another field "raw line"
        data: Option<String>,
    }

    impl BgLine {
        fn new(input_str: &str) -> Result<BgLine, String> {
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
                    Ok( BgLine{coords: chrom_geo::ChromSeg{chrom, start, stop}, data: Some(cols[3..].join("\t")) } )
                } else {
                    Ok( BgLine{coords: chrom_geo::ChromSeg{chrom, start, stop}, data: None} )
                }
            }
        }

        //TODO: implement this with the new_chrom to minimize string comparison?
        fn starts_before(&self, pos: &chrom_geo::ChromPos) -> bool {
            self.coords.start_pos() <= *pos 
        }
    }

    /*
    truncate line at split_pt, if possible
    the split_pt is at the end or after the line, return "None"
    the split_pt is before the end of line, split and take the latter portion
    */
    fn truncate_after(line: BgLine, split_pt: &chrom_geo::ChromPos) -> Option<BgLine> {
        let coords = line.coords;
        if &coords.stop_pos() > split_pt  {
            Some(BgLine{
                    coords: chrom_geo::ChromSeg{chrom: coords.chrom, start: split_pt.index, stop: coords.stop}, 
                    ..line
                })
        } else {
            None
        }
    }

    #[derive(Debug)]
    pub struct BgIterator {
        reader: BufReader<File>,
        lineno: u32
    }

    impl BgIterator {
        pub fn new(fname: &str) -> Result<BgIterator, String> {
            //consider removing... this information is in the reader
            match File::open(fname) {
                Err(x) => Err(x.to_string()),
                Ok(handle) =>
                    Ok( BgIterator{ reader: BufReader::new(handle), lineno: 0 }   )
            }
        }
    }

    impl Iterator for BgIterator {
        type Item = BgLine;

        fn next(&mut self) -> Option<Self::Item> {
            //TODO: allocate to be the size of the previous line?
            let mut temp = String::new();
            match self.reader.read_line(&mut temp) {
                Err(err) => {eprintln!("{}", err.to_string()); None},
                Ok(bytes) => {
                    match bytes {
                        0 => None,
                        _ => {
                            self.lineno += 1;
                            Some(BgLine::new(&temp).unwrap())
                        }
                    }
                }
            }
        }
    }
    
    enum UnionLine {
        In(BgLine),
        Out(BgLine),
        Done
    }

    struct BgUnion {
        readers: Vec<BgIterator>,
        lines: Vec<UnionLine>,
        curr: chrom_geo::ChromPos,
    }

    impl BgUnion {
        fn new(mut readers: Vec<BgIterator>) -> Result<BgUnion, &'static str> {
            let mut lines: Vec<UnionLine> = Vec::with_capacity(readers.len());
            for rdr in readers.iter_mut() {
                match rdr.next() {
                    Some(bgl) => lines.push(UnionLine::Out(bgl)),
                    None => lines.push(UnionLine::Done),
                }
            }
            //remove this hard coding
            let curr = chrom_geo::ChromPos{chrom: "chr1".to_string(), index: 0};
            Ok( BgUnion{readers, lines, curr} )
        }

        fn next_transition(&self) -> chrom_geo::ChromPos {
            //TODO: add logic for empty spaces / provided sizes file
            self.lines.iter().filter_map(
                | x | match x {
                        UnionLine::Done => None,
                        UnionLine::In(ref line) => Some(line.coords.stop_pos()),
                        UnionLine::Out(ref line) => Some(line.coords.start_pos()),
            }).min().unwrap()
        }
    }

    impl Iterator for BgUnion {
        type Item = BgLine

        fn next(&mut self) -> Option<self::Item> {
            if self.all_done {
                return None;
            }
            let next_trans = self.next_transition();
            let all_out = self.lines.iter().all(| x | match x { Out(_) => true, _ => false});
            //prep the data... do this in a better way if possible
            let line = format!("")
            //advance all the readers / lines based on the next transition
            
            // the region we're about to mark out must be "empty"
            if all_out {
                if self.yield_empty {
                    //yield the line
                } else {
                   self.next() 
                }
            } else {
                //yield the line
            }
        }
    }


    #[cfg(test)]
    mod test_bedgraph {
        use super::*;
        
        //helper functions for checking chrom_seg and data
        fn check_segment(line: &Option<BgLine>, coords: chrom_geo::ChromSeg) {
            if let Some(line) = line {
                assert_eq!(line.coords, coords);
            } else {
                panic!(format!("Last line is None: {:?}", line));
            }
        }

        fn check_data(line: &Option<BgLine>, data: Option<String>) {
            if let Some(line) = line {
                assert_eq!(line.data, data);
            } else {
                panic!(format!("Last line is None: {:?}", line));
            }
        }
        
        #[test]
        fn line_count() {
            //create a bedgraph iterator for a test file with 9 lines
            let bedgraph = BgIterator::new("test/unionbedg/1+2+3.bg").unwrap();
            //check the number of lines
            let count = bedgraph.count();
            assert_eq!(count, 9);
        }

        #[test]
        fn iterator_next() {
            let mut bedgraph = BgIterator::new("test/unionbedg/1.bg").unwrap();
            let last_line = bedgraph.next();
            check_segment(&last_line, chrom_geo::ChromSeg{chrom: "chr1".to_string(), start: 1000, stop: 1500 } );
            check_data(&last_line, Some("10".to_string()));;
            let last_line = bedgraph.next();
            check_segment(&last_line, chrom_geo::ChromSeg{chrom: "chr1".to_string(), start: 2000, stop: 2100 } );
            check_data(&last_line, Some("20".to_string()));;
            let last_line = bedgraph.next();
            assert_eq!(last_line, None);
        }
        
        #[test]
        fn min_iterators() {
            let bedgraph1 = BgIterator::new("test/unionbedg/1.bg").unwrap();
            let bedgraph2 = BgIterator::new("test/unionbedg/2.bg").unwrap();
            let bedgraph3 = BgIterator::new("test/unionbedg/3.bg").unwrap();
            let mut readers: Vec<BgIterator> = vec![bedgraph1, bedgraph2, bedgraph3];
            let lines: Vec<BgLine> = readers.iter_mut().map(|x| x.next().unwrap()).collect();
            let min_start: chrom_geo::ChromPos = lines.iter().map(|x| x.coords.start_pos()).min().unwrap();
            assert_eq!(min_start, chrom_geo::ChromPos{chrom: "chr1".to_string(), index: 900});
            let min_start: chrom_geo::ChromPos = lines.iter().map(|x| x.coords.stop_pos()).min().unwrap();
            assert_eq!(min_start, chrom_geo::ChromPos{chrom: "chr1".to_string(), index: 1500});
        }

        /*
        #[test]
        fn test_truncate() {
            let mut line = BgLine::new("chr1 900 1600 37").unwrap();
            line.truncate(950);
            assert_eq!(line, BgLine::new("chr1 950 1600 37").unwrap());
        }
        */
    }
}