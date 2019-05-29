mod chrom_geo {
    use std::fmt;

    //this will have to all be manually implemented
    //when we have a "new_chrom" flag
    #[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
    struct ChromPos<'a> {
        chrom: &'a str,
        index: u32
    }

    //TODO: replace String with str
    #[derive(Debug, PartialEq, Eq)]
    pub struct ChromSeg {
        chrom: String,
        start: u32,
        stop:  u32
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
        fn test_start_stop() {
            let seg = ChromSeg{chrom: "chr3".to_string(), start: 100, stop: 250};
            assert_eq!(seg.start_pos(), ChromPos{chrom: "chr3", index: 100});
            assert_eq!(seg.stop_pos(), ChromPos{chrom: "chr3", index: 250});
        }

        #[test]
        fn test_sort() {
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
        fn test_min() {
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


mod bedgraph {
    use super::chrom_geo;

    #[derive(Debug, PartialEq, Eq)]
    struct BedGraphLine {
        coords: chrom_geo::ChromSeg,
        //turn the data into a str that references another field "raw line"
        data: Option<String>,
    }
}

fn main() {

}