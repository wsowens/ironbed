extern crate ironbed;
#[macro_use]
extern crate clap;

use clap::{Arg, App, SubCommand};

use ironbed::bedgraph::{UnionConfig, union_main};


fn main() {
    let matches = App::new("ironbed")
                          .version(crate_version!())
                          .about("An implementation of bedtools in Rust")
                          .author("William S. Owens <wowens@ufl.edu>")
                          .subcommand(SubCommand::with_name("unionbedg")
                                      .about("Combines multiple bedGraph files into a single file.")
                                      .arg(Arg::with_name("input")
                                           .short("i")
                                           .multiple(true)
                                           .required(true)
                                           .help("Input bedGraph files. Input files cannot contain overlapping intervals and should be sorted by chrom, start. (Use the command 'sort -k1,1 -k2,2n for the correct order.')")))
                          .get_matches();

    match matches.subcommand() {
        ("unionbedg", Some(ubg_matches)) => {
            //this operation is safe, because "-i" is required
            //if there wasn't a value, clap would have caught it
            //let readers = 
        }, 
        ("", None) => eprintln!("No subcommand provided. Try 'ironbed help' for available subcommands."),
        _ => unreachable!(),
    }
}