extern crate mergebg;
use mergebg::bedgraph::BgIterator;


fn main() {
    let readers: Vec<BgIterator> = std::env::args().skip(1).map(
        | x | BgIterator::new(&x).unwrap()
    ).collect();
    let union = mergebg::bedgraph::BgUnion::new(readers).unwrap();
    for line in union {
        println!("{}", line);
    }
}