mod chrom_geo {
    use std::fmt;

    //this will have to all be manually implemented
    //when we have a "new_chrom" flag
    #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
    pub struct ChromPos<'a> {
        pub chrom: &'a str,
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
        fn start_pos(&self) -> ChromPos {
            ChromPos{chrom: &self.chrom, index: self.start}
        }
        
        fn stop_pos(&self) -> ChromPos {
            ChromPos{chrom: &self.chrom, index: self.stop}
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
            assert_eq!(seg.start_pos(), ChromPos{chrom: "chr3", index: 100});
            assert_eq!(seg.stop_pos(), ChromPos{chrom: "chr3", index: 250});
        }

        #[test]
        fn sort() {
            let mut chrom_vec = vec![ChromPos{chrom: "chr4", index: 1000},
                                 ChromPos{chrom: "chr1", index: 5000},
                                 ChromPos{chrom: "chr15", index: 100},
                                 ChromPos{chrom: "chr1", index: 3000}];
            let sorted = vec![ChromPos{chrom: "chr1", index: 3000},
                              ChromPos{chrom: "chr1", index: 5000},
                              ChromPos{chrom: "chr15", index: 100},
                              ChromPos{chrom: "chr4", index: 1000}];
            chrom_vec.sort_unstable();
            assert_eq!(chrom_vec, sorted);
        }

        #[test]
        fn min_max() {
            let chrom_vec = vec![ChromPos{chrom: "chr4", index: 1000},
                                 ChromPos{chrom: "chr1", index: 5000},
                                 ChromPos{chrom: "chr15", index: 100},
                                 ChromPos{chrom: "chr1", index: 3000}];
            let min = chrom_vec.iter().min().unwrap();
            let max = chrom_vec.iter().max().unwrap();
            assert_eq!(*min, ChromPos{chrom: "chr1", index: 3000});
            assert_eq!(*max, ChromPos{chrom: "chr4", index: 1000});
        }
    }
}


pub mod bedgraph {
    use super::chrom_geo;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Debug, PartialEq, Eq)]
    pub struct BedGraphLine {
        coords: chrom_geo::ChromSeg,
        //TODO: turn the data into a str that references another field "raw line"
        data: Option<String>,
    }

    impl BedGraphLine {
        fn new(input_str: &str) -> Result<BedGraphLine, String> {
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
                    Ok( BedGraphLine{coords: chrom_geo::ChromSeg{chrom, start, stop}, data: Some(cols[3..].join("\t")) } )
                } else {
                    Ok( BedGraphLine{coords: chrom_geo::ChromSeg{chrom, start, stop}, data: None} )
                }
            }
        }

        //TODO: implement this
        fn starts_before(&self, pos: &chrom_geo::ChromPos) -> bool {
            false
        }
    
        //TODO: implement this
        fn ends_after(&self, pos: &chrom_geo::ChromPos) -> bool {
            false
        }

        //TODO: implement this
        fn truncate_before(&mut self, pos: &chrom_geo::ChromPos) -> bool {
            false
        }
        
        //TODO: implement this
        fn truncate_after(&mut self, pos: &chrom_geo::ChromPos) -> bool {
            false
        }
    }

    #[derive(Debug)]
    pub struct BedGraphIterator {
        reader: BufReader<File>,
        lineno: u32
    }

    impl BedGraphIterator {
        pub fn new(fname: &str) -> Result<BedGraphIterator, String> {
            //consider removing... this information is in the reader
            match File::open(fname) {
                Err(x) => Err(x.to_string()),
                Ok(handle) =>
                    Ok( BedGraphIterator{ reader: BufReader::new(handle), lineno: 0 }   )
            }
        }
    }

    impl Iterator for BedGraphIterator {
        type Item = BedGraphLine;

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
                            Some(BedGraphLine::new(&temp).unwrap())
                        }
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod test_bedgraph {
        use super::*;
        
        //helper functions for checking chrom_seg and data
        fn check_segment(line: &Option<BedGraphLine>, coords: chrom_geo::ChromSeg) {
            if let Some(line) = line {
                assert_eq!(line.coords, coords);
            } else {
                panic!(format!("Last line is None: {:?}", line));
            }
        }

        fn check_data(line: &Option<BedGraphLine>, data: Option<String>) {
            if let Some(line) = line {
                assert_eq!(line.data, data);
            } else {
                panic!(format!("Last line is None: {:?}", line));
            }
        }

        #[test]
        fn line_count() {
            //create a bedgraph iterator for a test file with 9 lines
            let mut bedgraph = BedGraphIterator::new("test/unionbedg/1+2+3.bg").unwrap();
            //check the number of lines
            let count = bedgraph.count();
            assert_eq!(count, 9);
        }

        #[test]
        fn iterator_next() {
            let mut bedgraph = BedGraphIterator::new("test/unionbedg/1.bg").unwrap();
            let last_line = bedgraph.next();
            check_segment(&last_line, chrom_geo::ChromSeg{chrom: "chr1".to_string(), start: 1000, stop: 1500 } );
            check_data(&last_line, Some("10".to_string()));;
            let last_line = bedgraph.next();
            check_segment(&last_line, chrom_geo::ChromSeg{chrom: "chr1".to_string(), start: 2000, stop: 2100 } );
            check_data(&last_line, Some("20".to_string()));;
            let last_line = bedgraph.next();
            assert_eq!(last_line, None);
        }
        
        /*
        #[test]
        fn test_truncate() {
            let mut line = BedGraphLine::new("chr1 900 1600 37").unwrap();
            line.truncate(950);
            assert_eq!(line, BedGraphLine::new("chr1 950 1600 37").unwrap());
        }
        */
    }
}