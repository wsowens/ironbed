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

pub mod chrom_sizes {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct ChromSizes {
        filename: String,
        sizes: HashMap<String, u32>,
    }

    impl ChromSizes {
        pub fn new(filename: &str) -> Result<ChromSizes, String> {
            match File::open(filename) {
                Err(msg) => Err(format!("Error with '{}': {}", filename, msg)),
                Ok(handle) => {
                    let mut handle = BufReader::new(handle);
                    let mut sizes: HashMap<String, u32> = HashMap::new();
                    let mut lineno = 0;
                    for line in handle.lines() {
                        lineno += 1;
                        match line {
                            Err(msg) => return Err(format!("Error with '{}': {}", filename, msg)),
                            Ok(line) => {
                                let cols: Vec<&str> = line.split_whitespace().collect();
                                match cols.len() {
                                    2 => {
                                        match cols[1].parse() {
                                            Ok(size) => sizes.insert(cols[0].to_string(), size),
                                            Err(_) => return Err(format!("Error in '{}', line {}: expected unsigned integer, received '{}'", filename, lineno, cols[1])),
                                        };
                                    },
                                    _ => {
                                        return Err(format!("Error in '{}', line {}: expected exactly 2 fields, received {}", filename, lineno, cols.len()))
                                    }
                                }
                            }
                        }
                        
                    }
                    Ok(ChromSizes{filename: filename.to_string(), sizes})
                }
            }
        }
    }

    #[cfg(test)]
    mod test_chrom_sizes {
        use super::*;

        #[test]
        fn test_hg38_chrom_sizes() {
            let hg38 = ChromSizes::new("test/chrom.sizes/hg38.chrom.sizes").unwrap();
            let pairs: Vec<(&str, u32)> = vec![("chr1", 248956422),
                                               ("chr2", 242193529),
                                               ("chrX", 156040895),
                                               ("chr19", 58617616),
                                               ("chr19_GL383573v1_alt", 385657),
                                               ("chrUn_KI270580v1", 1553),
                                               ("chrUn_KI270394v1", 970)];
            for (chrom, size) in pairs {
                assert_eq!(hg38.sizes.get(chrom).unwrap(), &size);
            }
            
        }

        #[test]
        fn test_tair10_chrom_sizes() {
            let tair10 = ChromSizes::new("test/chrom.sizes/tair10.chrom.sizes").unwrap();
            let pairs: Vec<(&str, u32)> = vec![("Chr1", 30427671),
                                               ("Chr2", 19698289),
                                               ("Chr3", 23459830),
                                               ("Chr4", 18585056),
                                               ("Chr5", 26975502),
                                               ("ChrC", 154478),
                                               ("ChrM", 366924),];
            for (chrom, size) in pairs {
                assert_eq!(tair10.sizes.get(chrom).unwrap(), &size);
            }
        }

        #[test]
        fn chrom_sizes_not_exist() {
            let expect = String::from("Error with 'test/chrom.sizes/does_not_exist': No such file or directory (os error 2)");
            if let Err(msg) = ChromSizes::new("test/chrom.sizes/does_not_exist") {
                assert_eq!(msg, expect)
            } else {
                panic!("Expected Err from ChromSizes::new(), received Ok(_) instead");
            }
        }

        #[test]
        fn test_badfield1_chrom_sizes() {
            let expect = String::from("Error in 'test/chrom.sizes/bad_field1.chrom.sizes', line 6: expected exactly 2 fields, received 1");
            if let Err(msg) = ChromSizes::new("test/chrom.sizes/bad_field1.chrom.sizes") {
                assert_eq!(msg, expect)
            } else {
                panic!("Expected Err from ChromSizes::new(), received Ok(_) instead");
            }
        }

        #[test]
        fn test_badfield2_chrom_sizes() {
            let expect = String::from("Error in 'test/chrom.sizes/bad_field2.chrom.sizes', line 3: expected exactly 2 fields, received 3");
            if let Err(msg) = ChromSizes::new("test/chrom.sizes/bad_field2.chrom.sizes") {
                assert_eq!(msg, expect)
            } else {
                panic!("Expected Err from ChromSizes::new(), received Ok(_) instead");
            }
        }

        #[test]
        fn test_badsize1_chrom_sizes() {
            let expect = String::from("Error in 'test/chrom.sizes/bad_size1.chrom.sizes', line 2: expected unsigned integer, received '-19698289'");
            if let Err(msg) = ChromSizes::new("test/chrom.sizes/bad_size1.chrom.sizes") {
                assert_eq!(msg, expect)
            } else {
                panic!("Expected Err from ChromSizes::new(), received Ok(_) instead");
            }
        }

        #[test]
        fn test_badsize2_chrom_sizes() {
            let expect = String::from("Error in 'test/chrom.sizes/bad_size2.chrom.sizes', line 4: expected unsigned integer, received 'apple'");
            if let Err(msg) = ChromSizes::new("test/chrom.sizes/bad_size2.chrom.sizes") {
                assert_eq!(msg, expect)
            } else {
                panic!("Expected Err from ChromSizes::new(), received Ok(_) instead");
            }
        }
    }
}

pub mod bedgraph {
    use super::chrom_geo;
    use std::fmt;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Debug, PartialEq, Eq)]
    pub struct BgLine {
        pub coords: chrom_geo::ChromSeg,
        //TODO: turn the data into a str that references another field "raw line"
        pub data: Option<String>,
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
        pub fn starts_after(&self, pos: &chrom_geo::ChromPos) -> bool {
            self.coords.start_pos() > *pos 
        }

        pub fn ends_before(&self, pos: &chrom_geo::ChromPos) -> bool {
            self.coords.stop_pos() <= *pos 
        }
    }

    impl fmt::Display for BgLine {
        
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.data {
                Some(ref data) =>  
                    write!(f, "{}\t{}\t{}\t{}", self.coords.chrom, self.coords.start, self.coords.stop, data),
                None =>
                    write!(f, "{}\t{}\t{}", self.coords.chrom, self.coords.start, self.coords.stop)
            }
            
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
        fn starts_after() {
            let bg = BgIterator::new("test/unionbedg/1+2+3.bg").unwrap();
            let pos = chrom_geo::ChromPos{chrom: "chr1".to_string(), index: 1980};
            let test_values = bg.map(|x| x.starts_after(&pos)).collect::<Vec<bool>>();;
            let expected_values = vec![false, false, false, false, false, true, true, true, true];
            assert_eq!(test_values, expected_values);
            let bg = BgIterator::new("test/unionbedg/long.bg").unwrap();
            let pos = chrom_geo::ChromPos{chrom: "chr2".to_string(), index: 4000};
            let test_values = bg.map(|x| x.starts_after(&pos)).collect::<Vec<bool>>();
            let expected_values = vec![false, false, false, false, true, true, true, true];
            assert_eq!(test_values, expected_values);
        }

        #[test]
        fn ends_before() {
            let bg = BgIterator::new("test/unionbedg/1+2+3.bg").unwrap();
            let pos = chrom_geo::ChromPos{chrom: "chr1".to_string(), index: 1980};
            let test_values = bg.map(|x| x.ends_before(&pos)).collect::<Vec<bool>>();;
            let expected_values = vec![true, true, true, true, false, false, false, false, false];
            assert_eq!(test_values, expected_values);
            let bg = BgIterator::new("test/unionbedg/long.bg").unwrap();
            let pos = chrom_geo::ChromPos{chrom: "chr2".to_string(), index: 4000};
            let test_values = bg.map(|x| x.ends_before(&pos)).collect::<Vec<bool>>();
            let expected_values = vec![true, true, true, true, false, false, false, false];
            assert_eq!(test_values, expected_values);
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
    }
}

pub mod union {
    use super::chrom_geo;
    use super::bedgraph::{BgIterator, BgLine};
    use super::chrom_sizes::ChromSizes;

    //Each reader can have three states:
    // In - the current position of the Union interesects with the Reader at BgLine
    // Out - the current position of the Union does NOT intersect with the Reader at BgLine
    // Done - the reader has nothing left
    enum UnionLine {
        In(BgLine),
        Out(BgLine),
        Done
    }

    #[derive(Debug)]
    pub struct UnionConfig<'a> {
        pub report_empty: bool,
        pub filler: &'a str,
        pub genome: Option<ChromSizes>,
    }

    pub struct BgUnion<'a> {
        readers: Vec<BgIterator>,
        lines: Vec<UnionLine>,
        curr: chrom_geo::ChromPos,
        config: UnionConfig<'a>,
    }

    impl<'a> BgUnion<'a> {
        pub fn new(readers: Vec<BgIterator>) -> Result<BgUnion<'static>, &'static str> {
            // simply call the "with_config" method using the default config below
            let config = UnionConfig{report_empty: false, filler: "0", genome: None};
            BgUnion::with_config(readers, config)
        }
        
        pub fn with_config(mut readers: Vec<BgIterator>, config: UnionConfig) -> Result<BgUnion, &'static str> {
            let mut lines: Vec<UnionLine> = Vec::with_capacity(readers.len());
            for rdr in readers.iter_mut() {
                match rdr.next() {
                    Some(bgl) => lines.push(UnionLine::Out(bgl)),
                    None => lines.push(UnionLine::Done),
                }
            }
            //TODO: add logic to find the first site
            //remove this hard coding
            let curr = chrom_geo::ChromPos{chrom: "chr1".to_string(), index: 0};
            Ok( BgUnion{readers, lines, curr, config} )
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

        fn advance_lines(&mut self, curr: &chrom_geo::ChromPos) {
            self.lines = std::mem::replace(&mut self.lines, vec![]).into_iter().zip(self.readers.iter_mut()).map(| (old_line, reader) | {
                match old_line {
                    UnionLine::Done => UnionLine::Done,
                    UnionLine::Out(line_data) => {
                        // here we assume that if a line is "outside"
                        // then we either haven't reached it yet, or have, and it is in
                        if line_data.starts_after(curr) {
                            UnionLine::Out(line_data)
                        } else {
                            // line has been reached!
                            UnionLine::In(line_data)
                        }
                    },
                    UnionLine::In(line_data) => {
                        // here we assume that the line has been reached
                        // thus we will either remain inside it, or move on and get a new line
                        if line_data.ends_before(curr) {
                            match reader.next() {
                                Some(new_line) => {
                                    if new_line.starts_after(curr) {
                                        UnionLine::Out(new_line)
                                    } else {
                                    // line has been reached!
                                        UnionLine::In(new_line)
                                    }
                                },
                                //reader has nothing left
                                None => UnionLine::Done,
                            }
                        } else {
                            UnionLine::In(line_data)
                        }
                    },
                }
            }).collect();
        }
    }

    impl<'a> Iterator for BgUnion<'a> {
        type Item = BgLine;

        fn next(&mut self) -> Option<Self::Item> {
            //make this a flag that gets updated automatically
            if self.lines.iter().all(| x | match x { UnionLine::Done => true, _ => false}) {
                return None;
            }
            let next_trans = self.next_transition();
            let has_in = self.lines.iter().any(| x | match x { UnionLine::In(_) => true, _ => false});
            //prep the data... do this in a better way if possible
            let formatted_data: String = self.lines.iter().map(| x | {
                match x { 
                    UnionLine::In(ref line) => {
                        match line.data {
                            Some(ref line) => line,
                            None => self.config.filler,
                        }
                    },
                    _ => self.config.filler,
                }}).collect::<Vec<&str>>().join("\t");
            //advance all the readers / lines based on the next transition
            self.advance_lines(&next_trans);
            //swap "curr" with the new transition
            //next_trans is now the 
            let old_trans = std::mem::replace(&mut self.curr, next_trans);
            let coords = chrom_geo::ChromSeg{
                    chrom: old_trans.chrom,
                    start: old_trans.index,
                    stop: self.curr.index
            };
            let line = BgLine{coords, data: Some(formatted_data)};
            // check there are any bedgraph data for this region
            if has_in {
                //yield the line
                Some(line)
            } else {
                // otherwise, check if we should yield this
                // line or not, depending on the settings
                if self.config.report_empty {
                    Some(line)
                } else {
                   self.next() 
                }
            }
        }
    }


    pub fn union_main(filenames: Vec<&str>, filler: &str, report_empty: bool, genome_file: Option<&str>) -> Result<(), String> {
        // open the bedgraph files
        let mut bg_iters: Vec<BgIterator> = Vec::with_capacity(filenames.len());
        for fname in filenames {
            match BgIterator::new(fname) {
                Ok(bg_iter) => bg_iters.push(bg_iter),
                Err(e) => return Err(format!("{}: '{}'", e, fname)),
            }
        }
        //prepare the config
        let genome = match genome_file {
            None => None,
            Some(fname) => Some(ChromSizes::new(fname)?),
        };
        let config = UnionConfig{filler, report_empty, genome};
        let union = BgUnion::with_config(bg_iters, config)?;
        for line in union {
            println!("{}", line);
        }
        Ok(())
    }

    
    #[cfg(test)]
    mod test_union {
        use super::*;

        #[test]
        fn union_defaults() {
            //gather the correct inputs into a union
            let inputs: Vec<BgIterator> = vec!["test/unionbedg/1.bg", 
                                               "test/unionbedg/2.bg",
                                                "test/unionbedg/3.bg"].iter()
                                                                      .map(|name| BgIterator::new(name).unwrap())
                                                                      .collect();
            let union = BgUnion::new(inputs).unwrap();
            let expected_iterator = BgIterator::new("test/unionbedg/1+2+3.bg").unwrap();
            for (actual, expected) in union.zip(expected_iterator) {
                assert_eq!(actual, expected);
            }
        }
        
        #[test]
        fn union_defaults2() {
            //gather the correct inputs into a union
            let inputs: Vec<BgIterator> = vec!["test/unionbedg/empty-1.bg",
                                               "test/unionbedg/empty-2.bg"].iter()
                                                            .map(|name| BgIterator::new(name).unwrap())
                                                            .collect();
            let union = BgUnion::new(inputs).unwrap();
            let expected_iterator = BgIterator::new("test/unionbedg/empty-1+2.bg").unwrap();
            for (actual, expected) in union.zip(expected_iterator) {
                assert_eq!(actual, expected);
            }
        }

        #[test]
        fn union_filler1() {
            let inputs: Vec<BgIterator> = vec!["test/unionbedg/1.bg", 
                                               "test/unionbedg/2.bg",
                                                "test/unionbedg/3.bg"].iter()
                                                                      .map(|name| BgIterator::new(name).unwrap())
                                                                      .collect();
            let config = UnionConfig{report_empty: false, filler: "NA", genome: None};
            let union = BgUnion::with_config(inputs, config).unwrap();
            let expected_iterator = BgIterator::new("test/unionbedg/1+2+3.NA-filling.bg").unwrap();
            for (actual, expected) in union.zip(expected_iterator) {
                assert_eq!(actual, expected);
            }
        }
        
        #[test]
        fn union_filler2() {
            //gather the correct inputs into a union
            let inputs: Vec<BgIterator> = vec!["test/unionbedg/empty-1.bg",
                                               "test/unionbedg/empty-2.bg"].iter()
                                                            .map(|name| BgIterator::new(name).unwrap())
                                                            .collect();
            let config = UnionConfig{report_empty: false, filler: "apple", genome: None};
            let union = BgUnion::with_config(inputs, config).unwrap();
            let expected_iterator = BgIterator::new("test/unionbedg/empty-1+2.apple-filling.bg").unwrap();
            for (actual, expected) in union.zip(expected_iterator) {
                assert_eq!(actual, expected);
            }
        }
    }
}