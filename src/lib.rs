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
    fn truncate_after(line: BedGraphLine, split_pt: chrom_geo::ChromPos) -> Option<BedGraphLine> {
        let coords = &line.coords;
        if coords.stop_pos() > split_pt  {
            Some(chrom_geo::ChromSeg{
                BedGraphLine
            })
        }
        None
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

    enum UnionResult {
        New(BedGraphLine),
        Old,
        Done,
    }

    pub fn union(mut readers: Vec<BedGraphIterator>, filler_str: &str) {
        use self::UnionResult::*;
        
        //create a list of expected results, set them all to "Old"
        let mut lines: Vec<UnionResult> = Vec::with_capacity(readers.len());
        for _ in 0..readers.len() {
            lines.push(UnionResult::Old)
        }
        let mut all_done = false;
        while !all_done {
            //assume that we are done processing
            all_done = true;

            //iterate through and retrieve new lines as necessary
            let read_iter = readers.iter_mut();
            lines = lines.into_iter()
                             .zip(read_iter)
                             .map(
                                 | (l, r) | {
                                     match l {
                                         New(line) => {
                                             all_done = false;
                                             New(line)
                                         },
                                         Old => {
                                             match r.next() {
                                                 None => Done,
                                                 Some(line) => {
                                                     all_done = false;
                                                     New(line)
                                                 }
                                             }
                                         }
                                         Done => Done,
                                     }
                                 }
                             ).collect();
            
            //get the minimum start and minimum stop of each line
            //find some way of cloning this
            let min_start: chrom_geo::ChromPos = lines.iter()
                                                     .filter_map(
                                                         |r| if let New(line) = r {
                                                             Some(line.coords.start_pos())
                                                         } else {
                                                             None
                                                         }
                                                     )
                                                     .min().unwrap();
            
            let min_stop: chrom_geo::ChromPos = lines.iter()
                                                     .filter_map(
                                                         |r| if let New(line) = r {
                                                             Some(line.coords.stop_pos())
                                                         } else {
                                                             None
                                                         }
                                                     )
                                                     .min().unwrap();
            
            //https://www.reddit.com/r/rust/comments/6q4uqc/help_whats_the_best_way_to_join_an_iterator_of/
            let data = lines.iter()
                            .map(|x| {
                                match x {
                                    New(line) => {
                                        if line.starts_before(&min_stop) {
                                            //reference to the line's data
                                            match line.data {
                                                Some(ref value) => value,
                                                None => filler_str,
                                            }
                                        } else {
                                            filler_str
                                        }
                                    },
                                    _ => filler_str
                                }
                            })
                            .collect::<Vec<&str>>()
                            .join("\t");
            //TODO: refactor this format line
            println!("{}\t{}\t{}\t{}", min_start.chrom, min_start.index, min_stop.index, data);
            lines = lines.into_iter()
                          .map(|x| {
                              match x {
                                  Done => Done,
                                  Old => Old,
                                  New(line) => {

                                  }
                              }
                          })
                          .collect();
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
            let bedgraph = BedGraphIterator::new("test/unionbedg/1+2+3.bg").unwrap();
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
        
        #[test]
        fn min_iterators() {
            let bedgraph1 = BedGraphIterator::new("test/unionbedg/1.bg").unwrap();
            let bedgraph2 = BedGraphIterator::new("test/unionbedg/2.bg").unwrap();
            let bedgraph3 = BedGraphIterator::new("test/unionbedg/3.bg").unwrap();
            let mut readers: Vec<BedGraphIterator> = vec![bedgraph1, bedgraph2, bedgraph3];
            let lines: Vec<BedGraphLine> = readers.iter_mut().map(|x| x.next().unwrap()).collect();
            let min_start: chrom_geo::ChromPos = lines.iter().map(|x| x.coords.start_pos()).min().unwrap();
            assert_eq!(min_start, chrom_geo::ChromPos{chrom: "chr1".to_string(), index: 900});
            let min_start: chrom_geo::ChromPos = lines.iter().map(|x| x.coords.stop_pos()).min().unwrap();
            assert_eq!(min_start, chrom_geo::ChromPos{chrom: "chr1".to_string(), index: 1500});
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